// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// GUARDIAN — над-доменный контроль соблюдения CODEX + GENOME правил

use axiom_config::DomainConfig;
use axiom_core::{Token, STATE_LOCKED};
use axiom_domain::DomainState;
use axiom_genome::{Genome, GenomeIndex, ModuleId, Permission, ResourceId};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Публичные типы
// ============================================================================

/// Причина вето рефлекса.
#[derive(Debug, Clone, PartialEq)]
pub enum VetoReason {
    /// Токен заблокирован (STATE_LOCKED)
    TokenLocked,
    /// Токен с валентностью не имеет массы
    ValenceWithoutMass,
    /// Нулевой sutra_id — недопустимый токен
    ZeroSutraId,
    /// Запрещено GENOME (Arbiter не имеет доступа Execute на AshtiField)
    GenomeDenied,
    /// Попытка записать Frame-анкер в SUTRA вне состояния DREAMING.
    /// Инвариант: SUTRA-промоция только через DreamCycle.
    SutraFrameWriteOutsideDream,
}

/// Решение GUARDIAN по рефлексу.
#[derive(Debug, Clone, PartialEq)]
pub enum ReflexDecision {
    /// Рефлекс разрешён
    Allow,
    /// Рефлекс заблокирован с указанием причины
    Veto(VetoReason),
}

impl ReflexDecision {
    /// Возвращает `true` если рефлекс разрешён
    pub fn is_allowed(&self) -> bool {
        matches!(self, ReflexDecision::Allow)
    }
}

/// Причина ингибирования домена.
#[derive(Debug, Clone, PartialEq)]
pub enum InhibitReason {
    /// Токен с валентностью не имеет массы
    ValenceWithoutMass {
        /// Индекс токена в списке домена
        token_index: usize,
    },
}

/// Действие ингибирования для домена.
#[derive(Debug, Clone, PartialEq)]
pub struct InhibitAction {
    /// Причина ингибирования
    pub reason: InhibitReason,
}

/// Действия над CODEX.
#[derive(Debug, Clone)]
pub enum CodexAction {
    /// Добавить новый токен-правило в CODEX-домен
    AddRule(Token),
    /// Сбросить все накопленные нарушения
    ResetViolations,
}

/// Ошибки операций GUARDIAN.
#[derive(Debug, Clone, PartialEq)]
pub enum GuardianError {
    /// Недостаточно прав по GENOME
    AccessDenied,
    /// CODEX-домен заполнен
    CodexFull,
}

impl std::fmt::Display for GuardianError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardianError::AccessDenied => write!(f, "Guardian: access denied by GENOME"),
            GuardianError::CodexFull => write!(f, "Guardian: CODEX domain is full"),
        }
    }
}

/// Статистика рефлексов по роли домена — входные данные для adapt_thresholds.
#[derive(Debug, Clone)]
pub struct RoleStats {
    /// Структурная роль домена (1..8 = ASHTI)
    pub role: u8,
    /// Общая доля успехов [0.0 .. 1.0]
    pub success_rate: f32,
    /// Общее число попыток
    pub total_calls: u32,
}

/// Статистика GUARDIAN.
#[derive(Debug, Default, Clone)]
pub struct GuardianStats {
    /// Число разрешённых рефлексов
    pub reflex_allowed: u64,
    /// Число заблокированных рефлексов
    pub reflex_vetoed: u64,
    /// Число отказов доступа по GENOME
    pub access_denied: u64,
    /// Число нарушений протокола
    pub protocol_denied: u64,
    /// Число просканированных доменов
    pub domains_scanned: u64,
    /// Число адаптаций порогов (Этап 6)
    pub thresholds_adapted: u64,
    /// Число DREAM-предложений (Этап 6)
    pub dream_proposals: u64,
}

// ============================================================================
// GuardianConfig
// ============================================================================

/// Параметры агрессивности адаптации Guardian.
///
/// Управляет скоростью обучения модели: насколько быстро пороги Arbiter и
/// физика домена меняются в ответ на feedback успеха/неудачи.
///
/// Загружается из `axiom-cli.yaml` (секция `guardian`). При отсутствии —
/// используются значения Default.
#[derive(Debug, Clone)]
pub struct GuardianConfig {
    /// success_rate выше которого reflex_threshold снижается (default: 0.8)
    pub high_success_threshold: f32,
    /// success_rate ниже которого reflex_threshold повышается (default: 0.3)
    pub low_success_threshold: f32,
    /// success_rate выше которого temperature снижается (default: 0.7)
    pub physics_high_threshold: f32,
    /// Δ reflex_threshold за цикл адаптации (default: 5)
    pub threshold_step: u8,
    /// Δ temperature за цикл адаптации, Кельвин (default: 5.0)
    pub temp_step: f32,
    /// Нижний предел temperature после адаптации (default: 0.1)
    pub temp_min: f32,
    /// Верхний предел temperature после адаптации (default: 500.0)
    pub temp_max: f32,
    /// Δ resonance_freq за цикл адаптации (default: 10)
    pub resonance_step: u16,
    /// Верхняя граница валидного ML-confidence (default: 0.99)
    pub confidence_ceiling: f32,
}

impl Default for GuardianConfig {
    fn default() -> Self {
        Self {
            high_success_threshold: 0.8,
            low_success_threshold: 0.3,
            physics_high_threshold: 0.7,
            threshold_step: 5,
            temp_step: 5.0,
            temp_min: 0.1,
            temp_max: 500.0,
            resonance_step: 10,
            confidence_ceiling: 0.99,
        }
    }
}

// ============================================================================
// Guardian
// ============================================================================

/// GUARDIAN — проверяет рефлексы и домены на соответствие CODEX + GENOME.
///
/// Два источника правил:
/// - GENOME (абсолютные, неизменяемые конституционные ограничения)
/// - CODEX  (пластичные правила в `DomainState`)
pub struct Guardian {
    genome: Arc<Genome>,
    genome_index: GenomeIndex,
    stats: GuardianStats,
    violation_count: u32,
}

impl Guardian {
    /// Создать Guardian с указанным Genome.
    pub fn new(genome: Arc<Genome>) -> Self {
        let genome_index = GenomeIndex::build(&genome);
        Self {
            genome,
            genome_index,
            stats: GuardianStats::default(),
            violation_count: 0,
        }
    }

    /// Создать Guardian с захардкоженным Ashti_Core Genome (удобный конструктор).
    pub fn with_default_genome() -> Self {
        Self::new(Arc::new(Genome::default_ashti_core()))
    }

    // ============================================================
    // GENOME enforcement — O(1) через GenomeIndex
    // ============================================================

    /// Проверить права доступа модуля к ресурсу по GENOME.
    pub fn enforce_access(
        &mut self,
        module: ModuleId,
        resource: ResourceId,
        operation: Permission,
    ) -> bool {
        let allowed = self.genome_index.check_access(module, resource, operation);
        if !allowed {
            self.stats.access_denied += 1;
            self.violation_count += 1;
        }
        allowed
    }

    /// Проверить разрешённость маршрута source→target по GENOME.
    pub fn enforce_protocol(&mut self, source: ModuleId, target: ModuleId) -> bool {
        let allowed = self.genome_index.check_protocol(source, target);
        if !allowed {
            self.stats.protocol_denied += 1;
            self.violation_count += 1;
        }
        allowed
    }

    // ============================================================
    // DREAM Phase state guards
    // ============================================================

    /// Проверить допустимость записи Frame-анкера в SUTRA.
    ///
    /// Инвариант: FRAME_ANCHOR-записи в SUTRA (domain 100 для level 1) допустимы
    /// только в состоянии DREAMING. Обычные токены в SUTRA не ограничены.
    ///
    /// Возвращает `Some(VetoReason)` если запись должна быть заблокирована.
    pub fn check_frame_anchor_sutra_write(
        &self,
        cmd_flags: u8,
        target_domain_id: u16,
        dream_state: crate::over_domain::DreamPhaseState,
    ) -> Option<VetoReason> {
        use crate::over_domain::DreamPhaseState;
        use axiom_ucl::flags::FRAME_ANCHOR;

        // Проверяем только FRAME_ANCHOR-записи в SUTRA (domain_id % 100 == 0)
        if cmd_flags & FRAME_ANCHOR == 0 {
            return None;
        }
        if !target_domain_id.is_multiple_of(100) {
            return None;
        }
        if dream_state == DreamPhaseState::Dreaming {
            return None;
        }

        Some(VetoReason::SutraFrameWriteOutsideDream)
    }

    // ============================================================
    // Reflex validation (CODEX + GENOME)
    // ============================================================

    /// Проверить рефлексный токен на соответствие CODEX + GENOME.
    pub fn validate_reflex(&mut self, token: &Token) -> ReflexDecision {
        // GENOME: Arbiter должен иметь Execute на AshtiField для отправки рефлекса
        if !self.genome_index.check_access(
            ModuleId::Arbiter,
            ResourceId::AshtiField,
            Permission::Execute,
        ) {
            self.violation_count += 1;
            self.stats.reflex_vetoed += 1;
            return ReflexDecision::Veto(VetoReason::GenomeDenied);
        }

        // CODEX правило 1: заблокированный токен не может порождать рефлекс
        if token.state == STATE_LOCKED {
            self.violation_count += 1;
            self.stats.reflex_vetoed += 1;
            return ReflexDecision::Veto(VetoReason::TokenLocked);
        }

        // CODEX правило 2: токен с валентностью должен иметь массу
        if token.valence != 0 && token.mass == 0 {
            self.violation_count += 1;
            self.stats.reflex_vetoed += 1;
            return ReflexDecision::Veto(VetoReason::ValenceWithoutMass);
        }

        // CODEX правило 3: нулевой sutra_id — недопустимый токен
        if token.sutra_id == 0 {
            self.violation_count += 1;
            self.stats.reflex_vetoed += 1;
            return ReflexDecision::Veto(VetoReason::ZeroSutraId);
        }

        self.stats.reflex_allowed += 1;
        ReflexDecision::Allow
    }

    // ============================================================
    // Domain scan
    // ============================================================

    /// Сканировать состояние домена на нарушения CODEX.
    ///
    /// Возвращает список действий ингибирования (пустой = чисто).
    pub fn scan_domain(&mut self, state: &DomainState) -> Vec<InhibitAction> {
        self.stats.domains_scanned += 1;
        let mut actions = Vec::new();

        for (i, token) in state.tokens.iter().enumerate() {
            if token.valence != 0 && token.mass == 0 {
                self.violation_count += 1;
                actions.push(InhibitAction {
                    reason: InhibitReason::ValenceWithoutMass { token_index: i },
                });
            }
        }

        actions
    }

    // ============================================================
    // CODEX management
    // ============================================================

    /// Применить действие к CODEX-домену.
    pub fn update_codex(
        &mut self,
        codex_domain: &mut DomainState,
        action: CodexAction,
    ) -> Result<(), GuardianError> {
        // GENOME: Guardian должен иметь ReadWrite на CodexRules
        if !self.genome_index.check_access(
            ModuleId::Guardian,
            ResourceId::CodexRules,
            Permission::ReadWrite,
        ) {
            return Err(GuardianError::AccessDenied);
        }

        match action {
            CodexAction::AddRule(token) => {
                codex_domain
                    .add_token(token)
                    .map_err(|_| GuardianError::CodexFull)?;
            }
            CodexAction::ResetViolations => {
                self.violation_count = 0;
            }
        }
        Ok(())
    }

    // ============================================================
    // Этап 6: Адаптивные пороги
    // ============================================================

    /// Адаптировать reflex_threshold доменов на основе статистики REFLECTOR.
    ///
    /// Пороги управляются `GuardianConfig`:
    /// - success_rate > high_threshold && calls ≥ 10 → снизить на threshold_step
    /// - success_rate < low_threshold  && calls ≥ 10 → повысить на threshold_step
    ///
    /// Возвращает список domain_id у которых изменился порог.
    pub fn adapt_thresholds(
        &mut self,
        stats: &[RoleStats],
        configs: &mut HashMap<u16, DomainConfig>,
        cfg: &GuardianConfig,
    ) -> Vec<u16> {
        let mut updated = Vec::new();

        for (domain_id, config) in configs.iter_mut() {
            let role = config.structural_role;
            let Some(role_stat) = stats.iter().find(|s| s.role == role) else {
                continue;
            };

            if role_stat.total_calls < 10 {
                continue;
            }

            let changed = if role_stat.success_rate > cfg.high_success_threshold
                && config.reflex_threshold > cfg.threshold_step
            {
                config.reflex_threshold =
                    config.reflex_threshold.saturating_sub(cfg.threshold_step);
                true
            } else if role_stat.success_rate < cfg.low_success_threshold
                && config.reflex_threshold < 255 - cfg.threshold_step
            {
                config.reflex_threshold =
                    config.reflex_threshold.saturating_add(cfg.threshold_step);
                true
            } else {
                false
            };

            if changed {
                self.stats.thresholds_adapted += 1;
                updated.push(*domain_id);
            }
        }

        updated
    }

    /// Адаптировать физические параметры доменов (temperature, resonance_freq).
    ///
    /// Параметры управляются `GuardianConfig`:
    /// - success_rate > physics_high_threshold → охладить и ускорить резонанс
    /// - success_rate < low_success_threshold  → нагреть и замедлить резонанс
    ///
    /// Возвращает список domain_id у которых изменились параметры.
    pub fn adapt_domain_physics(
        &mut self,
        stats: &[RoleStats],
        configs: &mut HashMap<u16, DomainConfig>,
        cfg: &GuardianConfig,
    ) -> Vec<u16> {
        let mut updated = Vec::new();

        for (domain_id, config) in configs.iter_mut() {
            let role = config.structural_role;
            let Some(role_stat) = stats.iter().find(|s| s.role == role) else {
                continue;
            };

            if role_stat.total_calls < 10 {
                continue;
            }

            let changed = if role_stat.success_rate > cfg.physics_high_threshold {
                config.temperature = (config.temperature - cfg.temp_step).max(cfg.temp_min);
                config.resonance_freq = config.resonance_freq.saturating_add(cfg.resonance_step);
                true
            } else if role_stat.success_rate < cfg.low_success_threshold {
                config.temperature = (config.temperature + cfg.temp_step).min(cfg.temp_max);
                config.resonance_freq = config.resonance_freq.saturating_sub(cfg.resonance_step);
                true
            } else {
                false
            };

            if changed {
                self.stats.thresholds_adapted += 1;
                updated.push(*domain_id);
            }
        }

        updated
    }

    /// DREAM(7) — предложить изменения CODEX на основе высокоактивных паттернов.
    ///
    /// Принимает срез кандидатов (токенов с высоким весом из EXPERIENCE).
    /// Возвращает CodexAction::AddRule для каждого кандидата (до 5 за вызов).
    pub fn dream_propose(&mut self, candidates: &[Token]) -> Vec<CodexAction> {
        let proposals: Vec<CodexAction> = candidates
            .iter()
            .take(5)
            .map(|t| CodexAction::AddRule(*t))
            .collect();
        self.stats.dream_proposals += proposals.len() as u64;
        proposals
    }

    // ============================================================
    // Accessors
    // ============================================================

    /// Текущее число накопленных нарушений CODEX.
    pub fn violation_count(&self) -> u32 {
        self.violation_count
    }

    /// Сбросить счётчик нарушений.
    pub fn reset_violations(&mut self) {
        self.violation_count = 0;
    }

    /// Статистика операций GUARDIAN.
    pub fn stats(&self) -> &GuardianStats {
        &self.stats
    }

    /// Ссылка на активный Genome.
    pub fn genome(&self) -> &Genome {
        &self.genome
    }

    /// Валидировать уверенность ML-модели с явным ceiling из GuardianConfig.
    ///
    /// Возвращает `false` если:
    /// - `confidence < threshold` — слишком низкая уверенность
    /// - `confidence > cfg.confidence_ceiling` — аномально высокая (adversarial defense)
    pub fn validate_ml_confidence_cfg(
        confidence: f32,
        threshold: f32,
        cfg: &GuardianConfig,
    ) -> bool {
        confidence >= threshold && confidence <= cfg.confidence_ceiling
    }

    /// Валидировать уверенность ML-модели (legacy, ceiling = 0.99).
    pub fn validate_ml_confidence(confidence: f32, threshold: f32) -> bool {
        confidence >= threshold && confidence <= 0.99
    }

    /// Валидировать выход ML-модели как вектор уверенностей.
    ///
    /// Все элементы должны пройти `validate_ml_confidence`.
    /// Пустой вектор — невалиден (возвращает `false`).
    pub fn validate_ml_output(output: &[f32], threshold: f32) -> bool {
        !output.is_empty()
            && output
                .iter()
                .all(|&c| Self::validate_ml_confidence(c, threshold))
    }
}

impl Default for Guardian {
    fn default() -> Self {
        Self::with_default_genome()
    }
}
