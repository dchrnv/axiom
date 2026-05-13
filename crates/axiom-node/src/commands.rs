// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// commands — обработка EngineCommand от Workstation.

use std::sync::Arc;

use tracing::debug;

use axiom_agent::perceptors::text::TextPerceptor;
use axiom_broadcasting::{build_system_snapshot, BroadcastHandle};
use axiom_config::AnchorSet;
use axiom_protocol::{
    commands::EngineCommand,
    messages::{CommandResultData, EngineMessage},
};
use axiom_runtime::{AxiomEngine, DreamPhaseState};

use crate::shutdown::ShutdownSignal;

pub fn handle_engine_command(
    cmd_id: u64,
    cmd: EngineCommand,
    engine: &mut AxiomEngine,
    handle: &BroadcastHandle,
    perceptor: &mut TextPerceptor,
    anchor_set: &Option<Arc<AnchorSet>>,
    last_tick_ns: u64,
    shutdown: &ShutdownSignal,
) {
    let _ = anchor_set;
    match cmd {
        EngineCommand::SubmitText { text, .. } => {
            debug!("SubmitText: {:?}", text);
            let ucl = perceptor.perceive(&text);
            engine.process_and_observe(&ucl);
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::None),
            });
        }

        EngineCommand::RequestFullSnapshot => {
            let snap = build_system_snapshot(engine, last_tick_ns);
            handle.update_snapshot(snap.clone());
            handle.publish(EngineMessage::Snapshot(snap));
        }

        EngineCommand::RunBench { spec } => {
            // TODO A4: запуск встроенных бенчмарков через axiom-bench
            debug!("RunBench: {:?}", spec);
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Err("bench not yet implemented in axiom-node".to_string()),
            });
        }

        EngineCommand::ForceSleep => {
            engine.dream_phase_state = DreamPhaseState::FallingAsleep;
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::None),
            });
        }

        EngineCommand::ForceWake => {
            engine.dream_phase_state = DreamPhaseState::Wake;
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::None),
            });
        }

        EngineCommand::GracefulShutdown => {
            debug!("GracefulShutdown received");
            shutdown.trigger();
        }

        EngineCommand::ForceShutdown => {
            debug!("ForceShutdown received");
            shutdown.trigger();
        }

        EngineCommand::InjectToken { domain_id, layer, content } => {
            // TODO A5: map content → Token (requires FNV-1a / embedding lookup)
            debug!("InjectToken stub: domain={} layer={} content={:?}", domain_id, layer, content);
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Err("InjectToken not yet implemented in axiom-node".to_string()),
            });
        }

        EngineCommand::InjectConnection { from_domain, to_domain } => {
            debug!("InjectConnection: {} → {}", from_domain, to_domain);
            // TODO A5: add ConnectionInject UCL opcode
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Err(format!("InjectConnection({} → {}) not yet implemented", from_domain, to_domain)),
            });
        }

        EngineCommand::RequestFrameDetails { anchor_id } => {
            let snap = build_system_snapshot(engine, last_tick_ns);
            handle.update_snapshot(snap.clone());
            handle.publish(EngineMessage::Snapshot(snap));
            debug!("RequestFrameDetails anchor_id={}: sent full snapshot", anchor_id);
        }

        other => {
            debug!("unhandled command: {:?}", other);
        }
    }
}
