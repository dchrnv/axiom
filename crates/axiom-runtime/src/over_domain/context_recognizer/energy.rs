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

/// Минимальная энергия для достоверной классификации подсистемы.
///
/// energy = 1 / (dist² + 1). При threshold = 5e-9 токен должен быть
/// не дальше ~14142 единиц от ближайшего якоря (sqrt(1/5e-9 - 1) ≈ 14142).
/// Низкий порог нужен потому что TextPerceptor даёт BLENDED позиции
/// (weighted avg нескольких якорей) — они дальше от каждого конкретного якоря.
/// FNV-токены (random in 32767³) в среднем на ~16000 единиц от любого кластера
/// → для них dominant определяется случайно, но classify_position отличает их
/// по меньшей уверенности относительно других подсистем.
const SUBSYSTEM_CONFIDENCE_THRESHOLD: f32 = 5e-9;

/// Определить доминирующую подсистему с порогом достоверности.
///
/// Возвращает Unknown если нет совпадений с достаточно высокой энергией.
/// Используется вместо dominant_subsystem в on_tick для фильтрации FNV-шума.
pub fn dominant_subsystem_confident(energies: &[SubsystemEnergy]) -> SubsystemId {
    energies
        .first()
        .filter(|e| e.energy >= SUBSYSTEM_CONFIDENCE_THRESHOLD)
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

/// Shell-boost factor: shell similarity can amplify distance-based energy by up to this fraction.
const SHELL_FACTOR: f32 = 0.3;

/// Extended subsystem reference: position + shell profile of each anchor.
pub type SubsystemShellRefs = HashMap<SubsystemId, Vec<([i16; 3], [u8; 8])>>;

/// Вычислить энергии подсистем с учётом Shell-близости.
///
/// Идентично `compute_energies`, но для каждого токена добавляется бонус:
///   `energy *= 1.0 + cosine_sim(nearest_ref_shell, subsystem_template_shell) * SHELL_FACTOR`
///
/// `shell_refs`: опорные точки + shell каждой подсистемы.
pub fn compute_energies_with_shell(
    maya_tokens: &[Token],
    shell_refs: &SubsystemShellRefs,
) -> Vec<SubsystemEnergy> {
    if shell_refs.is_empty() {
        return vec![];
    }

    // Precompute per-subsystem average shell template (f32 for cosine)
    let templates: HashMap<SubsystemId, [f32; 8]> = shell_refs
        .iter()
        .map(|(&ss, refs)| {
            let mut avg = [0f32; 8];
            for &(_, shell) in refs {
                for (a, &s) in avg.iter_mut().zip(shell.iter()) {
                    *a += s as f32;
                }
            }
            let n = refs.len() as f32;
            (ss, avg.map(|v| v / n))
        })
        .collect();

    let mut energies: HashMap<SubsystemId, (f32, u32)> = HashMap::new();

    for token in maya_tokens {
        for (&subsystem, refs) in shell_refs {
            // Find nearest reference by distance AND take its shell
            let mut min_dist2 = f32::MAX;
            let mut nearest_shell = [0u8; 8];
            for &(ref_pos, ref_shell) in refs {
                let d2 = sq_dist(token.position, ref_pos);
                if d2 < min_dist2 {
                    min_dist2 = d2;
                    nearest_shell = ref_shell;
                }
            }

            let base = token.mass as f32 / (min_dist2 + 1.0);

            // Shell bonus: cosine similarity of nearest anchor's shell vs subsystem template
            let shell_sim = if let Some(tmpl) = templates.get(&subsystem) {
                cosine_sim_8(nearest_shell, tmpl)
            } else {
                0.0
            };
            let contribution = base * (1.0 + shell_sim * SHELL_FACTOR);

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

    result.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap_or(std::cmp::Ordering::Equal));
    result
}

/// Cosine similarity between a u8 shell and an f32 template (both 8-dim). Returns [0, 1].
fn cosine_sim_8(a: [u8; 8], b: &[f32; 8]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(&ai, &bi)| ai as f32 * bi).sum();
    let norm_a: f32 = a.iter().map(|&ai| (ai as f32).powi(2)).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|&bi| bi.powi(2)).sum::<f32>().sqrt();
    if norm_a < 1e-6 || norm_b < 1e-6 {
        return 0.0;
    }
    (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
}

/// Определить подсистему для позиции по ближайшему кластеру (без токена).
///
/// Возвращает Unknown если расстояние до ближайшего якоря превышает порог
/// (FNV-хэши и случайные позиции отфильтровываются).
pub fn classify_position(
    position: [i16; 3],
    refs: &HashMap<SubsystemId, Vec<[i16; 3]>>,
) -> SubsystemId {
    let mut best_energy = SUBSYSTEM_CONFIDENCE_THRESHOLD;
    let mut best_sub = SubsystemId::Unknown;
    for (&sub, positions) in refs {
        let min_dist2 = positions
            .iter()
            .map(|&p| sq_dist_pub(position, p))
            .fold(f32::MAX, f32::min);
        let energy = 1.0 / (min_dist2 + 1.0);
        if energy > best_energy {
            best_energy = energy;
            best_sub = sub;
        }
    }
    best_sub
}

fn sq_dist_pub(a: [i16; 3], b: [i16; 3]) -> f32 {
    let dx = (a[0] as f32) - (b[0] as f32);
    let dy = (a[1] as f32) - (b[1] as f32);
    let dz = (a[2] as f32) - (b[2] as f32);
    dx * dx + dy * dy + dz * dz
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
    fn test_classify_position_exact_match() {
        let r = refs();
        // position exactly on prim_vline of Writing → should return Writing
        let sub = classify_position([10000i16, 3000, 12000], &r);
        assert_eq!(sub, SubsystemId::Writing, "exact match should return Writing");
    }

    #[test]
    fn test_classify_position_fnv_random_returns_unknown() {
        let r = refs();
        // random far position → low energy → Unknown
        let sub = classify_position([20000i16, 20000, 20000], &r);
        // dist² to nearest Writing: (20000-10000)²+(20000-3000)²+(20000-12000)² = 100M+289M+64M=453M
        // energy = 1/(453M+1) ≈ 2.2e-9 < threshold 5e-7 → Unknown
        assert_eq!(sub, SubsystemId::Unknown, "far position should return Unknown");
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
