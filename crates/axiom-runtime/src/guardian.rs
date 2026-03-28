// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// GUARDIAN — над-доменный контроль соблюдения CODEX + GENOME правил

use std::sync::Arc;
use axiom_core::{Token, STATE_LOCKED};
use axiom_domain::DomainState;
use axiom_genome::{Genome, GenomeIndex, ModuleId, ResourceId, Permission};

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
}

/// Решение GUARDIAN по рефлексу.
#[derive(Debug, Clone, PartialEq)]
pub enum ReflexDecision {
    Allow,
    Veto(VetoReason),
}

impl ReflexDecision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, ReflexDecision::Allow)
    }
}

/// Причина ингибирования домена.
#[derive(Debug, Clone, PartialEq)]
pub enum InhibitReason {
    /// Токен с валентностью не имеет массы
    ValenceWithoutMass { token_index: usize },
}

/// Действие ингибирования для домена.
#[derive(Debug, Clone, PartialEq)]
pub struct InhibitAction {
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
            GuardianError::CodexFull    => write!(f, "Guardian: CODEX domain is full"),
        }
    }
}

/// Статистика GUARDIAN.
#[derive(Debug, Default, Clone)]
pub struct GuardianStats {
    pub reflex_allowed:  u64,
    pub reflex_vetoed:   u64,
    pub access_denied:   u64,
    pub protocol_denied: u64,
    pub domains_scanned: u64,
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
    genome:          Arc<Genome>,
    genome_index:    GenomeIndex,
    stats:           GuardianStats,
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
        module:    ModuleId,
        resource:  ResourceId,
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
                codex_domain.add_token(token).map_err(|_| GuardianError::CodexFull)?;
            }
            CodexAction::ResetViolations => {
                self.violation_count = 0;
            }
        }
        Ok(())
    }

    // ============================================================
    // Accessors
    // ============================================================

    pub fn violation_count(&self) -> u32 {
        self.violation_count
    }

    pub fn reset_violations(&mut self) {
        self.violation_count = 0;
    }

    pub fn stats(&self) -> &GuardianStats {
        &self.stats
    }

    pub fn genome(&self) -> &Genome {
        &self.genome
    }
}

impl Default for Guardian {
    fn default() -> Self {
        Self::with_default_genome()
    }
}
