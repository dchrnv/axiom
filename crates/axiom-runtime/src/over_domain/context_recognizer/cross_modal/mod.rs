// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CrossModalDetector V1.0 — детектор ко-активации Frame из разных модальностей.
//
// Источник: Cross_Modal_Binding_V1_0.md §3–4
//
// Алгоритм:
//   on_tick: сгруппировать active frame_ids по модальности (из modality_store).
//            Для пар (Text-Frame, Vision-Frame): инкрементировать CrossModalCandidate.
//            При достижении MIN_CROSS_MODAL_COACTIVATION: отправить в DREAM-очередь.
//
//   drain_pending_bond_commands: вызывается из DreamPhase.
//            Возвращает UCL BondTokens для каждого готового кандидата.
//            Очищает очередь — повторно не предлагается до отзыва.

pub mod candidate;

use std::collections::HashMap;

use axiom_core::FLAG_ACTIVE;
use axiom_experience::{Modality, ModalityStore};
use axiom_shell::link_types;
use axiom_ucl::{BondTokensPayload, OpCode, UclCommand};

use candidate::CrossModalCandidate;

pub use candidate::MIN_CROSS_MODAL_COACTIVATION;

/// Детектор cross-modal ко-активации.
pub struct CrossModalDetector {
    /// Активные кандидаты: (min_sutra_id, max_sutra_id) → candidate.
    candidates: HashMap<(u32, u32), CrossModalCandidate>,
    /// Кандидаты, достигшие порога, ожидающие DREAM Phase.
    pending_dream: Vec<CrossModalCandidate>,
    /// sutra_id уже созданных bond — не создавать повторно.
    existing_bonds: std::collections::HashSet<(u32, u32)>,
}

impl CrossModalDetector {
    pub fn new() -> Self {
        Self {
            candidates: HashMap::new(),
            pending_dream: Vec::new(),
            existing_bonds: std::collections::HashSet::new(),
        }
    }

    /// Обновить ко-активацию на текущем тике CR.
    ///
    /// Берёт список активных frame_ids и их модальности.
    /// Обновляет счётчики для пар разных модальностей.
    /// Готовые кандидаты перемещаются в pending_dream.
    pub fn update(
        &mut self,
        frame_ids: &[u32],
        modality_store: &ModalityStore,
        tick: u64,
    ) {
        if frame_ids.len() < 2 {
            return;
        }

        // Группировать по модальности
        let mut by_modality: HashMap<Modality, Vec<u32>> = HashMap::new();
        for &fid in frame_ids {
            by_modality.entry(modality_store.get(fid)).or_default().push(fid);
        }

        // Нужно минимум 2 разных модальности
        if by_modality.len() < 2 {
            return;
        }

        let modalities: Vec<(Modality, Vec<u32>)> = by_modality.into_iter().collect();

        // Пары только между разными модальностями
        for i in 0..modalities.len() {
            for j in (i + 1)..modalities.len() {
                let (mod_a, frames_a) = &modalities[i];
                let (mod_b, frames_b) = &modalities[j];
                for &fa in frames_a {
                    for &fb in frames_b {
                        let key = (fa.min(fb), fa.max(fb));
                        if self.existing_bonds.contains(&key) {
                            continue;
                        }
                        let cand = self.candidates.entry(key).or_insert_with(|| {
                            CrossModalCandidate::new(fa, fb, *mod_a, *mod_b, tick)
                        });
                        cand.increment(tick);
                        if cand.ready_for_dream {
                            let ready = self.candidates.remove(&key).unwrap();
                            self.existing_bonds.insert(key);
                            self.pending_dream.push(ready);
                        }
                    }
                }
            }
        }
    }

    /// Число активных кандидатов (не достигших порога).
    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    /// Число кандидатов, ожидающих DREAM.
    pub fn pending_count(&self) -> usize {
        self.pending_dream.len()
    }

    /// Число уже созданных bond.
    pub fn bond_count(&self) -> usize {
        self.existing_bonds.len()
    }

    /// Дрейнировать DREAM-очередь и сгенерировать UCL BondTokens команды.
    ///
    /// Вызывается из engine после DREAM-цикла.
    /// Каждый ready-кандидат → 1 BondTokens команда в EXPERIENCE.
    pub fn drain_pending_bond_commands(&mut self, exp_domain_id: u16) -> Vec<UclCommand> {
        if self.pending_dream.is_empty() {
            return vec![];
        }
        let mut cmds = Vec::with_capacity(self.pending_dream.len());
        for cand in self.pending_dream.drain(..) {
            let payload = BondTokensPayload {
                source_id: cand.frame_a,
                target_id: cand.frame_b,
                domain_id: exp_domain_id,
                link_type: link_types::CROSS_MODAL_BOND,
                strength: cand.normalized_strength(),
                conn_flags: FLAG_ACTIVE as u32,
                origin_domain: exp_domain_id,
                role_id: link_types::CROSS_MODAL_BOND,
                reserved: [0; 24],
            };
            cmds.push(UclCommand::new(OpCode::BondTokens, 0, 10, 0).with_payload(&payload));
        }
        cmds
    }

    /// Пометить bond как существующий (не создавать повторно).
    ///
    /// Вызывается если bond уже был создан (после reload).
    pub fn register_existing_bond(&mut self, frame_a: u32, frame_b: u32) {
        self.existing_bonds.insert((frame_a.min(frame_b), frame_a.max(frame_b)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_store(pairs: &[(u32, Modality)]) -> ModalityStore {
        let mut s = ModalityStore::new();
        for &(id, m) in pairs {
            s.insert(id, m);
        }
        s
    }

    #[test]
    fn test_no_update_single_modality() {
        let mut det = CrossModalDetector::new();
        let store = make_store(&[(1, Modality::Text), (2, Modality::Text)]);
        det.update(&[1, 2], &store, 0);
        assert_eq!(det.candidate_count(), 0);
    }

    #[test]
    fn test_cross_modal_pair_tracked() {
        let mut det = CrossModalDetector::new();
        let store = make_store(&[(1, Modality::Text), (2, Modality::Vision)]);
        det.update(&[1, 2], &store, 0);
        assert_eq!(det.candidate_count(), 1);
    }

    #[test]
    fn test_reaches_threshold_and_moves_to_pending() {
        let mut det = CrossModalDetector::new();
        let store = make_store(&[(10, Modality::Text), (20, Modality::Vision)]);
        // MIN_CROSS_MODAL_COACTIVATION = 50; each update increments by 1
        for tick in 0..MIN_CROSS_MODAL_COACTIVATION {
            det.update(&[10, 20], &store, tick as u64);
        }
        assert_eq!(det.candidate_count(), 0, "should have moved to pending");
        assert_eq!(det.pending_count(), 1);
    }

    #[test]
    fn test_no_duplicate_bond() {
        let mut det = CrossModalDetector::new();
        let store = make_store(&[(1, Modality::Text), (2, Modality::Vision)]);
        // Reach threshold
        for tick in 0..MIN_CROSS_MODAL_COACTIVATION {
            det.update(&[1, 2], &store, tick as u64);
        }
        let cmds = det.drain_pending_bond_commands(109);
        assert_eq!(cmds.len(), 1);
        // Continue updating — should NOT create duplicate candidate
        for tick in 50..100 {
            det.update(&[1, 2], &store, tick);
        }
        assert_eq!(det.candidate_count(), 0);
        assert_eq!(det.pending_count(), 0);
    }

    #[test]
    fn test_drain_generates_bond_tokens_command() {
        let mut det = CrossModalDetector::new();
        let store = make_store(&[(5, Modality::Text), (10, Modality::Vision)]);
        for tick in 0..MIN_CROSS_MODAL_COACTIVATION {
            det.update(&[5, 10], &store, tick as u64);
        }
        let cmds = det.drain_pending_bond_commands(109);
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].opcode, OpCode::BondTokens as u16);
    }

    #[test]
    fn test_drain_clears_queue() {
        let mut det = CrossModalDetector::new();
        let store = make_store(&[(1, Modality::Text), (2, Modality::Vision)]);
        for tick in 0..MIN_CROSS_MODAL_COACTIVATION {
            det.update(&[1, 2], &store, tick as u64);
        }
        let _ = det.drain_pending_bond_commands(109);
        let second = det.drain_pending_bond_commands(109);
        assert!(second.is_empty());
    }

    #[test]
    fn test_multiple_pairs() {
        let mut det = CrossModalDetector::new();
        let store = make_store(&[
            (1, Modality::Text),
            (2, Modality::Vision),
            (3, Modality::Vision),
        ]);
        det.update(&[1, 2, 3], &store, 0);
        // (1,2) and (1,3) should both be tracked
        assert_eq!(det.candidate_count(), 2);
    }
}
