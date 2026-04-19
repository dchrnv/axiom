// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// tick_loop — главный цикл AXIOM.
//
// Единственный writer AxiomEngine. Все адаптеры взаимодействуют
// через command_rx (входящие команды) и broadcast_tx (исходящие события).
//
// EA-TD-03 ЗАКРЫТ: PerfTracker, event_log, watch_fields, multipass-счётчики
//   живут в CliState внутри tick_loop. :perf / :events / :watch работают корректно.
// EA-TD-04 ЗАКРЫТ: Адаптивная частота тиков — tokio::time::sleep с интервалом
//   из engine.tick_schedule.adaptive_tick.interval_ms().
// EA-TD-05 ЗАКРЫТ: ConfigWatcher передаётся в tick_loop и поллится каждый тик.
// EA-TD-06 ЗАКРЫТ: CLI-вывод для Inject использует DetailLevel через format_result.

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::path::Path;
use std::time::Instant;

use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::time::Duration;
use axiom_core::Event;
use axiom_runtime::{AxiomEngine, BroadcastSnapshot, TickRateReason};
use axiom_ucl::{UclCommand, OpCode};
use axiom_config::{AnchorSet, ConfigWatcher};
use axiom_persist::AutoSaver;

use crate::adapter_command::{AdapterCommand, AdapterPayload, CommandResponse};
use crate::adapters_config::AdaptersConfig;
use crate::channels::cli::{CliConfig, PerfTracker, fmt_ns};
use crate::effectors::message::domain_name;
use crate::meta_commands::{handle_meta_read, handle_meta_mutate, MetaAction};
use crate::perceptors::text::TextPerceptor;
use crate::protocol::ServerMessage;

const EVENT_LOG_CAPACITY: usize = 256;

/// CLI-специфичное состояние tick_loop — заполняет EA-TD-03.
struct CliState {
    perf:             PerfTracker,
    event_log:        VecDeque<Event>,
    watch_fields:     HashSet<String>,
    multipass_count:  u64,
    last_multipass_n: u8,
}

impl CliState {
    fn new() -> Self {
        Self {
            perf:             PerfTracker::new(200),
            event_log:        VecDeque::with_capacity(EVENT_LOG_CAPACITY),
            watch_fields:     HashSet::new(),
            multipass_count:  0,
            last_multipass_n: 0,
        }
    }
}

/// Главный цикл — единственный writer AxiomEngine.
///
/// Принимает engine по значению (владеет им).
/// Завершается когда command_rx закрывается или получает :quit.
///
/// `config_watcher` — опциональный наблюдатель за config/axiom.yaml (EA-TD-05).
pub async fn tick_loop(
    mut engine:          AxiomEngine,
    mut command_rx:      mpsc::Receiver<AdapterCommand>,
    broadcast_tx:        broadcast::Sender<ServerMessage>,
    snapshot:            Arc<RwLock<BroadcastSnapshot>>,
    mut auto_saver:      AutoSaver,
    anchor_set:          Option<Arc<AnchorSet>>,
    config:              AdaptersConfig,
    config_watcher:      Option<ConfigWatcher>,
) {
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let base_tick_ms = 1000u64 / config.tick_hz.max(1) as u64;
    let mut perceptor = make_perceptor(&anchor_set);
    let mut cli_state = CliState::new();
    let mut config_watcher = config_watcher;

    // Применяем TickSchedule из конфига
    engine.tick_schedule = config.tick_schedule;

    loop {
        // EA-TD-04: адаптивный интервал или фиксированный
        let sleep_ms = if config.adaptive_tick_rate {
            engine.tick_schedule.adaptive_tick.interval_ms()
        } else {
            base_tick_ms
        };
        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

        // EA-TD-05: горячая перезагрузка конфигурации (axiom.yaml)
        // Обнаруживаем изменение и логируем; применение domain-пресетов к running engine
        // требует engine.apply_domain_config() — будет добавлено отдельно.
        if let Some(ref watcher) = config_watcher {
            if let Some(_new_cfg) = watcher.poll() {
                eprintln!("[config] hot-reload: axiom.yaml changed (domain config reload not yet applied to running engine)");
            }
        }

        // 1. Drain входящих команд (non-blocking)
        let mut had_commands = false;
        loop {
            match command_rx.try_recv() {
                Ok(cmd) => {
                    had_commands = true;
                    match process_adapter_command(
                        cmd.payload, cmd.id,
                        &mut engine, &mut auto_saver,
                        &mut perceptor, &anchor_set,
                        &config, &mut cli_state,
                    ) {
                        CommandResponse::Message(msg) => { let _ = broadcast_tx.send(msg); }
                        CommandResponse::Quit => {
                            if auto_saver.config.enabled {
                                let _ = auto_saver.force_save(
                                    &engine, Path::new(&config.data_dir)
                                );
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

        // 2. Tick ядра (с замером времени для EA-TD-03 PerfTracker)
        let tick_start = Instant::now();
        engine.process_command(&tick_cmd);
        let tick_ns = tick_start.elapsed().as_nanos() as u64;
        let t = engine.tick_count;
        cli_state.perf.record(tick_ns, t);

        // Drain COM-событий в event_log (EA-TD-03)
        for ev in engine.drain_events() {
            if cli_state.event_log.len() >= EVENT_LOG_CAPACITY {
                cli_state.event_log.pop_front();
            }
            cli_state.event_log.push_back(ev);
        }

        // EA-TD-04: адаптивная логика idle-тика
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
        if bcast_interval > 0 && t % bcast_interval == 0 {
            let _ = broadcast_tx.send(ServerMessage::Tick {
                tick_count:   t,
                traces:       engine.trace_count()  as u32,
                tension:      engine.tension_count() as u32,
                last_matched: engine.last_matched(),
            });
        }

        // 4. State-snapshot broadcast
        let state_interval = config.websocket.state_broadcast_interval as u64;
        if state_interval > 0 && t % state_interval == 0 {
            let snap = engine.snapshot_for_broadcast();
            let for_bcast = snap.clone();
            *snapshot.write().await = snap;
            let _ = broadcast_tx.send(ServerMessage::State {
                tick_count: t,
                snapshot:   for_bcast,
            });
        }

        // 5. Автосохранение
        auto_saver.tick(&engine, Path::new(&config.data_dir));
    }
}

/// Обработать одну команду адаптера. Возвращает ответ для broadcast.
pub(crate) fn process_adapter_command(
    payload:    AdapterPayload,
    id:         String,
    engine:     &mut AxiomEngine,
    auto_saver: &mut AutoSaver,
    perceptor:  &mut TextPerceptor,
    anchor_set: &Option<Arc<AnchorSet>>,
    config:     &AdaptersConfig,
    cli_state:  &mut CliState,
) -> CommandResponse {
    match payload {

        AdapterPayload::Inject { text } => {
            let ucl = perceptor.perceive(&text);
            let r   = engine.process_and_observe(&ucl);

            // EA-TD-03: обновить multipass-счётчики
            use axiom_runtime::ProcessingPath;
            if matches!(r.path, ProcessingPath::MultiPass(_)) {
                cli_state.multipass_count += 1;
                if let ProcessingPath::MultiPass(n) = r.path {
                    cli_state.last_multipass_n = n;
                }
            }

            CommandResponse::Message(ServerMessage::Result {
                command_id:     id,
                path:           format!("{:?}", r.path),
                domain_id:      r.dominant_domain_id,
                domain_name:    domain_name(r.dominant_domain_id).to_string(),
                coherence:      r.coherence_score.unwrap_or(0.0),
                reflex_hit:     r.reflex_hit,
                traces_matched: r.traces_matched,
                position:       r.output_position,
                shell:          r.output_shell,
                event_id:       r.event_id,
            })
        }

        AdapterPayload::MetaRead { cmd } => {
            let cli_cfg = make_cli_config(config);
            let output = handle_meta_read(
                &cmd, engine, anchor_set.as_deref(),
                &cli_cfg,
                &cli_state.watch_fields, &cli_state.event_log,
                &cli_state.perf,
                cli_state.multipass_count, cli_state.last_multipass_n,
            );

            // Мутации CLI-состояния в ответ на :watch / :unwatch
            let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
            match parts[0] {
                ":watch" => {
                    let arg = parts.get(1).copied().unwrap_or("");
                    match arg {
                        "off"                          => cli_state.watch_fields.clear(),
                        field if !field.is_empty()     => {
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

            CommandResponse::Message(ServerMessage::CommandResult { command_id: id, output })
        }

        AdapterPayload::MetaMutate { cmd } => {
            let cli_cfg = make_cli_config(config);
            let result = handle_meta_mutate(&cmd, engine, auto_saver, &cli_cfg);
            match result.action {
                MetaAction::Quit => CommandResponse::Quit,
                MetaAction::EngineReplaced => {
                    *perceptor = make_perceptor(anchor_set);
                    CommandResponse::Message(ServerMessage::CommandResult {
                        command_id: id, output: result.output,
                    })
                }
                _ => CommandResponse::Message(ServerMessage::CommandResult {
                    command_id: id, output: result.output,
                }),
            }
        }

        AdapterPayload::DomainSnapshot { domain_id } => {
            match engine.domain_detail_snapshot(domain_id) {
                Some(snap) => CommandResponse::Message(ServerMessage::DomainDetail(snap)),
                None => CommandResponse::Message(ServerMessage::Error {
                    command_id: Some(id),
                    message:    format!("domain {} not found", domain_id),
                }),
            }
        }

        AdapterPayload::Subscribe { .. } | AdapterPayload::Unsubscribe { .. } => {
            CommandResponse::None // обрабатывается per-connection в WebSocket handler
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_perceptor(anchor_set: &Option<Arc<AnchorSet>>) -> TextPerceptor {
    match anchor_set {
        Some(a) => TextPerceptor::with_anchors(Arc::clone(a)),
        None    => TextPerceptor::new(),
    }
}

/// Собрать минимальный CliConfig из AdaptersConfig для вызовов handle_meta_*.
fn make_cli_config(config: &AdaptersConfig) -> CliConfig {
    CliConfig {
        tick_hz:            config.tick_hz,
        data_dir:           config.data_dir.clone(),
        verbose:            config.verbose,
        detail_level:       config.detail_level,
        adaptive_tick_rate: config.adaptive_tick_rate,
        ..CliConfig::default()
    }
}
