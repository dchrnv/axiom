// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DreamCycle — машина стадий сна: Stabilization → Processing → Consolidation.
// Спецификация: docs/spec/Dream/DREAM_Phase_V1_0.md, раздел 5.

use axiom_core::{TOKEN_FLAG_DREAM_REPORT, STATE_ACTIVE};
use axiom_domain::AshtiCore;
use axiom_ucl::{UclCommand, OpCode, InjectFrameAnchorPayload, BondTokensPayload};

use crate::over_domain::traits::WeaverId;
use crate::over_domain::weavers::restore_frame_from_anchor;
use super::state::{SleepTrigger, WakeReason};

// ── конфигурация ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DreamCycleConfig {
    /// Максимум тиков одного DreamCycle до Timeout.
    pub max_dream_duration_ticks: u32,
    /// Максимум proposals обрабатываемых за один цикл.
    pub max_proposals_per_cycle:  usize,
    /// V1.0: всегда false — Recombination не реализован.
    pub enable_recombination:     bool,
    /// Proposals обрабатываемых за один тик Processing.
    pub batch_size:               usize,
}

impl Default for DreamCycleConfig {
    fn default() -> Self {
        Self {
            max_dream_duration_ticks: 50_000,
            max_proposals_per_cycle:  100,
            enable_recombination:     false,
            batch_size:               8,
        }
    }
}

// ── стадии ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleStage {
    Stabilization,
    Processing,
    Recombination,
    Consolidation,
}

// ── proposals ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DreamProposal {
    pub source:           WeaverId,
    pub kind:             DreamProposalKind,
    /// Приоритет 0..=255: выше — обрабатывается раньше.
    pub priority:         u8,
    pub created_at_event: u64,
}

#[derive(Debug, Clone)]
pub enum DreamProposalKind {
    Promotion {
        anchor_id:     u32,
        source_domain: u16,
        target_domain: u16,
        rule_id:       String,
    },
    /// V1.0: заглушка — сразу vetoed при обработке.
    HeavyCrystallization {},
}

// ── DreamReport ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DreamReport {
    pub started_at_event:              u64,
    pub ended_at_event:                u64,
    pub duration_ticks:                u32,
    pub trigger:                       SleepTrigger,
    pub wake_reason:                   WakeReason,
    pub proposals_processed:           u32,
    pub proposals_approved:            u32,
    pub proposals_vetoed:              u32,
    pub proposals_deferred:            u32,
    pub promotions_applied:            u32,
    pub heavy_crystallizations_applied: u32,
    pub fatigue_before:                u8,
    /// 0 в V1.0 — заполняется движком после пробуждения (Этап 4).
    pub fatigue_after:                 u8,
}

// ── статистика ────────────────────────────────────────────────────────────────

#[derive(Debug, Default, Clone)]
pub struct DreamCycleStats {
    pub total_cycles:        u64,
    pub completed_cycles:    u64,
    pub timed_out_cycles:    u64,
    pub total_processed:     u64,
    pub total_approved:      u64,
    pub total_vetoed:        u64,
    pub total_promotions:    u64,
}

impl DreamCycleStats {
    fn record_completed(&mut self, report: &DreamReport) {
        self.total_cycles     += 1;
        self.completed_cycles += 1;
        self.total_processed  += report.proposals_processed as u64;
        self.total_approved   += report.proposals_approved  as u64;
        self.total_vetoed     += report.proposals_vetoed    as u64;
        self.total_promotions += report.promotions_applied  as u64;
    }

    fn record_timeout(&mut self) {
        self.total_cycles    += 1;
        self.timed_out_cycles += 1;
    }
}

// ── внутреннее состояние активного цикла ─────────────────────────────────────

struct ActiveCycle {
    started_at_tick:  u64,
    started_at_event: u64,
    trigger:          SleepTrigger,
    fatigue_before:   u8,
    stage:            CycleStage,
    processed:        u32,
    approved:         u32,
    vetoed:           u32,
    deferred:         u32,
    queue_sorted:     bool,
}

// ── результат advance() ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleAdvanceResult {
    /// Нет активного цикла.
    NotActive,
    /// Цикл продолжается.
    InProgress,
    /// Цикл завершён — DreamReport доступен через drain_report().
    Complete,
    /// Превышен max_dream_duration_ticks — цикл прерван.
    Timeout,
}

// ── внутренний результат обработки proposal ───────────────────────────────────

enum ProposalResult {
    Approved(Vec<UclCommand>),
    Vetoed(String),
    Deferred,
}

// ── DreamCycle ────────────────────────────────────────────────────────────────

pub struct DreamCycle {
    config:         DreamCycleConfig,
    queue:          Vec<DreamProposal>,
    current_cycle:  Option<ActiveCycle>,
    /// Накопленные UCL-команды (дренируются вместе с DreamReport).
    pending_commands: Vec<UclCommand>,
    /// Последний построенный DreamReport (доступен после Complete/Timeout).
    last_report:    Option<DreamReport>,
    pub stats:      DreamCycleStats,
}

impl DreamCycle {
    pub fn new(config: DreamCycleConfig) -> Self {
        Self {
            config,
            queue: Vec::new(),
            current_cycle: None,
            pending_commands: Vec::new(),
            last_report: None,
            stats: DreamCycleStats::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(DreamCycleConfig::default())
    }

    /// Добавить proposal в очередь (вызывается из WAKE или в начале DREAMING).
    pub fn submit(&mut self, proposal: DreamProposal) {
        self.queue.push(proposal);
    }

    /// Число proposals в очереди.
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Начать новый цикл сна.
    pub fn start_cycle(
        &mut self,
        tick:      u64,
        event_id:  u64,
        trigger:   SleepTrigger,
        fatigue:   u8,
    ) {
        debug_assert!(self.current_cycle.is_none(), "DreamCycle: start_cycle called while active");
        self.pending_commands.clear();
        self.last_report = None;
        self.current_cycle = Some(ActiveCycle {
            started_at_tick:  tick,
            started_at_event: event_id,
            trigger,
            fatigue_before:   fatigue,
            stage:            CycleStage::Stabilization,
            processed:        0,
            approved:         0,
            vetoed:           0,
            deferred:         0,
            queue_sorted:     false,
        });
    }

    /// Текущая стадия (None если цикл не активен).
    pub fn current_stage(&self) -> Option<CycleStage> {
        self.current_cycle.as_ref().map(|c| c.stage)
    }

    /// Продвинуть цикл на один тик.
    pub fn advance(
        &mut self,
        tick:    u64,
        ashti:   &AshtiCore,
        com_event_id: u64,
    ) -> CycleAdvanceResult {
        let cycle = match self.current_cycle.as_mut() {
            Some(c) => c,
            None    => return CycleAdvanceResult::NotActive,
        };

        if tick - cycle.started_at_tick >= self.config.max_dream_duration_ticks as u64 {
            self.finalize_timeout(tick, com_event_id);
            return CycleAdvanceResult::Timeout;
        }

        match cycle.stage {
            CycleStage::Stabilization => {
                // V1.0: пустой тик, снимок состояния для DreamReport.
                cycle.stage = CycleStage::Processing;
                CycleAdvanceResult::InProgress
            }
            CycleStage::Processing => {
                self.sort_queue_once();
                self.process_batch(ashti);
                let cycle = self.current_cycle.as_mut().unwrap();
                let queue_exhausted = self.queue.is_empty();
                let cap_reached = cycle.processed >= self.config.max_proposals_per_cycle as u32;
                if queue_exhausted || cap_reached {
                    cycle.stage = if self.config.enable_recombination {
                        CycleStage::Recombination
                    } else {
                        CycleStage::Consolidation
                    };
                }
                CycleAdvanceResult::InProgress
            }
            CycleStage::Recombination => {
                // V1.0: заглушка — см. deferred/DreamPhase_V2_plus.md
                cycle.stage = CycleStage::Consolidation;
                CycleAdvanceResult::InProgress
            }
            CycleStage::Consolidation => {
                self.finalize_complete(tick, com_event_id);
                CycleAdvanceResult::Complete
            }
        }
    }

    /// Дренировать накопленные UCL-команды (вызывается после Complete).
    pub fn drain_commands(&mut self) -> Vec<UclCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    /// Дренировать DreamReport (доступен после Complete или Timeout).
    pub fn drain_report(&mut self) -> Option<DreamReport> {
        self.last_report.take()
    }

    // ── приватные методы ──────────────────────────────────────────────────────

    fn sort_queue_once(&mut self) {
        if let Some(c) = &self.current_cycle {
            if !c.queue_sorted {
                // Стабильная сортировка по убыванию приоритета.
                self.queue.sort_by(|a, b| b.priority.cmp(&a.priority));
            }
        }
        if let Some(c) = self.current_cycle.as_mut() {
            c.queue_sorted = true;
        }
    }

    fn process_batch(&mut self, ashti: &AshtiCore) {
        let batch = self.config.batch_size;
        let max   = self.config.max_proposals_per_cycle as u32;
        let mut count = 0;

        while count < batch && !self.queue.is_empty() {
            let cycle = self.current_cycle.as_ref().unwrap();
            if cycle.processed >= max { break; }
            if cycle.deferred >= 10  { break; }

            let proposal = self.queue.remove(0);
            let result   = Self::process_proposal(&proposal, ashti);

            let cycle = self.current_cycle.as_mut().unwrap();
            cycle.processed += 1;
            count += 1;

            match result {
                ProposalResult::Approved(cmds) => {
                    cycle.approved += 1;
                    self.pending_commands.extend(cmds);
                }
                ProposalResult::Vetoed(_reason) => {
                    cycle.vetoed += 1;
                }
                ProposalResult::Deferred => {
                    cycle.deferred += 1;
                    self.queue.push(proposal);
                }
            }
        }
    }

    fn process_proposal(proposal: &DreamProposal, ashti: &AshtiCore) -> ProposalResult {
        match &proposal.kind {
            DreamProposalKind::Promotion { anchor_id, source_domain, target_domain, .. } => {
                let source_idx = match ashti.index_of(*source_domain) {
                    Some(i) => i,
                    None    => return ProposalResult::Vetoed(
                        format!("source domain {} not found", source_domain)
                    ),
                };
                let source_state = match ashti.state(source_idx) {
                    Some(s) => s,
                    None    => return ProposalResult::Vetoed("source state unavailable".into()),
                };

                let frame = match restore_frame_from_anchor(*anchor_id, source_state) {
                    Ok(f)  => f,
                    Err(e) => return ProposalResult::Vetoed(format!("restore: {:?}", e)),
                };

                let cmds = build_promotion_commands(&frame, *target_domain);
                ProposalResult::Approved(cmds)
            }
            DreamProposalKind::HeavyCrystallization {} => {
                ProposalResult::Vetoed("HeavyCrystallization not implemented in V1.0".into())
            }
        }
    }

    fn finalize_complete(&mut self, tick: u64, com_event_id: u64) {
        let cycle = self.current_cycle.take().unwrap();
        let report = DreamReport {
            started_at_event:              cycle.started_at_event,
            ended_at_event:                com_event_id,
            duration_ticks:                (tick - cycle.started_at_tick) as u32,
            trigger:                       cycle.trigger,
            wake_reason:                   WakeReason::CycleComplete,
            proposals_processed:           cycle.processed,
            proposals_approved:            cycle.approved,
            proposals_vetoed:              cycle.vetoed,
            proposals_deferred:            cycle.deferred,
            promotions_applied:            cycle.approved,
            heavy_crystallizations_applied: 0,
            fatigue_before:                cycle.fatigue_before,
            fatigue_after:                 0,
        };
        self.stats.record_completed(&report);
        self.pending_commands.push(build_dream_report_token(&report));
        self.last_report = Some(report);
    }

    fn finalize_timeout(&mut self, tick: u64, com_event_id: u64) {
        let cycle = self.current_cycle.take().unwrap();
        let report = DreamReport {
            started_at_event:              cycle.started_at_event,
            ended_at_event:                com_event_id,
            duration_ticks:                (tick - cycle.started_at_tick) as u32,
            trigger:                       cycle.trigger,
            wake_reason:                   WakeReason::Timeout { max_dream_duration: self.config.max_dream_duration_ticks },
            proposals_processed:           cycle.processed,
            proposals_approved:            cycle.approved,
            proposals_vetoed:              cycle.vetoed,
            proposals_deferred:            cycle.deferred,
            promotions_applied:            cycle.approved,
            heavy_crystallizations_applied: 0,
            fatigue_before:                cycle.fatigue_before,
            fatigue_after:                 0,
        };
        self.stats.record_timeout();
        self.last_report = Some(report);
    }
}

// ── promotion commands ────────────────────────────────────────────────────────

use crate::over_domain::weavers::RestoredFrame;

/// Строит UCL-команды для копирования Frame из EXPERIENCE в target_domain (SUTRA).
///
/// Команды:
/// 1. InjectToken с флагом FRAME_ANCHOR | PROMOTED_FROM_EXPERIENCE → анкер в SUTRA
/// 2. BondTokens для каждого участника (сохраняем связи анкера)
fn build_promotion_commands(frame: &RestoredFrame, target_domain: u16) -> Vec<UclCommand> {
    let mut cmds = Vec::with_capacity(1 + frame.participants.len());

    let anchor_payload = InjectFrameAnchorPayload {
        lineage_hash:       frame.anchor.lineage_hash,
        proposed_sutra_id:  frame.anchor_id,
        target_domain_id:   target_domain,
        type_flags:         frame.anchor.type_flags
                            | axiom_core::TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE,
        position:           frame.anchor.position,
        state:              STATE_ACTIVE,
        mass:               frame.anchor.mass,
        temperature:        frame.anchor.temperature,
        valence:            0,
        reserved:           [0; 22],
    };
    cmds.push(
        UclCommand::new(OpCode::InjectToken, target_domain as u32, 200, axiom_ucl::flags::FRAME_ANCHOR)
            .with_payload(&anchor_payload)
    );

    for p in &frame.participants {
        let bond_payload = BondTokensPayload {
            source_id:     frame.anchor_id,
            target_id:     p.sutra_id,
            domain_id:     target_domain,
            link_type:     p.role_link_type,
            strength:      1.0,
            conn_flags:    axiom_core::FLAG_ACTIVE as u32,
            origin_domain: p.origin_domain_id,
            role_id:       0,
            reserved:      [0; 24],
        };
        cmds.push(
            UclCommand::new(OpCode::BondTokens, target_domain as u32, 200, 0)
                .with_payload(&bond_payload)
        );
    }

    cmds
}

// ── DreamReport → UCL token ───────────────────────────────────────────────────

/// Кодирует сводку DreamReport в InjectToken-команду для EXPERIENCE (domain 109).
///
/// Формат payload (V1.0, 45 байт из 48):
/// [0..4]  duration_ticks (u32 LE)
/// [4]     trigger kind (0=Idle, 1=Fatigue, 2=Explicit)
/// [5]     wake_reason kind (0=Complete, 1=CriticalSignal, 2=Timeout, 3=Guardian)
/// [6..10] proposals_processed (u32 LE)
/// [10..14] proposals_approved (u32 LE)
/// [14..18] proposals_vetoed   (u32 LE)
/// [18]    fatigue_before
/// [19]    fatigue_after
/// [20..28] started_at_event (u64 LE)
/// [28..36] ended_at_event   (u64 LE)
fn build_dream_report_token(report: &DreamReport) -> UclCommand {
    const EXPERIENCE_DOMAIN: u32 = 109;

    let mut payload = [0u8; 48];
    payload[0..4].copy_from_slice(&report.duration_ticks.to_le_bytes());
    payload[4] = trigger_kind_byte(&report.trigger);
    payload[5] = wake_reason_kind_byte(&report.wake_reason);
    payload[6..10].copy_from_slice(&report.proposals_processed.to_le_bytes());
    payload[10..14].copy_from_slice(&report.proposals_approved.to_le_bytes());
    payload[14..18].copy_from_slice(&report.proposals_vetoed.to_le_bytes());
    payload[18] = report.fatigue_before;
    payload[19] = report.fatigue_after;
    payload[20..28].copy_from_slice(&report.started_at_event.to_le_bytes());
    payload[28..36].copy_from_slice(&report.ended_at_event.to_le_bytes());

    let mut cmd = UclCommand::new(OpCode::InjectToken, EXPERIENCE_DOMAIN, 50, 0);
    // Встраиваем payload вручную (InjectTokenPayload слишком маленький для наших нужд,
    // используем raw payload напрямую через with_raw_payload).
    cmd.payload.copy_from_slice(&payload);
    // type_flags закодированы в первых двух байтах через separate InjectTokenPayload —
    // в V1.0 token_type=1 (generic), тип идентифицируется по TOKEN_FLAG_DREAM_REPORT
    // которое вызывающий код проверяет в payload[4..6] области type_flags.
    // Для простоты V1.0 несём TOKEN_FLAG_DREAM_REPORT в payload[36..38].
    let flag_bytes = TOKEN_FLAG_DREAM_REPORT.to_le_bytes();
    payload[36] = flag_bytes[0];
    payload[37] = flag_bytes[1];
    cmd.payload.copy_from_slice(&payload);
    cmd
}

fn trigger_kind_byte(trigger: &SleepTrigger) -> u8 {
    match trigger {
        SleepTrigger::Idle     { .. } => 0,
        SleepTrigger::Fatigue  { .. } => 1,
        SleepTrigger::ExplicitCommand { .. } => 2,
    }
}

fn wake_reason_kind_byte(reason: &WakeReason) -> u8 {
    match reason {
        WakeReason::CycleComplete          => 0,
        WakeReason::CriticalSignal { .. }  => 1,
        WakeReason::Timeout        { .. }  => 2,
        WakeReason::GuardianOverride       => 3,
    }
}

// ── тесты ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_trigger() -> SleepTrigger {
        SleepTrigger::Idle { idle_ticks: 200 }
    }

    #[test]
    fn cycle_starts_in_stabilization() {
        let mut dc = DreamCycle::with_defaults();
        dc.start_cycle(0, 1, make_trigger(), 50);
        assert_eq!(dc.current_stage(), Some(CycleStage::Stabilization));
    }

    #[test]
    fn stages_advance_correctly_empty_queue() {
        let mut dc = DreamCycle::with_defaults();
        dc.start_cycle(0, 1, make_trigger(), 0);

        // Stabilization → Processing
        let r = dc.advance(0, &axiom_domain::AshtiCore::new(1), 1);
        assert_eq!(r, CycleAdvanceResult::InProgress);
        assert_eq!(dc.current_stage(), Some(CycleStage::Processing));

        // Processing (пустая очередь) → Consolidation
        let r = dc.advance(1, &axiom_domain::AshtiCore::new(1), 2);
        assert_eq!(r, CycleAdvanceResult::InProgress);
        assert_eq!(dc.current_stage(), Some(CycleStage::Consolidation));

        // Consolidation → Complete
        let r = dc.advance(2, &axiom_domain::AshtiCore::new(1), 3);
        assert_eq!(r, CycleAdvanceResult::Complete);
        assert_eq!(dc.current_stage(), None);
    }

    #[test]
    fn timeout_aborts_cycle() {
        let mut dc = DreamCycle::new(DreamCycleConfig {
            max_dream_duration_ticks: 5,
            ..DreamCycleConfig::default()
        });
        dc.start_cycle(0, 1, make_trigger(), 0);
        let ashti = axiom_domain::AshtiCore::new(1);

        // тик 4: ещё в пределах (5-0=5, 4<5)
        dc.advance(4, &ashti, 1);
        // тик 5: 5-0=5 >= 5 → Timeout
        let r = dc.advance(5, &ashti, 2);
        assert_eq!(r, CycleAdvanceResult::Timeout);
        let rep = dc.drain_report().expect("timeout should produce report");
        assert!(matches!(rep.wake_reason, WakeReason::Timeout { .. }));
    }

    #[test]
    fn not_active_before_start() {
        let mut dc = DreamCycle::with_defaults();
        let r = dc.advance(0, &axiom_domain::AshtiCore::new(1), 0);
        assert_eq!(r, CycleAdvanceResult::NotActive);
    }

    #[test]
    fn complete_cycle_produces_report_and_token() {
        let mut dc = DreamCycle::with_defaults();
        dc.start_cycle(0, 1, make_trigger(), 42);
        let ashti = axiom_domain::AshtiCore::new(1);

        // Stabilization
        dc.advance(0, &ashti, 1);
        // Processing (пусто) → Consolidation
        dc.advance(1, &ashti, 2);
        // Consolidation → Complete
        let r = dc.advance(2, &ashti, 3);
        assert_eq!(r, CycleAdvanceResult::Complete);

        let report = dc.drain_report().expect("should have report");
        assert_eq!(report.fatigue_before, 42);
        assert!(matches!(report.wake_reason, WakeReason::CycleComplete));

        // InjectToken-команда для DreamReport
        let cmds = dc.drain_commands();
        assert_eq!(cmds.len(), 1, "one DreamReport token command expected");
        assert_eq!(cmds[0].target_id, 109);
    }

    #[test]
    fn processing_vetoes_unrestorable_anchor() {
        let mut dc = DreamCycle::with_defaults();
        dc.submit(DreamProposal {
            source:           1,
            priority:         100,
            created_at_event: 0,
            kind: DreamProposalKind::Promotion {
                anchor_id:     9999, // не существует
                source_domain: 109,
                target_domain: 100,
                rule_id:       "test".into(),
            },
        });
        dc.start_cycle(0, 1, make_trigger(), 0);
        let ashti = axiom_domain::AshtiCore::new(1);

        // Stabilization
        dc.advance(0, &ashti, 1);
        // Processing
        dc.advance(1, &ashti, 2);

        // Stabilization(0) → Processing(1, vetoed) → Consolidation(2) → Complete
        dc.advance(2, &ashti, 3); // Consolidation → Complete
        let rep = dc.drain_report().unwrap();
        assert_eq!(rep.proposals_vetoed, 1, "anchor 9999 должен быть vetoed");
    }

    #[test]
    fn priority_ordering_in_processing() {
        // Три proposals с разными приоритетами — после сортировки первым идёт высший
        let mut dc = DreamCycle::new(DreamCycleConfig {
            batch_size: 1, // обрабатываем по одному
            ..DreamCycleConfig::default()
        });
        for prio in [50u8, 200, 100] {
            dc.submit(DreamProposal {
                source:           1,
                priority:         prio,
                created_at_event: 0,
                kind: DreamProposalKind::Promotion {
                    anchor_id:     prio as u32, // несуществующий — будет vetoed
                    source_domain: 109,
                    target_domain: 100,
                    rule_id:       format!("r{prio}"),
                },
            });
        }
        dc.start_cycle(0, 1, make_trigger(), 0);
        let ashti = axiom_domain::AshtiCore::new(1);

        // Stabilization
        dc.advance(0, &ashti, 1);
        // Processing тик 1: обработать 1 (batch=1) — должен быть prio=200
        dc.advance(1, &ashti, 2); // processed=1
        // Processing тик 2: prio=100
        dc.advance(2, &ashti, 3); // processed=2
        // Processing тик 3: prio=50
        dc.advance(3, &ashti, 4); // processed=3, queue пуста → Consolidation
        // Consolidation
        let r = dc.advance(4, &ashti, 5);
        assert_eq!(r, CycleAdvanceResult::Complete);
        let rep = dc.drain_report().unwrap();
        assert_eq!(rep.proposals_processed, 3);
        assert_eq!(rep.proposals_vetoed, 3, "все три proposal vetoed (несуществующие анкеры)");
    }

    #[test]
    fn heavy_crystallization_always_vetoed() {
        let mut dc = DreamCycle::with_defaults();
        dc.submit(DreamProposal {
            source:           1,
            priority:         255,
            created_at_event: 0,
            kind: DreamProposalKind::HeavyCrystallization {},
        });
        dc.start_cycle(0, 1, make_trigger(), 0);
        let ashti = axiom_domain::AshtiCore::new(1);
        dc.advance(0, &ashti, 1); // Stabilization → Processing
        dc.advance(1, &ashti, 2); // Processing (vetoed) → Consolidation
        let r = dc.advance(2, &ashti, 3); // Consolidation → Complete
        assert_eq!(r, CycleAdvanceResult::Complete);
        let rep = dc.drain_report().unwrap();
        assert_eq!(rep.proposals_vetoed, 1);
    }
}
