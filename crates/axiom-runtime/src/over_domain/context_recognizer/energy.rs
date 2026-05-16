// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Подсчёт энергии подсистем из MAYA-токенов.
// Источник: ContextRecognizer_V5_0.md §1, §5
//
// Энергия подсистемы = сколько активной "массы" MAYA находится вблизи примитивов подсистемы.
// Ближе → больше вклад. Масса токена умножает вклад.

use axiom_core::Token;
use axiom_experience::SubsystemId;
use std::collections::HashMap;

/// Вклад одной подсистемы в текущий контекст.
#[derive(Debug, Clone)]
pub struct SubsystemEnergy {
    pub subsystem: SubsystemId,
    /// Суммарная энергия (выше = подсистема активнее)
    pub energy: f32,
    /// Число токенов внёсших вклад
    pub contributing_tokens: u32,
}

/// Вычислить энергии подсистем для набора MAYA-токенов.
///
/// Для каждого токена находим ближайшую опорную точку каждой подсистемы.
/// Вклад токена = `token.mass / (distance² + 1.0)`.
///
/// `reference_positions`: позиции примитивов-якорей каждой подсистемы.
pub fn compute_energies(
    maya_tokens: &[Token],
    reference_positions: &HashMap<SubsystemId, Vec<[i16; 3]>>,
) -> Vec<SubsystemEnergy> {
    let mut energies: HashMap<SubsystemId, (f32, u32)> = HashMap::new();

    for token in maya_tokens {
        for (&subsystem, positions) in reference_positions {
            let min_dist2 = positions
                .iter()
                .map(|&ref_pos| sq_dist(token.position, ref_pos))
                .fold(f32::MAX, f32::min);
            let contribution = token.mass as f32 / (min_dist2 + 1.0);
            let entry = energies.entry(subsystem).or_insert((0.0, 0));
            entry.0 += contribution;
            entry.1 += 1;
        }
    }

    let mut result: Vec<SubsystemEnergy> = energies
        .into_iter()
        .map(|(s, (e, c))| SubsystemEnergy {
            subsystem: s,
            energy: e,
            contributing_tokens: c,
        })
        .collect();

    // Sort by energy descending
    result.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap_or(std::cmp::Ordering::Equal));
    result
}

/// Определить доминирующую подсистему из набора энергий.
///
/// Возвращает SubsystemId::Unknown если список пуст или первая энергия нулевая.
pub fn dominant_subsystem(energies: &[SubsystemEnergy]) -> SubsystemId {
    energies
        .first()
        .filter(|e| e.energy > 0.0)
        .map(|e| e.subsystem)
        .unwrap_or(SubsystemId::Unknown)
}

/// Конвертировать энергии в веса 0..255.
pub fn energies_to_weights(energies: &[SubsystemEnergy]) -> HashMap<SubsystemId, u8> {
    let total: f32 = energies.iter().map(|e| e.energy).sum();
    if total <= 0.0 {
        return HashMap::new();
    }
    energies
        .iter()
        .map(|e| {
            let w = ((e.energy / total) * 255.0).min(255.0) as u8;
            (e.subsystem, w)
        })
        .collect()
}

fn sq_dist(a: [i16; 3], b: [i16; 3]) -> f32 {
    let dx = (a[0] as f32) - (b[0] as f32);
    let dy = (a[1] as f32) - (b[1] as f32);
    let dz = (a[2] as f32) - (b[2] as f32);
    dx * dx + dy * dy + dz * dz
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tok(pos: [i16; 3], mass: u8) -> Token {
        let mut t = Token::new(1, 10, pos, 0);
        t.mass = mass;
        t
    }

    fn refs() -> HashMap<SubsystemId, Vec<[i16; 3]>> {
        let mut m = HashMap::new();
        m.insert(SubsystemId::Writing, vec![[0i16, 0, 6000], [10000, 3000, 12000]]);
        m.insert(SubsystemId::Mathematics, vec![[5000i16, 0, 8000], [12000, 10000, 9000]]);
        m
    }

    #[test]
    fn test_empty_tokens_returns_zero_energies() {
        let r = refs();
        let result = compute_energies(&[], &r);
        // all energies should be 0 or empty
        assert!(result.iter().all(|e| e.energy == 0.0));
    }

    #[test]
    fn test_token_near_writing_primitive_boosts_writing() {
        let r = refs();
        // Token near writing primitive prim_dot [0,0,6000]
        let tokens = vec![tok([0, 0, 6000], 200)];
        let result = compute_energies(&tokens, &r);
        let writing = result.iter().find(|e| e.subsystem == SubsystemId::Writing);
        let math = result.iter().find(|e| e.subsystem == SubsystemId::Mathematics);
        assert!(writing.is_some());
        assert!(math.is_some());
        assert!(writing.unwrap().energy > math.unwrap().energy);
    }

    #[test]
    fn test_dominant_subsystem_empty() {
        assert_eq!(dominant_subsystem(&[]), SubsystemId::Unknown);
    }

    #[test]
    fn test_dominant_subsystem_zero_energy() {
        let e = vec![SubsystemEnergy {
            subsystem: SubsystemId::Writing,
            energy: 0.0,
            contributing_tokens: 0,
        }];
        assert_eq!(dominant_subsystem(&e), SubsystemId::Unknown);
    }

    #[test]
    fn test_energies_to_weights_empty() {
        assert!(energies_to_weights(&[]).is_empty());
    }

    #[test]
    fn test_energies_to_weights_sums_to_255() {
        let energies = vec![
            SubsystemEnergy { subsystem: SubsystemId::Writing, energy: 100.0, contributing_tokens: 1 },
            SubsystemEnergy { subsystem: SubsystemId::Mathematics, energy: 100.0, contributing_tokens: 1 },
        ];
        let weights = energies_to_weights(&energies);
        let sum: u32 = weights.values().map(|&w| w as u32).sum();
        // Both should be ~127-128, sum ~254-256 range
        assert!(sum >= 254 && sum <= 256);
    }

    #[test]
    fn test_conflict_two_active_subsystems() {
        let mut r: HashMap<SubsystemId, Vec<[i16; 3]>> = HashMap::new();
        r.insert(SubsystemId::Writing, vec![[0i16, 0, 6000]]);
        r.insert(SubsystemId::Mathematics, vec![[0i16, 0, 6000]]); // same position = equal energy
        let tokens = vec![tok([0, 0, 6000], 100)];
        let result = compute_energies(&tokens, &r);
        // Both subsystems should have equal energy (same distance to same position)
        assert_eq!(result.len(), 2);
        assert!((result[0].energy - result[1].energy).abs() < 1e-3);
    }
}
