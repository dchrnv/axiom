// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DilemmaStore V1.1 — хранилище активных когнитивных дилемм.
//
// Источник: docs/architecture/primitives/Dilemmas_V1_0.md §3
//
// Инварианты:
//   - Только Type III (ValueConflict), IV (OntologicalConflict), V (Axiogenic) хранятся в active.
//   - Максимум 8 активных одновременно (MAX_ACTIVE).
//   - resolved хранит последние 64 записи (ring-буфер, MAX_RESOLVED).
//   - Type V создаётся только в DREAM Phase — DilemmaStore сам по себе не проверяет,
//     но DilemmaDetector (V2.0) обязан не вызывать push_active(Axiogenic) вне DREAM.
//
// V1.1: После разрешения дилемма кристаллизуется в EXPERIENCE как Frame-анкер.
//   Путь: DilemmaRecord resolved → pending_crystallizations
//         → caller вызывает crystallize_to_experience_commands()
//         → inject Frame анкер в EXPERIENCE (STATE_ACTIVE)
//         → FrameWeaver решает когда промотировать в SUTRA.

use std::collections::VecDeque;

use axiom_core::{FLAG_ACTIVE, FRAME_CATEGORY_SYNTAX, STATE_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_shell::link_types;
use axiom_ucl::{flags as ucl_flags, BondTokensPayload, InjectFrameAnchorPayload, OpCode, UclCommand};

/// Максимальное число одновременно активных дилемм.
pub const MAX_ACTIVE: usize = 8;

/// Размер ring-буфера разрешённых дилемм.
pub const MAX_RESOLVED: usize = 64;

/// Тип дилеммы по пятиуровневой таксономии (Dilemmas_V1_0.md §2).
///
/// Type I и II не хранятся в DilemmaStore — они обрабатываются на лету.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DilemmaType {
    /// Type I: конфликт данных (противоречивые факты). Не хранится.
    DataConflict,
    /// Type II: конфликт ресурсов (tradeoff без ценностного измерения). Не хранится.
    ResourceTradeoff,
    /// Type III: конфликт ценностей (val_X vs val_Y или moral_X vs val_Y).
    ValueConflict,
    /// Type IV: онтологический конфликт (несовместимые модели мира).
    OntologicalConflict,
    /// Type V: аксиогенная дилемма (требует рождения новой ценности). Только DREAM Phase.
    Axiogenic,
}

impl DilemmaType {
    /// Только Type III, IV, V подлежат хранению в DilemmaStore.
    pub fn is_storable(self) -> bool {
        matches!(self, DilemmaType::ValueConflict | DilemmaType::OntologicalConflict | DilemmaType::Axiogenic)
    }
}

/// Способ разрешения дилеммы.
#[derive(Debug, Clone)]
pub enum DilemmaResolution {
    /// Type I: данные уточнены, противоречие снято.
    DataClarified,
    /// Type II: выбрана точка Парето.
    ParetoCompromise,
    /// Type III: контекстуальный приоритет — один якорь победил.
    ContextualPriority { winner: u32 },
    /// Type IV: оба якоря дополнительны, не противоречат (дополнительность Бора).
    Complementarity,
    /// Type V: аксиогенез — создана новая ценность.
    NewValueCreated { new_anchor_id: u32 },
}

/// Запись об одной активной или разрешённой дилемме.
#[derive(Debug, Clone)]
pub struct DilemmaRecord {
    /// Уникальный ID записи (монотонно возрастает внутри DilemmaStore).
    pub id: u64,
    pub dilemma_type: DilemmaType,
    /// sutra_id якорей, находящихся в конфликте.
    pub anchors_in_conflict: Vec<u32>,
    /// Тик обнаружения.
    pub detected_at_tick: u64,
    /// Интенсивность конфликта 0.0..1.0.
    pub intensity: f32,
    /// Разрешена ли дилемма.
    pub resolved: bool,
    /// Способ разрешения (None пока active).
    pub resolution: Option<DilemmaResolution>,
}

/// Хранилище когнитивных дилемм.
///
/// Аналог FatigueStore по структуре.
/// Детектируется DilemmaDetector (V2.0), хранится здесь.
pub struct DilemmaStore {
    /// Активные нерешённые дилеммы (только Type III–V). Максимум MAX_ACTIVE.
    pub active: Vec<DilemmaRecord>,
    /// Ring-буфер разрешённых дилемм (последние MAX_RESOLVED).
    pub resolved: VecDeque<DilemmaRecord>,
    /// Очередь записей ожидающих кристаллизации в EXPERIENCE.
    /// Дрейнится caller'ом через drain_pending_crystallizations().
    pending_crystallizations: Vec<DilemmaRecord>,
    next_id: u64,
}

impl DilemmaStore {
    pub fn new() -> Self {
        Self {
            active: Vec::with_capacity(MAX_ACTIVE),
            resolved: VecDeque::with_capacity(MAX_RESOLVED),
            pending_crystallizations: Vec::new(),
            next_id: 1,
        }
    }

    /// Добавить новую дилемму в active.
    ///
    /// Возвращает `Some(id)` при успехе.
    /// Возвращает `None` если:
    ///   - тип не storable (Type I/II)
    ///   - активных уже MAX_ACTIVE
    pub fn push_active(
        &mut self,
        dilemma_type: DilemmaType,
        anchors_in_conflict: Vec<u32>,
        detected_at_tick: u64,
        intensity: f32,
    ) -> Option<u64> {
        if !dilemma_type.is_storable() {
            return None;
        }
        if self.active.len() >= MAX_ACTIVE {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.active.push(DilemmaRecord {
            id,
            dilemma_type,
            anchors_in_conflict,
            detected_at_tick,
            intensity: intensity.clamp(0.0, 1.0),
            resolved: false,
            resolution: None,
        });
        Some(id)
    }

    /// Разрешить дилемму по id — перемещает из active в resolved ring-буфер.
    ///
    /// Разрешённая запись также попадает в pending_crystallizations:
    /// caller должен дрейнить её через drain_pending_crystallizations()
    /// и инжектировать Frame анкер в EXPERIENCE.
    ///
    /// Возвращает `true` при успехе, `false` если id не найден.
    pub fn resolve(&mut self, id: u64, resolution: DilemmaResolution) -> bool {
        let Some(pos) = self.active.iter().position(|r| r.id == id) else {
            return false;
        };
        let mut record = self.active.remove(pos);
        record.resolved = true;
        record.resolution = Some(resolution);
        if self.resolved.len() >= MAX_RESOLVED {
            self.resolved.pop_front();
        }
        self.pending_crystallizations.push(record.clone());
        self.resolved.push_back(record);
        true
    }

    /// Дрейнить очередь кристаллизации.
    ///
    /// Caller берёт эти записи и вызывает crystallize_to_experience_commands()
    /// для каждой, передавая позицию конфликтующих якорей из domain state.
    pub fn drain_pending_crystallizations(&mut self) -> Vec<DilemmaRecord> {
        std::mem::take(&mut self.pending_crystallizations)
    }

    /// Есть ли записи ожидающие кристаллизации.
    pub fn has_pending_crystallizations(&self) -> bool {
        !self.pending_crystallizations.is_empty()
    }

    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    pub fn get_active(&self, id: u64) -> Option<&DilemmaRecord> {
        self.active.iter().find(|r| r.id == id)
    }

    pub fn get_active_mut(&mut self, id: u64) -> Option<&mut DilemmaRecord> {
        self.active.iter_mut().find(|r| r.id == id)
    }

    /// Обновить интенсивность активной дилеммы.
    pub fn update_intensity(&mut self, id: u64, intensity: f32) -> bool {
        if let Some(r) = self.get_active_mut(id) {
            r.intensity = intensity.clamp(0.0, 1.0);
            true
        } else {
            false
        }
    }

    /// Проверить что Type V не создаётся при capacity == MAX_ACTIVE.
    pub fn is_at_capacity(&self) -> bool {
        self.active.len() >= MAX_ACTIVE
    }
}

impl Default for DilemmaStore {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Кристаллизация дилемм в EXPERIENCE
// ============================================================================

/// Сгенерировать UCL-команды для кристаллизации разрешённой дилеммы в EXPERIENCE.
///
/// Caller отвечает за передачу корректного `position` — центроида позиций
/// конфликтующих якорей из domain state. Если позиции неизвестны → [0; 3].
///
/// Генерирует:
///   1. InjectToken (Frame-анкер) в `experience_domain_id`
///   2. BondTokens — по одной связи на каждый конфликтующий якорь
pub fn crystallize_to_experience_commands(
    record: &DilemmaRecord,
    position: [i16; 3],
    experience_domain_id: u16,
) -> Vec<UclCommand> {
    debug_assert!(record.resolved, "only resolved dilemmas should be crystallized");

    let lineage_hash = dilemma_lineage_hash(record);
    let proposed_sutra_id = ((lineage_hash >> 32) as u32) | 0x8000_0000;

    let mass = (50.0 + record.intensity * 150.0) as u8;
    let temperature = match record.dilemma_type {
        DilemmaType::ValueConflict => 100u8,
        DilemmaType::OntologicalConflict => 130u8,
        DilemmaType::Axiogenic => 180u8,
        // Type I/II не должны попадать сюда, но на случай graceful degradation
        _ => 80u8,
    };
    let valence = resolution_valence(record.resolution.as_ref());

    let anchor_payload = InjectFrameAnchorPayload {
        lineage_hash,
        proposed_sutra_id,
        target_domain_id: experience_domain_id,
        type_flags: TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX,
        position,
        state: STATE_ACTIVE,
        mass,
        temperature,
        valence,
        reserved: [0; 22],
    };
    let anchor_cmd =
        UclCommand::new(OpCode::InjectToken, 0, 10, ucl_flags::FRAME_ANCHOR)
            .with_payload(&anchor_payload);

    let mut cmds = vec![anchor_cmd];

    for &anchor_id in &record.anchors_in_conflict {
        let bond_payload = BondTokensPayload {
            source_id: proposed_sutra_id,
            target_id: anchor_id,
            domain_id: experience_domain_id,
            link_type: link_types::SYNTACTIC_COPULA_LINK,
            strength: record.intensity,
            conn_flags: FLAG_ACTIVE as u32,
            origin_domain: experience_domain_id,
            role_id: link_types::SYNTACTIC_COPULA_LINK,
            reserved: [0; 24],
        };
        cmds.push(UclCommand::new(OpCode::BondTokens, 0, 10, 0).with_payload(&bond_payload));
    }

    cmds
}

/// FNV-1a хэш DilemmaRecord: сортированные anchors_in_conflict + discriminant типа.
fn dilemma_lineage_hash(record: &DilemmaRecord) -> u64 {
    let type_disc = record.dilemma_type as u8 as u64;
    let mut sorted = record.anchors_in_conflict.clone();
    sorted.sort_unstable();
    let mut h: u64 = 0xcbf29ce484222325;
    h ^= type_disc;
    h = h.wrapping_mul(0x100000001b3);
    for id in sorted {
        h ^= id as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Валентность анкера по типу разрешения (−128..127).
fn resolution_valence(resolution: Option<&DilemmaResolution>) -> i8 {
    match resolution {
        Some(DilemmaResolution::DataClarified) => 0,
        Some(DilemmaResolution::ParetoCompromise) => 5,
        Some(DilemmaResolution::ContextualPriority { .. }) => 10,
        Some(DilemmaResolution::Complementarity) => 20,
        Some(DilemmaResolution::NewValueCreated { .. }) => 30,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_storable_types() {
        let mut store = DilemmaStore::new();
        let id = store.push_active(DilemmaType::ValueConflict, vec![1, 2], 100, 0.5);
        assert!(id.is_some());
        assert_eq!(store.active_count(), 1);

        let id = store.push_active(DilemmaType::OntologicalConflict, vec![3, 4], 101, 0.7);
        assert!(id.is_some());

        let id = store.push_active(DilemmaType::Axiogenic, vec![5], 102, 0.9);
        assert!(id.is_some());
        assert_eq!(store.active_count(), 3);
    }

    #[test]
    fn test_push_rejects_type_i_and_ii() {
        let mut store = DilemmaStore::new();
        assert!(store.push_active(DilemmaType::DataConflict, vec![1], 1, 0.5).is_none());
        assert!(store.push_active(DilemmaType::ResourceTradeoff, vec![2], 2, 0.5).is_none());
        assert_eq!(store.active_count(), 0);
    }

    #[test]
    fn test_capacity_limit() {
        let mut store = DilemmaStore::new();
        for i in 0..MAX_ACTIVE {
            let r = store.push_active(DilemmaType::ValueConflict, vec![i as u32], i as u64, 0.5);
            assert!(r.is_some(), "slot {i} should be accepted");
        }
        assert!(store.is_at_capacity());
        let overflow = store.push_active(DilemmaType::ValueConflict, vec![99], 99, 0.5);
        assert!(overflow.is_none(), "should reject when at capacity");
    }

    #[test]
    fn test_resolve_moves_to_resolved() {
        let mut store = DilemmaStore::new();
        let id = store.push_active(DilemmaType::ValueConflict, vec![1, 2], 10, 0.6).unwrap();
        assert_eq!(store.active_count(), 1);
        assert_eq!(store.resolved.len(), 0);

        let ok = store.resolve(id, DilemmaResolution::ContextualPriority { winner: 1 });
        assert!(ok);
        assert_eq!(store.active_count(), 0);
        assert_eq!(store.resolved.len(), 1);
        assert!(store.resolved[0].resolved);
    }

    #[test]
    fn test_resolve_unknown_id_returns_false() {
        let mut store = DilemmaStore::new();
        assert!(!store.resolve(999, DilemmaResolution::Complementarity));
    }

    #[test]
    fn test_resolved_ring_buffer_caps() {
        let mut store = DilemmaStore::new();
        for i in 0..=(MAX_RESOLVED + 5) as u64 {
            if let Some(id) = store.push_active(DilemmaType::ValueConflict, vec![], i, 0.1) {
                store.resolve(id, DilemmaResolution::Complementarity);
            }
        }
        assert_eq!(store.resolved.len(), MAX_RESOLVED);
    }

    #[test]
    fn test_ids_are_unique_and_monotone() {
        let mut store = DilemmaStore::new();
        let id1 = store.push_active(DilemmaType::ValueConflict, vec![], 1, 0.5).unwrap();
        let id2 = store.push_active(DilemmaType::OntologicalConflict, vec![], 2, 0.5).unwrap();
        assert!(id2 > id1);
    }

    #[test]
    fn test_update_intensity_clamps() {
        let mut store = DilemmaStore::new();
        let id = store.push_active(DilemmaType::ValueConflict, vec![], 1, 0.5).unwrap();
        assert!(store.update_intensity(id, 1.5));
        assert_eq!(store.get_active(id).unwrap().intensity, 1.0);
        assert!(store.update_intensity(id, -0.5));
        assert_eq!(store.get_active(id).unwrap().intensity, 0.0);
    }

    #[test]
    fn test_dilemma_type_is_storable() {
        assert!(!DilemmaType::DataConflict.is_storable());
        assert!(!DilemmaType::ResourceTradeoff.is_storable());
        assert!(DilemmaType::ValueConflict.is_storable());
        assert!(DilemmaType::OntologicalConflict.is_storable());
        assert!(DilemmaType::Axiogenic.is_storable());
    }

    #[test]
    fn test_resolve_enqueues_pending_crystallization() {
        let mut store = DilemmaStore::new();
        let id = store.push_active(DilemmaType::ValueConflict, vec![10, 20], 5, 0.7).unwrap();
        assert!(!store.has_pending_crystallizations());

        store.resolve(id, DilemmaResolution::Complementarity);

        assert!(store.has_pending_crystallizations());
        let pending = store.drain_pending_crystallizations();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, id);
        assert!(pending[0].resolved);
        assert!(!store.has_pending_crystallizations());
    }

    #[test]
    fn test_drain_clears_pending() {
        let mut store = DilemmaStore::new();
        let id1 = store.push_active(DilemmaType::ValueConflict, vec![1], 1, 0.5).unwrap();
        let id2 = store.push_active(DilemmaType::OntologicalConflict, vec![2], 2, 0.8).unwrap();
        store.resolve(id1, DilemmaResolution::ParetoCompromise);
        store.resolve(id2, DilemmaResolution::Complementarity);

        let first = store.drain_pending_crystallizations();
        assert_eq!(first.len(), 2);
        let second = store.drain_pending_crystallizations();
        assert!(second.is_empty());
    }

    #[test]
    fn test_crystallize_commands_count() {
        let mut store = DilemmaStore::new();
        let id = store
            .push_active(DilemmaType::OntologicalConflict, vec![100, 200, 300], 10, 0.9)
            .unwrap();
        store.resolve(id, DilemmaResolution::Complementarity);
        let pending = store.drain_pending_crystallizations();

        let cmds = crystallize_to_experience_commands(&pending[0], [1000, 2000, 3000], 109);
        // 1 InjectToken + 3 BondTokens (один на каждый конфликтующий якорь)
        assert_eq!(cmds.len(), 4);
    }

    #[test]
    fn test_crystallize_lineage_hash_differs_by_type() {
        // Одинаковые anchors, разный тип → разный hash → разный proposed_sutra_id
        let rec_vc = DilemmaRecord {
            id: 1,
            dilemma_type: DilemmaType::ValueConflict,
            anchors_in_conflict: vec![1, 2],
            detected_at_tick: 0,
            intensity: 0.5,
            resolved: true,
            resolution: Some(DilemmaResolution::Complementarity),
        };
        let rec_oc = DilemmaRecord {
            id: 2,
            dilemma_type: DilemmaType::OntologicalConflict,
            anchors_in_conflict: vec![1, 2],
            detected_at_tick: 0,
            intensity: 0.5,
            resolved: true,
            resolution: Some(DilemmaResolution::Complementarity),
        };
        assert_ne!(dilemma_lineage_hash(&rec_vc), dilemma_lineage_hash(&rec_oc));
    }

    #[test]
    fn test_value_conflict_resolves_on_intensity_decay() {
        let mut store = DilemmaStore::new();
        let id = store
            .push_active(DilemmaType::ValueConflict, vec![10, 20], 0, 0.5)
            .unwrap();

        // Имитируем decay до INTENSITY_FORCE_RESOLVE (< 0.02)
        let rec = store.get_active_mut(id).unwrap();
        rec.intensity = 0.01;

        assert!(store.resolve(id, DilemmaResolution::ContextualPriority { winner: 10 }));
        assert_eq!(store.active_count(), 0);
        assert_eq!(store.resolved.len(), 1);
        assert!(store.has_pending_crystallizations());
    }

    #[test]
    fn test_ontological_resolves_as_complementarity() {
        let mut store = DilemmaStore::new();
        let id = store
            .push_active(DilemmaType::OntologicalConflict, vec![30, 40], 0, 0.8)
            .unwrap();

        assert!(store.resolve(id, DilemmaResolution::Complementarity));
        assert_eq!(store.active_count(), 0);
        let resolved = &store.resolved[0];
        assert!(matches!(resolved.resolution, Some(DilemmaResolution::Complementarity)));
    }

    #[test]
    fn test_crystallization_generates_ucl_commands() {
        let mut store = DilemmaStore::new();
        let id = store
            .push_active(DilemmaType::ValueConflict, vec![1, 2], 0, 0.7)
            .unwrap();
        store.resolve(id, DilemmaResolution::ContextualPriority { winner: 1 });

        let pending = store.drain_pending_crystallizations();
        assert_eq!(pending.len(), 1);

        let cmds = crystallize_to_experience_commands(&pending[0], [0i16; 3], 109);
        // Должны быть: InjectFrameAnchor + 2 BondTokens (по одному на каждый anchor)
        assert!(cmds.len() >= 2);
        assert!(!store.has_pending_crystallizations());
    }
}
