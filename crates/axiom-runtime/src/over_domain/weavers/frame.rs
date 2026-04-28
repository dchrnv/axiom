// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// FrameWeaver V1.1 — первый Weaver Over-Domain Layer.
// Спецификация: docs/spec/Weaver/FrameWeaver_V1_1.md
//
// Цикл: MAYA → scan → candidates → (stable) → EXPERIENCE (kристаллизация)
//                                           → ReinforceFrame (реактивация)
//       EXPERIENCE → check_promotion → PromotionProposal (→ SUTRA через CODEX)

#![allow(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;

use axiom_core::{
    Connection, Token, FLAG_ACTIVE, STATE_ACTIVE, STATE_LOCKED,
    TOKEN_FLAG_FRAME_ANCHOR, TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE,
    FRAME_CATEGORY_SYNTAX, FRAME_CATEGORY_MASK,
};
use axiom_domain::{AshtiCore, DomainState};
use axiom_genome::{Genome, ModuleId};
use axiom_ucl::{
    UclCommand, OpCode,
    InjectFrameAnchorPayload, BondTokensPayload, ReinforceFramePayload,
    flags as ucl_flags,
};
use axiom_shell::link_types;

use crate::over_domain::traits::{
    CrystallizationProposal, OverDomainComponent, OverDomainError,
    PromotionProposal, Weaver, WeaverId,
};

/// Numeric ID для TickSchedule.weaver_scan_intervals.
pub const FRAME_WEAVER_ID: WeaverId = 1;

// ============================================================================
// Структуры данных
// ============================================================================

/// Кандидат в Frame — синтаксический узор, обнаруженный в MAYA.
#[derive(Debug, Clone)]
pub struct FrameCandidate {
    /// Центр масс позиций участников (для анкера EXPERIENCE)
    pub anchor_position: [i16; 3],
    /// Участники Frame с синтаксическими ролями
    pub participants: Vec<Participant>,
    /// tick обнаружения
    pub detected_at_tick: u64,
    /// Сколько сканов подряд узор существует без изменений
    pub stability_count: u32,
    /// Категория Frame (V1.1 — только FRAME_CATEGORY_SYNTAX)
    pub category: u16,
    /// FNV-1a хэш sutra_id всех участников (включая head)
    pub lineage_hash: u64,
}

/// Участник Frame — токен с конкретной синтаксической ролью.
#[derive(Debug, Clone)]
pub struct Participant {
    /// sutra_id токена-участника
    pub sutra_id: u32,
    /// домен, из которого пришёл участник (обычно MAYA)
    pub origin_domain_id: u16,
    /// Синтаксическая роль (link_type 0x08XX)
    pub role_link_type: u16,
    /// Слой (S1=0 … S8=7), вычисляется из link_type
    pub layer: u8,
}

/// Конфигурация FrameWeaver (загружается из Schema Configuration).
#[derive(Debug, Clone)]
pub struct FrameWeaverConfig {
    /// Интервал сканирования MAYA в тиках (default: 20)
    pub scan_interval_ticks: u32,
    /// Порог стабильности для предложения DREAM (default: 3)
    pub stability_threshold: u32,
    /// Минимум участников для признания узора Frame (default: 2)
    pub min_participants: usize,
    /// Максимальная глубина хранения (0 = без ограничений)
    pub max_storage_depth: u8,
    /// Глубина развёртывания по умолчанию
    pub default_unfold_depth: u8,
    /// Максимальная глубина развёртывания
    pub max_unfold_depth: u8,
    /// Стратегия обработки циклов в EXPERIENCE
    pub cycle_handling: CycleStrategy,
    /// Правила промоции EXPERIENCE → SUTRA
    pub promotion_rules: Vec<PromotionRule>,
    /// Правила кристаллизации
    pub crystallization_rules: Vec<CrystallizationRule>,
}

impl Default for FrameWeaverConfig {
    fn default() -> Self {
        Self {
            scan_interval_ticks: 20,
            stability_threshold: 3,
            min_participants: 2,
            max_storage_depth: 0,
            default_unfold_depth: 3,
            max_unfold_depth: 8,
            cycle_handling: CycleStrategy::Allow,
            promotion_rules: vec![PromotionRule::default()],
            crystallization_rules: vec![],
        }
    }
}

/// Правило промоции Frame из EXPERIENCE в SUTRA.
#[derive(Debug, Clone)]
pub struct PromotionRule {
    pub id: String,
    /// Минимальный возраст Frame в тиках
    pub min_age_ticks: u64,
    /// Минимальное число реактиваций
    pub min_reactivations: u32,
    /// Минимальная устоявшаяся температура
    pub min_temperature: u8,
    /// Минимальная масса
    pub min_mass: u8,
    /// Минимум участников, которые сами anchor'ы SUTRA
    pub min_participant_anchors: usize,
    /// Требует явного одобрения CODEX
    pub requires_codex_approval: bool,
}

impl Default for PromotionRule {
    fn default() -> Self {
        Self {
            id: "default_promotion".to_string(),
            min_age_ticks: 100_000,
            min_reactivations: 10,
            min_temperature: 200,
            min_mass: 100,
            min_participant_anchors: 3,
            requires_codex_approval: true,
        }
    }
}

/// Правило кристаллизации кандидата в Frame.
#[derive(Debug, Clone)]
pub struct CrystallizationRule {
    pub id: String,
    pub priority: u8,
    pub trigger: RuleTrigger,
    pub conditions: Vec<RuleCondition>,
    pub action: RuleAction,
}

#[derive(Debug, Clone)]
pub enum RuleTrigger {
    StabilityReached(u32),
    DreamCycle,
    RepeatedAssembly { window_ticks: u32 },
    HighConfidence(f32),
}

#[derive(Debug, Clone)]
pub enum RuleCondition {
    DominantLayer(u8),
    MinParticipants(usize),
    RequiresParticipantFromDomain(u16),
    LayerPresent(u8),
    MaxDepth(u8),
}

#[derive(Debug, Clone)]
pub enum RuleAction {
    CrystallizeFull,
    CrystallizeAnchorOnly,
    Defer { ticks: u32 },
    Reject,
}

/// Стратегия обработки циклов при кристаллизации Frame в EXPERIENCE.
///
/// В V1.1 EXPERIENCE допускает циклы (опыт может быть противоречивым).
/// DAG-инвариант применяется только при промоции в SUTRA.
#[derive(Debug, Clone)]
pub enum CycleStrategy {
    /// Цикл рвётся в произвольном месте
    Break,
    /// Цикл сохраняется с пометкой на ребре (CONNECTION_FLAG_CYCLE)
    Mark,
    /// Цикл сохраняется без пометок (default для EXPERIENCE)
    Allow,
}

/// Статистика FrameWeaver для BroadcastSnapshot.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "adapters", derive(serde::Serialize))]
pub struct FrameWeaverStats {
    pub scans_performed: u64,
    pub candidates_detected: u64,
    pub candidates_proposed_to_dream: u64,
    pub crystallizations_approved: u64,
    pub crystallizations_vetoed: u64,
    pub frames_in_experience: u64,
    pub frame_reactivations: u64,
    pub promotions_proposed: u64,
    pub promotions_approved: u64,
    pub promotions_vetoed: u64,
    pub frames_in_sutra: u64,
    pub unfold_requests: u64,
    pub cycles_handled: u64,
}

// ============================================================================
// restore_frame_from_anchor — восстановление Frame из анкера
// ============================================================================

/// Ошибка восстановления Frame из EXPERIENCE/SUTRA.
#[derive(Debug)]
pub enum RestoreError {
    /// Анкер-токен не найден в source_state
    AnchorNotFound,
    /// Токен найден, но не является Frame-анкером (нет TOKEN_FLAG_FRAME_ANCHOR)
    NotAFrameAnchor,
    /// Связь ведёт к несуществующему токену
    DanglingParticipant(u32),
    /// Связь от анкера с link_type не из категории 0x08
    InvalidLinkType(u16),
}

impl std::fmt::Display for RestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestoreError::AnchorNotFound          => write!(f, "anchor not found"),
            RestoreError::NotAFrameAnchor         => write!(f, "token is not a frame anchor"),
            RestoreError::DanglingParticipant(id) => write!(f, "dangling participant: {id}"),
            RestoreError::InvalidLinkType(lt)     => write!(f, "invalid link type: {lt:#06x}"),
        }
    }
}

impl std::error::Error for RestoreError {}

/// Восстановленный Frame из EXPERIENCE или SUTRA.
#[derive(Debug)]
pub struct RestoredFrame {
    pub anchor: Token,
    pub anchor_id: u32,
    pub category: u16,
    pub participants: Vec<Participant>,
}

/// Восстановить Frame из анкера в source_state.
///
/// Обходит граф связей, декодирует reserved_gate и собирает список участников.
/// Read-only операция — UCL-команды не генерируются.
pub fn restore_frame_from_anchor(
    anchor_id: u32,
    source_state: &DomainState,
) -> Result<RestoredFrame, RestoreError> {
    let anchor = source_state.tokens.iter()
        .find(|t| t.sutra_id == anchor_id)
        .copied()
        .ok_or(RestoreError::AnchorNotFound)?;

    if (anchor.type_flags & TOKEN_FLAG_FRAME_ANCHOR) == 0 {
        return Err(RestoreError::NotAFrameAnchor);
    }

    let category = anchor.type_flags & FRAME_CATEGORY_MASK;

    let mut participants = Vec::new();
    for conn in source_state.connections.iter() {
        if conn.source_id != anchor_id || (conn.flags & FLAG_ACTIVE) == 0 {
            continue;
        }
        if (conn.link_type >> 8) != 0x08 {
            return Err(RestoreError::InvalidLinkType(conn.link_type));
        }
        if !source_state.tokens.iter().any(|t| t.sutra_id == conn.target_id) {
            return Err(RestoreError::DanglingParticipant(conn.target_id));
        }
        let origin_domain = u16::from_be_bytes([conn.reserved_gate[0], conn.reserved_gate[1]]);
        let layer = ((conn.link_type & 0x00F0) >> 4) as u8;
        participants.push(Participant {
            sutra_id: conn.target_id,
            origin_domain_id: origin_domain,
            role_link_type: conn.link_type,
            layer,
        });
    }

    Ok(RestoredFrame { anchor, anchor_id, category, participants })
}

// ============================================================================
// FrameWeaver
// ============================================================================

/// FrameWeaver — Over-Domain компонент для сборки и кристаллизации
/// синтаксических узоров MAYA → EXPERIENCE.
///
/// Реализует `Weaver` (и `OverDomainComponent`). Хранится по значению в
/// AxiomEngine (не через `Box<dyn Weaver>` из-за type Pattern).
pub struct FrameWeaver {
    config: FrameWeaverConfig,
    /// Незакристаллизованные кандидаты, ключ = lineage_hash
    candidates: HashMap<u64, FrameCandidate>,
    /// Команды, накопленные за тики, для исполнения engine'ом (Phase 4)
    pending_commands: Vec<UclCommand>,
    /// Счётчики реактиваций: anchor_id → count
    reactivation_counts: HashMap<u32, u32>,
    pub stats: FrameWeaverStats,
}

impl FrameWeaver {
    pub fn new(config: FrameWeaverConfig) -> Self {
        Self {
            config,
            candidates: HashMap::new(),
            pending_commands: Vec::new(),
            reactivation_counts: HashMap::new(),
            stats: FrameWeaverStats::default(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(FrameWeaverConfig::default())
    }

    /// Слить накопленные команды (вызывается AxiomEngine в Phase 4).
    pub fn drain_commands(&mut self) -> Vec<UclCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    /// Синтаксический слой из link_type: S1=0 … S8=7 (соответствует (link_type & 0x00F0) >> 4).
    fn layer_of(link_type: u16) -> u8 {
        ((link_type & 0x00F0) >> 4) as u8
    }

    /// FNV-1a над отсортированным списком sutra_id.
    fn fnv1a_lineage_hash(ids: &[u32]) -> u64 {
        let mut sorted = ids.to_vec();
        sorted.sort_unstable();
        let mut h: u64 = 0xcbf29ce484222325;
        for id in sorted {
            h ^= id as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }

    /// Вычислить центроид позиций участников по данным MAYA.
    fn compute_centroid(participants: &[Participant], tokens: &[Token]) -> [i16; 3] {
        if participants.is_empty() {
            return [0; 3];
        }
        let mut sum = [0i64; 3];
        let mut count = 0i64;
        for p in participants {
            if let Some(tok) = tokens.iter().find(|t| t.sutra_id == p.sutra_id) {
                sum[0] += tok.position[0] as i64;
                sum[1] += tok.position[1] as i64;
                sum[2] += tok.position[2] as i64;
                count += 1;
            }
        }
        if count == 0 {
            return [0; 3];
        }
        [
            (sum[0] / count) as i16,
            (sum[1] / count) as i16,
            (sum[2] / count) as i16,
        ]
    }

    /// Детерминированный proposed_sutra_id из lineage_hash (нижние 32 бита, не 0).
    fn proposed_id_from_hash(hash: u64) -> u32 {
        let low = (hash & 0xFFFF_FFFF) as u32;
        if low == 0 { 1 } else { low }
    }

    /// Есть ли Frame с данным lineage_hash в EXPERIENCE?
    fn find_existing_anchor(experience_state: &DomainState, lineage_hash: u64) -> Option<u32> {
        experience_state.tokens.iter().find(|t| {
            (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0 && t.lineage_hash == lineage_hash
        }).map(|t| t.sutra_id)
    }

    /// Сканировать DomainState (MAYA) на синтаксические узоры.
    ///
    /// Возвращает кандидатов: группы синтаксических связей с source_id как Frame-голова.
    /// Требование: ≥ 2 различных слоя и ≥ min_participants участников.
    pub fn scan_state_pub(&self, maya_state: &DomainState, maya_domain_id: u16) -> Vec<FrameCandidate> {
        self.scan_state(maya_state, maya_domain_id)
    }

    fn scan_state(&self, maya_state: &DomainState, maya_domain_id: u16) -> Vec<FrameCandidate> {
        // Фильтровать активные синтаксические связи (категория 0x08)
        let syn_conns: Vec<&Connection> = maya_state.connections.iter()
            .filter(|c| (c.link_type >> 8) == 0x08 && (c.flags & FLAG_ACTIVE) != 0)
            .collect();

        if syn_conns.is_empty() {
            return Vec::new();
        }

        // Группировать по source_id (Frame-голова)
        let mut groups: HashMap<u32, Vec<&Connection>> = HashMap::new();
        for conn in &syn_conns {
            groups.entry(conn.source_id).or_default().push(conn);
        }

        let mut candidates = Vec::new();

        for (source_id, conns) in groups {
            // Проверить минимум участников (targets + head)
            if conns.len() + 1 < self.config.min_participants {
                continue;
            }

            // Проверить ≥ 2 различных слоя
            let layers: std::collections::HashSet<u8> = conns.iter()
                .map(|c| Self::layer_of(c.link_type))
                .collect();
            if layers.len() < 2 {
                continue;
            }

            // Участники: Frame-голова (PREDICATE) + все targets
            let mut participants = Vec::with_capacity(conns.len() + 1);
            participants.push(Participant {
                sutra_id: source_id,
                origin_domain_id: maya_domain_id,
                role_link_type: link_types::SYNTACTIC_PREDICATE,
                layer: Self::layer_of(link_types::SYNTACTIC_PREDICATE),
            });
            for conn in &conns {
                participants.push(Participant {
                    sutra_id: conn.target_id,
                    origin_domain_id: maya_domain_id,
                    role_link_type: conn.link_type,
                    layer: Self::layer_of(conn.link_type),
                });
            }

            let all_ids: Vec<u32> = participants.iter().map(|p| p.sutra_id).collect();
            let lineage_hash = Self::fnv1a_lineage_hash(&all_ids);
            let anchor_position = Self::compute_centroid(&participants, &maya_state.tokens);

            candidates.push(FrameCandidate {
                anchor_position,
                participants,
                detected_at_tick: 0,
                stability_count: 0,
                category: FRAME_CATEGORY_SYNTAX,
                lineage_hash,
            });
        }

        candidates
    }

    /// Построить UCL-команды для кристаллизации Frame в EXPERIENCE.
    fn build_crystallization_commands(&self, candidate: &FrameCandidate, experience_domain: u16) -> Vec<UclCommand> {
        let proposed_sutra_id = Self::proposed_id_from_hash(candidate.lineage_hash);
        let mass = (candidate.participants.len() as u8).saturating_mul(16).max(32);

        let anchor_payload = InjectFrameAnchorPayload {
            lineage_hash:      candidate.lineage_hash,
            proposed_sutra_id,
            target_domain_id:  experience_domain,
            type_flags:        TOKEN_FLAG_FRAME_ANCHOR | candidate.category,
            position:          candidate.anchor_position,
            state:             STATE_ACTIVE,
            mass,
            temperature:       128,
            valence:           0,
            reserved:          [0; 22],
        };

        let anchor_cmd = UclCommand::new(OpCode::InjectToken, 0, 10, ucl_flags::FRAME_ANCHOR)
            .with_payload(&anchor_payload);

        let mut cmds = vec![anchor_cmd];

        for participant in &candidate.participants {
            let bond_payload = BondTokensPayload {
                source_id:     proposed_sutra_id,
                target_id:     participant.sutra_id,
                domain_id:     experience_domain,
                link_type:     participant.role_link_type,
                strength:      1.0,
                conn_flags:    0,
                origin_domain: participant.origin_domain_id,
                role_id:       participant.role_link_type,
                reserved:      [0; 24],
            };
            cmds.push(
                UclCommand::new(OpCode::BondTokens, 0, 10, 0)
                    .with_payload(&bond_payload)
            );
        }

        cmds
    }

    /// Построить UCL-команду для реактивации существующего Frame-анкера.
    fn build_reinforce_command(&self, anchor_id: u32) -> UclCommand {
        let payload = ReinforceFramePayload {
            anchor_id,
            delta_mass:        4,
            delta_temperature: 8,
            reserved:          [0; 42],
        };
        UclCommand::new(OpCode::ReinforceFrame, 0, 8, 0)
            .with_payload(&payload)
    }

    /// Построить UCL-команды для промоции Frame из EXPERIENCE в SUTRA.
    fn build_promotion_commands(candidate: &FrameCandidate, experience_anchor_id: u32, sutra_domain: u16) -> Vec<UclCommand> {
        // Новый уникальный ID для SUTRA-анкера (отличается от EXPERIENCE-анкера)
        let proposed_sutra_id = Self::proposed_id_from_hash(
            candidate.lineage_hash.wrapping_add(0xDEAD_BEEF_0000_0000)
        );

        let anchor_payload = InjectFrameAnchorPayload {
            lineage_hash:      candidate.lineage_hash,
            proposed_sutra_id,
            target_domain_id:  sutra_domain,
            type_flags:        TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE | candidate.category,
            position:          candidate.anchor_position,
            state:             STATE_LOCKED,
            mass:              (candidate.participants.len() as u8).saturating_mul(32),
            temperature:       255,
            valence:           0,
            reserved:          [0; 22],
        };

        let anchor_cmd = UclCommand::new(OpCode::InjectToken, 0, 15, ucl_flags::FRAME_ANCHOR)
            .with_payload(&anchor_payload);

        let mut cmds = vec![anchor_cmd];

        for participant in &candidate.participants {
            let bond_payload = BondTokensPayload {
                source_id:     proposed_sutra_id,
                target_id:     participant.sutra_id,
                domain_id:     sutra_domain,
                link_type:     participant.role_link_type,
                strength:      1.0,
                conn_flags:    0,
                origin_domain: participant.origin_domain_id,
                role_id:       participant.role_link_type,
                reserved:      [0; 24],
            };
            cmds.push(
                UclCommand::new(OpCode::BondTokens, 0, 15, 0)
                    .with_payload(&bond_payload)
            );
        }

        let _ = experience_anchor_id; // оригинал в EXPERIENCE не трогается
        cmds
    }

    /// Проверить кандидата на выполнение правил кристаллизации.
    /// Возвращает RuleAction с наибольшим priority.
    fn evaluate_crystallization_rules(&self, candidate: &FrameCandidate) -> RuleAction {
        if self.config.crystallization_rules.is_empty() {
            // Дефолт: кристаллизовать при достижении stability_threshold
            if candidate.stability_count >= self.config.stability_threshold {
                return RuleAction::CrystallizeFull;
            }
            return RuleAction::Defer { ticks: self.config.scan_interval_ticks };
        }

        let mut best: Option<(u8, &RuleAction)> = None;
        for rule in &self.config.crystallization_rules {
            if !self.trigger_matches(rule, candidate) {
                continue;
            }
            if !self.conditions_met(rule, candidate) {
                continue;
            }
            if best.map_or(true, |(p, _)| rule.priority > p) {
                best = Some((rule.priority, &rule.action));
            }
        }

        match best {
            Some((_, action)) => action.clone(),
            None => RuleAction::Defer { ticks: self.config.scan_interval_ticks },
        }
    }

    fn trigger_matches(&self, rule: &CrystallizationRule, candidate: &FrameCandidate) -> bool {
        match &rule.trigger {
            RuleTrigger::StabilityReached(n) => candidate.stability_count >= *n,
            RuleTrigger::HighConfidence(_)   => false, // confidence не реализован в V1.1
            RuleTrigger::DreamCycle          => false, // DREAM-фаза пока не сигнализирует сюда
            RuleTrigger::RepeatedAssembly { .. } => false, // deferred
        }
    }

    fn conditions_met(&self, rule: &CrystallizationRule, candidate: &FrameCandidate) -> bool {
        for cond in &rule.conditions {
            match cond {
                RuleCondition::MinParticipants(n) => {
                    if candidate.participants.len() < *n {
                        return false;
                    }
                }
                RuleCondition::LayerPresent(layer) => {
                    if !candidate.participants.iter().any(|p| p.layer == *layer) {
                        return false;
                    }
                }
                RuleCondition::MaxDepth(d) => {
                    let max_layer = candidate.participants.iter()
                        .map(|p| p.layer)
                        .max()
                        .unwrap_or(0);
                    if max_layer > *d {
                        return false;
                    }
                }
                RuleCondition::DominantLayer(layer) => {
                    let count_dominant = candidate.participants.iter()
                        .filter(|p| p.layer == *layer)
                        .count();
                    let total = candidate.participants.len();
                    if count_dominant * 2 < total {
                        return false;
                    }
                }
                RuleCondition::RequiresParticipantFromDomain(domain_id) => {
                    if !candidate.participants.iter().any(|p| p.origin_domain_id == *domain_id) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Проверить Frame-анкер в EXPERIENCE на соответствие правилу промоции.
    fn qualifies_for_promotion(&self, token: &Token, rule: &PromotionRule, current_tick: u64) -> bool {
        let age_ticks = current_tick.saturating_sub(token.last_event_id);
        if age_ticks < rule.min_age_ticks {
            return false;
        }
        let reactivations = self.reactivation_counts.get(&token.sutra_id).copied().unwrap_or(0);
        if reactivations < rule.min_reactivations {
            return false;
        }
        if token.temperature < rule.min_temperature {
            return false;
        }
        if token.mass < rule.min_mass {
            return false;
        }
        true
        // min_participant_anchors — проверка участников в SUTRA (deferred: requires cross-domain lookup)
        // requires_codex_approval — проверяется GUARDIAN, не здесь
    }

    /// Обновить кандидат-карту по результатам нового скана.
    fn update_candidates(&mut self, new_scan: Vec<FrameCandidate>, current_tick: u64) {
        // Собрать lineage_hash из нового скана
        let new_hashes: std::collections::HashSet<u64> = new_scan.iter()
            .map(|c| c.lineage_hash)
            .collect();

        // Удалить исчезнувшие кандидаты, увеличить stability существующих
        let mut to_remove = Vec::new();
        for (hash, candidate) in &mut self.candidates {
            if new_hashes.contains(hash) {
                candidate.stability_count += 1;
            } else {
                to_remove.push(*hash);
            }
        }
        for hash in to_remove {
            self.candidates.remove(&hash);
        }

        // Добавить новые кандидаты
        for mut candidate in new_scan {
            self.candidates.entry(candidate.lineage_hash).or_insert_with(|| {
                candidate.detected_at_tick = current_tick;
                candidate.stability_count = 1;
                self.stats.candidates_detected += 1;
                candidate
            });
        }
    }
}

// ============================================================================
// OverDomainComponent
// ============================================================================

impl OverDomainComponent for FrameWeaver {
    fn name(&self) -> &'static str {
        "FrameWeaver"
    }

    fn module_id(&self) -> ModuleId {
        ModuleId::FrameWeaver
    }

    fn on_boot(&mut self, _genome: &Arc<Genome>) -> Result<(), OverDomainError> {
        // TODO Phase 4: проверить права GENOME для ModuleId::FrameWeaver на EXPERIENCE и SUTRA
        Ok(())
    }

    fn on_tick_interval(&self) -> u32 {
        self.config.scan_interval_ticks
    }

    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<(), OverDomainError> {
        let level = ashti.level_id();
        let maya_domain_id = level * 100 + 10;
        let exp_domain_id  = level * 100 + 9;
        let sutra_domain_id = level * 100;

        // Получить состояния доменов
        let maya_state = ashti.index_of(maya_domain_id)
            .and_then(|i| ashti.state(i));
        let exp_state = ashti.index_of(exp_domain_id)
            .and_then(|i| ashti.state(i));

        self.stats.scans_performed += 1;

        // ── 1. Сканировать MAYA ──────────────────────────────────────────────
        let new_candidates = if let Some(state) = maya_state {
            self.scan_state(state, maya_domain_id)
        } else {
            Vec::new()
        };

        // ── 2. Обновить кандидат-карту ───────────────────────────────────────
        self.update_candidates(new_candidates, tick);

        // ── 3. Обработать стабильные кандидаты ──────────────────────────────
        let stable_hashes: Vec<u64> = self.candidates.keys()
            .filter(|&&hash| {
                self.candidates[&hash].stability_count >= self.config.stability_threshold
            })
            .copied()
            .collect();

        for hash in stable_hashes {
            let candidate = match self.candidates.get(&hash) {
                Some(c) => c.clone(),
                None    => continue,
            };

            // Проверить оценку правил кристаллизации
            match self.evaluate_crystallization_rules(&candidate) {
                RuleAction::Reject => {
                    self.candidates.remove(&hash);
                    continue;
                }
                RuleAction::Defer { .. } => continue,
                RuleAction::CrystallizeFull | RuleAction::CrystallizeAnchorOnly => {}
            }

            // Проверить: уже есть Frame с таким lineage_hash в EXPERIENCE?
            let existing_anchor = exp_state.and_then(|state| {
                Self::find_existing_anchor(state, candidate.lineage_hash)
            });

            if let Some(anchor_id) = existing_anchor {
                // Реактивация существующего Frame
                let reinforce_cmd = self.build_reinforce_command(anchor_id);
                self.pending_commands.push(reinforce_cmd);
                *self.reactivation_counts.entry(anchor_id).or_insert(0) += 1;
                self.stats.frame_reactivations += 1;
            } else {
                // Новая кристаллизация в EXPERIENCE
                let cmds = self.build_crystallization_commands(&candidate, exp_domain_id);
                self.stats.crystallizations_approved += 1;
                self.stats.candidates_proposed_to_dream += 1;
                self.pending_commands.extend(cmds);
            }

            // Кандидат обработан — удалить из активных
            self.candidates.remove(&hash);
        }

        // ── 4. Обновить счётчик frames_in_experience ─────────────────────────
        if let Some(state) = exp_state {
            self.stats.frames_in_experience = state.tokens.iter()
                .filter(|t| (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0
                    && (t.type_flags & TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE) == 0)
                .count() as u64;
            self.stats.frames_in_sutra = state.tokens.iter()
                .filter(|t| (t.type_flags & TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE) != 0)
                .count() as u64;
        }

        // ── 5. Проверить промоцию EXPERIENCE → SUTRA ────────────────────────
        if let Some(state) = exp_state {
            let frame_anchors: Vec<Token> = state.tokens.iter()
                .filter(|t| (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0
                    && (t.type_flags & TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE) == 0)
                .copied()
                .collect();

            for token in &frame_anchors {
                for rule in &self.config.promotion_rules.clone() {
                    if self.qualifies_for_promotion(token, rule, tick) {
                        let restored = match restore_frame_from_anchor(token.sutra_id, state) {
                            Ok(r)  => r,
                            Err(_) => break, // аномалия данных — пропустить кандидата
                        };
                        let candidate = FrameCandidate {
                            anchor_position: restored.anchor.position,
                            participants:    restored.participants,
                            detected_at_tick: restored.anchor.last_event_id,
                            stability_count:  0,
                            category:         restored.category,
                            lineage_hash:     restored.anchor.lineage_hash,
                        };
                        let cmds = Self::build_promotion_commands(
                            &candidate, token.sutra_id, sutra_domain_id
                        );
                        self.pending_commands.extend(cmds);
                        self.stats.promotions_proposed += 1;
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn on_shutdown(&mut self) -> Vec<UclCommand> {
        std::mem::take(&mut self.pending_commands)
    }
}

// ============================================================================
// Weaver trait
// ============================================================================

impl Weaver for FrameWeaver {
    type Pattern = FrameCandidate;

    fn scan(&mut self, tick: u64, maya_state: &DomainState) -> Vec<FrameCandidate> {
        // Вызывается напрямую (для unit-тестов и DREAM-интеграции).
        // В on_tick используется scan_state с явным domain_id.
        let candidates = self.scan_state(maya_state, 0);
        self.update_candidates(candidates.clone(), tick);
        candidates
    }

    fn propose_to_dream(&self, patterns: &[FrameCandidate]) -> Vec<CrystallizationProposal> {
        // DREAM-интеграция — Phase 4. Пока возвращаем каждый стабильный паттерн
        // как предложение с target_domain = EXPERIENCE.
        patterns.iter()
            .filter(|c| c.stability_count >= self.config.stability_threshold)
            .map(|_c| CrystallizationProposal {
                weaver_id:     self.weaver_id(),
                target_domain: 109,
                commands:      Vec::new(), // заполняется движком при обработке
                priority:      100,
            })
            .collect()
    }

    fn check_promotion(
        &self,
        tick: u64,
        experience_state: &DomainState,
        anchors: &[&Token],
    ) -> Vec<PromotionProposal> {
        let mut proposals = Vec::new();

        for anchor in anchors {
            if (anchor.type_flags & TOKEN_FLAG_FRAME_ANCHOR) == 0 {
                continue;
            }
            for rule in &self.config.promotion_rules {
                if self.qualifies_for_promotion(anchor, rule, tick) {
                    proposals.push(PromotionProposal {
                        weaver_id: self.weaver_id(),
                        anchor_id: anchor.sutra_id,
                        commands:  Vec::new(),
                    });
                    break;
                }
            }
        }

        let _ = experience_state;
        proposals
    }

    fn weaver_id(&self) -> WeaverId {
        FRAME_WEAVER_ID
    }

    fn target_domain(&self) -> u16 {
        109 // EXPERIENCE
    }
}

// ============================================================================
// Тесты
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_config::DomainConfig;
    use axiom_domain::{AshtiCore, DomainState};
    use axiom_ucl::OpCode;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn make_syn_conn(source: u32, target: u32, layer: u8) -> Connection {
        let mut c = Connection::new(source, target, 110, 1);
        c.link_type = 0x0800 | ((layer as u16) << 4);
        c
    }

    fn empty_state() -> DomainState {
        DomainState::new(&DomainConfig::default())
    }

    fn state_with_conns(conns: Vec<Connection>) -> DomainState {
        let mut s = empty_state();
        s.connections = conns;
        s
    }

    // ── fnv1a_lineage_hash ───────────────────────────────────────────────────

    #[test]
    fn fnv1a_deterministic() {
        let h1 = FrameWeaver::fnv1a_lineage_hash(&[1, 2, 3]);
        let h2 = FrameWeaver::fnv1a_lineage_hash(&[1, 2, 3]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn fnv1a_order_independent() {
        let h1 = FrameWeaver::fnv1a_lineage_hash(&[1, 2, 3]);
        let h2 = FrameWeaver::fnv1a_lineage_hash(&[3, 1, 2]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn fnv1a_different_ids_differ() {
        let h1 = FrameWeaver::fnv1a_lineage_hash(&[1, 2, 3]);
        let h2 = FrameWeaver::fnv1a_lineage_hash(&[1, 2, 4]);
        assert_ne!(h1, h2);
    }

    // ── proposed_id_from_hash ───────────────────────────────────────────────

    #[test]
    fn proposed_id_nonzero_when_low_bits_are_zero() {
        let id = FrameWeaver::proposed_id_from_hash(0xDEAD_BEEF_0000_0000);
        assert_eq!(id, 1);
    }

    #[test]
    fn proposed_id_uses_low_32_bits() {
        let id = FrameWeaver::proposed_id_from_hash(0x0000_0000_0000_ABCD);
        assert_eq!(id, 0xABCD);
    }

    // ── scan_state ──────────────────────────────────────────────────────────

    #[test]
    fn scan_empty_state_returns_no_candidates() {
        let fw = FrameWeaver::with_default_config();
        assert!(fw.scan_state(&empty_state(), 110).is_empty());
    }

    #[test]
    fn scan_single_layer_not_detected() {
        let fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![
            make_syn_conn(10, 20, 1),
            make_syn_conn(10, 30, 1),
        ]);
        assert!(fw.scan_state(&state, 110).is_empty());
    }

    #[test]
    fn scan_two_layer_pattern_detected() {
        let fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![
            make_syn_conn(10, 20, 1),
            make_syn_conn(10, 30, 2),
        ]);
        let result = fw.scan_state(&state, 110);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].participants.len(), 3); // head + 2 targets
    }

    #[test]
    fn scan_inactive_connections_ignored() {
        let fw = FrameWeaver::with_default_config();
        let mut c1 = make_syn_conn(10, 20, 1);
        let mut c2 = make_syn_conn(10, 30, 2);
        c1.flags = 0;
        c2.flags = 0;
        assert!(fw.scan_state(&state_with_conns(vec![c1, c2]), 110).is_empty());
    }

    #[test]
    fn scan_non_syntactic_connections_ignored() {
        let fw = FrameWeaver::with_default_config();
        let mut c = Connection::new(10, 20, 110, 1);
        c.link_type = 0x0110; // category 0x01, not 0x08
        assert!(fw.scan_state(&state_with_conns(vec![c]), 110).is_empty());
    }

    #[test]
    fn scan_lineage_hash_order_independent() {
        let fw = FrameWeaver::with_default_config();
        let s1 = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let s2 = state_with_conns(vec![make_syn_conn(10, 30, 2), make_syn_conn(10, 20, 1)]);
        let r1 = fw.scan_state(&s1, 110);
        let r2 = fw.scan_state(&s2, 110);
        assert_eq!(r1[0].lineage_hash, r2[0].lineage_hash);
    }

    #[test]
    fn scan_category_is_syntax() {
        let fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let result = fw.scan_state(&state, 110);
        assert_eq!(result[0].category, FRAME_CATEGORY_SYNTAX);
    }

    // ── build_crystallization_commands ──────────────────────────────────────

    #[test]
    fn crystallization_commands_count() {
        let fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let candidates = fw.scan_state(&state, 110);
        let cmds = fw.build_crystallization_commands(&candidates[0], 109);
        assert_eq!(cmds.len(), 4); // 1 anchor + 3 bonds (head + 2 targets)
    }

    #[test]
    fn crystallization_first_cmd_is_inject_token() {
        let fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let candidates = fw.scan_state(&state, 110);
        let cmds = fw.build_crystallization_commands(&candidates[0], 109);
        assert_eq!(cmds[0].opcode, OpCode::InjectToken as u16);
    }

    #[test]
    fn crystallization_remaining_cmds_are_bond_tokens() {
        let fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let candidates = fw.scan_state(&state, 110);
        let cmds = fw.build_crystallization_commands(&candidates[0], 109);
        for cmd in &cmds[1..] {
            assert_eq!(cmd.opcode, OpCode::BondTokens as u16);
        }
    }

    // ── update_candidates ───────────────────────────────────────────────────

    #[test]
    fn update_candidates_adds_new_with_stability_one() {
        let mut fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let scan = fw.scan_state(&state, 110);
        let hash = scan[0].lineage_hash;
        fw.update_candidates(scan, 1);
        assert_eq!(fw.candidates[&hash].stability_count, 1);
    }

    #[test]
    fn update_candidates_increments_stability_on_repeat() {
        let mut fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let hash = fw.scan_state(&state, 110)[0].lineage_hash;
        fw.update_candidates(fw.scan_state(&state, 110), 1);
        fw.update_candidates(fw.scan_state(&state, 110), 2);
        assert_eq!(fw.candidates[&hash].stability_count, 2);
    }

    #[test]
    fn update_candidates_removes_disappeared() {
        let mut fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let hash = fw.scan_state(&state, 110)[0].lineage_hash;
        fw.update_candidates(fw.scan_state(&state, 110), 1);
        fw.update_candidates(Vec::new(), 2);
        assert!(!fw.candidates.contains_key(&hash));
    }

    // ── on_tick — кристаллизация ─────────────────────────────────────────────

    #[test]
    fn on_tick_crystallizes_at_stability_threshold() {
        let mut fw = FrameWeaver::new(FrameWeaverConfig {
            scan_interval_ticks: 1,
            stability_threshold: 2,
            ..Default::default()
        });
        let mut ashti = AshtiCore::new(1);
        ashti.inject_connection(110, make_syn_conn(10, 20, 1)).unwrap();
        ashti.inject_connection(110, make_syn_conn(10, 30, 2)).unwrap();

        fw.on_tick(1, &ashti).unwrap();
        assert!(fw.drain_commands().is_empty(), "tick 1: stability=1, not yet stable");

        fw.on_tick(2, &ashti).unwrap();
        let cmds = fw.drain_commands();
        assert!(!cmds.is_empty(), "tick 2: stability=2 >= threshold, should crystallize");
        assert_eq!(cmds[0].opcode, OpCode::InjectToken as u16);
    }

    #[test]
    fn on_tick_no_crystallization_below_threshold() {
        let mut fw = FrameWeaver::new(FrameWeaverConfig {
            scan_interval_ticks: 1,
            stability_threshold: 5,
            ..Default::default()
        });
        let mut ashti = AshtiCore::new(1);
        ashti.inject_connection(110, make_syn_conn(10, 20, 1)).unwrap();
        ashti.inject_connection(110, make_syn_conn(10, 30, 2)).unwrap();

        for tick in 1..5 {
            fw.on_tick(tick, &ashti).unwrap();
            assert!(fw.drain_commands().is_empty(), "tick {tick}: still below threshold");
        }
    }

    // ── on_tick — реактивация ────────────────────────────────────────────────

    #[test]
    fn on_tick_reactivates_existing_anchor() {
        let mut fw = FrameWeaver::new(FrameWeaverConfig {
            scan_interval_ticks: 1,
            stability_threshold: 2,
            ..Default::default()
        });
        let mut ashti = AshtiCore::new(1);
        ashti.inject_connection(110, make_syn_conn(10, 20, 1)).unwrap();
        ashti.inject_connection(110, make_syn_conn(10, 30, 2)).unwrap();

        // Вычислить hash и proposed_id, пока borrow на ashti ещё не активен
        let hash = {
            let maya = ashti.state(ashti.index_of(110).unwrap()).unwrap();
            fw.scan_state(maya, 110)[0].lineage_hash
        };
        let proposed_id = FrameWeaver::proposed_id_from_hash(hash);

        // Посадить анкер в EXPERIENCE заранее
        let mut anchor = Token::new(proposed_id, 109, [0; 3], 1);
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR;
        anchor.lineage_hash = hash;
        ashti.inject_token(109, anchor).unwrap();

        fw.on_tick(1, &ashti).unwrap();
        fw.drain_commands(); // tick 1: stability=1, skip

        fw.on_tick(2, &ashti).unwrap();
        let cmds = fw.drain_commands();
        assert!(!cmds.is_empty());
        assert_eq!(cmds[0].opcode, OpCode::ReinforceFrame as u16);
    }

    // ── drain_commands ───────────────────────────────────────────────────────

    #[test]
    fn drain_commands_empties_pending() {
        let mut fw = FrameWeaver::with_default_config();
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        let candidates = fw.scan_state(&state, 110);
        fw.pending_commands.extend(fw.build_crystallization_commands(&candidates[0], 109));

        assert!(!fw.drain_commands().is_empty());
        assert!(fw.pending_commands.is_empty());
    }

    // ── check_promotion ──────────────────────────────────────────────────────

    #[test]
    fn check_promotion_fails_without_reactivations() {
        let fw = FrameWeaver::with_default_config();
        let mut anchor = Token::new(42, 109, [0; 3], 0);
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR;
        anchor.temperature = 255;
        anchor.mass = 255;
        // min_reactivations=10, но reactivation_counts пустой
        let proposals = fw.check_promotion(0, &empty_state(), &[&anchor]);
        assert!(proposals.is_empty());
    }

    #[test]
    fn check_promotion_skips_non_anchors() {
        let fw = FrameWeaver::with_default_config();
        let token = Token::new(42, 109, [0; 3], 0);
        // type_flags=0, не TOKEN_FLAG_FRAME_ANCHOR
        let proposals = fw.check_promotion(0, &empty_state(), &[&token]);
        assert!(proposals.is_empty());
    }

    // ── stats ────────────────────────────────────────────────────────────────

    #[test]
    fn stats_scans_increments_on_tick() {
        let mut fw = FrameWeaver::new(FrameWeaverConfig { scan_interval_ticks: 1, ..Default::default() });
        let ashti = AshtiCore::new(1);
        fw.on_tick(1, &ashti).unwrap();
        fw.on_tick(2, &ashti).unwrap();
        assert_eq!(fw.stats.scans_performed, 2);
    }

    // ── этап 1: tick в трейт-методах ────────────────────────────────────────

    #[test]
    fn check_promotion_respects_min_age() {
        let config = FrameWeaverConfig {
            promotion_rules: vec![PromotionRule {
                min_age_ticks: 500,
                min_reactivations: 0,
                min_temperature: 0,
                min_mass: 0,
                min_participant_anchors: 0,
                requires_codex_approval: false,
                id: "test".to_string(),
            }],
            ..Default::default()
        };
        let fw = FrameWeaver::new(config);
        let mut anchor = Token::new(42, 109, [0; 3], 100); // last_event_id = 100
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR;

        // tick=200: возраст = 200-100 = 100 < 500 → не дозрел
        let proposals = fw.check_promotion(200, &empty_state(), &[&anchor]);
        assert!(proposals.is_empty(), "не должен продвигать слишком молодой Frame");

        // tick=700: возраст = 700-100 = 600 >= 500 → дозрел
        let proposals = fw.check_promotion(700, &empty_state(), &[&anchor]);
        assert!(!proposals.is_empty(), "должен предложить промоцию зрелого Frame");
    }

    #[test]
    fn scan_records_correct_detection_tick() {
        let mut fw = FrameWeaver::new(FrameWeaverConfig {
            scan_interval_ticks: 1,
            ..Default::default()
        });
        let state = state_with_conns(vec![make_syn_conn(10, 20, 1), make_syn_conn(10, 30, 2)]);
        fw.scan(50, &state);
        // после scan() кандидат должен быть добавлен с detected_at_tick = 50
        let hash = fw.scan_state(&state, 0)[0].lineage_hash;
        assert_eq!(fw.candidates[&hash].detected_at_tick, 50);
    }

    #[test]
    fn stats_candidates_detected_increments() {
        let mut fw = FrameWeaver::new(FrameWeaverConfig { scan_interval_ticks: 1, ..Default::default() });
        let mut ashti = AshtiCore::new(1);
        ashti.inject_connection(110, make_syn_conn(10, 20, 1)).unwrap();
        ashti.inject_connection(110, make_syn_conn(10, 30, 2)).unwrap();

        fw.on_tick(1, &ashti).unwrap();
        assert_eq!(fw.stats.candidates_detected, 1);
        // Второй тик: тот же кандидат уже существует в map → не детектируется снова
        fw.on_tick(2, &ashti).unwrap();
        assert_eq!(fw.stats.candidates_detected, 1);
    }

    // ── этап 2: restore_frame_from_anchor ────────────────────────────────────

    fn make_frame_state(anchor_id: u32, participant_ids: &[u32]) -> DomainState {
        let mut state = empty_state();
        // Anchor token
        let mut anchor = Token::new(anchor_id, 109, [0; 3], 0);
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX;
        anchor.lineage_hash = FrameWeaver::fnv1a_lineage_hash(
            &std::iter::once(anchor_id).chain(participant_ids.iter().copied()).collect::<Vec<_>>()
        );
        state.tokens.push(anchor);
        // Participant tokens + connections
        for (i, &pid) in participant_ids.iter().enumerate() {
            let ptok = Token::new(pid, 100, [0; 3], 0);
            state.tokens.push(ptok);
            let layer = (i as u8 % 8) + 1;
            let link_type = 0x0800u16 | ((layer as u16) << 4);
            let mut conn = Connection::new(anchor_id, pid, 109, 1);
            conn.link_type = link_type;
            conn.reserved_gate[0] = 0;   // origin_domain BE hi
            conn.reserved_gate[1] = 110; // origin_domain BE lo (=110=MAYA)
            state.connections.push(conn);
        }
        state
    }

    #[test]
    fn restore_simple_frame() {
        let state = make_frame_state(1000, &[1001, 1002]);
        let r = restore_frame_from_anchor(1000, &state).unwrap();
        assert_eq!(r.anchor_id, 1000);
        assert_eq!(r.category, FRAME_CATEGORY_SYNTAX);
        assert_eq!(r.participants.len(), 2);
        assert!(r.participants.iter().all(|p| p.origin_domain_id == 110));
    }

    #[test]
    fn restore_returns_error_for_non_anchor() {
        let mut state = empty_state();
        let tok = Token::new(42, 109, [0; 3], 0); // type_flags=0
        state.tokens.push(tok);
        let err = restore_frame_from_anchor(42, &state).unwrap_err();
        assert!(matches!(err, RestoreError::NotAFrameAnchor));
    }

    #[test]
    fn restore_returns_error_for_missing_anchor() {
        let err = restore_frame_from_anchor(9999, &empty_state()).unwrap_err();
        assert!(matches!(err, RestoreError::AnchorNotFound));
    }

    #[test]
    fn restore_detects_dangling_participant() {
        let mut state = empty_state();
        let mut anchor = Token::new(1000, 109, [0; 3], 0);
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR;
        state.tokens.push(anchor);
        // Связь к target 9999, которого нет в tokens
        let mut conn = Connection::new(1000, 9999, 109, 1);
        conn.link_type = 0x0810; // syntactic, layer 1
        state.connections.push(conn);
        let err = restore_frame_from_anchor(1000, &state).unwrap_err();
        assert!(matches!(err, RestoreError::DanglingParticipant(9999)));
    }

    #[test]
    fn restore_extracts_correct_layers() {
        let mut state = empty_state();
        let mut anchor = Token::new(1000, 109, [0; 3], 0);
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR;
        state.tokens.push(anchor);
        // 8 участников, по одному на каждый слой
        for layer in 1u8..=8 {
            let pid = 2000 + layer as u32;
            state.tokens.push(Token::new(pid, 100, [0; 3], 0));
            let link_type = 0x0800u16 | ((layer as u16) << 4);
            let mut conn = Connection::new(1000, pid, 109, 1);
            conn.link_type = link_type;
            state.connections.push(conn);
        }
        let r = restore_frame_from_anchor(1000, &state).unwrap();
        assert_eq!(r.participants.len(), 8);
        let mut found_layers: Vec<u8> = r.participants.iter().map(|p| p.layer).collect();
        found_layers.sort_unstable();
        assert_eq!(found_layers, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    // ── этап 2: промоция с restore_frame_from_anchor ─────────────────────────

    fn make_promotion_ashti(anchor_id: u32, participant_ids: &[u32]) -> (AshtiCore, u64) {
        let mut ashti = AshtiCore::new(1);
        // EXPERIENCE = level*100+9 = 109
        let mut anchor = Token::new(anchor_id, 109, [0; 3], 0);
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX;
        anchor.lineage_hash = FrameWeaver::fnv1a_lineage_hash(
            &std::iter::once(anchor_id).chain(participant_ids.iter().copied()).collect::<Vec<_>>()
        );
        anchor.temperature = 255;
        anchor.mass = 255;
        ashti.inject_token(109, anchor).unwrap();
        // Inject participant tokens into SUTRA (100) и в EXPERIENCE (109) для restore_frame_from_anchor
        for &pid in participant_ids {
            ashti.inject_token(100, Token::new(pid, 100, [0; 3], 0)).unwrap();
            ashti.inject_token(109, Token::new(pid, 109, [0; 3], 0)).unwrap();
        }
        // Inject syntactic bonds in EXPERIENCE
        for (i, &pid) in participant_ids.iter().enumerate() {
            let layer = (i as u8 % 8) + 1;
            let link_type = 0x0800u16 | ((layer as u16) << 4);
            let mut conn = Connection::new(anchor_id, pid, 109, 1);
            conn.link_type = link_type;
            conn.reserved_gate[1] = 110; // origin_domain=110
            ashti.inject_connection(109, conn).unwrap();
        }
        let hash = FrameWeaver::fnv1a_lineage_hash(
            &std::iter::once(anchor_id).chain(participant_ids.iter().copied()).collect::<Vec<_>>()
        );
        (ashti, hash)
    }

    #[test]
    fn promotion_creates_sutra_frame_with_participants() {
        let config = FrameWeaverConfig {
            scan_interval_ticks: 1,
            promotion_rules: vec![PromotionRule {
                min_age_ticks:           0,
                min_reactivations:       0,
                min_temperature:         200,
                min_mass:                100,
                min_participant_anchors: 0,
                requires_codex_approval: false,
                id:                      "test_promo".to_string(),
            }],
            ..Default::default()
        };
        let mut fw = FrameWeaver::new(config);
        let anchor_id = 5000u32;
        let participants = &[5001u32, 5002, 5003];
        let (ashti, _hash) = make_promotion_ashti(anchor_id, participants);

        // Запустить тик — промоция должна сработать
        fw.on_tick(200_000, &ashti).unwrap();
        let cmds = fw.drain_commands();

        // Должна быть хотя бы одна команда InjectToken (SUTRA-анкер) + 3 BondTokens
        let inject_count = cmds.iter().filter(|c| c.opcode == OpCode::InjectToken as u16).count();
        let bond_count = cmds.iter().filter(|c| c.opcode == OpCode::BondTokens as u16).count();
        assert!(inject_count >= 1, "ожидается SUTRA-анкер");
        assert_eq!(bond_count, participants.len(), "ожидается {n} bonds", n = participants.len());
        assert_eq!(fw.stats.promotions_proposed, 1);
    }

    #[test]
    fn promotion_skipped_for_dangling_anchor() {
        let config = FrameWeaverConfig {
            scan_interval_ticks: 1,
            promotion_rules: vec![PromotionRule {
                min_age_ticks:           0,
                min_reactivations:       0,
                min_temperature:         200,
                min_mass:                100,
                min_participant_anchors: 0,
                requires_codex_approval: false,
                id:                      "test_promo".to_string(),
            }],
            ..Default::default()
        };
        let mut fw = FrameWeaver::new(config);
        let mut ashti = AshtiCore::new(1);
        // Анкер есть, но нет связей → restore вернёт Ok с пустыми participants
        let mut anchor = Token::new(7000, 109, [0; 3], 0);
        anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX;
        anchor.temperature = 255;
        anchor.mass = 255;
        ashti.inject_token(109, anchor).unwrap();
        // Добавить связь к несуществующему токену
        let mut bad_conn = Connection::new(7000, 9999, 109, 1);
        bad_conn.link_type = 0x0810;
        ashti.inject_connection(109, bad_conn).unwrap();

        fw.on_tick(200_000, &ashti).unwrap();
        let cmds = fw.drain_commands();
        // restore вернёт DanglingParticipant → промоция пропущена
        assert!(cmds.is_empty(), "промоция не должна выполняться при dangling participant");
        assert_eq!(fw.stats.promotions_proposed, 0);
    }
}
