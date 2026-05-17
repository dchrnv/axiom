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
    config::{
        ConfigCategory, ConfigField, ConfigFieldType, ConfigSchema, ConfigSection, ConfigValue,
    },
    messages::{CommandResultData, EngineMessage},
};
use axiom_runtime::{AxiomEngine, DreamPhaseState};
use axiom_ucl::{OpCode, UclCommand};

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

        EngineCommand::ApproveEmergentCandidate { sutra_id } => {
            let ucl = UclCommand::new(OpCode::ApproveEmergentCandidate, 0, 100, 0)
                .with_payload(&sutra_id);
            engine.process_command(&ucl);
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::None),
            });
        }

        EngineCommand::GetConfigSchema => {
            let schema = build_config_schema(engine);
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::ConfigSchema(schema)),
            });
        }

        EngineCommand::UpdateConfigField { section_id, field_id, value } => {
            apply_config_field(engine, &section_id, &field_id, &value);
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::ConfigUpdateApplied { hot_reloaded: true }),
            });
        }

        EngineCommand::ListAdapters => {
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::AdapterList(vec![])),
            });
        }

        EngineCommand::StartImport { .. } | EngineCommand::CancelImport { .. } => {
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Err("file import not yet implemented in axiom-node".to_string()),
            });
        }

        other => {
            debug!("unhandled command: {:?}", other);
        }
    }
}

fn build_config_schema(engine: &AxiomEngine) -> ConfigSchema {
    use axiom_config::DreamConfig;
    let current = engine.current_dream_config();
    let defaults = DreamConfig::default();
    let sched = &current.scheduler;
    let fw = &current.fatigue_weights;
    let cycle = &current.cycle;

    let scheduler_section = ConfigSection {
        id: "engine.dream.scheduler".to_string(),
        label: "Sleep Scheduler".to_string(),
        category: ConfigCategory::Engine,
        fields: vec![
            ConfigField {
                id: "min_wake_ticks".to_string(),
                label: "Min Wake Ticks".to_string(),
                description: Some(
                    "Minimum ticks awake before sleep is allowed".to_string(),
                ),
                field_type: ConfigFieldType::UInt { min: 0, max: 100_000 },
                current_value: ConfigValue::UInt(sched.min_wake_ticks as u64),
                default_value: ConfigValue::UInt(defaults.scheduler.min_wake_ticks as u64),
                hot_reloadable: true,
                readonly: false,
            },
            ConfigField {
                id: "idle_threshold".to_string(),
                label: "Idle Threshold".to_string(),
                description: Some(
                    "Consecutive idle ticks before falling asleep".to_string(),
                ),
                field_type: ConfigFieldType::UInt { min: 1, max: 10_000 },
                current_value: ConfigValue::UInt(sched.idle_threshold as u64),
                default_value: ConfigValue::UInt(defaults.scheduler.idle_threshold as u64),
                hot_reloadable: true,
                readonly: false,
            },
            ConfigField {
                id: "fatigue_threshold".to_string(),
                label: "Fatigue Threshold".to_string(),
                description: Some("Fatigue score (0–255) that triggers sleep".to_string()),
                field_type: ConfigFieldType::UInt { min: 0, max: 255 },
                current_value: ConfigValue::UInt(sched.fatigue_threshold as u64),
                default_value: ConfigValue::UInt(defaults.scheduler.fatigue_threshold as u64),
                hot_reloadable: true,
                readonly: false,
            },
        ],
        subsections: vec![],
    };

    let fatigue_section = ConfigSection {
        id: "engine.dream.fatigue".to_string(),
        label: "Fatigue Weights".to_string(),
        category: ConfigCategory::Engine,
        fields: vec![
            ConfigField {
                id: "uncrystallized_candidates".to_string(),
                label: "Uncrystallized Candidates".to_string(),
                description: Some("Weight for FrameWeaver candidates without crystallization".to_string()),
                field_type: ConfigFieldType::UInt { min: 0, max: 255 },
                current_value: ConfigValue::UInt(fw.uncrystallized_candidates as u64),
                default_value: ConfigValue::UInt(defaults.fatigue_weights.uncrystallized_candidates as u64),
                hot_reloadable: true,
                readonly: false,
            },
            ConfigField {
                id: "experience_pressure".to_string(),
                label: "Experience Pressure".to_string(),
                description: Some("Weight for EXPERIENCE domain fill ratio".to_string()),
                field_type: ConfigFieldType::UInt { min: 0, max: 255 },
                current_value: ConfigValue::UInt(fw.experience_pressure as u64),
                default_value: ConfigValue::UInt(defaults.fatigue_weights.experience_pressure as u64),
                hot_reloadable: true,
                readonly: false,
            },
            ConfigField {
                id: "pending_heavy_proposals".to_string(),
                label: "Pending Heavy Proposals".to_string(),
                description: Some("Weight for heavy proposals in DreamCycle queue".to_string()),
                field_type: ConfigFieldType::UInt { min: 0, max: 255 },
                current_value: ConfigValue::UInt(fw.pending_heavy_proposals as u64),
                default_value: ConfigValue::UInt(defaults.fatigue_weights.pending_heavy_proposals as u64),
                hot_reloadable: true,
                readonly: false,
            },
            ConfigField {
                id: "causal_horizon_growth_rate".to_string(),
                label: "Causal Horizon Growth".to_string(),
                description: Some("Weight for causal horizon growth rate".to_string()),
                field_type: ConfigFieldType::UInt { min: 0, max: 255 },
                current_value: ConfigValue::UInt(fw.causal_horizon_growth_rate as u64),
                default_value: ConfigValue::UInt(defaults.fatigue_weights.causal_horizon_growth_rate as u64),
                hot_reloadable: true,
                readonly: false,
            },
        ],
        subsections: vec![],
    };

    let cycle_section = ConfigSection {
        id: "engine.dream.cycle".to_string(),
        label: "Dream Cycle".to_string(),
        category: ConfigCategory::Engine,
        fields: vec![
            ConfigField {
                id: "max_dream_duration_ticks".to_string(),
                label: "Max Dream Duration".to_string(),
                description: Some("Maximum ticks a single dream cycle may last".to_string()),
                field_type: ConfigFieldType::UInt { min: 10, max: 100_000 },
                current_value: ConfigValue::UInt(cycle.max_dream_duration_ticks as u64),
                default_value: ConfigValue::UInt(defaults.cycle.max_dream_duration_ticks as u64),
                hot_reloadable: true,
                readonly: false,
            },
            ConfigField {
                id: "max_proposals_per_cycle".to_string(),
                label: "Max Proposals / Cycle".to_string(),
                description: Some("Max crystallization proposals per dream cycle".to_string()),
                field_type: ConfigFieldType::UInt { min: 1, max: 10_000 },
                current_value: ConfigValue::UInt(cycle.max_proposals_per_cycle as u64),
                default_value: ConfigValue::UInt(defaults.cycle.max_proposals_per_cycle as u64),
                hot_reloadable: true,
                readonly: false,
            },
            ConfigField {
                id: "batch_size".to_string(),
                label: "Batch Size".to_string(),
                description: Some("Proposals processed per dream tick batch".to_string()),
                field_type: ConfigFieldType::UInt { min: 1, max: 1_000 },
                current_value: ConfigValue::UInt(cycle.batch_size as u64),
                default_value: ConfigValue::UInt(defaults.cycle.batch_size as u64),
                hot_reloadable: true,
                readonly: false,
            },
        ],
        subsections: vec![],
    };

    let dream_section = ConfigSection {
        id: "engine.dream".to_string(),
        label: "Dream Phase".to_string(),
        category: ConfigCategory::Engine,
        fields: vec![],
        subsections: vec![scheduler_section, fatigue_section, cycle_section],
    };

    ConfigSchema { sections: vec![dream_section] }
}

fn apply_config_field(engine: &mut AxiomEngine, section_id: &str, field_id: &str, value: &ConfigValue) {
    let mut cfg = engine.current_dream_config();

    let changed = match (section_id, field_id) {
        ("engine.dream.scheduler", "min_wake_ticks") => {
            if let ConfigValue::UInt(v) = value { cfg.scheduler.min_wake_ticks = *v as u32; true } else { false }
        }
        ("engine.dream.scheduler", "idle_threshold") => {
            if let ConfigValue::UInt(v) = value { cfg.scheduler.idle_threshold = *v as u32; true } else { false }
        }
        ("engine.dream.scheduler", "fatigue_threshold") => {
            if let ConfigValue::UInt(v) = value { cfg.scheduler.fatigue_threshold = *v as u8; true } else { false }
        }
        ("engine.dream.fatigue", "uncrystallized_candidates") => {
            if let ConfigValue::UInt(v) = value { cfg.fatigue_weights.uncrystallized_candidates = *v as u8; true } else { false }
        }
        ("engine.dream.fatigue", "experience_pressure") => {
            if let ConfigValue::UInt(v) = value { cfg.fatigue_weights.experience_pressure = *v as u8; true } else { false }
        }
        ("engine.dream.fatigue", "pending_heavy_proposals") => {
            if let ConfigValue::UInt(v) = value { cfg.fatigue_weights.pending_heavy_proposals = *v as u8; true } else { false }
        }
        ("engine.dream.fatigue", "causal_horizon_growth_rate") => {
            if let ConfigValue::UInt(v) = value { cfg.fatigue_weights.causal_horizon_growth_rate = *v as u8; true } else { false }
        }
        ("engine.dream.cycle", "max_dream_duration_ticks") => {
            if let ConfigValue::UInt(v) = value { cfg.cycle.max_dream_duration_ticks = *v as u32; true } else { false }
        }
        ("engine.dream.cycle", "max_proposals_per_cycle") => {
            if let ConfigValue::UInt(v) = value { cfg.cycle.max_proposals_per_cycle = *v as u32; true } else { false }
        }
        ("engine.dream.cycle", "batch_size") => {
            if let ConfigValue::UInt(v) = value { cfg.cycle.batch_size = *v as u32; true } else { false }
        }
        _ => {
            debug!("UpdateConfigField: unrecognized {}/{}", section_id, field_id);
            false
        }
    };

    if changed {
        engine.apply_dream_config(&cfg);
    }
}
