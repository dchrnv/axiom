// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// tick_loop — главный цикл AXIOM.
//
// Единственный writer AxiomEngine. Все адаптеры взаимодействуют
// через command_rx (входящие команды) и broadcast_tx (исходящие события).

use std::collections::{HashSet, VecDeque};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use axiom_broadcasting::{build_system_snapshot, BroadcastHandle};
use axiom_config::{AnchorSet, ConfigWatcher};
use axiom_core::Event;
use axiom_persist::AutoSaver;
use axiom_protocol::{
    bench::{BenchEnvironment, BenchResults},
    commands::EngineCommand,
    events::EngineEvent,
    messages::{CommandResultData, EngineMessage},
};
use axiom_runtime::{AxiomEngine, BroadcastSnapshot, TickRateReason};
use axiom_ucl::{OpCode, UclCommand};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::time::Duration;

use crate::adapter_command::{AdapterCommand, AdapterPayload, CommandResponse};
use crate::adapters_config::AdaptersConfig;
use crate::channels::cli::{CliConfig, PerfTracker};
use crate::effectors::message::domain_name;
use crate::meta_commands::{handle_meta_mutate, handle_meta_read, MetaAction};
use crate::perceptors::text::TextPerceptor;
use crate::protocol::ServerMessage;

const EVENT_LOG_CAPACITY: usize = 256;

/// CLI-специфичное состояние tick_loop: производительность, лог событий, watch-поля.
pub(crate) struct CliState {
    perf: PerfTracker,
    event_log: VecDeque<Event>,
    watch_fields: HashSet<String>,
    multipass_count: u64,
    last_multipass_n: u8,
}

impl CliState {
    fn new() -> Self {
        Self {
            perf: PerfTracker::new(200),
            event_log: VecDeque::with_capacity(EVENT_LOG_CAPACITY),
            watch_fields: HashSet::new(),
            multipass_count: 0,
            last_multipass_n: 0,
        }
    }
}

/// Главный цикл — единственный writer AxiomEngine.
///
/// Принимает engine по значению (владеет им).
/// Завершается когда command_rx закрывается или получает :quit.
///
/// `config_watcher` — опциональный наблюдатель за config/axiom.yaml.
#[allow(clippy::too_many_arguments)]
pub async fn tick_loop(
    mut engine: AxiomEngine,
    mut command_rx: mpsc::Receiver<AdapterCommand>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
    snapshot: Arc<RwLock<BroadcastSnapshot>>,
    mut auto_saver: AutoSaver,
    anchor_set: Option<Arc<AnchorSet>>,
    config: AdaptersConfig,
    config_watcher: Option<ConfigWatcher>,
    wstation_handle: Option<Arc<BroadcastHandle>>,
) {
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let base_tick_ms = 1000u64 / config.tick_hz.max(1) as u64;
    let mut perceptor = make_perceptor(&anchor_set);
    let mut cli_state = CliState::new();
    let config_watcher = config_watcher;

    // Применяем TickSchedule из конфига
    engine.tick_schedule = config.tick_schedule.clone();

    loop {
        let sleep_ms = if config.adaptive_tick_rate {
            engine.tick_schedule.adaptive_tick.interval_ms()
        } else {
            base_tick_ms
        };
        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

        // Горячая перезагрузка axiom.yaml: применяем обновлённые domain-пресеты к running engine.
        if let Some(ref watcher) = config_watcher {
            if let Some(new_cfg) = watcher.poll() {
                let mut applied = 0usize;
                for (name, domain_cfg) in &new_cfg.domains {
                    let domain_id = domain_cfg.domain_id;
                    if domain_id == 0 {
                        continue; // пресет без привязанного domain_id — пропускаем
                    }
                    engine.apply_domain_config(domain_id, domain_cfg);
                    applied += 1;
                    eprintln!("[config] applied domain '{}' (id={}) hot-reload", name, domain_id);
                }
                if applied == 0 {
                    eprintln!("[config] axiom.yaml changed — no domain presets with domain_id found");
                }
            }
        }

        // 1. Drain входящих команд (non-blocking)
        let mut had_commands = false;
        loop {
            match command_rx.try_recv() {
                Ok(cmd) => {
                    had_commands = true;
                    match process_adapter_command(
                        cmd.payload,
                        cmd.id,
                        &mut engine,
                        &mut auto_saver,
                        &mut perceptor,
                        &anchor_set,
                        &config,
                        &mut cli_state,
                    ) {
                        CommandResponse::Message(msg) => {
                            let _ = broadcast_tx.send(msg);
                        }
                        CommandResponse::Quit => {
                            if auto_saver.config.enabled {
                                let _ = auto_saver.force_save(&engine, Path::new(&config.data_dir));
                            }
                            return;
                        }
                        CommandResponse::None => {}
                    }
                }
                Err(mpsc::error::TryRecvError::Empty) => break,
                Err(mpsc::error::TryRecvError::Disconnected) => return,
            }
        }

        // 1b. Drain Workstation commands
        if let Some(ref h) = wstation_handle {
            loop {
                match h.try_recv_command().await {
                    Some((cmd_id, cmd)) => {
                        handle_wstation_command(
                            cmd_id, cmd, &mut engine, h, &mut perceptor,
                        );
                    }
                    None => break,
                }
            }
        }

        // 2. Tick ядра
        let tick_start = Instant::now();
        engine.process_command(&tick_cmd);
        let tick_ns = tick_start.elapsed().as_nanos() as u64;
        let t = engine.tick_count;
        cli_state.perf.record(tick_ns, t);

        for ev in engine.drain_events() {
            if cli_state.event_log.len() >= EVENT_LOG_CAPACITY {
                cli_state.event_log.pop_front();
            }
            cli_state.event_log.push_back(ev);
        }

        if config.adaptive_tick_rate {
            if had_commands || engine.tension_count() > 0 {
                let reason = if had_commands {
                    TickRateReason::ExternalInput
                } else {
                    TickRateReason::TensionHigh
                };
                engine.tick_schedule.adaptive_tick.trigger(reason);
            } else {
                engine.tick_schedule.adaptive_tick.on_idle_tick();
            }
        }

        // 3. Tick-broadcast
        let bcast_interval = config.websocket.tick_broadcast_interval as u64;
        if bcast_interval > 0 && t.is_multiple_of(bcast_interval) {
            let _ = broadcast_tx.send(ServerMessage::Tick {
                tick_count: t,
                traces: engine.trace_count() as u32,
                tension: engine.tension_count() as u32,
                last_matched: engine.last_matched(),
            });
        }
        if let Some(ref h) = wstation_handle {
            h.publish(EngineMessage::Event(EngineEvent::Tick {
                tick: t,
                event: engine.com_next_id,
                hot_path_ns: tick_ns,
            }));
        }

        // 4. State-snapshot broadcast
        let state_interval = config.websocket.state_broadcast_interval as u64;
        if state_interval > 0 && t.is_multiple_of(state_interval) {
            let snap = engine.snapshot_for_broadcast();
            let for_bcast = snap.clone();
            *snapshot.write().await = snap;
            let _ = broadcast_tx.send(ServerMessage::State {
                tick_count: t,
                snapshot: for_bcast,
            });
            if let Some(ref h) = wstation_handle {
                h.update_snapshot(build_system_snapshot(&engine, tick_ns));
            }
        }

        // 5. Автосохранение
        auto_saver.tick(&engine, Path::new(&config.data_dir));
    }
}

/// Обработать одну команду адаптера. Возвращает ответ для broadcast.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_adapter_command(
    payload: AdapterPayload,
    id: String,
    engine: &mut AxiomEngine,
    auto_saver: &mut AutoSaver,
    perceptor: &mut TextPerceptor,
    anchor_set: &Option<Arc<AnchorSet>>,
    config: &AdaptersConfig,
    cli_state: &mut CliState,
) -> CommandResponse {
    match payload {
        AdapterPayload::Inject { text } => {
            let ucl = perceptor.perceive(&text);
            let r = engine.process_and_observe(&ucl);

            use axiom_runtime::ProcessingPath;
            if matches!(r.path, ProcessingPath::MultiPass(_)) {
                cli_state.multipass_count += 1;
                if let ProcessingPath::MultiPass(n) = r.path {
                    cli_state.last_multipass_n = n;
                }
            }

            CommandResponse::Message(ServerMessage::Result {
                command_id: id,
                path: format!("{:?}", r.path),
                domain_id: r.dominant_domain_id,
                domain_name: domain_name(r.dominant_domain_id).to_string(),
                coherence: r.coherence_score.unwrap_or(0.0),
                reflex_hit: r.reflex_hit,
                traces_matched: r.traces_matched,
                position: r.output_position,
                shell: r.output_shell,
                event_id: r.event_id,
            })
        }

        AdapterPayload::MetaRead { cmd } => {
            let cli_cfg = make_cli_config(config);
            let output = handle_meta_read(
                &cmd,
                engine,
                anchor_set.as_deref(),
                &cli_cfg,
                &cli_state.watch_fields,
                &cli_state.event_log,
                &cli_state.perf,
                cli_state.multipass_count,
                cli_state.last_multipass_n,
            );

            // Мутации CLI-состояния в ответ на :watch / :unwatch
            let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
            match parts[0] {
                ":watch" => {
                    let arg = parts.get(1).copied().unwrap_or("");
                    match arg {
                        "off" => cli_state.watch_fields.clear(),
                        field if !field.is_empty() => {
                            cli_state.watch_fields.insert(field.to_string());
                        }
                        _ => {}
                    }
                }
                ":unwatch" => {
                    if let Some(f) = parts.get(1) {
                        cli_state.watch_fields.remove(*f);
                    }
                }
                _ => {}
            }

            CommandResponse::Message(ServerMessage::CommandResult {
                command_id: id,
                output,
            })
        }

        AdapterPayload::MetaMutate { cmd } => {
            let cli_cfg = make_cli_config(config);
            let result = handle_meta_mutate(&cmd, engine, auto_saver, &cli_cfg);
            match result.action {
                MetaAction::Quit => CommandResponse::Quit,
                MetaAction::EngineReplaced => {
                    *perceptor = make_perceptor(anchor_set);
                    CommandResponse::Message(ServerMessage::CommandResult {
                        command_id: id,
                        output: result.output,
                    })
                }
                _ => CommandResponse::Message(ServerMessage::CommandResult {
                    command_id: id,
                    output: result.output,
                }),
            }
        }

        AdapterPayload::DomainSnapshot { domain_id } => {
            match engine.domain_detail_snapshot(domain_id) {
                Some(snap) => CommandResponse::Message(ServerMessage::DomainDetail(snap)),
                None => CommandResponse::Message(ServerMessage::Error {
                    command_id: Some(id),
                    message: format!("domain {} not found", domain_id),
                }),
            }
        }

        AdapterPayload::Subscribe { .. } | AdapterPayload::Unsubscribe { .. } => {
            CommandResponse::None // обрабатывается per-connection в WebSocket handler
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn handle_wstation_command(
    cmd_id: u64,
    cmd: EngineCommand,
    engine: &mut AxiomEngine,
    handle: &BroadcastHandle,
    perceptor: &mut TextPerceptor,
) {
    match cmd {
        EngineCommand::SubmitText { text, .. } => {
            let ucl = perceptor.perceive(&text);
            engine.process_and_observe(&ucl);
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::None),
            });
        }
        EngineCommand::RequestFullSnapshot => {
            handle.update_snapshot(build_system_snapshot(engine, 0));
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::None),
            });
        }
        EngineCommand::RunBench { spec } => {
            let run_id = cmd_id;
            handle.publish(EngineMessage::Event(EngineEvent::BenchStarted {
                bench_id: spec.bench_id.clone(),
                run_id,
            }));

            let bench_tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
            let n = spec.iterations.max(1) as usize;
            let mut timings_ns: Vec<u64> = Vec::with_capacity(n);

            for i in 0..n {
                let t0 = Instant::now();
                engine.process_command(&bench_tick_cmd);
                timings_ns.push(t0.elapsed().as_nanos() as u64);

                if i % 50 == 49 {
                    handle.publish(EngineMessage::Event(EngineEvent::BenchProgress {
                        run_id,
                        completed: (i + 1) as u32,
                        total: n as u32,
                    }));
                }
            }

            timings_ns.sort_unstable();
            let len = timings_ns.len();
            let median = timings_ns[len / 2] as f64;
            let p50 = median;
            let p99 = timings_ns[(len * 99 / 100).min(len - 1)] as f64;
            let mean = timings_ns.iter().sum::<u64>() as f64 / len as f64;
            let variance = timings_ns.iter().map(|&t| {
                let d = t as f64 - mean;
                d * d
            }).sum::<f64>() / len as f64;
            let std_dev = variance.sqrt();

            let results = BenchResults {
                bench_id: spec.bench_id,
                iterations: n as u32,
                median_ns: median,
                p50_ns: p50,
                p99_ns: p99,
                std_dev_ns: std_dev,
                environment: BenchEnvironment {
                    os: std::env::consts::OS.to_string(),
                    arch: std::env::consts::ARCH.to_string(),
                    engine_version: 0,
                },
            };

            handle.publish(EngineMessage::Event(EngineEvent::BenchFinished {
                run_id,
                results,
            }));
            handle.publish(EngineMessage::CommandResult {
                command_id: cmd_id,
                result: Ok(CommandResultData::None),
            });
        }
        // Remaining commands: ForceSleep, ForceWake, config, adapters
        _ => {}
    }
}

fn make_perceptor(anchor_set: &Option<Arc<AnchorSet>>) -> TextPerceptor {
    match anchor_set {
        Some(a) => TextPerceptor::with_anchors(Arc::clone(a)),
        None => TextPerceptor::new(),
    }
}

/// Собрать минимальный CliConfig из AdaptersConfig для вызовов handle_meta_*.
fn make_cli_config(config: &AdaptersConfig) -> CliConfig {
    CliConfig {
        tick_hz: config.tick_hz,
        data_dir: config.data_dir.clone(),
        verbose: config.verbose,
        detail_level: config.detail_level,
        adaptive_tick_rate: config.adaptive_tick_rate,
        ..CliConfig::default()
    }
}
