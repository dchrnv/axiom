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
    TOKEN_FLAG_FRAME_ANCHOR, TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE, FRAME_CATEGORY_SYNTAX,
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
                        // Reconstruction FrameCandidate из анкера для build_promotion_commands
                        let dummy_candidate = FrameCandidate {
                            anchor_position: token.position,
                            participants: Vec::new(), // упрощение: участники не восстанавливаются здесь
                            detected_at_tick: token.last_event_id,
                            stability_count: 0,
                            category: token.type_flags & axiom_core::FRAME_CATEGORY_MASK,
                            lineage_hash: token.lineage_hash,
                        };
                        let cmds = Self::build_promotion_commands(
                            &dummy_candidate, token.sutra_id, sutra_domain_id
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

    fn scan(&mut self, maya_state: &DomainState) -> Vec<FrameCandidate> {
        // Вызывается напрямую (для unit-тестов и DREAM-интеграции в Phase 4).
        // В on_tick используется scan_state с явным domain_id.
        self.scan_state(maya_state, 0)
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
        experience_state: &DomainState,
        anchors: &[&Token],
    ) -> Vec<PromotionProposal> {
        let mut proposals = Vec::new();
        let tick_proxy = 0u64; // tick недоступен в этой сигнатуре; используется 0 (deferred: передать tick через параметр)

        for anchor in anchors {
            if (anchor.type_flags & TOKEN_FLAG_FRAME_ANCHOR) == 0 {
                continue;
            }
            for rule in &self.config.promotion_rules {
                if self.qualifies_for_promotion(anchor, rule, tick_proxy) {
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
