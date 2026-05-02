use crate::adapters::{AdapterInfo, AdapterOption, AdapterProgress, AdapterStatus};
use crate::bench::{BenchEnvironment, BenchOptions, BenchResults, BenchSpec};
use crate::commands::{EngineCommand, ImportOptions};
use crate::config::{
    ConfigCategory, ConfigField, ConfigFieldType, ConfigSchema, ConfigSection, ConfigValue,
};
use crate::events::{AlertLevel, EngineEvent, EngineState, SleepTrigger};
use crate::messages::{ClientKind, ClientMessage, CommandResultData, EngineMessage, ShutdownReason};
use crate::snapshot::{
    DomainConfigSummary, DomainSnapshot, DreamPhaseStats, DreamReport, FatigueSnapshot,
    FrameDetails, FrameWeaverStats, GuardianStats, OverDomainSnapshot, SystemSnapshot,
};

fn round_trip<T: serde::Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug + PartialEq>(
    value: &T,
) {
    let bytes = postcard::to_stdvec(value).expect("serialize");
    let decoded: T = postcard::from_bytes(&bytes).expect("deserialize");
    assert_eq!(value, &decoded);
}

fn make_snapshot() -> SystemSnapshot {
    SystemSnapshot {
        engine_state: EngineState::Wake,
        current_tick: 42,
        current_event: 7,
        domains: vec![DomainSnapshot {
            id: 100,
            name: "LOGIC".into(),
            config_summary: DomainConfigSummary { capacity: 1000, temperature_decay: 5 },
            token_count: 10,
            connection_count: 3,
            temperature_avg: 128,
            recent_activity: 2,
            layer_activations: [1, 0, 0, 0, 0, 0, 0, 0],
        }],
        over_domain: OverDomainSnapshot {
            total_tokens: 10,
            total_connections: 3,
            cross_domain_events_recent: 0,
            layer_activations: [1, 0, 0, 0, 0, 0, 0, 0],
        },
        fatigue: FatigueSnapshot {
            current: 0.3,
            threshold: 0.8,
            ticks_since_dream: 1000,
            token_rate: 0.1,
            history: vec![0.1, 0.2, 0.3],
        },
        last_dream_report: None,
        frame_weaver_stats: Some(FrameWeaverStats {
            total_frames: 5,
            frames_in_sutra: 1,
            promotions_since_wake: 1,
            last_crystallization_tick: 40,
        }),
        guardian_stats: GuardianStats {
            total_vetoes: 0,
            vetoes_since_wake: 0,
            last_veto_reason: None,
        },
        dream_phase_stats: DreamPhaseStats {
            cycles_completed: 2,
            last_transition_tick: 30,
        },
        adapter_progress: vec![],
    }
}

// EngineMessage variants

#[test]
fn engine_message_hello() {
    round_trip(&EngineMessage::Hello { version: 1, capabilities: 0 });
}

#[test]
fn engine_message_snapshot() {
    round_trip(&EngineMessage::Snapshot(make_snapshot()));
}

#[test]
fn engine_message_event_tick() {
    round_trip(&EngineMessage::Event(EngineEvent::Tick {
        tick: 100,
        event: 50,
        hot_path_ns: 238,
    }));
}

#[test]
fn engine_message_command_result_ok() {
    round_trip(&EngineMessage::CommandResult {
        command_id: 1,
        result: Ok(CommandResultData::None),
    });
}

#[test]
fn engine_message_command_result_err() {
    round_trip(&EngineMessage::CommandResult {
        command_id: 2,
        result: Err("not found".into()),
    });
}

#[test]
fn engine_message_bye() {
    round_trip(&EngineMessage::Bye { reason: ShutdownReason::Normal });
}

// ClientMessage variants

#[test]
fn client_message_hello() {
    round_trip(&ClientMessage::Hello { version: 1, client_kind: ClientKind::Workstation });
}

#[test]
fn client_message_request_snapshot() {
    round_trip(&ClientMessage::RequestSnapshot);
}

#[test]
fn client_message_subscribe() {
    round_trip(&ClientMessage::Subscribe { event_categories: crate::event_category::DEFAULT });
}

#[test]
fn client_message_command() {
    round_trip(&ClientMessage::Command {
        command_id: 1,
        command: EngineCommand::ForceSleep,
    });
}

#[test]
fn client_message_bye() {
    round_trip(&ClientMessage::Bye);
}

// EngineEvent variants

#[test]
fn event_domain_activity() {
    round_trip(&EngineEvent::DomainActivity {
        domain_id: 100,
        recent_activity: 5,
        layer_activations: [1, 0, 2, 0, 0, 0, 0, 0],
    });
}

#[test]
fn event_dream_phase_transition() {
    round_trip(&EngineEvent::DreamPhaseTransition {
        from: EngineState::Wake,
        to: EngineState::FallingAsleep,
        trigger: SleepTrigger::FatigueThreshold,
    });
}

#[test]
fn event_frame_crystallized() {
    round_trip(&EngineEvent::FrameCrystallized {
        anchor_id: 42,
        layers_present: 3,
        participant_count: 5,
    });
}

#[test]
fn event_frame_reactivated() {
    round_trip(&EngineEvent::FrameReactivated { anchor_id: 42, new_temperature: 200 });
}

#[test]
fn event_frame_promoted() {
    round_trip(&EngineEvent::FramePromoted { source_anchor_id: 42, sutra_anchor_id: 99 });
}

#[test]
fn event_guardian_veto() {
    round_trip(&EngineEvent::GuardianVeto {
        reason: "CODEX violation".into(),
        command_summary: "InjectToken".into(),
    });
}

#[test]
fn event_adapter_started() {
    round_trip(&EngineEvent::AdapterStarted {
        adapter_id: "pdf".into(),
        source: "/tmp/doc.pdf".into(),
    });
}

#[test]
fn event_adapter_progress() {
    round_trip(&EngineEvent::AdapterProgress {
        adapter_id: "pdf".into(),
        processed: 50,
        total: 100,
    });
}

#[test]
fn event_adapter_finished() {
    round_trip(&EngineEvent::AdapterFinished {
        adapter_id: "pdf".into(),
        tokens_added: 300,
        errors: 2,
    });
}

#[test]
fn event_bench_started() {
    round_trip(&EngineEvent::BenchStarted { bench_id: "hot_path_tick".into(), run_id: 1 });
}

#[test]
fn event_bench_progress() {
    round_trip(&EngineEvent::BenchProgress { run_id: 1, completed: 500, total: 10000 });
}

#[test]
fn event_bench_finished() {
    round_trip(&EngineEvent::BenchFinished {
        run_id: 1,
        results: BenchResults {
            bench_id: "hot_path_tick".into(),
            iterations: 10000,
            median_ns: 238.5,
            p50_ns: 235.0,
            p99_ns: 290.0,
            std_dev_ns: 12.3,
            environment: BenchEnvironment {
                os: "linux".into(),
                arch: "x86_64".into(),
                engine_version: 1,
            },
        },
    });
}

#[test]
fn event_alert() {
    round_trip(&EngineEvent::Alert {
        level: AlertLevel::Warning,
        category: "memory".into(),
        message: "domain capacity at 90%".into(),
    });
}

// EngineCommand variants (C2)

#[test]
fn command_get_config_schema() {
    round_trip(&EngineCommand::GetConfigSchema);
}

#[test]
fn command_get_config_section() {
    round_trip(&EngineCommand::GetConfigSection { id: "engine.dream_phase".into() });
}

#[test]
fn command_update_config_field() {
    round_trip(&EngineCommand::UpdateConfigField {
        section_id: "engine.dream_phase".into(),
        field_id: "fatigue_threshold".into(),
        value: ConfigValue::Float(0.8),
    });
}

#[test]
fn command_start_import() {
    round_trip(&EngineCommand::StartImport {
        adapter_id: "pdf".into(),
        source_path: "/tmp/doc.pdf".into(),
        options: ImportOptions { params: vec![], target_domain: None },
    });
}

#[test]
fn command_submit_text() {
    round_trip(&EngineCommand::SubmitText {
        text: "hello world".into(),
        target_domain: 100,
    });
}

// ConfigSchema (C2)

#[test]
fn config_schema_round_trip() {
    let schema = ConfigSchema {
        sections: vec![ConfigSection {
            id: "engine.dream_phase".into(),
            label: "Dream Phase".into(),
            category: ConfigCategory::Engine,
            fields: vec![
                ConfigField {
                    id: "fatigue_threshold".into(),
                    label: "Fatigue threshold".into(),
                    description: Some("Trigger dream when fatigue exceeds this value".into()),
                    field_type: ConfigFieldType::Float { min: 0.0, max: 1.0, step: Some(0.01) },
                    current_value: ConfigValue::Float(0.8),
                    default_value: ConfigValue::Float(0.8),
                    hot_reloadable: true,
                    readonly: false,
                },
                ConfigField {
                    id: "tick_interval".into(),
                    label: "Tick interval".into(),
                    description: None,
                    field_type: ConfigFieldType::UInt { min: 1, max: 1000 },
                    current_value: ConfigValue::UInt(100),
                    default_value: ConfigValue::UInt(100),
                    hot_reloadable: true,
                    readonly: false,
                },
            ],
            subsections: vec![],
        }],
    };
    round_trip(&schema);
}

#[test]
fn config_value_variants() {
    round_trip(&ConfigValue::Bool(true));
    round_trip(&ConfigValue::Integer(-42));
    round_trip(&ConfigValue::UInt(1000));
    round_trip(&ConfigValue::Float(3.14));
    round_trip(&ConfigValue::EnumVariant("FifoDropOldest".into()));
    round_trip(&ConfigValue::Duration(10000));
    round_trip(&ConfigValue::Domain(100));
}

// Snapshot

#[test]
fn snapshot_full_round_trip() {
    round_trip(&make_snapshot());
}

#[test]
fn snapshot_with_dream_report() {
    let mut snap = make_snapshot();
    snap.last_dream_report = Some(DreamReport {
        cycle_id: 1,
        started_at_tick: 100,
        ended_at_tick: 200,
        proposals_accepted: 3,
        proposals_rejected: 1,
        sutra_written: 1,
        fatigue_before: 0.85,
        fatigue_after: 0.2,
    });
    round_trip(&snap);
}

#[test]
fn frame_details_round_trip() {
    round_trip(&FrameDetails {
        anchor_id: 42,
        layers_present: 3,
        participant_count: 5,
        temperature: 180,
        crystallized_at_tick: 1000,
        last_reactivated_at_tick: Some(1200),
        promotion_rule: Some("semantic_density > 0.7".into()),
    });
}

// AdapterInfo

#[test]
fn adapter_info_round_trip() {
    round_trip(&AdapterInfo {
        id: "pdf".into(),
        name: "PDF Adapter".into(),
        description: "Imports PDF documents".into(),
        supported_extensions: vec!["pdf".into()],
        options_schema: vec![AdapterOption {
            key: "page_limit".into(),
            label: "Page limit".into(),
            description: None,
            required: false,
            default: Some("100".into()),
        }],
    });
}

// BenchResults

#[test]
fn bench_results_round_trip() {
    round_trip(&BenchResults {
        bench_id: "frameweaver_overhead".into(),
        iterations: 5000,
        median_ns: 145.3,
        p50_ns: 142.0,
        p99_ns: 210.0,
        std_dev_ns: 8.5,
        environment: BenchEnvironment {
            os: "linux".into(),
            arch: "x86_64".into(),
            engine_version: 1,
        },
    });
}

#[test]
fn bench_spec_round_trip() {
    round_trip(&BenchSpec {
        bench_id: "hot_path_tick".into(),
        iterations: 10000,
        options: BenchOptions { params: vec![("warmup".into(), "true".into())] },
    });
}

// AdapterProgress

#[test]
fn adapter_progress_round_trip() {
    round_trip(&AdapterProgress {
        adapter_id: "pdf".into(),
        source: "/tmp/doc.pdf".into(),
        processed: 75,
        total: 100,
        status: AdapterStatus::Running,
    });
}

// EngineState all variants

#[test]
fn engine_state_all_variants() {
    round_trip(&EngineState::Wake);
    round_trip(&EngineState::FallingAsleep);
    round_trip(&EngineState::Dreaming);
    round_trip(&EngineState::Waking);
}

// CommandResultData C2 variants

#[test]
fn command_result_config_update_applied() {
    round_trip(&CommandResultData::ConfigUpdateApplied { hot_reloaded: true });
}

#[test]
fn command_result_config_validation_error() {
    round_trip(&CommandResultData::ConfigValidationError {
        field_id: "fatigue_threshold".into(),
        message: "value must be between 0.0 and 1.0".into(),
    });
}
