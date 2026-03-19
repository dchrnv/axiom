// AXIOM MODULE: MAYA PROCESSOR - Консолидация результатов ASHTI
//
// MAYA (Домен 10) - интерфейсный слой, который принимает результаты
// от всех 8 ASHTI доменов и формирует единый консолидированный ответ.
//
// Связанные спецификации:
// - docs/spec/Ashti_Core_v2_0.md (MAYA - Домен 10)
// - docs/spec/Domain V2.0.md

use crate::token::Token;
use crate::domain::DomainConfig;

/// MAYA процессор - консолидация результатов обработки
pub struct MayaProcessor;

impl MayaProcessor {
    /// Консолидирует результаты от 8 ASHTI доменов в единый токен
    ///
    /// Стратегия: усреднение с весами, медиана для позиции
    pub fn consolidate_results(ashti_results: Vec<Token>, _domain: &DomainConfig) -> Token {
        if ashti_results.is_empty() {
            return Token::default();
        }

        if ashti_results.len() == 1 {
            return ashti_results[0].clone();
        }

        let mut consolidated = ashti_results[0].clone();

        // Усредняем температуру
        consolidated.temperature = Self::average_u8(
            &ashti_results.iter().map(|t| t.temperature).collect::<Vec<_>>(),
        );

        // Усредняем массу
        consolidated.mass = Self::average_u8(
            &ashti_results.iter().map(|t| t.mass).collect::<Vec<_>>(),
        );

        // Усредняем valence
        consolidated.valence = Self::average_i8(
            &ashti_results.iter().map(|t| t.valence).collect::<Vec<_>>(),
        );

        // Медиана для позиции (устойчивость к выбросам)
        consolidated.position = Self::median_position(
            &ashti_results.iter().map(|t| t.position).collect::<Vec<_>>(),
        );

        // Усредняем velocity
        consolidated.velocity = Self::average_velocity(
            &ashti_results.iter().map(|t| t.velocity).collect::<Vec<_>>(),
        );

        // Мажоритарное голосование для state
        consolidated.state = Self::majority_state(
            &ashti_results.iter().map(|t| t.state).collect::<Vec<_>>(),
        );

        // OR для type_flags (если хотя бы один домен пометил)
        consolidated.type_flags = ashti_results
            .iter()
            .fold(0u16, |acc, t| acc | t.type_flags);

        consolidated
    }

    /// Усреднение u8 значений
    fn average_u8(values: &[u8]) -> u8 {
        if values.is_empty() {
            return 0;
        }
        let sum: u32 = values.iter().map(|&v| v as u32).sum();
        (sum / values.len() as u32) as u8
    }

    /// Усреднение i8 значений
    fn average_i8(values: &[i8]) -> i8 {
        if values.is_empty() {
            return 0;
        }
        let sum: i32 = values.iter().map(|&v| v as i32).sum();
        (sum / values.len() as i32) as i8
    }

    /// Усреднение velocity
    fn average_velocity(velocities: &[[i16; 3]]) -> [i16; 3] {
        if velocities.is_empty() {
            return [0, 0, 0];
        }

        let sum_x: i32 = velocities.iter().map(|v| v[0] as i32).sum();
        let sum_y: i32 = velocities.iter().map(|v| v[1] as i32).sum();
        let sum_z: i32 = velocities.iter().map(|v| v[2] as i32).sum();

        let count = velocities.len() as i32;

        [
            (sum_x / count) as i16,
            (sum_y / count) as i16,
            (sum_z / count) as i16,
        ]
    }

    /// Медиана позиции (устойчива к выбросам)
    fn median_position(positions: &[[i16; 3]]) -> [i16; 3] {
        if positions.is_empty() {
            return [0, 0, 0];
        }

        let mut xs: Vec<i16> = positions.iter().map(|p| p[0]).collect();
        let mut ys: Vec<i16> = positions.iter().map(|p| p[1]).collect();
        let mut zs: Vec<i16> = positions.iter().map(|p| p[2]).collect();

        xs.sort_unstable();
        ys.sort_unstable();
        zs.sort_unstable();

        let mid = positions.len() / 2;

        [xs[mid], ys[mid], zs[mid]]
    }

    /// Мажоритарное голосование для state
    fn majority_state(states: &[u8]) -> u8 {
        if states.is_empty() {
            return 0;
        }

        // Подсчитываем частоты
        let mut counts: std::collections::HashMap<u8, usize> =
            std::collections::HashMap::new();

        for &state in states {
            *counts.entry(state).or_insert(0) += 1;
        }

        // Находим наиболее частое значение
        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(state, _)| state)
            .unwrap_or(0)
    }

    /// Взвешенное усреднение (для будущего использования)
    ///
    /// Позволяет дать разным ASHTI доменам разные веса
    #[allow(dead_code)]
    fn weighted_average_u8(values: &[u8], weights: &[f32]) -> u8 {
        if values.is_empty() || weights.is_empty() || values.len() != weights.len() {
            return 0;
        }

        let weighted_sum: f32 = values
            .iter()
            .zip(weights.iter())
            .map(|(&v, &w)| v as f32 * w)
            .sum();

        let total_weight: f32 = weights.iter().sum();

        if total_weight > 0.0 {
            (weighted_sum / total_weight) as u8
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_token(id: u32, temp: u8, mass: u8, valence: i8) -> Token {
        let mut token = Token::default();
        token.sutra_id = id;
        token.temperature = temp;
        token.mass = mass;
        token.valence = valence;
        token.position = [(id % 100) as i16, (id % 100) as i16, 0];
        token.velocity = [10, 10, 10];
        token
    }

    fn create_test_domain() -> DomainConfig {
        let mut domain = DomainConfig::default();
        domain.domain_id = 10;
        domain.structural_role = 10; // MAYA
        domain
    }

    #[test]
    fn test_consolidate_empty() {
        let domain = create_test_domain();
        let result = MayaProcessor::consolidate_results(vec![], &domain);

        // Empty input returns default token
        assert_eq!(result.sutra_id, 0);
    }

    #[test]
    fn test_consolidate_single() {
        let domain = create_test_domain();
        let token = create_test_token(1, 100, 150, 10);

        let result = MayaProcessor::consolidate_results(vec![token.clone()], &domain);

        // Single input returns same token
        assert_eq!(result.temperature, token.temperature);
        assert_eq!(result.mass, token.mass);
        assert_eq!(result.valence, token.valence);
    }

    #[test]
    fn test_consolidate_identical() {
        let domain = create_test_domain();
        let token = create_test_token(1, 100, 150, 10);

        let results = vec![token.clone(); 8];
        let consolidated = MayaProcessor::consolidate_results(results, &domain);

        // Identical tokens produce identical result
        assert_eq!(consolidated.temperature, token.temperature);
        assert_eq!(consolidated.mass, token.mass);
        assert_eq!(consolidated.valence, token.valence);
    }

    #[test]
    fn test_consolidate_average() {
        let domain = create_test_domain();

        let tokens = vec![
            create_test_token(1, 100, 100, -10),
            create_test_token(2, 200, 200, 10),
        ];

        let consolidated = MayaProcessor::consolidate_results(tokens, &domain);

        // Should average temperature and mass
        assert_eq!(consolidated.temperature, 150); // (100 + 200) / 2
        assert_eq!(consolidated.mass, 150); // (100 + 200) / 2
        assert_eq!(consolidated.valence, 0); // (-10 + 10) / 2
    }

    #[test]
    fn test_consolidate_8_tokens() {
        let domain = create_test_domain();

        let tokens: Vec<Token> = (0..8)
            .map(|i| create_test_token(i, 100 + i as u8 * 10, 100, 0))
            .collect();

        let consolidated = MayaProcessor::consolidate_results(tokens, &domain);

        // Temperature should be averaged: (100 + 110 + 120 + 130 + 140 + 150 + 160 + 170) / 8
        // = 1080 / 8 = 135
        assert_eq!(consolidated.temperature, 135);
    }

    #[test]
    fn test_median_position() {
        let positions = vec![
            [10, 20, 30],
            [40, 50, 60],
            [70, 80, 90],
            [100, 110, 120],
            [130, 140, 150],
        ];

        let median = MayaProcessor::median_position(&positions);

        // Median of sorted values at index 2
        assert_eq!(median, [70, 80, 90]);
    }

    #[test]
    fn test_majority_state() {
        use crate::token::STATE_ACTIVE;

        let states = vec![
            STATE_ACTIVE,
            STATE_ACTIVE,
            STATE_ACTIVE,
            0,
            0,
        ];

        let majority = MayaProcessor::majority_state(&states);

        assert_eq!(majority, STATE_ACTIVE);
    }

    #[test]
    fn test_type_flags_or() {
        let domain = create_test_domain();

        let mut token1 = create_test_token(1, 100, 100, 0);
        token1.type_flags = 0b0001;

        let mut token2 = create_test_token(2, 100, 100, 0);
        token2.type_flags = 0b0010;

        let mut token3 = create_test_token(3, 100, 100, 0);
        token3.type_flags = 0b0100;

        let consolidated = MayaProcessor::consolidate_results(
            vec![token1, token2, token3],
            &domain,
        );

        // Should OR all flags: 0b0001 | 0b0010 | 0b0100 = 0b0111
        assert_eq!(consolidated.type_flags, 0b0111);
    }

    #[test]
    fn test_velocity_averaging() {
        let velocities = vec![
            [10, 20, 30],
            [20, 40, 60],
            [30, 60, 90],
        ];

        let avg = MayaProcessor::average_velocity(&velocities);

        assert_eq!(avg, [20, 40, 60]); // (10+20+30)/3, (20+40+60)/3, (30+60+90)/3
    }

    #[test]
    fn test_weighted_average() {
        let values = vec![100, 200];
        let weights = vec![0.25, 0.75];

        let result = MayaProcessor::weighted_average_u8(&values, &weights);

        // 100 * 0.25 + 200 * 0.75 = 25 + 150 = 175
        assert_eq!(result, 175);
    }

    #[test]
    fn test_consolidate_outliers() {
        let domain = create_test_domain();

        // Create tokens with one outlier
        let mut tokens: Vec<Token> = (0..7)
            .map(|i| create_test_token(i, 100, 100, 0))
            .collect();

        // Add outlier
        let mut outlier = create_test_token(7, 250, 250, 100);
        outlier.position = [1000, 1000, 1000];
        tokens.push(outlier);

        let consolidated = MayaProcessor::consolidate_results(tokens, &domain);

        // Average should be pulled slightly by outlier
        // (100*7 + 250) / 8 = 850 / 8 = 106
        assert!(consolidated.temperature > 100 && consolidated.temperature < 150);

        // Median position should be resistant to outlier
        assert!(consolidated.position[0] < 500); // Not pulled to 1000
    }
}
