// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SubsystemDiscovery — H1 + H2.
//
// H1: кластеризация approved EmergentPrimitives по discovery_octant →
//     SubsystemCandidate с centroid, evidence_strength, YAML-черновик.
//
// H2: SubsystemLifecycleState машина состояний:
//     Proposed → Candidate → InReview → Active → Mature → Deprecated → Archived
//
// Источник: docs/ROADMAP.md §Phase H

use std::collections::HashMap;

use axiom_experience::{EmergentPrimitive, Octant};

// ── H2: lifecycle ──────────────────────────────────────────────────────────────

/// Жизненный цикл кандидата в подсистемы.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "adapters", derive(serde::Serialize))]
pub enum SubsystemLifecycleState {
    /// Автоматически обнаружен системой (кластер ≥2 approved primitives).
    Proposed = 0,
    /// Оператор одобрил для дальнейшего изучения.
    Candidate = 1,
    /// Детальный анализ оператором.
    InReview = 2,
    /// Интегрирован в таксономию подсистем.
    Active = 3,
    /// Долгосрочно устойчивый.
    Mature = 4,
    /// Выводится из использования.
    Deprecated = 5,
    /// Удалён из активного использования.
    Archived = 6,
}

impl SubsystemLifecycleState {
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    /// Допустимые переходы состояний.
    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Proposed,    Self::Candidate)
            | (Self::Candidate, Self::InReview)
            | (Self::InReview,  Self::Active)
            | (Self::Active,    Self::Mature)
            | (Self::Active,    Self::Deprecated)
            | (Self::Mature,    Self::Deprecated)
            | (Self::Deprecated, Self::Archived)
        )
    }
}

impl std::fmt::Display for SubsystemLifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Proposed   => write!(f, "proposed"),
            Self::Candidate  => write!(f, "candidate"),
            Self::InReview   => write!(f, "in_review"),
            Self::Active     => write!(f, "active"),
            Self::Mature     => write!(f, "mature"),
            Self::Deprecated => write!(f, "deprecated"),
            Self::Archived   => write!(f, "archived"),
        }
    }
}

// ── H1: SubsystemCandidate ─────────────────────────────────────────────────────

/// Кандидат в новую подсистему — кластер approved EmergentPrimitives.
#[derive(Debug, Clone)]
pub struct SubsystemCandidate {
    pub id: u32,
    pub lifecycle: SubsystemLifecycleState,
    /// sutra_id примитивов в кластере.
    pub emergent_primitives: Vec<u32>,
    /// Центр масс октанта кластера (детерминированный из octant index).
    pub centroid_position: [i16; 3],
    /// Доминирующий октант кластера.
    pub primary_octant: Octant,
    /// Сила свидетельства: масштабируется с числом примитивов, насыщение при 5+.
    pub evidence_strength: f32,
    pub created_at_event: u64,
    /// Авто-сгенерированный YAML-черновик для ревью оператором.
    pub yaml_draft: String,
}

impl SubsystemCandidate {
    fn new(
        id: u32,
        primitive_ids: Vec<u32>,
        primary_octant: Octant,
        event_id: u64,
    ) -> Self {
        let centroid = octant_centroid(primary_octant);
        let evidence_strength = (primitive_ids.len() as f32 / 5.0).min(1.0);
        let yaml_draft = generate_yaml_draft(id, primary_octant, &primitive_ids, evidence_strength, event_id);
        Self {
            id,
            lifecycle: SubsystemLifecycleState::Proposed,
            emergent_primitives: primitive_ids,
            centroid_position: centroid,
            primary_octant,
            evidence_strength,
            created_at_event: event_id,
            yaml_draft,
        }
    }
}

// ── H2: SubsystemCandidateStore ────────────────────────────────────────────────

/// Хранилище кандидатов в подсистемы с управлением жизненным циклом.
pub struct SubsystemCandidateStore {
    candidates: HashMap<u32, SubsystemCandidate>,
    next_id: u32,
}

impl SubsystemCandidateStore {
    pub fn new() -> Self {
        Self { candidates: HashMap::new(), next_id: 1 }
    }

    pub fn get(&self, id: u32) -> Option<&SubsystemCandidate> {
        self.candidates.get(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = &SubsystemCandidate> {
        self.candidates.values()
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    /// Есть ли уже кандидат для этого октанта (избегаем дублей).
    pub fn has_for_octant(&self, octant: Octant) -> bool {
        self.candidates.values().any(|c| c.primary_octant == octant)
    }

    /// Вставить кандидата. Возвращает присвоенный id.
    pub fn insert(&mut self, primitives: Vec<u32>, octant: Octant, event_id: u64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let candidate = SubsystemCandidate::new(id, primitives, octant, event_id);
        self.candidates.insert(id, candidate);
        id
    }

    /// H2: оператор одобряет Proposed → Candidate.
    /// Возвращает true если переход выполнен.
    pub fn approve(&mut self, id: u32) -> bool {
        if let Some(c) = self.candidates.get_mut(&id) {
            if c.lifecycle.can_transition_to(SubsystemLifecycleState::Candidate) {
                c.lifecycle = SubsystemLifecycleState::Candidate;
                return true;
            }
        }
        false
    }

    /// Продвинуть кандидата в следующее состояние.
    pub fn transition(&mut self, id: u32, next: SubsystemLifecycleState) -> bool {
        if let Some(c) = self.candidates.get_mut(&id) {
            if c.lifecycle.can_transition_to(next) {
                c.lifecycle = next;
                return true;
            }
        }
        false
    }

    /// Итерировать по кандидатам в состоянии `state`.
    pub fn in_state(&self, state: SubsystemLifecycleState) -> impl Iterator<Item = &SubsystemCandidate> {
        self.candidates.values().filter(move |c| c.lifecycle == state)
    }
}

impl Default for SubsystemCandidateStore {
    fn default() -> Self {
        Self::new()
    }
}

// ── H1: clustering ────────────────────────────────────────────────────────────

/// Кластеризация approved EmergentPrimitives по discovery_octant.
///
/// V1: кластер = все approved примитивы в одном октанте.
/// Минимум кластера: 2 примитива.
/// Возвращает вектор пар (octant, Vec<sutra_id>) для октантов, которых ещё нет в store.
pub fn cluster_emergent_primitives<'a>(
    primitives: &'a [EmergentPrimitive],
    store: &SubsystemCandidateStore,
) -> Vec<(Octant, Vec<u32>)> {
    // Группировать approved по octant
    let mut groups: HashMap<usize, Vec<u32>> = HashMap::new();
    for prim in primitives {
        if prim.approved {
            groups
                .entry(prim.discovery_octant.index())
                .or_default()
                .push(prim.sutra_id);
        }
    }

    // Отфильтровать: ≥2 примитивов и октант ещё не представлен в store
    let all_octants = [
        Octant::CreativeAffirmation,
        Octant::EcstaticAffirmation,
        Octant::HeroicFatal,
        Octant::DestructiveActivating,
        Octant::IdealizedConsoling,
        Octant::PassiveSentimental,
        Octant::FormalDenying,
        Octant::SelfDestructiveApathic,
    ];

    let mut result = Vec::new();
    for (idx, ids) in groups {
        if ids.len() >= 2 {
            let octant = all_octants[idx];
            if !store.has_for_octant(octant) {
                result.push((octant, ids));
            }
        }
    }
    result
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Детерминированный центр масс октанта в семантическом пространстве [0..32767]³.
///
/// X высокое = Apollo (бит 0 очищен), X низкое = Dionysus (бит 0 установлен).
/// Y высокое = Eros, Z высокое = Will.
pub fn octant_centroid(oct: Octant) -> [i16; 3] {
    let idx = oct.index();
    // bit layout of octant index: bit2=X_low, bit1=Y_low, bit0=Z_low
    let x: i16 = if (idx & 4) == 0 { 24000 } else { 8000 };
    let y: i16 = if (idx & 2) == 0 { 24000 } else { 8000 };
    let z: i16 = if (idx & 1) == 0 { 24000 } else { 8000 };
    [x, y, z]
}

fn generate_yaml_draft(
    id: u32,
    octant: Octant,
    primitive_ids: &[u32],
    evidence: f32,
    event_id: u64,
) -> String {
    let primitives_yaml: String = primitive_ids
        .iter()
        .map(|sid| format!("    - sutra_id: {sid}\n"))
        .collect();

    format!(
        "# SubsystemCandidate draft — auto-generated by SubsystemDiscovery\n\
         # id: {id}  |  created_at_event: {event_id}  |  status: proposed\n\
         # Review and edit before promoting to active subsystem.\n\
         subsystem:\n  \
           name: \"candidate_{id}\"\n  \
           display_name: \"Candidate ({octant:?})\"\n  \
           primary_octant: {octant:?}\n  \
           evidence_strength: {evidence:.2}\n  \
           source_primitives:\n\
         {primitives_yaml}"
    )
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::EmergentPrimitive;

    fn approved_prim(sutra_id: u32, octant: Octant) -> EmergentPrimitive {
        let mut p = EmergentPrimitive::new(sutra_id, 100, octant, 3000);
        p.approved = true;
        p
    }

    fn pending_prim(sutra_id: u32, octant: Octant) -> EmergentPrimitive {
        EmergentPrimitive::new(sutra_id, 100, octant, 3000)
    }

    // ─── lifecycle tests ──────────────────────────────────────────────────────

    #[test]
    fn test_lifecycle_initial_is_proposed() {
        let store = SubsystemCandidateStore::new();
        assert!(store.is_empty());
    }

    #[test]
    fn test_approve_transitions_proposed_to_candidate() {
        let mut store = SubsystemCandidateStore::new();
        let id = store.insert(vec![1, 2], Octant::CreativeAffirmation, 100);
        assert!(store.approve(id));
        assert_eq!(store.get(id).unwrap().lifecycle, SubsystemLifecycleState::Candidate);
    }

    #[test]
    fn test_approve_nonexistent_returns_false() {
        let mut store = SubsystemCandidateStore::new();
        assert!(!store.approve(999));
    }

    #[test]
    fn test_approve_already_candidate_returns_false() {
        let mut store = SubsystemCandidateStore::new();
        let id = store.insert(vec![1, 2], Octant::CreativeAffirmation, 100);
        store.approve(id);
        // Cannot approve again (Candidate → Candidate not allowed)
        assert!(!store.approve(id));
    }

    #[test]
    fn test_lifecycle_can_transition_valid() {
        assert!(SubsystemLifecycleState::Proposed.can_transition_to(SubsystemLifecycleState::Candidate));
        assert!(SubsystemLifecycleState::Active.can_transition_to(SubsystemLifecycleState::Deprecated));
        assert!(SubsystemLifecycleState::Deprecated.can_transition_to(SubsystemLifecycleState::Archived));
    }

    #[test]
    fn test_lifecycle_cannot_skip_states() {
        assert!(!SubsystemLifecycleState::Proposed.can_transition_to(SubsystemLifecycleState::Active));
        assert!(!SubsystemLifecycleState::Candidate.can_transition_to(SubsystemLifecycleState::Archived));
    }

    #[test]
    fn test_in_state_filter() {
        let mut store = SubsystemCandidateStore::new();
        let id1 = store.insert(vec![1, 2], Octant::CreativeAffirmation, 100);
        store.insert(vec![3, 4], Octant::EcstaticAffirmation, 100);
        store.approve(id1);
        assert_eq!(store.in_state(SubsystemLifecycleState::Candidate).count(), 1);
        assert_eq!(store.in_state(SubsystemLifecycleState::Proposed).count(), 1);
    }

    // ─── clustering tests ─────────────────────────────────────────────────────

    #[test]
    fn test_cluster_two_approved_same_octant() {
        let store = SubsystemCandidateStore::new();
        let prims = vec![
            approved_prim(1, Octant::CreativeAffirmation),
            approved_prim(2, Octant::CreativeAffirmation),
        ];
        let clusters = cluster_emergent_primitives(&prims, &store);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].0, Octant::CreativeAffirmation);
        assert_eq!(clusters[0].1.len(), 2);
    }

    #[test]
    fn test_cluster_one_approved_no_cluster() {
        let store = SubsystemCandidateStore::new();
        let prims = vec![approved_prim(1, Octant::CreativeAffirmation)];
        let clusters = cluster_emergent_primitives(&prims, &store);
        assert!(clusters.is_empty());
    }

    #[test]
    fn test_cluster_pending_not_counted() {
        let store = SubsystemCandidateStore::new();
        let prims = vec![
            approved_prim(1, Octant::HeroicFatal),
            pending_prim(2, Octant::HeroicFatal),
            pending_prim(3, Octant::HeroicFatal),
        ];
        // Only 1 approved → no cluster
        let clusters = cluster_emergent_primitives(&prims, &store);
        assert!(clusters.is_empty());
    }

    #[test]
    fn test_cluster_skips_existing_octant() {
        let mut store = SubsystemCandidateStore::new();
        store.insert(vec![1, 2], Octant::CreativeAffirmation, 100);
        let prims = vec![
            approved_prim(3, Octant::CreativeAffirmation),
            approved_prim(4, Octant::CreativeAffirmation),
        ];
        // Octant already in store → no new cluster
        let clusters = cluster_emergent_primitives(&prims, &store);
        assert!(clusters.is_empty());
    }

    #[test]
    fn test_cluster_multiple_octants() {
        let store = SubsystemCandidateStore::new();
        let prims = vec![
            approved_prim(1, Octant::CreativeAffirmation),
            approved_prim(2, Octant::CreativeAffirmation),
            approved_prim(3, Octant::HeroicFatal),
            approved_prim(4, Octant::HeroicFatal),
            approved_prim(5, Octant::HeroicFatal),
        ];
        let clusters = cluster_emergent_primitives(&prims, &store);
        assert_eq!(clusters.len(), 2);
    }

    // ─── evidence strength ────────────────────────────────────────────────────

    #[test]
    fn test_evidence_strength_scales_with_primitives() {
        let mut store = SubsystemCandidateStore::new();
        let id = store.insert(vec![1, 2], Octant::CreativeAffirmation, 100);
        // 2 primitives → 2/5 = 0.4
        assert!((store.get(id).unwrap().evidence_strength - 0.4).abs() < 1e-5);
    }

    #[test]
    fn test_evidence_strength_saturates_at_5() {
        let mut store = SubsystemCandidateStore::new();
        let id = store.insert(vec![1, 2, 3, 4, 5, 6], Octant::CreativeAffirmation, 100);
        // 6 primitives → min(6/5, 1.0) = 1.0
        assert!((store.get(id).unwrap().evidence_strength - 1.0).abs() < 1e-5);
    }

    // ─── centroid ─────────────────────────────────────────────────────────────

    #[test]
    fn test_centroid_creative_affirmation_high() {
        let c = octant_centroid(Octant::CreativeAffirmation);
        // index=0 (000) → all high (24000)
        assert_eq!(c, [24000, 24000, 24000]);
    }

    #[test]
    fn test_centroid_self_destructive_apathic_low() {
        let c = octant_centroid(Octant::SelfDestructiveApathic);
        // index=7 (111) → all low (8000)
        assert_eq!(c, [8000, 8000, 8000]);
    }

    // ─── yaml draft ───────────────────────────────────────────────────────────

    #[test]
    fn test_yaml_draft_contains_id_and_octant() {
        let mut store = SubsystemCandidateStore::new();
        let id = store.insert(vec![10, 20], Octant::HeroicFatal, 500);
        let yaml = &store.get(id).unwrap().yaml_draft;
        assert!(yaml.contains(&format!("id: {id}")));
        assert!(yaml.contains("HeroicFatal"));
        assert!(yaml.contains("sutra_id: 10"));
        assert!(yaml.contains("sutra_id: 20"));
    }
}
