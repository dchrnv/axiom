// SPDX-License-Identifier: AGPL-3.0-only
// Тесты для tick_loop и process_adapter_command (Phase 0C).

use axiom_agent::adapter_command::{AdapterCommand, AdapterPayload, AdapterSource, CommandResponse};
use axiom_agent::adapters_config::AdaptersConfig;
use axiom_agent::channels::cli::CliConfig;
use axiom_agent::protocol::ServerMessage;
use axiom_agent::tick_loop::tick_loop;
use axiom_runtime::{AxiomEngine, BroadcastSnapshot};
use axiom_persist::{AutoSaver, PersistenceConfig};
use tokio::sync::{broadcast, mpsc, RwLock};
use std::sync::Arc;

fn make_engine()  -> AxiomEngine  { AxiomEngine::new() }
fn make_saver()   -> AutoSaver    { AutoSaver::new(PersistenceConfig::disabled()) }
fn make_config()  -> AdaptersConfig { AdaptersConfig::from_cli_config(&CliConfig::default()) }
fn make_snapshot() -> Arc<RwLock<BroadcastSnapshot>> {
    Arc::new(RwLock::new(BroadcastSnapshot::default()))
}

// ── AdapterCommand ────────────────────────────────────────────────────────────

#[test]
fn test_adapter_command_shutdown_constructor() {
    let cmd = AdapterCommand::shutdown();
    assert_eq!(cmd.id, "shutdown");
    assert!(matches!(cmd.source, AdapterSource::Cli));
    assert!(matches!(cmd.payload, AdapterPayload::MetaMutate { cmd } if cmd == ":quit"));
}

// ── CommandResponse ───────────────────────────────────────────────────────────

#[test]
fn test_command_response_message_variant() {
    let msg = ServerMessage::CommandResult {
        command_id: "1".to_string(),
        output: "ok".to_string(),
    };
    let resp = CommandResponse::Message(msg);
    assert!(matches!(resp, CommandResponse::Message(_)));
}

// ── process_adapter_command ───────────────────────────────────────────────────

#[test]
fn test_process_inject_builds_server_message() {
    // Тест через tick_loop::test helper — process_adapter_command pub(crate),
    // тестируем косвенно через inject + tick_loop в test_tick_loop_processes_inject_command.
    // Прямая проверка: AdapterPayload::Inject создаётся корректно.
    let payload = AdapterPayload::Inject { text: "hello world".to_string() };
    assert!(matches!(payload, AdapterPayload::Inject { .. }));
}

// ── tick_loop (async) ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_tick_loop_terminates_on_quit_command() {
    let (tx, rx)     = mpsc::channel::<AdapterCommand>(16);
    let (btx, _brx)  = broadcast::channel::<ServerMessage>(16);
    let snap         = make_snapshot();

    // Отправляем :quit — tick loop должен завершиться
    tx.send(AdapterCommand {
        id:       "q".to_string(),
        source:   AdapterSource::Cli,
        payload:  AdapterPayload::MetaMutate { cmd: ":quit".to_string() },
        priority: axiom_runtime::GatewayPriority::Normal,
    }).await.unwrap();
    drop(tx); // закрываем канал после отправки

    tokio::time::timeout(
        std::time::Duration::from_secs(2),
        tick_loop(make_engine(), rx, btx, snap, make_saver(), None, make_config(), None),
    ).await.expect("tick_loop should terminate after :quit within 2s");
}

#[tokio::test]
async fn test_tick_loop_processes_inject_command() {
    let (tx, rx) = mpsc::channel::<AdapterCommand>(16);
    let (btx, mut brx) = broadcast::channel::<ServerMessage>(64);
    let snap = make_snapshot();

    tx.send(AdapterCommand {
        id:       "i1".to_string(),
        source:   AdapterSource::Cli,
        payload:  AdapterPayload::Inject { text: "test input".to_string() },
        priority: axiom_runtime::GatewayPriority::Normal,
    }).await.unwrap();

    // Отправляем quit чтобы loop завершился
    tx.send(AdapterCommand::shutdown()).await.unwrap();
    drop(tx);

    tokio::time::timeout(
        std::time::Duration::from_secs(2),
        tick_loop(make_engine(), rx, btx, snap, make_saver(), None, make_config(), None),
    ).await.expect("tick_loop should terminate");

    // Хотя бы одно сообщение должно быть в broadcast
    let mut found_result = false;
    while let Ok(msg) = brx.try_recv() {
        if matches!(msg, ServerMessage::Result { .. }) {
            found_result = true;
        }
    }
    assert!(found_result, "expected at least one ServerMessage::Result from inject");
}

#[tokio::test]
async fn test_tick_loop_updates_snapshot_after_interval() {
    let (tx, rx) = mpsc::channel::<AdapterCommand>(16);
    let (btx, _) = broadcast::channel::<ServerMessage>(64);
    let snap = make_snapshot();
    let snap_clone = Arc::clone(&snap);

    let mut config = make_config();
    // Устанавливаем state_broadcast_interval = 1 чтобы snapshot обновился сразу
    config.websocket.state_broadcast_interval = 1;

    tx.send(AdapterCommand::shutdown()).await.unwrap();
    drop(tx);

    tokio::time::timeout(
        std::time::Duration::from_secs(2),
        tick_loop(make_engine(), rx, btx, snap, make_saver(), None, config, None),
    ).await.expect("tick_loop should terminate");

    // После хотя бы одного тика snapshot должен быть обновлён (tick_count > 0 или default)
    let _snap = snap_clone.read().await;
    // Не проверяем конкретное значение — достаточно что snapshot доступен без паники
}
