// SKILLSET — кристаллизованные навыки (Этап 4)
//
// Skill: кристаллизованный след опыта с высоким весом и подтверждениями.
// SkillSet: коллекция навыков + механизм кристаллизации + поиск.

use axiom_core::Token;
use crate::experience::ExperienceTrace;

/// Кристаллизованный навык
///
/// Навык — это ExperienceTrace с весом ≥ crystallization_threshold
/// и success_count ≥ min_success_count. Раз кристаллизовавшись,
/// навык возвращается как единый быстрый ответ без обращения к физике поля.
#[derive(Debug, Clone)]
pub struct Skill {
    /// Паттерн токена — суть навыка
    pub pattern: Token,
    /// Вес при кристаллизации (≥ crystallization_threshold)
    pub activation_weight: f32,
    /// Событие, при котором навык был кристаллизован
    pub created_at: u64,
    /// Число успешных активаций после кристаллизации
    pub success_count: u32,
    /// Хэш паттерна для быстрого поиска
    pub pattern_hash: u64,
}

/// Набор кристаллизованных навыков
pub struct SkillSet {
    skills: Vec<Skill>,
    /// Минимальный вес следа для кристаллизации
    pub crystallization_threshold: f32,
    /// Минимальное число успехов следа для кристаллизации
    pub min_success_count: u32,
    /// Порог схожести для активации навыка (выше reflex_threshold)
    pub activation_similarity: f32,
}

impl SkillSet {
    /// Создать SkillSet с параметрами по умолчанию
    pub fn new() -> Self {
        Self {
            skills: Vec::new(),
            crystallization_threshold: 0.8,
            min_success_count: 50,
            activation_similarity: 0.9,
        }
    }

    /// Попытаться кристаллизовать след в навык
    ///
    /// Возвращает true если навык был создан.
    /// Критерии: weight ≥ threshold AND success_count ≥ min_success_count.
    /// Дублирование: не кристаллизует если уже есть навык с близким паттерном.
    pub fn try_crystallize(&mut self, trace: &ExperienceTrace) -> bool {
        if trace.weight < self.crystallization_threshold {
            return false;
        }
        if trace.success_count < self.min_success_count {
            return false;
        }

        // Проверить дубликат
        let similarity_threshold = self.activation_similarity;
        if self.skills.iter().any(|s| {
            let hash_dist = (s.pattern_hash ^ trace.pattern_hash).count_ones();
            if hash_dist > 40 {
                return false;
            }
            skill_pattern_similarity(&s.pattern, &trace.pattern) >= similarity_threshold
        }) {
            return false;
        }

        self.skills.push(Skill {
            pattern: trace.pattern,
            activation_weight: trace.weight,
            created_at: trace.created_at,
            success_count: 0,
            pattern_hash: trace.pattern_hash,
        });

        true
    }

    /// Найти подходящий навык для токена
    ///
    /// Возвращает первый навык с similarity ≥ activation_similarity.
    pub fn find_skill(&self, token: &Token) -> Option<&Skill> {
        let input_hash = quick_hash(token);
        let threshold = self.activation_similarity;

        for skill in &self.skills {
            let hash_dist = (input_hash ^ skill.pattern_hash).count_ones();
            if hash_dist > 40 {
                continue;
            }
            if skill_pattern_similarity(token, &skill.pattern) >= threshold {
                return Some(skill);
            }
        }

        None
    }

    /// Зафиксировать успешную активацию навыка (по индексу)
    pub fn record_activation(&mut self, skill_idx: usize) {
        if let Some(skill) = self.skills.get_mut(skill_idx) {
            skill.success_count = skill.success_count.saturating_add(1);
            skill.activation_weight = (skill.activation_weight + 0.01).min(1.0);
        }
    }

    /// Найти навык и вернуть его индекс вместе со ссылкой
    pub fn find_skill_with_idx(&self, token: &Token) -> Option<(usize, &Skill)> {
        let input_hash = quick_hash(token);
        let threshold = self.activation_similarity;

        for (i, skill) in self.skills.iter().enumerate() {
            let hash_dist = (input_hash ^ skill.pattern_hash).count_ones();
            if hash_dist > 40 {
                continue;
            }
            if skill_pattern_similarity(token, &skill.pattern) >= threshold {
                return Some((i, skill));
            }
        }

        None
    }

    /// Количество кристаллизованных навыков
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    /// Получить все навыки (для экспорта/инспекции)
    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    /// Импортировать навык из другого экземпляра с пониженным весом
    pub fn import_skill(&mut self, mut skill: Skill) {
        skill.activation_weight *= 0.3; // Импортированные начинают с низкого веса
        skill.success_count = 0;
        self.skills.push(skill);
    }

    /// Экспортировать все навыки (клоны) для передачи другому экземпляру.
    pub fn export(&self) -> Vec<Skill> {
        self.skills.clone()
    }

    /// Импортировать пакет навыков из другого экземпляра.
    ///
    /// Каждый навык:
    /// - проходит дедупликацию (не импортируется если уже есть похожий)
    /// - получает вес × 0.3 и success_count = 0
    ///
    /// Возвращает число фактически импортированных навыков.
    pub fn import_batch(&mut self, skills: &[Skill]) -> usize {
        let mut imported = 0;
        for skill in skills {
            let is_dup = {
                let threshold = self.activation_similarity;
                self.skills.iter().any(|s| {
                    let hash_dist = (s.pattern_hash ^ skill.pattern_hash).count_ones();
                    if hash_dist > 40 { return false; }
                    skill_pattern_similarity(&s.pattern, &skill.pattern) >= threshold
                })
            };
            if !is_dup {
                self.import_skill(skill.clone());
                imported += 1;
            }
        }
        imported
    }

    /// Очистить все навыки.
    pub fn clear(&mut self) {
        self.skills.clear();
    }
}

impl Default for SkillSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Сходство паттернов (аналогично experience::pattern_similarity)
fn skill_pattern_similarity(a: &Token, b: &Token) -> f32 {
    let temp_diff = (a.temperature as i16 - b.temperature as i16).unsigned_abs() as f32 / 255.0;
    let mass_diff = (a.mass as i16 - b.mass as i16).unsigned_abs() as f32 / 255.0;
    let val_diff = (a.valence as i16 - b.valence as i16).unsigned_abs() as f32 / 254.0;

    let dx = (a.position[0] as i32 - b.position[0] as i32) as f32;
    let dy = (a.position[1] as i32 - b.position[1] as i32) as f32;
    let dz = (a.position[2] as i32 - b.position[2] as i32) as f32;
    let pos_diff = (dx * dx + dy * dy + dz * dz).sqrt() / 56755.0;

    let avg_diff = (temp_diff + mass_diff + val_diff + pos_diff) * 0.25;
    1.0 - avg_diff.min(1.0)
}

/// Быстрый хэш токена (FNV-1a — идентично experience::pattern_hash)
pub(crate) fn quick_hash(token: &Token) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    h ^= token.temperature as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.mass as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= (token.valence as u8) as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[0] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[1] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[2] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h
}
