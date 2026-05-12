// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AXIOM MODULE: ARBITER V1.0 - Над-доменная маршрутизация
//
// Arbiter принимает решения о маршрутизации между быстрым путём
// (рефлекс из EXPERIENCE) и медленным путём (ASHTI 1-8).
//
// Позиция в потоке:
// SUTRA(0) → EXPERIENCE(9) → [ Arbiter ] → ASHTI(1-8) и/или MAYA(10)
//
// Связанные спецификации:
// - docs/spec/Arbiter_V1_0.md (каноническая)
// - docs/spec/Ashti_Core_v2_0.md

mod ashti_processor;
mod com;
pub mod experience;
mod gridhash;
mod maya_processor;
mod reflector;
mod skillset;

use ashti_processor::AshtiProcessor;
use axiom_config::DomainConfig;
use axiom_core::Token;
use experience::{Experience, ResonanceLevel};
use maya_processor::MayaProcessor;
use std::collections::HashMap;

// Re-export for tests and axiom-persist
pub use com::COM;
pub use experience::{
    Experience as ExperienceModule, ExperienceTrace, ResonanceLevel as ResonanceLevelEnum,
    TensionTrace,
};
pub use gridhash::{grid_hash, grid_hash_with_shell, AssociativeIndex};
pub use reflector::{DomainProfile, Reflector, ReflexStats};
pub use skillset::{Skill, SkillSet};

// ── Cognitive Depth V1.0 — 13D: Goal & Curiosity ─────────────────────────────

// Re-export из axiom-core — единственный источник истины для этого флага.
pub use axiom_core::TOKEN_FLAG_GOAL;
pub use axiom_core::TOKEN_FLAG_IMPULSE;

/// Порог веса следа при котором цель считается достигнутой.
/// Выше этого значения goal-импульсы не генерируются.
pub const GOAL_ACHIEVED_WEIGHT: f32 = 0.9;

/// Допуск по temperature при сравнении рефлекса с результатом ASHTI.
/// Токены считаются схожими если |reflex.temperature - ashti.temperature| < порог.
pub const TOKEN_COMPARE_TEMP_TOLERANCE: i16 = 10;

/// Допуск по mass при сравнении рефлекса с результатом ASHTI.
pub const TOKEN_COMPARE_MASS_TOLERANCE: i16 = 5;

/// Допуск по valence при сравнении рефлекса с результатом ASHTI.
pub const TOKEN_COMPARE_VALENCE_TOLERANCE: i16 = 2;

// ── Cognitive Depth V1.0 — 13C: Internal Impulse ─────────────────────────────

/// Источник внутреннего импульса (Cognitive Depth V1.0 — 13C).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpulseSource {
    /// Внешний сигнал от Perceptors
    External,
    /// Напряжение — незавершённая или низко-coherent обработка
    Tension,
    /// Незавершение — timeout без ответа
    Incompletion,
    /// Любопытство — следы near crystallization threshold
    Curiosity,
    /// Цель — CODEX не зафиксировал достижение цели
    Goal,
}

/// Внутренний импульс — источник + вес + паттерн.
///
/// Вес 0.0..1.0 определяет силу импульса относительно внешних сигналов.
/// Сравнивается с urgency внешнего сигнала (temperature / 255.0) с учётом
/// `internal_dominance_factor` из DomainConfig.
#[derive(Debug, Clone)]
pub struct InternalImpulse {
    /// Тип источника
    pub source: ImpulseSource,
    /// Вес импульса (0.0..=1.0)
    pub weight: f32,
    /// Паттерн для повторной обработки
    pub pattern: Token,
}

/// Результат маршрутизации токена
#[derive(Debug, Clone)]
pub struct RoutingResult {
    /// ID события для отслеживания
    pub event_id: u64,
    /// Рефлекс (fast path), если резонанс достаточно высокий
    pub reflex: Option<Token>,
    /// Результаты slow path через ASHTI 1-8
    pub slow_path: Vec<Token>,
    /// Консолидированный результат от MAYA
    pub consolidated: Option<Token>,
    /// События маршрутизации (для COM tracking)
    pub routed_events: Vec<u64>,
    /// Оценка согласованности ASHTI-результатов (0.0..=1.0).
    /// < 0.6 → система создаёт tension trace (Cognitive Depth V1.0)
    pub confidence: f32,
    /// Число выполненных проходов (1 = обычный, >1 = multi-pass)
    pub passes: u8,
}

impl RoutingResult {
    pub fn error(message: &str) -> Self {
        println!("Routing error: {}", message);
        Self {
            event_id: 0,
            reflex: None,
            slow_path: Vec::new(),
            consolidated: None,
            routed_events: Vec::new(),
            confidence: 1.0,
            passes: 0,
        }
    }
}

/// Ожидающее сравнение рефлекса с результатом ASHTI
#[derive(Debug, Clone)]
pub struct PendingComparison {
    /// Входной паттерн
    pub input_pattern: Token,
    /// Предсказание рефлекса (если было)
    pub reflex_prediction: Option<Token>,
    /// Результаты от ASHTI 1-8
    pub ashti_results: Vec<Token>,
    /// Консолидированный результат от MAYA
    pub consolidated_result: Option<Token>,
    /// Время создания (event_id)
    pub created_at: u64,
    /// Индекс следа, который сгенерировал рефлекс
    pub trace_index: Option<usize>,
}

/// Реестр доменов по их ролям
#[derive(Debug, Clone)]
pub struct DomainRegistry {
    sutra: Option<u16>,
    experience: Option<u16>,
    ashti: [Option<u16>; 8], // Indexed by role 1-8
    maya: Option<u16>,
}

impl DomainRegistry {
    fn new() -> Self {
        Self {
            sutra: None,
            experience: None,
            ashti: [None; 8],
            maya: None,
        }
    }

    fn is_complete(&self) -> bool {
        self.sutra.is_some()
            && self.experience.is_some()
            && self.maya.is_some()
            && self.ashti.iter().all(|d| d.is_some())
    }
}

/// Arbiter - над-доменный модуль маршрутизации (Arbiter V1.0)
pub struct Arbiter {
    /// Опыт и ассоциативная память
    experience: Experience,
    /// Реестр доменов
    registry: DomainRegistry,
    /// Ожидающие сравнения (публично для тестов)
    pub pending_comparisons: HashMap<u64, PendingComparison>,
    /// Ссылка на домены (для обработки)
    domains: HashMap<u16, DomainConfig>,
    /// COM для событий
    com: COM,
    /// REFLECTOR: статистика рефлексов + профили доменов
    pub reflector: Reflector,
    /// SKILLSET: кристаллизованные навыки
    pub skillset: SkillSet,
}

impl Arbiter {
    /// Создать новый Arbiter
    pub fn new(domains: HashMap<u16, DomainConfig>, com: COM) -> Self {
        Self {
            experience: Experience::new(),
            registry: DomainRegistry::new(),
            pending_comparisons: HashMap::new(),
            domains,
            com,
            reflector: Reflector::new(),
            skillset: SkillSet::new(),
        }
    }

    /// Зарегистрировать домен по structural_role
    pub fn register_domain(&mut self, role: u8, domain_id: u16) -> Result<(), String> {
        match role {
            0 => {
                self.registry.sutra = Some(domain_id);
                Ok(())
            }
            9 => {
                self.registry.experience = Some(domain_id);
                Ok(())
            }
            1..=8 => {
                self.registry.ashti[(role - 1) as usize] = Some(domain_id);
                Ok(())
            }
            10 => {
                self.registry.maya = Some(domain_id);
                Ok(())
            }
            _ => Err(format!("Invalid structural_role: {}", role)),
        }
    }

    /// Проверить что все необходимые домены зарегистрированы
    pub fn is_ready(&self) -> bool {
        self.registry.is_complete()
    }

    /// Главная функция маршрутизации
    pub fn route_token(&mut self, token: Token, source_domain: u8) -> RoutingResult {
        if !self.is_ready() {
            return RoutingResult::error("Not all domains registered");
        }

        match source_domain {
            0 => self.route_from_sutra(token),
            9 => self.route_from_experience(token),
            1..=8 => self.route_from_ashti(token, source_domain),
            10 => self.route_from_maya(token),
            _ => RoutingResult::error("Invalid source domain"),
        }
    }

    /// Параллельная маршрутизация токена (Axiom Sentinel V1.0, Фаза 2).
    ///
    /// Для domain 0 (SUTRA) использует параллельный resonance search.
    /// Для остальных доменов делегирует в `route_token` (параллелизм там не нужен).
    pub fn route_token_parallel(
        &mut self,
        token: Token,
        source_domain: u8,
        pool: &rayon::ThreadPool,
    ) -> RoutingResult {
        if !self.is_ready() {
            return RoutingResult::error("Not all domains registered");
        }
        match source_domain {
            0 => self.route_from_sutra_parallel(token, pool),
            _ => self.route_token(token, source_domain),
        }
    }

    /// SUTRA (0) → EXPERIENCE (9)
    fn route_from_sutra(&mut self, token: Token) -> RoutingResult {
        let _event_id = self.com.next_event_id(0);
        self.route_from_experience_core(token, None)
    }

    /// SUTRA (0) → EXPERIENCE (9) [параллельный, Sentinel Фаза 2]
    fn route_from_sutra_parallel(
        &mut self,
        token: Token,
        pool: &rayon::ThreadPool,
    ) -> RoutingResult {
        let _event_id = self.com.next_event_id(0);
        self.route_from_experience_core(token, Some(pool))
    }

    /// EXPERIENCE (9) → Dual Path: reflex OR (ASHTI 1-8 → MAYA) [последовательный]
    fn route_from_experience(&mut self, token: Token) -> RoutingResult {
        self.route_from_experience_core(token, None)
    }

    /// EXPERIENCE (9) → Dual Path [параллельный, Sentinel Фаза 2]
    pub fn route_from_experience_parallel(
        &mut self,
        token: Token,
        pool: &rayon::ThreadPool,
    ) -> RoutingResult {
        self.route_from_experience_core(token, Some(pool))
    }

    /// EXPERIENCE (9) → Dual Path: reflex OR (ASHTI 1-8 → MAYA) [параллельный, Sentinel Фаза 2].
    ///
    /// При `pool = Some(p)` Phase 2 resonance search выполняется параллельно через rayon.
    /// При `pool = None` — обычный последовательный поиск.
    /// Если traces < PARALLEL_THRESHOLD, pool игнорируется.
    fn route_from_experience_core(
        &mut self,
        token: Token,
        pool: Option<&rayon::ThreadPool>,
    ) -> RoutingResult {
        let event_id = self.com.next_event_id(9);

        // 0. SKILLSET: мгновенный ответ если паттерн кристаллизован
        if let Some((skill_idx, skill)) = self.skillset.find_skill_with_idx(&token) {
            let reflex_token = skill.pattern;
            self.skillset.record_activation(skill_idx);

            let ashti_results = self.route_to_ashti(token, None, 8);
            let consolidated = self.route_to_maya(ashti_results.clone());

            self.pending_comparisons.insert(
                event_id,
                PendingComparison {
                    input_pattern: token,
                    reflex_prediction: Some(reflex_token),
                    ashti_results: ashti_results.clone(),
                    consolidated_result: consolidated,
                    created_at: event_id,
                    trace_index: None,
                },
            );

            return RoutingResult {
                event_id,
                reflex: Some(reflex_token),
                slow_path: ashti_results,
                consolidated,
                routed_events: vec![event_id],
                confidence: 1.0,
                passes: 1,
            };
        }

        // 1. Резонансный поиск (последовательный или параллельный)
        let resonance = match pool {
            Some(p) => self.experience.resonance_search_parallel(&token, p),
            None => self.experience.resonance_search(&token),
        };

        // 2. Fast path (conditional) - рефлекс
        let reflex = if resonance.level == ResonanceLevel::Reflex {
            resonance.trace.as_ref().map(|t| t.pattern)
        } else {
            None
        };

        // 3. Slow path (ALWAYS) - через ASHTI 1-8
        let hint = if resonance.level == ResonanceLevel::Association {
            resonance.trace.as_ref()
        } else {
            None
        };

        let ashti_results = self.route_to_ashti(token, hint, 8);

        // 4. Консолидация через MAYA с confidence + multi-pass (Cognitive Depth 13A)
        let (max_passes, min_coherence_f) = self.maya_multipass_params();
        let (mut consolidated, mut confidence) =
            self.route_to_maya_with_confidence(ashti_results.clone());
        let mut final_ashti = ashti_results;
        let mut passes = 1u8;

        if max_passes > 1 && confidence < min_coherence_f {
            let mut cur_pat = token;
            for pass in 1..max_passes {
                if let Some(ref cons) = consolidated {
                    cur_pat.temperature = cur_pat.temperature.saturating_add(cons.temperature / 2);
                }
                let extra_ashti = self.route_to_ashti(cur_pat, None, 8);
                let (extra_cons, extra_conf) =
                    self.route_to_maya_with_confidence(extra_ashti.clone());
                if extra_conf > confidence {
                    confidence = extra_conf;
                    final_ashti = extra_ashti;
                    consolidated = extra_cons;
                }
                passes = pass + 1;
                if confidence >= min_coherence_f {
                    break;
                }
            }
        }

        // Создать tension trace если итоговый confidence ниже порога.
        // Impulse-токены (TOKEN_FLAG_IMPULSE) не создают новый tension — иначе петля.
        let is_impulse = token.type_flags & axiom_core::TOKEN_FLAG_IMPULSE != 0;
        if !is_impulse && max_passes > 0 && confidence < min_coherence_f {
            let tension_temp = ((1.0 - confidence) * 255.0) as u8;
            self.experience
                .add_tension_trace(token, tension_temp, event_id);
        }

        // 5. Сохранить для сравнения
        self.pending_comparisons.insert(
            event_id,
            PendingComparison {
                input_pattern: token,
                reflex_prediction: reflex,
                ashti_results: final_ashti.clone(),
                consolidated_result: consolidated,
                created_at: event_id,
                trace_index: None,
            },
        );

        RoutingResult {
            event_id,
            reflex,
            slow_path: final_ashti,
            consolidated,
            routed_events: vec![event_id],
            confidence,
            passes,
        }
    }

    /// Маршрутизация через ASHTI домены 1..=max_role (S5: layer priority gate).
    fn route_to_ashti(&self, token: Token, hint: Option<&ExperienceTrace>, max_role: u8) -> Vec<Token> {
        let mut results = Vec::new();

        for role in 1..=(max_role.min(8) as usize) {
            if let Some(domain_id) = self.registry.ashti[role - 1] {
                if let Some(domain) = self.domains.get(&domain_id) {
                    let result = AshtiProcessor::process_token(&token, domain, hint);
                    results.push(result);
                }
            }
        }

        results
    }

    /// Маршрутизация с ограниченным набором ролей (S5: TickBudget layer priority).
    ///
    /// Используется когда бюджет тика > 80% — пропускаем роли 4–8, выполняем 1–3.
    /// Без resonance search (уже выполнен caller-ом). Без multi-pass.
    pub fn route_token_limited(
        &mut self,
        token: Token,
        pool: Option<&rayon::ThreadPool>,
        max_role: u8,
    ) -> RoutingResult {
        let event_id = self.com.next_event_id(9);

        let resonance = match pool {
            Some(p) => self.experience.resonance_search_parallel(&token, p),
            None => self.experience.resonance_search(&token),
        };

        let reflex = if resonance.level == ResonanceLevel::Reflex {
            resonance.trace.as_ref().map(|t| t.pattern)
        } else {
            None
        };
        let hint = if resonance.level == ResonanceLevel::Association {
            resonance.trace.as_ref()
        } else {
            None
        };

        let ashti_results = self.route_to_ashti(token, hint, max_role);
        let (consolidated, confidence) = self.route_to_maya_with_confidence(ashti_results.clone());

        self.pending_comparisons.insert(
            event_id,
            PendingComparison {
                input_pattern: token,
                reflex_prediction: reflex,
                ashti_results: ashti_results.clone(),
                consolidated_result: consolidated,
                created_at: event_id,
                trace_index: None,
            },
        );

        RoutingResult {
            event_id,
            reflex,
            slow_path: ashti_results,
            consolidated,
            routed_events: vec![event_id],
            confidence,
            passes: 1,
        }
    }

    /// Консолидация результатов ASHTI через MAYA (без confidence)
    fn route_to_maya(&self, ashti_results: Vec<Token>) -> Option<Token> {
        if ashti_results.is_empty() {
            return None;
        }

        let maya_id = self.registry.maya?;
        let maya_domain = self.domains.get(&maya_id)?;

        Some(MayaProcessor::consolidate_results(
            ashti_results,
            maya_domain,
        ))
    }

    /// Консолидация результатов ASHTI через MAYA с оценкой coherence.
    fn route_to_maya_with_confidence(&self, ashti_results: Vec<Token>) -> (Option<Token>, f32) {
        if ashti_results.is_empty() {
            return (None, 1.0);
        }

        let maya_id = match self.registry.maya {
            Some(id) => id,
            None => return (None, 1.0),
        };
        let maya_domain = match self.domains.get(&maya_id) {
            Some(d) => d,
            None => return (None, 1.0),
        };

        let (token, confidence) =
            MayaProcessor::consolidate_with_confidence(ashti_results, maya_domain);
        (Some(token), confidence)
    }

    /// Multi-pass маршрутизация (Cognitive Depth V1.0 — 13A).
    ///
    /// Если MAYA возвращает confidence < min_coherence и не исчерпан лимит
    /// проходов — паттерн обогащается результатом и обрабатывается снова.
    /// При низком итоговом confidence создаётся tension trace в EXPERIENCE.
    ///
    /// `max_passes` и `min_coherence` берутся из конфига домена MAYA.
    pub fn route_with_multipass(&mut self, token: Token) -> RoutingResult {
        if !self.is_ready() {
            return RoutingResult::error("Not all domains registered");
        }

        let (max_passes, min_coherence_f) = self.maya_multipass_params();

        if max_passes == 0 {
            // Multi-pass отключён — обычная маршрутизация
            return self.route_from_experience(token);
        }

        let event_id = self.com.next_event_id(9);
        let resonance = self.experience.resonance_search(&token);
        let reflex = if resonance.level == ResonanceLevel::Reflex {
            resonance.trace.as_ref().map(|t| t.pattern)
        } else {
            None
        };
        let hint = if resonance.level == ResonanceLevel::Association {
            resonance.trace.as_ref()
        } else {
            None
        };

        let mut current_pattern = token;
        let mut final_confidence = 1.0f32;
        let mut final_ashti: Vec<Token> = Vec::new();
        let mut final_consolidated: Option<Token> = None;
        let mut passes: u8 = 0;

        for pass in 0..max_passes {
            passes = pass + 1;
            let hint_this_pass = if pass == 0 { hint } else { None };
            let ashti_results = self.route_to_ashti(current_pattern, hint_this_pass, 8);
            let (consolidated, confidence) =
                self.route_to_maya_with_confidence(ashti_results.clone());

            final_confidence = confidence;
            final_ashti = ashti_results;
            final_consolidated = consolidated;

            if confidence >= min_coherence_f || pass + 1 >= max_passes {
                break;
            }

            // Обогащаем паттерн для следующего прохода
            if let Some(ref cons) = final_consolidated {
                current_pattern.temperature = current_pattern
                    .temperature
                    .saturating_add(cons.temperature / 2);
            }
        }

        // Если итоговый confidence низкий — создаём tension trace
        if final_confidence < min_coherence_f {
            let tension_temp = ((1.0 - final_confidence) * 255.0) as u8;
            self.experience
                .add_tension_trace(token, tension_temp, event_id);
        }

        self.pending_comparisons.insert(
            event_id,
            PendingComparison {
                input_pattern: token,
                reflex_prediction: reflex,
                ashti_results: final_ashti.clone(),
                consolidated_result: final_consolidated,
                created_at: event_id,
                trace_index: None,
            },
        );

        RoutingResult {
            event_id,
            reflex,
            slow_path: final_ashti,
            consolidated: final_consolidated,
            routed_events: vec![event_id],
            confidence: final_confidence,
            passes,
        }
    }

    /// Получить параметры multi-pass из конфига MAYA.
    /// Возвращает (max_passes, min_coherence как 0.0..1.0).
    fn maya_multipass_params(&self) -> (u8, f32) {
        let maya_id = match self.registry.maya {
            Some(id) => id,
            None => return (0, 0.6),
        };
        let maya_domain = match self.domains.get(&maya_id) {
            Some(d) => d,
            None => return (0, 0.6),
        };
        (
            maya_domain.max_passes,
            maya_domain.min_coherence as f32 / 255.0,
        )
    }

    /// ASHTI (1-8) → MAYA (уже обработано в route_from_experience)
    fn route_from_ashti(&mut self, token: Token, _source_role: u8) -> RoutingResult {
        // ASHTI результаты уже консолидированы в route_from_experience
        // Эта функция на случай прямой маршрутизации из ASHTI
        let event_id = self.com.next_event_id(_source_role as u16);

        RoutingResult {
            event_id,
            reflex: None,
            slow_path: vec![token],
            consolidated: None,
            routed_events: vec![event_id],
            confidence: 1.0,
            passes: 1,
        }
    }

    /// MAYA (10) → Финальный output
    fn route_from_maya(&mut self, token: Token) -> RoutingResult {
        let event_id = self.com.next_event_id(10);

        RoutingResult {
            event_id,
            reflex: None,
            slow_path: vec![],
            consolidated: Some(token),
            routed_events: vec![event_id],
            confidence: 1.0,
            passes: 1,
        }
    }

    /// Финализация сравнения и обучение
    pub fn finalize_comparison(&mut self, event_id: u64) -> Result<(), String> {
        let comparison = self
            .pending_comparisons
            .remove(&event_id)
            .ok_or("Comparison not found")?;

        // Сравнить reflex с консолидированным результатом
        if let (Some(reflex), Some(consolidated)) =
            (comparison.reflex_prediction, comparison.consolidated_result)
        {
            let match_result = self.compare_tokens(&reflex, &consolidated);

            // REFLECTOR: фиксируем результат рефлекса
            let input_hash = skillset::quick_hash(&comparison.input_pattern);
            self.reflector.record_reflex(input_hash, match_result);

            let weight = if match_result { 0.7 } else { 0.3 };
            self.experience
                .strengthen_or_add(consolidated, weight, event_id);

            // Усиливаем след если рефлекс был успешен
            if match_result {
                self.experience.strengthen_by_hash(input_hash, 0.05);
            }
        } else {
            // Если не было рефлекса — усиляем существующий паттерн или добавляем новый
            if let Some(consolidated) = comparison.consolidated_result {
                self.experience
                    .strengthen_or_add(consolidated, 0.5, event_id);
            }
        }

        // SKILLSET: проверяем кристаллизацию
        let weight_threshold = self.skillset.crystallization_threshold;
        let min_success = self.skillset.min_success_count;
        let candidates = self
            .experience
            .find_crystallizable(weight_threshold, min_success);
        for trace in candidates {
            self.skillset.try_crystallize(&trace);
        }

        Ok(())
    }

    /// Сравнение двух токенов на схожесть (публично для тестов)
    pub fn compare_tokens(&self, reflex: &Token, ashti: &Token) -> bool {
        // Берём пороги из конфига домена-источника рефлекса, fallback → модульные константы
        let cfg = self.domains.get(&reflex.domain_id);
        let temp_tol = cfg
            .map(|c| c.token_compare_temp_tolerance)
            .unwrap_or(TOKEN_COMPARE_TEMP_TOLERANCE);
        let mass_tol = cfg
            .map(|c| c.token_compare_mass_tolerance)
            .unwrap_or(TOKEN_COMPARE_MASS_TOLERANCE);
        let valence_tol = cfg
            .map(|c| c.token_compare_valence_tolerance)
            .unwrap_or(TOKEN_COMPARE_VALENCE_TOLERANCE);

        // Проверяем ключевые свойства
        let temp_match = (reflex.temperature as i16 - ashti.temperature as i16).abs() < temp_tol;
        let mass_match = (reflex.mass as i16 - ashti.mass as i16).abs() < mass_tol;
        let valence_match = (reflex.valence - ashti.valence).abs() < valence_tol as i8;

        // Позиция: Евклидово расстояние
        let pos_dist = self.euclidean_distance(&reflex.position, &ashti.position);
        let pos_match = pos_dist < 100.0;

        // Считаем match если хотя бы 3 из 4 свойств совпадают
        let matches = [temp_match, mass_match, valence_match, pos_match]
            .iter()
            .filter(|&&m| m)
            .count();

        matches >= 3
    }

    /// Вычисление Евклидова расстояния между позициями (публично для тестов)
    pub fn euclidean_distance(&self, a: &[i16; 3], b: &[i16; 3]) -> f32 {
        let dx = (a[0] - b[0]) as f32;
        let dy = (a[1] - b[1]) as f32;
        let dz = (a[2] - b[2]) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Получить reference на experience модуль
    pub fn experience(&self) -> &Experience {
        &self.experience
    }

    /// Получить mutable reference на experience модуль
    pub fn experience_mut(&mut self) -> &mut Experience {
        &mut self.experience
    }

    /// Очистка старых сравнений (cleanup)
    pub fn cleanup_old_comparisons(&mut self, current_event_id: u64, max_age: u64) {
        self.pending_comparisons
            .retain(|_, comp| current_event_id.saturating_sub(comp.created_at) <= max_age);
    }

    /// Добавить или обновить конфигурацию домена (для динамического добавления доменов из Engine)
    pub fn add_domain_config(&mut self, domain_id: u16, config: DomainConfig) {
        self.domains.insert(domain_id, config);
    }

    /// Mutable доступ к HashMap конфигураций доменов (для адаптации порогов)
    pub fn domain_configs_mut(&mut self) -> &mut HashMap<u16, DomainConfig> {
        &mut self.domains
    }

    /// Применить пороги из конфига домена EXPERIENCE к модулю Experience.
    ///
    /// Вызывается после обновления конфигов через adapt_thresholds.
    pub fn apply_experience_thresholds(&mut self) {
        let exp_domain_id = match self.registry.experience {
            Some(id) => id,
            None => return,
        };
        if let Some(config) = self.domains.get(&exp_domain_id) {
            self.experience
                .set_thresholds(config.reflex_threshold, config.association_threshold);
        }
    }

    /// Обработчик пульса Heartbeat (Cognitive Depth V1.0 — 13B).
    ///
    /// Вызывается каждый раз когда `HeartbeatGenerator::on_event()` возвращает `Some(pulse)`.
    /// Если `enable_internal_drive` — остужает следы напряжения и возвращает горячие
    /// токены-импульсы для повторной обработки в pipeline.
    ///
    /// Параметр `enable_internal_drive` берётся из `HeartbeatConfig::enable_internal_drive`.
    ///
    /// Возвращает Vec<Token> с горячими импульсами (пустой если Internal Drive отключён
    /// или нет горячих следов).
    pub fn on_heartbeat_pulse(
        &mut self,
        _pulse_number: u64,
        enable_internal_drive: bool,
    ) -> Vec<Token> {
        if !enable_internal_drive {
            return Vec::new();
        }

        // Остужаем все следы напряжения на TENSION_DECAY единиц
        self.experience.cool_tension_traces(TENSION_DECAY);

        // Сливаем горячие следы в импульсы
        self.experience.drain_hot_impulses(TENSION_DRAIN_THRESHOLD)
    }

    /// Выбрать следующий паттерн для обработки (Cognitive Depth V1.0 — 13C).
    ///
    /// Сравнивает внешний сигнал и внутренний импульс с учётом
    /// `internal_dominance_factor` из конфига домена MAYA.
    ///
    /// - `dominance_factor`: 0..255 → 0.0..2.0 (128 = равновесие)
    ///   - 0   = чисто реактивная (external всегда побеждает)
    ///   - 128 = равновесие (сравниваются напрямую)
    ///   - 255 ≈ 2.0 = задумчивая (internal почти всегда побеждает)
    ///
    /// Возвращает `(Token, ImpulseSource)` или `None` если оба пустые (idle).
    pub fn select_next(
        external: Option<Token>,
        internal: Option<InternalImpulse>,
        dominance_factor: u8,
    ) -> Option<(Token, ImpulseSource)> {
        let factor = dominance_factor as f32 / 128.0; // 0..255 → 0.0..~2.0

        match (external, internal) {
            (None, None) => None,
            (Some(ext), None) => Some((ext, ImpulseSource::External)),
            (None, Some(imp)) => Some((imp.pattern, imp.source)),
            (Some(ext), Some(imp)) => {
                let ext_urgency = ext.temperature as f32 / 255.0;
                let int_priority = imp.weight * factor;
                if int_priority > ext_urgency {
                    Some((imp.pattern, imp.source))
                } else {
                    Some((ext, ImpulseSource::External))
                }
            }
        }
    }

    /// Генерировать Goal-импульсы для незавершённых целей (Cognitive Depth V1.0 — 13D).
    ///
    /// Запускается каждые `check_interval` пульсов (при `pulse_number % check_interval == 0`).
    /// Возвращает `InternalImpulse` для каждого следа с GOAL-флагом и weight < GOAL_ACHIEVED_WEIGHT.
    ///
    /// Вес импульса = насколько далеко от достижения: 1.0 = только создана, ~0 = почти достигнута.
    pub fn generate_goal_impulses(
        &self,
        pulse_number: u64,
        check_interval: u64,
    ) -> Vec<InternalImpulse> {
        if check_interval == 0 || !pulse_number.is_multiple_of(check_interval) {
            return Vec::new();
        }

        self.experience
            .check_goal_traces(GOAL_ACHIEVED_WEIGHT)
            .into_iter()
            .map(|(pattern, weight)| InternalImpulse {
                source: ImpulseSource::Goal,
                weight,
                pattern,
            })
            .collect()
    }

    /// Генерировать Curiosity-импульсы для следов near crystallization threshold
    /// (Cognitive Depth V1.0 — 13D).
    ///
    /// `skill_threshold` — порог кристаллизации (тот же что в `SkillSet`).
    /// Следы в зоне [0.8 * threshold, threshold) порождают импульс с source=Curiosity.
    pub fn generate_curiosity_impulses(&self, skill_threshold: f32) -> Vec<InternalImpulse> {
        self.experience
            .check_curiosity_candidates(skill_threshold)
            .into_iter()
            .map(|(pattern, weight)| InternalImpulse {
                source: ImpulseSource::Curiosity,
                weight,
                pattern,
            })
            .collect()
    }

    /// Получить `internal_dominance_factor` из конфига домена MAYA.
    pub fn internal_dominance_factor(&self) -> u8 {
        let maya_id = match self.registry.maya {
            Some(id) => id,
            None => return 0,
        };
        self.domains
            .get(&maya_id)
            .map(|d| d.internal_dominance_factor)
            .unwrap_or(0)
    }
}

/// Скорость остывания следа напряжения за один пульс Heartbeat.
/// 10/255 ≈ 4% за пульс → след живёт ~25 пульсов при начальной temperature=255.
const TENSION_DECAY: u8 = 10;

/// Порог temperature для считывания следа как "горячего" импульса.
/// 128/255 ≈ 0.5 — половина шкалы активности.
const TENSION_DRAIN_THRESHOLD: u8 = 128;
