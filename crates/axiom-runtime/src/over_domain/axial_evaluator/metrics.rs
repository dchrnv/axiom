// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Метрики AxialEvaluator — детерминированные, целочисленные, 0..255.
// Источник: AxialEvaluator_V1_0.md §4

use axiom_core::{Connection, Token};

/// Энтропия позиций участников Frame (X-ось: Дионис/Аполлон).
///
/// Высокая дисперсия позиций → высокая энтропия → Дионис.
/// Низкая дисперсия (одинаковые паттерны) → Аполлон.
pub fn entropy_score(positions: &[[i16; 3]]) -> u8 {
    if positions.len() < 2 {
        return 0;
    }
    let n = positions.len() as f32;
    let mean = [
        positions.iter().map(|p| p[0] as f32).sum::<f32>() / n,
        positions.iter().map(|p| p[1] as f32).sum::<f32>() / n,
        positions.iter().map(|p| p[2] as f32).sum::<f32>() / n,
    ];
    let variance: f32 = positions
        .iter()
        .map(|p| {
            let dx = p[0] as f32 - mean[0];
            let dy = p[1] as f32 - mean[1];
            let dz = p[2] as f32 - mean[2];
            dx * dx + dy * dy + dz * dz
        })
        .sum::<f32>()
        / n;
    // Max variance ≈ 32767² * 3 ≈ 3.2e9; normalize against 1e8 (practical ceiling)
    ((variance / 100_000_000.0).min(1.0) * 255.0) as u8
}

/// Плотность графа связей между участниками Frame (Y-ось: Эрос/Танатос).
///
/// Высокая плотность → много связей → Эрос (когерентность, жизнь).
pub fn graph_density(participant_ids: &[u32], connections: &[Connection]) -> u8 {
    let n = participant_ids.len();
    if n < 2 {
        return 0;
    }
    let max_possible = n * (n - 1);
    let actual = connections
        .iter()
        .filter(|c| {
            participant_ids.contains(&c.source_id) && participant_ids.contains(&c.target_id)
        })
        .count();
    ((actual * 255) / max_possible.max(1)).min(255) as u8
}

/// Оценка валентности участников Frame (доп. вклад в Y-ось).
///
/// Возвращает (eros_add, thanatos_add) — дополнительные вклады на каждый полюс.
pub fn valence_score(tokens: &[Token]) -> (u8, u8) {
    if tokens.is_empty() {
        return (0, 0);
    }
    let (pos, neg): (i32, i32) = tokens.iter().fold((0, 0), |(p, n), t| {
        if t.valence > 0 {
            (p + t.valence as i32, n)
        } else {
            (p, n + (-t.valence) as i32)
        }
    });
    let total = (pos + neg).max(1) as f32;
    let pos_score = ((pos as f32 / total) * 128.0) as u8;
    let neg_score = ((neg as f32 / total) * 128.0) as u8;
    (pos_score, neg_score)
}

/// Воля-оценка участников Frame (Z-ось: Воля/Ничто).
///
/// Высокая mass * temperature → высокая энергетика → Воля.
pub fn will_score(tokens: &[Token]) -> u8 {
    if tokens.is_empty() {
        return 0;
    }
    let total_energy: u32 = tokens
        .iter()
        .map(|t| (t.mass as u32) * (t.temperature as u32))
        .sum();
    let max_per_token: u32 = 255 * 255;
    let avg_energy = total_energy / tokens.len() as u32;
    ((avg_energy * 255) / max_per_token.max(1)).min(255) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_identical_positions_is_zero() {
        let positions = vec![[10000i16, 10000, 10000]; 4];
        assert_eq!(entropy_score(&positions), 0);
    }

    #[test]
    fn test_entropy_single_position_is_zero() {
        assert_eq!(entropy_score(&[[1000i16, 2000, 3000]]), 0);
    }

    #[test]
    fn test_entropy_spread_is_nonzero() {
        let positions = vec![
            [0i16, 0, 0],
            [30000, 30000, 30000],
            [-30000, -30000, -30000],
        ];
        assert!(entropy_score(&positions) > 0);
    }

    #[test]
    fn test_graph_density_no_participants() {
        assert_eq!(graph_density(&[], &[]), 0);
    }

    #[test]
    fn test_graph_density_single_participant() {
        assert_eq!(graph_density(&[1], &[]), 0);
    }

    #[test]
    fn test_will_score_empty() {
        assert_eq!(will_score(&[]), 0);
    }

    #[test]
    fn test_valence_balanced() {
        let (p, n) = valence_score(&[]);
        assert_eq!(p, 0);
        assert_eq!(n, 0);
    }
}
