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

use crate::token::Token;
use crate::experience::{Experience, ExperienceTrace, ResonanceLevel};
use crate::domain::DomainConfig;
use crate::ashti_processor::AshtiProcessor;
use crate::maya_processor::MayaProcessor;
use crate::com::COM;
use std::collections::HashMap;

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
        }
    }
}

/// Ожидающее сравнение рефлекса с результатом ASHTI
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PendingComparison {
    /// Входной паттерн
    input_pattern: Token,
    /// Предсказание рефлекса (если было)
    reflex_prediction: Option<Token>,
    /// Результаты от ASHTI 1-8
    ashti_results: Vec<Token>,
    /// Консолидированный результат от MAYA
    consolidated_result: Option<Token>,
    /// Время создания (event_id)
    created_at: u64,
    /// Индекс следа, который сгенерировал рефлекс
    trace_index: Option<usize>,
}

/// Реестр доменов по их ролям
#[derive(Debug, Clone)]
struct DomainRegistry {
    sutra: Option<u32>,
    experience: Option<u32>,
    ashti: [Option<u32>; 8],  // Indexed by role 1-8
    maya: Option<u32>,
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
        self.sutra.is_some() &&
        self.experience.is_some() &&
        self.maya.is_some() &&
        self.ashti.iter().all(|d| d.is_some())
    }
}

/// Arbiter - над-доменный модуль маршрутизации (Arbiter V1.0)
pub struct Arbiter {
    /// Опыт и ассоциативная память
    experience: Experience,
    /// Реестр доменов
    registry: DomainRegistry,
    /// Ожидающие сравнения
    pending_comparisons: HashMap<u64, PendingComparison>,
    /// Ссылка на домены (для обработки)
    domains: HashMap<u32, DomainConfig>,
    /// COM для событий
    com: COM,
}

impl Arbiter {
    /// Создать новый Arbiter
    pub fn new(domains: HashMap<u32, DomainConfig>, com: COM) -> Self {
        Self {
            experience: Experience::new(),
            registry: DomainRegistry::new(),
            pending_comparisons: HashMap::new(),
            domains,
            com,
        }
    }

    /// Зарегистрировать домен по structural_role
    pub fn register_domain(&mut self, role: u8, domain_id: u32) -> Result<(), String> {
        match role {
            0 => {
                self.registry.sutra = Some(domain_id);
                Ok(())
            },
            9 => {
                self.registry.experience = Some(domain_id);
                Ok(())
            },
            1..=8 => {
                self.registry.ashti[(role - 1) as usize] = Some(domain_id);
                Ok(())
            },
            10 => {
                self.registry.maya = Some(domain_id);
                Ok(())
            },
            _ => Err(format!("Invalid structural_role: {}", role))
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

    /// SUTRA (0) → EXPERIENCE (9)
    fn route_from_sutra(&mut self, token: Token) -> RoutingResult {
        let _event_id = self.com.next_event_id(0);

        // Токен от SUTRA всегда идёт в EXPERIENCE
        self.route_from_experience(token)
    }

    /// EXPERIENCE (9) → Dual Path: reflex OR (ASHTI 1-8 → MAYA)
    fn route_from_experience(&mut self, token: Token) -> RoutingResult {
        let event_id = self.com.next_event_id(9);

        // 1. Резонансный поиск
        let resonance = self.experience.resonance_search(&token);

        // 2. Fast path (conditional) - рефлекс
        let reflex = if resonance.level == ResonanceLevel::Reflex {
            let reflex_token = resonance.trace.as_ref().unwrap().pattern.clone();
            Some(reflex_token)
        } else {
            None
        };

        // 3. Slow path (ALWAYS) - через ASHTI 1-8
        let hint = if resonance.level == ResonanceLevel::Association {
            resonance.trace.as_ref()
        } else {
            None
        };

        let ashti_results = self.route_to_ashti(token.clone(), hint);

        // 4. Консолидация через MAYA
        let consolidated = self.route_to_maya(ashti_results.clone());

        // 5. Сохранить для сравнения
        self.pending_comparisons.insert(event_id, PendingComparison {
            input_pattern: token,
            reflex_prediction: reflex.clone(),
            ashti_results: ashti_results.clone(),
            consolidated_result: consolidated.clone(),
            created_at: event_id,
            trace_index: None,  // TODO: track which trace generated reflex
        });

        RoutingResult {
            event_id,
            reflex,
            slow_path: ashti_results,
            consolidated,
            routed_events: vec![event_id],
        }
    }

    /// Маршрутизация через все ASHTI 1-8 домены
    fn route_to_ashti(&self, token: Token, hint: Option<&ExperienceTrace>) -> Vec<Token> {
        let mut results = Vec::new();

        for role in 1..=8 {
            if let Some(domain_id) = self.registry.ashti[role - 1] {
                if let Some(domain) = self.domains.get(&domain_id) {
                    let result = AshtiProcessor::process_token(&token, domain, hint);
                    results.push(result);
                }
            }
        }

        results
    }

    /// Консолидация результатов ASHTI через MAYA
    fn route_to_maya(&self, ashti_results: Vec<Token>) -> Option<Token> {
        if ashti_results.is_empty() {
            return None;
        }

        let maya_id = self.registry.maya?;
        let maya_domain = self.domains.get(&maya_id)?;

        Some(MayaProcessor::consolidate_results(ashti_results, maya_domain))
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
        }
    }

    /// Финализация сравнения и обучение
    pub fn finalize_comparison(&mut self, event_id: u64) -> Result<(), String> {
        let comparison = self.pending_comparisons.remove(&event_id)
            .ok_or("Comparison not found")?;

        // Сравнить reflex с консолидированным результатом
        if let (Some(reflex), Some(consolidated)) = (comparison.reflex_prediction, comparison.consolidated_result) {
            let match_result = self.compare_tokens(&reflex, &consolidated);

            // Обучение на основе сравнения
            // TODO: найти правильный trace_index
            // Пока просто создаём новый trace
            let weight = if match_result { 0.7 } else { 0.3 };
            self.experience.add_trace(consolidated.clone(), weight, event_id);
        } else {
            // Если не было рефлекса, просто добавляем trace
            if let Some(consolidated) = comparison.consolidated_result {
                self.experience.add_trace(consolidated, 0.5, event_id);
            }
        }

        Ok(())
    }

    /// Сравнение двух токенов на схожесть
    fn compare_tokens(&self, reflex: &Token, ashti: &Token) -> bool {
        // Проверяем ключевые свойства
        let temp_match = (reflex.temperature as i16 - ashti.temperature as i16).abs() < 10;
        let mass_match = (reflex.mass as i16 - ashti.mass as i16).abs() < 5;
        let valence_match = (reflex.valence - ashti.valence).abs() < 2;

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

    /// Вычисление Евклидова расстояния между позициями
    fn euclidean_distance(&self, a: &[i16; 3], b: &[i16; 3]) -> f32 {
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
        self.pending_comparisons.retain(|_, comp| {
            current_event_id.saturating_sub(comp.created_at) <= max_age
        });
    }
}

