// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AnchorMatchTable — символьный/словарный матчинг текста к якорным примитивам.
//
// Путь А (anchor-matching): текст → декомпозиция → позиция = взвешенный центроид примитивов.
// Заполняет пробел когда word-level match_text() возвращает пустой результат.
//
// Источник: docs/deferred/OBS-01_Errata_Instructions.md §2

use std::collections::HashMap;
use std::sync::Arc;

use axiom_config::AnchorSet;

use super::decomposition_table::{char_signals, subsystem_from_anchor_id, word_signals};

/// Таблица декомпозиции: кэшированные позиции примитивов по anchor-id.
///
/// Строится из AnchorSet при конструировании — позиции не меняются в runtime.
pub struct AnchorMatchTable {
    id_to_position: HashMap<String, [i16; 3]>,
}

impl AnchorMatchTable {
    /// Построить таблицу из AnchorSet. Кэширует позиции всех известных якорей.
    pub fn build(anchor_set: &AnchorSet) -> Self {
        let mut id_to_position = HashMap::new();

        // Subsystem якоря
        for subsystem in ["writing", "mathematics", "music", "time", "logic", "values",
                          "morality", "abstractions"] {
            for anchor in anchor_set.get_subsystem(subsystem) {
                if !anchor.id.is_empty() {
                    id_to_position.insert(anchor.id.clone(), anchor.position);
                }
            }
        }

        // Domain якоря (D1..D8) — AE-TD-08 / Anchor-id (P4b)
        for domain_anchors in &anchor_set.domains {
            for anchor in domain_anchors {
                if !anchor.id.is_empty() {
                    id_to_position.insert(anchor.id.clone(), anchor.position);
                }
            }
        }

        // Layer якоря (L1..L8) — AE-TD-08 / Anchor-id (P4b)
        for layer_anchors in &anchor_set.layers {
            for anchor in layer_anchors {
                if !anchor.id.is_empty() {
                    id_to_position.insert(anchor.id.clone(), anchor.position);
                }
            }
        }

        Self { id_to_position }
    }

    /// Определить доминирующую подсистему через декомпозицию (без AnchorSet).
    ///
    /// Использует word_signals + char_signals из decomposition_table.
    /// Возвращает None если текст не содержит ни одного известного термина.
    pub fn dominant_subsystem(&self, text: &str) -> Option<String> {
        let mut scores: HashMap<&str, f64> = HashMap::new();
        let text_lower = text.to_lowercase();
        for word in text_lower.split(|c: char| !c.is_alphabetic()) {
            if word.len() < 2 {
                continue;
            }
            for &(anchor_id, weight) in word_signals(word) {
                if let Some(sub) = subsystem_from_anchor_id(anchor_id) {
                    *scores.entry(sub).or_insert(0.0) += weight as f64;
                }
            }
        }
        for c in text.chars() {
            for &(anchor_id, weight) in char_signals(c) {
                if let Some(sub) = subsystem_from_anchor_id(anchor_id) {
                    *scores.entry(sub).or_insert(0.0) += weight as f64 * 0.4;
                }
            }
        }
        scores
            .into_iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(name, _)| name.to_string())
    }

    /// Собрать уникальные anchor ID, совпавшие с текстом (словарный + символьный уровни).
    ///
    /// Используется в perceive_and_bond() для создания SEMANTIC_ANCHOR_BOND связей.
    pub fn matched_anchor_ids(&self, text: &str) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let text_lower = text.to_lowercase();
        for word in text_lower.split(|c: char| !c.is_alphabetic()) {
            if word.len() < 2 { continue; }
            for &(anchor_id, _weight) in word_signals(word) {
                if self.id_to_position.contains_key(anchor_id) {
                    seen.insert(anchor_id.to_string());
                }
            }
        }
        for c in text.chars() {
            for &(anchor_id, _weight) in char_signals(c) {
                if self.id_to_position.contains_key(anchor_id) {
                    seen.insert(anchor_id.to_string());
                }
            }
        }
        seen.into_iter().collect()
    }

    /// Вычислить позицию токена через декомпозицию текста.
    ///
    /// Возвращает `None` если ни один символ/слово не дал совпадения с известными примитивами.
    /// В этом случае caller должен использовать FNV-fallback.
    pub fn compute_position(&self, text: &str) -> Option<[i16; 3]> {
        let mut acc = [0i64; 3];
        let mut total_weight = 0.0f64;

        // ── Словарный уровень (приоритетный) ──────────────────────────────────
        let text_lower = text.to_lowercase();
        for word in text_lower.split(|c: char| !c.is_alphabetic()) {
            if word.len() < 2 {
                continue;
            }
            for &(anchor_id, weight) in word_signals(word) {
                if let Some(&pos) = self.id_to_position.get(anchor_id) {
                    let w = weight as f64;
                    acc[0] += (pos[0] as f64 * w) as i64;
                    acc[1] += (pos[1] as f64 * w) as i64;
                    acc[2] += (pos[2] as f64 * w) as i64;
                    total_weight += w;
                }
            }
        }

        // ── Символьный уровень (дополняет словарный) ──────────────────────────
        // Вес символов снижен относительно слов: char_weight × 0.4
        for c in text.chars() {
            for &(anchor_id, weight) in char_signals(c) {
                if let Some(&pos) = self.id_to_position.get(anchor_id) {
                    let w = weight as f64 * 0.4;
                    acc[0] += (pos[0] as f64 * w) as i64;
                    acc[1] += (pos[1] as f64 * w) as i64;
                    acc[2] += (pos[2] as f64 * w) as i64;
                    total_weight += w;
                }
            }
        }

        if total_weight < 0.01 {
            return None;
        }

        Some([
            (acc[0] as f64 / total_weight).round().clamp(0.0, 32767.0) as i16,
            (acc[1] as f64 / total_weight).round().clamp(0.0, 32767.0) as i16,
            (acc[2] as f64 / total_weight).round().clamp(0.0, 32767.0) as i16,
        ])
    }
}

/// Обёртка `Option<Arc<AnchorMatchTable>>` — строится в TextPerceptor при наличии якорей.
pub fn build_match_table(anchor_set: &Option<Arc<AnchorSet>>) -> Option<AnchorMatchTable> {
    anchor_set.as_ref().map(|a| AnchorMatchTable::build(a))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_config::AnchorSet;

    fn make_set() -> AnchorSet {
        // Минимальный AnchorSet с одним math и одним writing примитивом
        let mut set = AnchorSet::empty();
        // Добавим вручную через subsystems (нет pub API кроме load_dir/axes)
        // Используем get_subsystem proxy через реальные якоря из load_dir,
        // но для тестов — stub через axes (since axes are accessible).
        // Вместо этого тестируем что None возвращается при пустом set.
        set
    }

    #[test]
    fn test_empty_anchor_set_returns_none() {
        let table = AnchorMatchTable::build(&AnchorSet::empty());
        let pos = table.compute_position("2 + 2 = 4");
        // Нет известных якорей → нет совпадений
        assert!(pos.is_none());
    }

    #[test]
    fn test_empty_text_returns_none() {
        let table = AnchorMatchTable::build(&AnchorSet::empty());
        assert!(table.compute_position("").is_none());
        assert!(table.compute_position("   ").is_none());
    }

    #[test]
    fn test_position_in_valid_range() {
        // Строим AnchorSet с ручными якорями через load_dir path
        // Упрощённый тест: если позиция возвращается — она в диапазоне [0..32767]
        let table = AnchorMatchTable {
            id_to_position: {
                let mut m = HashMap::new();
                m.insert("math_element".to_string(), [5000i16, 0, 8000]);
                m.insert("math_operation".to_string(), [9000i16, 8000, 11000]);
                m.insert("math_relation".to_string(), [10000i16, 12000, 6000]);
                m
            },
        };
        let pos = table.compute_position("2 + 2 = 4");
        let p = pos.expect("math text should produce position");
        assert!(p[0] >= 0 && p[0] <= 32767, "x out of range: {}", p[0]);
        assert!(p[1] >= 0 && p[1] <= 32767, "y out of range: {}", p[1]);
        assert!(p[2] >= 0 && p[2] <= 32767, "z out of range: {}", p[2]);
    }

    #[test]
    fn test_math_text_pulls_toward_math_anchors() {
        let table = AnchorMatchTable {
            id_to_position: {
                let mut m = HashMap::new();
                // Math anchors at high-x region
                m.insert("math_element".to_string(), [20000i16, 10000, 10000]);
                m.insert("math_function".to_string(), [20000i16, 10000, 10000]);
                m.insert("math_relation".to_string(), [20000i16, 10000, 10000]);
                m.insert("math_operation".to_string(), [20000i16, 10000, 10000]);
                m.insert("math_limit".to_string(), [20000i16, 10000, 10000]);
                m.insert("math_group".to_string(), [20000i16, 10000, 10000]);
                // Writing anchors at low-x region
                m.insert("prim_dot".to_string(), [1000i16, 1000, 1000]);
                m.insert("prim_vline".to_string(), [1000i16, 1000, 1000]);
                m.insert("prim_hline".to_string(), [1000i16, 1000, 1000]);
                m.insert("prim_dslash".to_string(), [1000i16, 1000, 1000]);
                m.insert("prim_arc".to_string(), [1000i16, 1000, 1000]);
                m.insert("prim_hook".to_string(), [1000i16, 1000, 1000]);
                m
            },
        };

        let math_pos = table
            .compute_position("Теорема Пифагора: a² + b² = c². Число π иррационально. Предел последовательности.")
            .expect("math text must match");
        let writing_pos = table
            .compute_position("Однажды в тихом городе жил молодой писатель. Он сочинял рассказы каждый вечер.")
            .expect("writing text must match");

        // Math text → closer to x=20000; Writing text → closer to x=1000
        assert!(
            math_pos[0] > writing_pos[0],
            "math x={} should be > writing x={}",
            math_pos[0],
            writing_pos[0]
        );
    }

    #[test]
    fn test_word_level_dominates_char_level() {
        // Текст с математическими словами И кириллическими буквами.
        // Слова должны перевешивать символы (weight × 0.4 для символов).
        let table = AnchorMatchTable {
            id_to_position: {
                let mut m = HashMap::new();
                m.insert("math_function".to_string(), [30000i16, 30000, 30000]);
                m.insert("prim_vline".to_string(), [0i16, 0, 0]);
                m
            },
        };
        // "производная" → math_function(1.0); кириллические буквы → prim_vline(0.9 × 0.4)
        let pos = table
            .compute_position("производная")
            .expect("should match");
        // math_function weight=1.0 >> cyrillic char weight (11 chars × 0.9 × 0.4 = 3.96)
        // But wait: char_signals don't match cyrillic directly (only greek letters)
        // So it's purely word match: pos = [30000, 30000, 30000]
        assert_eq!(pos[0], 30000);
    }

    #[test]
    fn subsystem_from_anchor_id_handles_abstraction_prefix() {
        use super::super::decomposition_table::subsystem_from_anchor_id;
        assert_eq!(subsystem_from_anchor_id("abstraction_theory"), Some("abstractions"));
        assert_eq!(subsystem_from_anchor_id("abstraction_entity"), Some("abstractions"));
        assert_eq!(subsystem_from_anchor_id("abstraction_raw"), Some("abstractions"));
        assert_eq!(subsystem_from_anchor_id("moral_harm"), Some("morality"));
        assert_eq!(subsystem_from_anchor_id("prim_narrative"), Some("writing"));
        assert_eq!(subsystem_from_anchor_id("ddream_метафора"), None);
        assert_eq!(subsystem_from_anchor_id("L5_сравнение"), None);
    }

    #[test]
    fn integration_detect_subsystem_problematic_texts() {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let config_dir = manifest_dir.parent().and_then(|p| p.parent()).map(|p| p.join("config"));
        let config_dir = match config_dir { Some(p) if p.exists() => p, _ => return };
        let anchors = std::sync::Arc::new(AnchorSet::load_or_empty(&config_dir));
        let table = AnchorMatchTable::build(&anchors);

        let cases = [
            ("writing_metaphor",     "writing",      "Метафора переносит значение с одного предмета на другой. Сравнение делает абстрактное конкретным. Символ сжимает сложную идею до образа."),
            ("writing_style",        "writing",      "Краткость — сестра таланта. Каждое слово должно нести смысл. Ритм фразы создаёт настроение текста."),
            ("morality_consequences","morality",     "Утилитаризм оценивает действие по его последствиям. Наибольшее благо для наибольшего числа людей. Счастье измеримо и максимизируемо."),
            ("abstract_emergence",   "abstractions", "Эмерджентность — свойства целого не сводимые к свойствам частей. Сознание возникает из нейронов. Жизнь возникает из молекул. Целое превосходит сумму частей."),
            ("abstract_infinity",    "abstractions", "Бесконечность не является числом, но имеет разные размеры. Множество натуральных и множество вещественных чисел — разные бесконечности. Теорема Кантора это доказывает."),
            // Регрессия OBS-ACC-01: "форма"→abstractions и "закон"→abstractions создавали ничью
            ("logic_deductive",      "logic",        "Дедуктивное мышление строится от общего к частному. Истинные посылки дают истинный вывод. Силлогизм — классическая форма дедукции."),
            ("morality_duty",        "morality",     "Долг определяет обязательства независимо от последствий. Кантовский категорический императив требует универсализуемости поступков. Моральный закон внутри нас."),
        ];

        for (id, expected, text) in &cases {
            let matches = anchors.match_text(text);
            let p1 = anchors.dominant_subsystem_of(&matches);
            let p2 = table.dominant_subsystem(text);
            let detected = p1.as_deref().or(p2.as_deref());
            assert_eq!(
                detected, Some(*expected),
                "[{id}] expected={expected} got={detected:?} (p1={p1:?} p2={p2:?})"
            );
        }
    }
}
