// AXIOM MODULE: EXPERIENCE - Ассоциативная память и обучение
//
// Реализует Домен 9 из Ashti_Core v2.0:
// - Резонансный поиск похожего опыта
// - Рефлексы, ассоциации, обучение
// - Кристаллизация скиллов
//
// Связанные спецификации:
// - docs/spec/Ashti_Core_v2_0.md (раздел 2.2)
// - docs/spec/Domain V2.0.md

use crate::token::Token;
use crate::connection::Connection;

/// Уровень резонанса между входящим паттерном и существующим опытом
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResonanceLevel {
    /// Высокий резонанс - может выдать рефлекс
    Reflex,
    /// Средний резонанс - ассоциация как подсказка
    Association,
    /// Слабый или нулевой резонанс - тишина
    Silence,
}

/// След опыта в домене EXPERIENCE
///
/// След - это комбинация паттерна (токен) и его weight (устойчивость)
#[derive(Debug, Clone)]
pub struct ExperienceTrace {
    /// Токен, представляющий паттерн опыта
    pub pattern: Token,
    /// Вес следа (устойчивость, подтверждённость)
    /// Высокий weight → рефлекс, средний → ассоциация
    pub weight: f32,
    /// Количество активаций этого следа
    pub activation_count: u32,
    /// Последний event_id активации
    pub last_activation: u64,
}

impl ExperienceTrace {
    /// Создаёт новый след с начальным weight
    pub fn new(pattern: Token, initial_weight: f32, event_id: u64) -> Self {
        Self {
            pattern,
            weight: initial_weight.max(0.0),
            activation_count: 0,
            last_activation: event_id,
        }
    }

    /// Усиливает след (при совпадении с результатом обработки)
    pub fn reinforce(&mut self, delta: f32, event_id: u64) {
        self.weight = (self.weight + delta).min(1.0);
        self.activation_count += 1;
        self.last_activation = event_id;
    }

    /// Ослабляет след (при расхождении с результатом)
    /// Не может опуститься ниже min_intensity (память не стирается полностью)
    pub fn weaken(&mut self, delta: f32, min_intensity: f32, event_id: u64) {
        self.weight = (self.weight - delta).max(min_intensity);
        self.last_activation = event_id;
    }

    /// Вычисляет уровень резонанса на основе weight
    pub fn resonance_level(&self, reflex_threshold: f32, association_threshold: f32) -> ResonanceLevel {
        if self.weight >= reflex_threshold {
            ResonanceLevel::Reflex
        } else if self.weight >= association_threshold {
            ResonanceLevel::Association
        } else {
            ResonanceLevel::Silence
        }
    }

    /// Проверка готовности к кристаллизации в скилл
    pub fn is_crystallizable(&self, min_weight: f32, min_activations: u32) -> bool {
        self.weight >= min_weight && self.activation_count >= min_activations
    }
}

/// Скилл - кристаллизованный паттерн опыта
///
/// Группа связанных следов, достигших устойчивости
#[derive(Debug, Clone)]
pub struct Skill {
    /// ID скилла
    pub skill_id: u32,
    /// Связанные токены (паттерн скилла)
    pub tokens: Vec<Token>,
    /// Связи между токенами
    pub connections: Vec<Connection>,
    /// Общий weight скилла
    pub weight: f32,
    /// event_id создания
    pub created_at: u64,
}

impl Skill {
    /// Создаёт новый скилл из группы следов
    pub fn new(
        skill_id: u32,
        traces: Vec<ExperienceTrace>,
        connections: Vec<Connection>,
        event_id: u64,
    ) -> Self {
        let tokens: Vec<Token> = traces.iter().map(|t| t.pattern.clone()).collect();
        let total_weight = traces.iter().map(|t| t.weight).sum::<f32>() / traces.len() as f32;

        Self {
            skill_id,
            tokens,
            connections,
            weight: total_weight,
            created_at: event_id,
        }
    }
}

/// Результат резонансного поиска
#[derive(Debug, Clone)]
pub struct ResonanceResult {
    /// Уровень резонанса
    pub level: ResonanceLevel,
    /// Найденный след (если есть)
    pub trace: Option<ExperienceTrace>,
    /// Оценка схожести (0.0 - 1.0)
    pub similarity: f32,
}

/// EXPERIENCE модуль - ассоциативная память
pub struct Experience {
    /// Все следы опыта
    traces: Vec<ExperienceTrace>,
    /// Кристаллизованные скиллы
    skills: Vec<Skill>,
    /// Следующий ID для скилла
    next_skill_id: u32,

    // Пороги для определения уровня резонанса
    /// Weight >= reflex_threshold → Рефлекс
    reflex_threshold: f32,
    /// Weight >= association_threshold → Ассоциация
    association_threshold: f32,
    /// Минимальная интенсивность (ничто не забывается полностью)
    min_intensity: f32,

    // Параметры обучения
    /// Величина усиления при совпадении
    reinforcement_delta: f32,
    /// Величина ослабления при расхождении
    weakening_delta: f32,

    // Параметры кристаллизации
    /// Минимальный weight для кристаллизации
    crystallization_min_weight: f32,
    /// Минимальное количество активаций
    crystallization_min_activations: u32,
}

impl Experience {
    /// Создаёт новый EXPERIENCE модуль с конфигурацией по умолчанию
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            skills: Vec::new(),
            next_skill_id: 1,

            // Пороги согласно спеке v2.0
            reflex_threshold: 0.75,       // >= 75% - рефлекс
            association_threshold: 0.40,  // 40-75% - ассоциация
            min_intensity: 0.01,          // 1% - минимум (ничто не забывается)

            // Обучение
            reinforcement_delta: 0.05,    // +5% при совпадении
            weakening_delta: 0.10,        // -10% при расхождении

            // Кристаллизация
            crystallization_min_weight: 0.85,
            crystallization_min_activations: 10,
        }
    }

    /// Резонансный поиск похожего опыта
    ///
    /// Входящий паттерн сравнивается со всеми следами.
    /// Возвращает наиболее резонансный след.
    pub fn resonance_search(&self, input_pattern: &Token) -> ResonanceResult {
        if self.traces.is_empty() {
            return ResonanceResult {
                level: ResonanceLevel::Silence,
                trace: None,
                similarity: 0.0,
            };
        }

        // Поиск наиболее похожего следа
        let mut best_trace: Option<&ExperienceTrace> = None;
        let mut best_similarity = 0.0;

        for trace in &self.traces {
            let similarity = self.compute_similarity(input_pattern, &trace.pattern);
            if similarity > best_similarity {
                best_similarity = similarity;
                best_trace = Some(trace);
            }
        }

        if let Some(trace) = best_trace {
            let level = trace.resonance_level(self.reflex_threshold, self.association_threshold);

            ResonanceResult {
                level,
                trace: Some(trace.clone()),
                similarity: best_similarity,
            }
        } else {
            ResonanceResult {
                level: ResonanceLevel::Silence,
                trace: None,
                similarity: 0.0,
            }
        }
    }

    /// Вычисляет схожесть между двумя токенами
    ///
    /// Простая метрика на основе температуры, массы и позиции
    fn compute_similarity(&self, a: &Token, b: &Token) -> f32 {
        // Схожесть по температуре
        let temp_diff = (a.temperature as f32 - b.temperature as f32).abs();
        let temp_similarity = 1.0 - (temp_diff / 255.0).min(1.0);

        // Схожесть по массе
        let mass_diff = (a.mass as f32 - b.mass as f32).abs();
        let mass_similarity = 1.0 - (mass_diff / 255.0).min(1.0);

        // Близость по позиции (если в одной области пространства)
        let pos_dist = (
            (a.position[0] - b.position[0]).pow(2) as f32 +
            (a.position[1] - b.position[1]).pow(2) as f32 +
            (a.position[2] - b.position[2]).pow(2) as f32
        ).sqrt();
        let pos_similarity = 1.0 - (pos_dist / 10000.0).min(1.0);

        // Комбинированная метрика
        temp_similarity * 0.4 + mass_similarity * 0.3 + pos_similarity * 0.3
    }

    /// Добавляет новый след опыта
    pub fn add_trace(&mut self, pattern: Token, initial_weight: f32, event_id: u64) {
        let trace = ExperienceTrace::new(pattern, initial_weight, event_id);
        self.traces.push(trace);
    }

    /// Обучение: сравнение рефлекса с результатом обработки
    ///
    /// - match_result: true если рефлекс совпал с результатом ASHTI
    pub fn learn(&mut self, trace_index: usize, match_result: bool, event_id: u64) {
        if trace_index >= self.traces.len() {
            return;
        }

        if match_result {
            // Совпадение - усиливаем
            self.traces[trace_index].reinforce(self.reinforcement_delta, event_id);
        } else {
            // Расхождение - ослабляем
            self.traces[trace_index].weaken(
                self.weakening_delta,
                self.min_intensity,
                event_id,
            );
        }
    }

    /// Кристаллизация: проверка и создание скиллов из устойчивых следов
    pub fn crystallize_skills(&mut self, event_id: u64) -> Vec<u32> {
        let mut new_skill_ids = Vec::new();

        // Находим все следы, готовые к кристаллизации
        let ready_traces: Vec<ExperienceTrace> = self
            .traces
            .iter()
            .filter(|t| {
                t.is_crystallizable(
                    self.crystallization_min_weight,
                    self.crystallization_min_activations,
                )
            })
            .cloned()
            .collect();

        // TODO: группировка связанных следов (сейчас простая версия - по одному)
        for trace in ready_traces {
            let skill_id = self.next_skill_id;
            self.next_skill_id += 1;

            let skill = Skill::new(skill_id, vec![trace], Vec::new(), event_id);
            self.skills.push(skill);
            new_skill_ids.push(skill_id);
        }

        new_skill_ids
    }

    /// Получает количество следов
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    /// Получает количество скиллов
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    /// Получает скилл по ID
    pub fn get_skill(&self, skill_id: u32) -> Option<&Skill> {
        self.skills.iter().find(|s| s.skill_id == skill_id)
    }
}

impl Default for Experience {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_token(id: u32, temperature: u8) -> Token {
        let mut token = Token::default();
        token.sutra_id = id;
        token.temperature = temperature;
        token.mass = 100;
        token.position = [0, 0, 0];
        token
    }

    #[test]
    fn test_experience_creation() {
        let exp = Experience::new();
        assert_eq!(exp.trace_count(), 0);
        assert_eq!(exp.skill_count(), 0);
    }

    #[test]
    fn test_add_trace() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        exp.add_trace(token, 0.5, 1);
        assert_eq!(exp.trace_count(), 1);
    }

    #[test]
    fn test_resonance_search_empty() {
        let exp = Experience::new();
        let token = create_test_token(1, 100);

        let result = exp.resonance_search(&token);
        assert_eq!(result.level, ResonanceLevel::Silence);
        assert!(result.trace.is_none());
    }

    #[test]
    fn test_resonance_search_reflex() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        // Добавляем след с высоким weight (рефлекс)
        exp.add_trace(token.clone(), 0.80, 1);

        let result = exp.resonance_search(&token);
        assert_eq!(result.level, ResonanceLevel::Reflex);
        assert!(result.trace.is_some());
        assert!(result.similarity > 0.9);
    }

    #[test]
    fn test_resonance_search_association() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        // Добавляем след со средним weight (ассоциация)
        exp.add_trace(token.clone(), 0.50, 1);

        let result = exp.resonance_search(&token);
        assert_eq!(result.level, ResonanceLevel::Association);
    }

    #[test]
    fn test_resonance_search_silence() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        // Добавляем след с низким weight (тишина)
        exp.add_trace(token.clone(), 0.10, 1);

        let result = exp.resonance_search(&token);
        assert_eq!(result.level, ResonanceLevel::Silence);
    }

    #[test]
    fn test_learn_reinforce() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        exp.add_trace(token, 0.50, 1);

        let initial_weight = exp.traces[0].weight;
        exp.learn(0, true, 2); // Совпадение
        assert!(exp.traces[0].weight > initial_weight);
    }

    #[test]
    fn test_learn_weaken() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        exp.add_trace(token, 0.50, 1);

        let initial_weight = exp.traces[0].weight;
        exp.learn(0, false, 2); // Расхождение
        assert!(exp.traces[0].weight < initial_weight);
    }

    #[test]
    fn test_learn_min_intensity() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        exp.add_trace(token, 0.10, 1);

        // Много ослаблений не должны опустить ниже min_intensity
        for i in 0..20 {
            exp.learn(0, false, i + 2);
        }

        assert_eq!(exp.traces[0].weight, exp.min_intensity);
    }

    #[test]
    fn test_crystallize_skills() {
        let mut exp = Experience::new();
        let token = create_test_token(1, 100);

        // Добавляем след и усиливаем до готовности к кристаллизации
        exp.add_trace(token, 0.80, 1);

        // Активируем много раз
        for i in 0..15 {
            exp.traces[0].reinforce(0.01, i + 2);
        }

        let skill_ids = exp.crystallize_skills(20);
        assert_eq!(skill_ids.len(), 1);
        assert_eq!(exp.skill_count(), 1);
    }

    #[test]
    fn test_similarity_same_token() {
        let exp = Experience::new();
        let token = create_test_token(1, 100);

        let similarity = exp.compute_similarity(&token, &token);
        assert!(similarity > 0.99);
    }

    #[test]
    fn test_similarity_different_tokens() {
        let exp = Experience::new();
        let token1 = create_test_token(1, 100);
        let token2 = create_test_token(2, 200);

        let similarity = exp.compute_similarity(&token1, &token2);
        // Разные токены должны иметь меньшую схожесть чем одинаковые
        assert!(similarity < 0.99);
    }
}
