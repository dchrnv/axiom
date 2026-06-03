// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Три источника внутреннего ветра — §2 спеки Waves_Internal_Drive_V1_0.md.
//
// Все три читают уже существующие данные "под углом что тянет".
// Waves не создаёт новый материал — поднимает лежащее на дне.

use crate::over_domain::context_recognizer::ContextRecognizer;
use crate::over_domain::weavers::FrameWeaver;
use super::impulse::{Impulse, ImpulseSource};

/// Минимальная интенсивность дилеммы для включения в импульс.
const MIN_DILEMMA_INTENSITY: f32 = 0.2;
/// Минимальная глубина Frame для источника B (резонанс).
const MIN_RESONANCE_DEPTH: u16 = 500;
/// Минимальная устойчивость кандидата для источника C (незавершённость).
const MIN_CANDIDATE_STABILITY: u32 = 3;

// ——— Источник A: незавершённые дилеммы ———————————————————————————————————

/// Сканировать активные дилеммы → импульсы тяги вернуться к неразрешённому.
///
/// Чем выше intensity и дольше горит — тем сильнее тянет.
pub fn scan_dilemmas(cr: &ContextRecognizer, causal_time: u64) -> Vec<Impulse> {
    let mut impulses = Vec::new();
    let store = cr.dilemma_store();

    for record in &store.active {
        if record.intensity < MIN_DILEMMA_INTENSITY {
            continue;
        }
        // pull_strength: intensity × 200, age-бонус (дольше горит — сильнее тянет).
        let age = causal_time.saturating_sub(record.detected_at_tick);
        let age_factor = (1.0 + (age as f32 / 5000.0).min(1.0)) as f32;
        let strength = ((record.intensity * 200.0 * age_factor).min(255.0)) as u8;

        // Target: первый anchor в конфликте (или 0 — Waves всё равно запишет импульс).
        let target = record.anchors_in_conflict.first().copied().unwrap_or(0);

        impulses.push(Impulse::new(
            ImpulseSource::Dilemma,
            target,
            strength,
            record.detected_at_tick,
            None,
        ));
    }
    impulses
}

// ——— Источник B: глубокий резонанс ——————————————————————————————————————

/// Сканировать SutraDepthStore через known frame ids → Frame с глубокой укоренённостью.
///
/// Глубоко укоренённое тянет вернуться даже без повода.
pub fn scan_resonance(cr: &ContextRecognizer, causal_time: u64) -> Vec<Impulse> {
    let mut impulses = Vec::new();
    let depth_store = cr.depth_store();

    for &sutra_id in cr.all_known_frame_ids() {
        let Some(entry) = depth_store.get(sutra_id) else { continue };

        let max_depth = entry.max_depth();
        if max_depth < MIN_RESONANCE_DEPTH || entry.is_primitive() {
            continue;
        }

        // Сила = глубина / 200, усиленная reactivation_count.
        let base = ((max_depth as f32 / 200.0).min(255.0)) as u8;
        let reactivation_bonus = (entry.reactivation_count / 5).min(50) as u8;
        let strength = base.saturating_add(reactivation_bonus);

        // Октант: берём октант с максимальной глубиной.
        let octant = entry
            .depth_per_octant
            .iter()
            .enumerate()
            .max_by_key(|(_, &d)| d)
            .map(|(i, _)| i as u8);

        impulses.push(Impulse::new(
            ImpulseSource::Resonance,
            sutra_id,
            strength,
            causal_time,
            octant,
        ));
    }
    impulses
}

// ——— Источник C: незавершённая связь ——————————————————————————————————————

/// Сканировать FrameWeaver candidates → почти-связанные паттерны (хочется достроить).
///
/// stability_count и confidence = насколько близко к кристаллизации.
pub fn scan_unfinished(fw: &FrameWeaver, causal_time: u64) -> Vec<Impulse> {
    let mut impulses = Vec::new();

    for candidate in fw.iter_candidates() {
        if candidate.stability_count < MIN_CANDIDATE_STABILITY {
            continue;
        }

        // pull_strength: confidence × 150 + stability_count / 2 (близко → сильнее тянет).
        let conf_part = (candidate.confidence * 150.0).min(200.0) as u8;
        let stab_part = (candidate.stability_count / 2).min(55) as u8;
        let strength = conf_part.saturating_add(stab_part);

        // anchor_id кандидата (proposed sutra_id).
        let anchor_id = FrameWeaver::proposed_id_from_lineage_hash(candidate.lineage_hash);

        impulses.push(Impulse::new(
            ImpulseSource::Unfinished,
            anchor_id,
            strength,
            causal_time,
            None,
        ));
    }
    impulses
}
