// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MoralSignalDetector — мониторинг активации moral-якорей (Morality_V1_0.md §5.2).
//
// Детектирует активацию moral-примитивов из AnchorSet-матчей.
// Вычисляет moral_intensity (взвешенная сумма) и dominant_foundation.
// Передаёт MoralSignal в DilemmaDetector когда обнаружен конфликт оснований.
//
// Moral-якоря (7 штук): moral_care, moral_harm, moral_fair, moral_betrayal,
//   moral_loyalty, moral_purity, moral_desecration.

use axiom_config::AnchorMatch;

// ── Константы ─────────────────────────────────────────────────────────────────

/// Порог интенсивности для срабатывания MoralSignal.
pub const MORAL_INTENSITY_THRESHOLD: f32 = 0.3;

/// Минимальный score матча для учёта якоря.
const MIN_MATCH_SCORE: f32 = 0.1;

/// Moral-якоря: id-префикс для идентификации.
const MORAL_PREFIX: &str = "moral_";

// ── Типы ─────────────────────────────────────────────────────────────────────

/// Один активированный moral-якорь с весом.
#[derive(Debug, Clone)]
pub struct MoralFoundation {
    pub anchor_id: String,
    /// Взвешенный вклад якоря (match_score × якорный mass-нормализованный вес).
    pub weight: f32,
}

/// Сигнал моральной активации — результат работы MoralSignalDetector.
#[derive(Debug, Clone)]
pub struct MoralSignal {
    /// Суммарная интенсивность (0.0..1.0, saturating).
    pub intensity: f32,
    /// Доминирующий moral-якорь (с наибольшим weight).
    pub dominant: MoralFoundation,
    /// Второй по весу якорь (если обнаружен конфликт оснований).
    pub secondary: Option<MoralFoundation>,
    /// True если dominant и secondary являются антагонистической парой.
    pub conflict_detected: bool,
}

/// Детектор моральных сигналов. Вызывается из ContextRecognizer при каждом on_tick.
pub struct MoralSignalDetector;

impl MoralSignalDetector {
    /// Детектировать moral-сигнал из набора AnchorMatch.
    ///
    /// Возвращает `None` если суммарная интенсивность ниже MORAL_INTENSITY_THRESHOLD.
    pub fn detect<'a>(matches: &[AnchorMatch<'a>]) -> Option<MoralSignal> {
        // Собрать все moral-якоря с ненулевым score
        let mut foundations: Vec<MoralFoundation> = matches
            .iter()
            .filter(|m| m.anchor.id.starts_with(MORAL_PREFIX) && m.score >= MIN_MATCH_SCORE)
            .map(|m| MoralFoundation {
                anchor_id: m.anchor.id.clone(),
                weight: m.score,
            })
            .collect();

        if foundations.is_empty() {
            return None;
        }

        // Суммарная интенсивность: saturating при > 1.0
        let intensity = foundations.iter().map(|f| f.weight).sum::<f32>().min(1.0);

        if intensity < MORAL_INTENSITY_THRESHOLD {
            return None;
        }

        // Сортировать по убыванию weight
        foundations.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));

        let dominant = foundations.remove(0);
        let secondary = foundations.into_iter().next();
        let conflict_detected = secondary
            .as_ref()
            .map(|s| is_antagonistic_pair(&dominant.anchor_id, &s.anchor_id))
            .unwrap_or(false);

        Some(MoralSignal { intensity, dominant, secondary, conflict_detected })
    }
}

/// Проверить является ли пара moral-якорей антагонистической.
///
/// Антагонисты — якоря, которые при одновременной активации указывают на
/// прямое ценностное противоречие (например care vs harm).
fn is_antagonistic_pair(a: &str, b: &str) -> bool {
    const PAIRS: &[(&str, &str)] = &[
        ("moral_care",    "moral_harm"),
        ("moral_fair",    "moral_betrayal"),
        ("moral_loyalty", "moral_desecration"),
        ("moral_purity",  "moral_harm"),
    ];
    PAIRS.iter().any(|(x, y)| (a == *x && b == *y) || (a == *y && b == *x))
}

// ── Тесты ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_config::{Anchor, AnchorMatch, MatchType};

    fn make_anchor(id: &str) -> Anchor {
        Anchor {
            id: id.to_string(),
            word: id.to_string(),
            aliases: vec![],
            tags: vec![],
            position: [0; 3],
            shell: [0; 8],
            description: String::new(),
            layer: axiom_config::AnchorLayer::default(),
        }
    }

    fn make_match(anchor: &Anchor, score: f32) -> AnchorMatch<'_> {
        AnchorMatch { anchor, score, match_type: MatchType::Exact, matched_word: anchor.word.clone() }
    }

    #[test]
    fn test_no_signal_empty_matches() {
        assert!(MoralSignalDetector::detect(&[]).is_none());
    }

    #[test]
    fn test_no_signal_non_moral_anchors() {
        let anchor = make_anchor("math_function");
        let matches = [make_match(&anchor, 0.9)];
        assert!(MoralSignalDetector::detect(&matches).is_none());
    }

    #[test]
    fn test_no_signal_below_threshold() {
        let anchor = make_anchor("moral_care");
        let matches = [make_match(&anchor, 0.1)]; // intensity=0.1 < threshold=0.3
        assert!(MoralSignalDetector::detect(&matches).is_none());
    }

    #[test]
    fn test_signal_single_strong_anchor() {
        let anchor = make_anchor("moral_harm");
        let matches = [make_match(&anchor, 0.9)];
        let sig = MoralSignalDetector::detect(&matches).expect("should detect");
        assert_eq!(sig.dominant.anchor_id, "moral_harm");
        assert!(sig.intensity > MORAL_INTENSITY_THRESHOLD);
        assert!(sig.secondary.is_none());
        assert!(!sig.conflict_detected);
    }

    #[test]
    fn test_signal_two_anchors_selects_dominant() {
        let a1 = make_anchor("moral_care");
        let a2 = make_anchor("moral_fair");
        let matches = [make_match(&a1, 0.7), make_match(&a2, 0.4)];
        let sig = MoralSignalDetector::detect(&matches).expect("should detect");
        assert_eq!(sig.dominant.anchor_id, "moral_care");
        assert_eq!(sig.secondary.as_ref().unwrap().anchor_id, "moral_fair");
        assert!(!sig.conflict_detected); // care/fair не антагонисты
    }

    #[test]
    fn test_conflict_detected_care_vs_harm() {
        let a1 = make_anchor("moral_care");
        let a2 = make_anchor("moral_harm");
        let matches = [make_match(&a1, 0.8), make_match(&a2, 0.7)];
        let sig = MoralSignalDetector::detect(&matches).expect("should detect");
        assert!(sig.conflict_detected, "care vs harm should be antagonistic");
    }

    #[test]
    fn test_conflict_detected_fair_vs_betrayal() {
        let a1 = make_anchor("moral_fair");
        let a2 = make_anchor("moral_betrayal");
        let matches = [make_match(&a1, 0.6), make_match(&a2, 0.5)];
        let sig = MoralSignalDetector::detect(&matches).expect("should detect");
        assert!(sig.conflict_detected, "fair vs betrayal should be antagonistic");
    }

    #[test]
    fn test_conflict_symmetric_harm_vs_care() {
        // Порядок не должен влиять — dominant = harm (выше score)
        let a1 = make_anchor("moral_harm");
        let a2 = make_anchor("moral_care");
        let matches = [make_match(&a1, 0.9), make_match(&a2, 0.6)];
        let sig = MoralSignalDetector::detect(&matches).expect("should detect");
        assert!(sig.conflict_detected, "harm vs care should be antagonistic regardless of order");
    }

    #[test]
    fn test_intensity_saturates_at_one() {
        let a1 = make_anchor("moral_care");
        let a2 = make_anchor("moral_harm");
        let a3 = make_anchor("moral_fair");
        let matches = [make_match(&a1, 0.9), make_match(&a2, 0.9), make_match(&a3, 0.9)];
        let sig = MoralSignalDetector::detect(&matches).expect("should detect");
        assert!(sig.intensity <= 1.0, "intensity must be saturated at 1.0");
    }

    #[test]
    fn test_low_score_anchor_filtered() {
        let a1 = make_anchor("moral_purity");
        let a2 = make_anchor("moral_care");
        // a1 ниже MIN_MATCH_SCORE
        let matches = [make_match(&a1, 0.05), make_match(&a2, 0.8)];
        let sig = MoralSignalDetector::detect(&matches).expect("should detect");
        assert_eq!(sig.dominant.anchor_id, "moral_care");
        assert!(sig.secondary.is_none(), "purity below min score should be filtered");
    }

    #[test]
    fn test_all_seven_moral_anchors_accepted() {
        let ids = ["moral_care", "moral_harm", "moral_fair", "moral_betrayal",
                   "moral_loyalty", "moral_purity", "moral_desecration"];
        let anchors: Vec<Anchor> = ids.iter().map(|id| make_anchor(id)).collect();
        let matches: Vec<AnchorMatch<'_>> = anchors.iter().map(|a| make_match(a, 0.5)).collect();
        let sig = MoralSignalDetector::detect(&matches).expect("should detect with all 7 anchors");
        assert!(sig.intensity > 0.0);
    }
}
