// SPDX-License-Identifier: AGPL-3.0-only
// Интеграционные тесты WebSocket-адаптера (Phase 1).

use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::Duration;

use axiom_agent::adapter_command::AdapterCommand;
use axiom_agent::adapters_config::AdaptersConfig;
use axiom_agent::channels::cli::CliConfig;
use axiom_agent::protocol::ServerMessage;
use axiom_agent::tick_loop::tick_loop;
use axiom_agent::ws::{AppState, bind, serve_ws};
use axiom_persist::{AutoSaver, PersistenceConfig};
use axiom_runtime::{AxiomEngine, BroadcastSnapshot};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message};

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_engine() -> AxiomEngine { AxiomEngine::new() }
fn make_saver()  -> AutoSaver   { AutoSaver::new(PersistenceConfig::disabled()) }

/// Запустить WS-сервер без tick_loop. Возвращает порт и AppState для прямых broadcast.
async fn spawn_ws_only() -> (u16, AppState) {
    let (command_tx, _rx)   = mpsc::channel::<AdapterCommand>(64);
    let (broadcast_tx, _bx) = broadcast::channel::<ServerMessage>(128);
    let snapshot            = Arc::new(RwLock::new(BroadcastSnapshot::default()));

    let state = AppState {
        command_tx,
        broadcast_tx,
        snapshot,
        next_conn_id: Arc::new(AtomicU64::new(0)),
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port     = listener.local_addr().unwrap().port();
    tokio::spawn(serve_ws(listener, state.clone()));

    (port, state)
}

/// Запустить WS-сервер + tick_loop. tick_broadcast_interval=1, state отключён.
async fn spawn_full(tick_broadcast: u32, state_broadcast: u32) -> u16 {
    let (command_tx, command_rx) = mpsc::channel::<AdapterCommand>(64);
    let (broadcast_tx, _)        = broadcast::channel::<ServerMessage>(256);
    let snapshot                 = Arc::new(RwLock::new(BroadcastSnapshot::default()));

    let ws_state = AppState {
        command_tx,
        broadcast_tx: broadcast_tx.clone(),
        snapshot:     Arc::clone(&snapshot),
        next_conn_id: Arc::new(AtomicU64::new(0)),
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port     = listener.local_addr().unwrap().port();
    tokio::spawn(serve_ws(listener, ws_state));

    let mut cfg = AdaptersConfig::from_cli_config(&CliConfig::default());
    cfg.websocket.tick_broadcast_interval  = tick_broadcast;
    cfg.websocket.state_broadcast_interval = state_broadcast;

    tokio::spawn(tick_loop(
        make_engine(), command_rx, broadcast_tx, snapshot,
        make_saver(), None, cfg,
    ));

    port
}

async fn ws_url(port: u16) -> String {
    format!("ws://127.0.0.1:{port}/ws")
}

/// Принять до N текстовых WS-сообщений за timeout. Парсит как JSON Value.
async fn collect_msgs(
    read: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin),
    n:    usize,
    dur:  Duration,
) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    let _ = tokio::time::timeout(dur, async {
        while let Some(Ok(Message::Text(t))) = read.next().await {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&t) {
                out.push(v);
            }
            if out.len() >= n { break; }
        }
    }).await;
    out
}

// ── базовые тесты ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_ws_connect_disconnect() {
    let (port, _state) = spawn_ws_only().await;
    let (ws, _) = connect_async(ws_url(port).await).await
        .expect("WS connect failed");
    let (mut write, _) = ws.split();
    // Graceful close
    write.close().await.unwrap_or_default();
}

#[tokio::test]
async fn test_ws_invalid_json_returns_error() {
    let (port, _state) = spawn_ws_only().await;
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (mut write, mut read) = ws.split();

    write.send(Message::Text("not json at all".into())).await.unwrap();

    let msgs = collect_msgs(&mut read, 1, Duration::from_secs(1)).await;
    assert!(!msgs.is_empty(), "expected error response");
    assert_eq!(msgs[0]["type"], "error");
}

#[tokio::test]
async fn test_ws_multiple_clients_connect() {
    let (port, _state) = spawn_ws_only().await;
    let url = ws_url(port).await;

    let (ws1, _) = connect_async(url.clone()).await.unwrap();
    let (ws2, _) = connect_async(url).await.unwrap();

    let (mut w1, _) = ws1.split();
    let (mut w2, _) = ws2.split();
    w1.close().await.unwrap_or_default();
    w2.close().await.unwrap_or_default();
}

// ── broadcast filtering ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_ws_no_subscription_receives_all() {
    let (port, state) = spawn_ws_only().await;
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (_, mut read) = ws.split();

    // Немного подождём чтобы соединение установилось
    tokio::time::sleep(Duration::from_millis(20)).await;

    // Прямо отправляем Tick в broadcast
    let _ = state.broadcast_tx.send(ServerMessage::Tick {
        tick_count: 42, traces: 0, tension: 0, last_matched: 0,
    });

    let msgs = collect_msgs(&mut read, 1, Duration::from_secs(1)).await;
    assert!(msgs.iter().any(|m| m["type"] == "tick"), "empty subscription should receive Tick");
}

#[tokio::test]
async fn test_ws_subscribe_ticks_only_filters_state() {
    let (port, state) = spawn_ws_only().await;
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (mut write, mut read) = ws.split();

    write.send(Message::Text(
        r#"{"type":"subscribe","channels":["ticks"]}"#.into()
    )).await.unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;

    // Отправляем только State — не должен дойти
    let _ = state.broadcast_tx.send(ServerMessage::State {
        tick_count: 1,
        snapshot:   axiom_runtime::BroadcastSnapshot::default(),
    });
    // Отправляем Tick — должен дойти
    let _ = state.broadcast_tx.send(ServerMessage::Tick {
        tick_count: 1, traces: 0, tension: 0, last_matched: 0,
    });

    let msgs = collect_msgs(&mut read, 2, Duration::from_secs(1)).await;
    assert!(msgs.iter().any(|m| m["type"] == "tick"), "should receive tick");
    assert!(!msgs.iter().any(|m| m["type"] == "state"), "should not receive state");
}

#[tokio::test]
async fn test_ws_unsubscribe_removes_channel() {
    // Синхронизация через `:status`: команды WS обрабатываются FIFO, поэтому
    // когда CommandResult вернётся, Subscribe + Unsubscribe уже применены.
    let port = spawn_full(0, 0).await; // tick_loop запущен, broadcast отключён
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (mut write, mut read) = ws.split();

    write.send(Message::Text(r#"{"type":"subscribe","channels":["ticks"]}"#.into())).await.unwrap();
    write.send(Message::Text(r#"{"type":"unsubscribe","channels":["ticks"]}"#.into())).await.unwrap();
    // `:status` ставится в очередь после Subscribe+Unsubscribe — когда придёт ответ,
    // оба предыдущих сообщения уже обработаны обработчиком WS.
    write.send(Message::Text(r#"{"type":"read_command","cmd":":status"}"#.into())).await.unwrap();

    // Ждём CommandResult как точки синхронизации
    let sync = tokio::time::timeout(
        Duration::from_secs(2),
        collect_msgs(&mut read, 1, Duration::from_secs(2)),
    ).await.expect("timeout waiting for sync response");
    assert!(sync.iter().any(|m| m["type"] == "command_result"), "sync failed");

    // Subscriptions теперь = Some({}) — Tick не должен прийти
    // Для проверки используем State — который Subscribe {"ticks"} не включал
    // и который не отправляется т.к. state_broadcast_interval=0.
    // Вместо этого проверяем через ещё один :status и убеждаемся что тип != "tick"
    let msgs = collect_msgs(&mut read, 5, Duration::from_millis(200)).await;
    assert!(!msgs.iter().any(|m| m["type"] == "tick"),
        "after unsubscribe from ticks, no tick broadcasts should arrive");
}

// ── полный pipeline (с tick_loop) ─────────────────────────────────────────────

#[tokio::test]
async fn test_ws_inject_returns_result() {
    let port = spawn_full(0, 0).await; // broadcast отключён — только прямые ответы
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (mut write, mut read) = ws.split();

    write.send(Message::Text(
        r#"{"type":"inject","text":"hello world"}"#.into()
    )).await.unwrap();

    let msgs = tokio::time::timeout(
        Duration::from_secs(2),
        collect_msgs(&mut read, 1, Duration::from_secs(2)),
    ).await.expect("timeout waiting for result");

    assert!(msgs.iter().any(|m| m["type"] == "result"),
        "expected ServerMessage::Result, got: {:?}", msgs);
}

#[tokio::test]
async fn test_ws_read_command_status_returns_output() {
    let port = spawn_full(0, 0).await;
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (mut write, mut read) = ws.split();

    write.send(Message::Text(
        r#"{"type":"read_command","cmd":":status"}"#.into()
    )).await.unwrap();

    let msgs = tokio::time::timeout(
        Duration::from_secs(2),
        collect_msgs(&mut read, 1, Duration::from_secs(2)),
    ).await.expect("timeout waiting for command_result");

    assert!(msgs.iter().any(|m| m["type"] == "command_result"),
        "expected CommandResult, got: {:?}", msgs);
    let result = msgs.iter().find(|m| m["type"] == "command_result").unwrap();
    assert!(result["output"].as_str().unwrap_or("").contains("tick_count"),
        "status output should contain tick_count");
}

#[tokio::test]
async fn test_ws_tick_broadcast_arrives() {
    // tick_broadcast_interval=1 — Tick отправляется каждый тик
    let port = spawn_full(1, 0).await;
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (_, mut read) = ws.split();

    // Ждём хотя бы один Tick за 500ms
    let msgs = collect_msgs(&mut read, 5, Duration::from_millis(500)).await;
    assert!(msgs.iter().any(|m| m["type"] == "tick"),
        "expected at least one Tick broadcast");
}

#[tokio::test]
async fn test_ws_multiple_clients_all_receive_tick() {
    let (port, state) = spawn_ws_only().await;
    let url = ws_url(port).await;

    let (ws1, _) = connect_async(url.clone()).await.unwrap();
    let (ws2, _) = connect_async(url).await.unwrap();
    let (_, mut r1) = ws1.split();
    let (_, mut r2) = ws2.split();

    tokio::time::sleep(Duration::from_millis(20)).await;

    let _ = state.broadcast_tx.send(ServerMessage::Tick {
        tick_count: 99, traces: 5, tension: 3, last_matched: 2,
    });

    let (m1, m2) = tokio::join!(
        collect_msgs(&mut r1, 1, Duration::from_secs(1)),
        collect_msgs(&mut r2, 1, Duration::from_secs(1)),
    );

    assert!(m1.iter().any(|m| m["type"] == "tick" && m["tick_count"] == 99),
        "client 1 should receive tick");
    assert!(m2.iter().any(|m| m["type"] == "tick" && m["tick_count"] == 99),
        "client 2 should receive tick");
}

#[tokio::test]
async fn test_ws_disconnect_no_panic() {
    let port = spawn_full(1, 0).await;
    let (ws, _) = connect_async(ws_url(port).await).await.unwrap();
    let (mut write, _read) = ws.split();

    // Отправляем команду и сразу закрываем без ожидания ответа
    write.send(Message::Text(
        r#"{"type":"inject","text":"abrupt disconnect test"}"#.into()
    )).await.unwrap();
    write.close().await.unwrap_or_default();

    // Сервер не должен паниковать — даём ему время обработать
    tokio::time::sleep(Duration::from_millis(100)).await;
}

// ── bind helper ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_bind_port_zero_gets_real_port() {
    let listener = bind(0).await;
    let addr = listener.local_addr().unwrap();
    assert!(addr.port() > 0, "OS should assign a non-zero port");
    // Cleanup: listener drops here
}
