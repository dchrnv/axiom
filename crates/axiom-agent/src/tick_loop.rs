// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// tick_loop — главный цикл AXIOM.
//
// Единственный writer AxiomEngine. Все адаптеры взаимодействуют
// через command_rx (входящие команды) и broadcast_tx (исходящие события).
//
// Технический долг Phase 0C → DEFERRED (EA-TD-03..06):
//   EA-TD-03  PerfTracker, watch_fields, event_log, verbose — CLI-фичи,
//             удалены из tick_loop в Phase 0C. Вернуть в Phase 1 или
//             выделить в CliTickExtension.
//   EA-TD-04  Adaptive tick rate не перенесён в tick_loop.
//   EA-TD-05  hot_reload (ConfigWatcher) не перенесён в tick_loop.
//   EA-TD-06  Inject output — упрощённый формат. Полная MessageEffector
//             интеграция — в Phase 1 через CliAdapter.

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::path::Path;

use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::time::Duration;
use axiom_runtime::{AxiomEngine, BroadcastSnapshot};
use axiom_ucl::{UclCommand, OpCode};
use axiom_config::AnchorSet;
use axiom_persist::AutoSaver;

use crate::adapter_command::{AdapterCommand, AdapterPayload, CommandResponse};
use crate::adapters_config::AdaptersConfig;
use crate::channels::cli::PerfTracker;
use crate::effectors::message::domain_name;
use crate::meta_commands::{handle_meta_read, handle_meta_mutate, MetaAction};
use crate::perceptors::text::TextPerceptor;
use crate::protocol::ServerMessage;

/// Главный цикл — единственный writer AxiomEngine.
///
/// Принимает engine по значению (владеет им).
/// Завершается когда command_rx закрывается или получает :quit.
pub async fn tick_loop(
    mut engine:     AxiomEngine,
    mut command_rx: mpsc::Receiver<AdapterCommand>,
    broadcast_tx:   broadcast::Sender<ServerMessage>,
    snapshot:       Arc<RwLock<BroadcastSnapshot>>,
    mut auto_saver: AutoSaver,
    anchor_set:     Option<Arc<AnchorSet>>,
    config:         AdaptersConfig,
) {
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let tick_ms  = 1000u64 / config.tick_hz.max(1) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(tick_ms));
    let mut perceptor = make_perceptor(&anchor_set);

    // Применяем TickSchedule из конфига
    engine.tick_schedule = config.tick_schedule;

    loop {
        interval.tick().await;

        // 1. Drain входящих команд (non-blocking)
        loop {
            match command_rx.try_recv() {
                Ok(cmd) => {
                    match process_adapter_command(
                        cmd.payload, cmd.id,
                        &mut engine, &mut auto_saver,
                        &mut perceptor, &anchor_set, &config,
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

        // 2. Tick ядра
        engine.process_command(&tick_cmd);
        let t = engine.tick_count;

        // 3. Tick-broadcast
        let interval_ticks = config.websocket.tick_broadcast_interval as u64;
        if interval_ticks > 0 && t % interval_ticks == 0 {
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
        let data_dir = Path::new(&config.data_dir);
        auto_saver.tick(&engine, data_dir);
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
    _config:    &AdaptersConfig,  // EA-TD-03: будет использован в Phase 1 для CliConfig
) -> CommandResponse {
    match payload {

        AdapterPayload::Inject { text } => {
            let ucl = perceptor.perceive(&text);
            let r   = engine.process_and_observe(&ucl);
            // Собираем ServerMessage::Result с полными структурированными данными
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
            // PerfTracker и event_log не доступны в tick_loop — передаём пустые заглушки.
            // Команды зависящие от них (:perf, :events) вернут нулевые значения.
            // EA-TD-03: перенести PerfTracker в tick_loop в Phase 1.
            let output = handle_meta_read(
                &cmd, engine, anchor_set.as_deref(),
                &crate::channels::cli::CliConfig::default(),
                &HashSet::new(), &VecDeque::new(),
                &PerfTracker::new(0), 0, 0,
            );
            CommandResponse::Message(ServerMessage::CommandResult { command_id: id, output })
        }

        AdapterPayload::MetaMutate { cmd } => {
            let result = handle_meta_mutate(
                &cmd, engine, auto_saver,
                &crate::channels::cli::CliConfig::default(),
            );
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
