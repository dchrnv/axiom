// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Waves V1.0 — внутренний ветер. Импульс изнутри без внешнего камня.
//
// Спецификация: docs/spec/Waves_Internal_Drive_V1_0.md
//
// Природа: WAKE-only, преимущественно в тишине. Молчит в DREAMING.
// Читает: DilemmaStore, SutraDepthStore, FrameWeaver candidates, FatigueStore.
// Пишет: UCL реактивации (ReinforceFrame) + хранит impulses для Sensorium.
//
// Три источника (§2):
//   A — незавершённые дилеммы ("та несостыковка не даёт покоя")
//   B — глубокий резонанс SutraDepth ("возвращаюсь к этой области")
//   C — почти-кристаллизованный кандидат ("хочется доглядеть Math+Time")
//
// Защиты от штормов (§6): вход перебивает, затухание повтора, fatigue-потолок,
// MAX_ACTIVE_IMPULSES=4, DREAM сбрасывает.

pub mod dominance;
pub mod impulse;
pub mod sources;
pub mod storms;

pub use dominance::DOMINANCE_THRESHOLD;
pub use impulse::{Impulse, ImpulseSource};
pub use storms::MAX_ACTIVE_IMPULSES;

use axiom_genome::types::{ModuleId, Permission, ResourceId};
use axiom_genome::{Genome, GenomeIndex};
use axiom_ucl::{OpCode, ReinforceFramePayload, UclCommand};
use std::sync::Arc;

use crate::over_domain::context_recognizer::ContextRecognizer;
use crate::over_domain::dream_phase::DreamPhaseState;
use crate::over_domain::weavers::FrameWeaver;
use crate::over_domain::axial_evaluator::AxialEvaluator;

/// Интервал тика Waves: каждые 19 тиков (простое число, ветер не суетлив).
pub const WAVES_TICK_INTERVAL: u64 = 19;

/// Снимок движка для чтения Waves.
pub struct WavesView<'a> {
    pub tick: u64,
    pub causal_time: u64,
    pub had_intake: bool,
    pub dream_phase: DreamPhaseState,
    pub context_recognizer: &'a ContextRecognizer,
    pub axial_evaluator: &'a AxialEvaluator,
    pub frame_weaver: &'a FrameWeaver,
}

/// Waves — источник внутреннего ветра системы.
///
/// Хранит `internal_dominance_factor` (реактивное↔когнитивное) и
/// `active_impulses` — текущие активные импульсы.
/// Sensorium читает эти поля через WavesView для выражения тяги.
#[derive(Debug, Default)]
pub struct Waves {
    /// Насколько система сейчас живёт изнутри против реакции на вход. 0.0..1.0.
    pub internal_dominance_factor: f32,
    /// Активные импульсы (не более MAX_ACTIVE_IMPULSES).
    pub active_impulses: Vec<Impulse>,
    /// Тик последнего срабатывания (для диагностики).
    pub last_fired_tick: u64,
    /// Число реактиваций выпущенных через UCL за всё время.
    pub total_reactivations: u64,
}

impl Waves {
    pub fn new() -> Self {
        Self::default()
    }

    /// Проверить права доступа из GENOME при boot.
    pub fn on_boot(&self, genome: &Arc<Genome>) -> Result<(), String> {
        let index = GenomeIndex::build(genome);
        let checks = [
            (ResourceId::ExperienceMemory, Permission::ReadWrite),
            (ResourceId::AshtiField, Permission::Read),
            (ResourceId::MayaOutput, Permission::Read),
        ];
        for (resource, required) in checks {
            if !index.check_access(ModuleId::Waves, resource, required) {
                return Err(format!("Waves: GENOME denied {resource:?}/{required:?}"));
            }
        }
        Ok(())
    }

    /// Основной цикл Waves. Вызывается из engine при t % WAVES_TICK_INTERVAL == 0 && Wake.
    ///
    /// Возвращает UCL-команды реактивации (ReinforceFrame для Source B и C).
    pub fn on_tick(&mut self, view: &WavesView<'_>) -> Vec<UclCommand> {
        // Обновить internal_dominance_factor.
        let max_fatigue = view
            .context_recognizer
            .fatigue_store()
            .iter()
            .map(|(_, f)| f.activation_load)
            .fold(0.0f32, f32::max);

        self.internal_dominance_factor = dominance::update(
            self.internal_dominance_factor,
            view.had_intake,
            max_fatigue,
            self.active_impulses.len(),
        );

        // Ниже порога — ветер не дует.
        if self.internal_dominance_factor < DOMINANCE_THRESHOLD {
            // Декрементируем raise_count у существующих (они всё ещё тянут, просто тихо).
            storms::apply_decay(&mut self.active_impulses);
            return Vec::new();
        }

        // Собрать новые кандидаты из трёх источников.
        let mut candidates = Vec::new();
        candidates.extend(sources::scan_dilemmas(view.context_recognizer, view.causal_time));
        candidates.extend(sources::scan_resonance(view.context_recognizer, view.causal_time));
        candidates.extend(sources::scan_unfinished(view.frame_weaver, view.causal_time));

        // Применить защиты: затухание старых + лимит.
        storms::apply_decay(&mut self.active_impulses);
        // Смержить новые кандидаты с активными (обновить силу если уже есть).
        self.merge_candidates(candidates);
        storms::limit(&mut self.active_impulses);

        self.last_fired_tick = view.tick;

        // Генерировать UCL реактивации только для Source B и C (у них sutra_id в EXPERIENCE).
        let ucl_commands = self.build_ucl_commands(view.causal_time);
        self.total_reactivations += ucl_commands.len() as u64;
        ucl_commands
    }

    /// Обработать пробуждение из DREAM — DREAM сбрасывает (утро мудренее).
    pub fn on_dream_wake(&mut self) {
        storms::dream_reset(&mut self.active_impulses);
    }

    fn merge_candidates(&mut self, new: Vec<Impulse>) {
        for candidate in new {
            // Если уже есть импульс с тем же target — обновить силу (берём max).
            if let Some(existing) = self
                .active_impulses
                .iter_mut()
                .find(|imp| imp.target_sutra_id == candidate.target_sutra_id)
            {
                existing.pull_strength = existing.pull_strength.max(candidate.pull_strength);
            } else {
                self.active_impulses.push(candidate);
            }
        }
    }

    fn build_ucl_commands(&self, command_id_base: u64) -> Vec<UclCommand> {
        let mut cmds = Vec::new();
        for (i, imp) in self.active_impulses.iter().enumerate() {
            // Source A (Dilemma): sutra_id = anchor_in_conflict, не гарантированно в EXPERIENCE.
            // Пропускаем UCL для Dilemma источника в V1.0 (цель может не быть Frame-анкером).
            if imp.source == ImpulseSource::Dilemma {
                continue;
            }
            if imp.target_sutra_id == 0 {
                continue;
            }
            // ReinforceFrame: поднять температуру Frame-анкера в EXPERIENCE.
            let payload = ReinforceFramePayload {
                anchor_id: imp.target_sutra_id,
                delta_mass: 0,
                delta_temperature: 8,
                reserved: [0; 42],
            };
            let cmd = build_reinforce_frame_cmd(
                command_id_base + i as u64,
                payload,
            );
            cmds.push(cmd);
        }
        cmds
    }
}

fn build_reinforce_frame_cmd(command_id: u64, payload: ReinforceFramePayload) -> UclCommand {
    let mut raw = [0u8; 48];
    raw[0..4].copy_from_slice(&payload.anchor_id.to_le_bytes());
    raw[4] = payload.delta_mass;
    raw[5] = payload.delta_temperature;
    UclCommand {
        payload: raw,
        command_id,
        target_id: payload.anchor_id,
        opcode: OpCode::ReinforceFrame as u16,
        priority: 64, // Low priority — ветер не перебивает важные команды
        flags: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waves_new_has_zero_factor() {
        let w = Waves::new();
        assert_eq!(w.internal_dominance_factor, 0.0);
        assert!(w.active_impulses.is_empty());
    }

    #[test]
    fn on_boot_passes_with_default_genome() {
        let genome = Genome::default_ashti_core();
        let w = Waves::new();
        assert!(w.on_boot(&Arc::new(genome)).is_ok());
    }

    #[test]
    fn dream_wake_resets_impulses() {
        let mut w = Waves::new();
        w.active_impulses.push(Impulse::new(
            ImpulseSource::Resonance, 42, 200, 0, None,
        ));
        w.on_dream_wake();
        // Сила уменьшилась (75% от 200 = 150).
        assert!(w.active_impulses[0].pull_strength <= 150);
    }

    #[test]
    fn merge_candidates_deduplicates() {
        let mut w = Waves::new();
        w.active_impulses.push(Impulse::new(
            ImpulseSource::Resonance, 10, 50, 0, None,
        ));
        // Тот же target с большей силой.
        let new_impulse = Impulse::new(ImpulseSource::Resonance, 10, 150, 0, None);
        w.merge_candidates(vec![new_impulse]);
        assert_eq!(w.active_impulses.len(), 1);
        assert_eq!(w.active_impulses[0].pull_strength, 150);
    }
}
