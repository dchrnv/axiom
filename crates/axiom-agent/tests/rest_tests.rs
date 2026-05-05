// SPDX-License-Identifier: AGPL-3.0-only
// Интеграционные тесты REST-адаптера (Phase 2).

use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use axiom_agent::adapter_command::AdapterCommand;
use axiom_agent::adapters_config::AdaptersConfig;
use axiom_agent::channels::cli::CliConfig;
use axiom_agent::protocol::ServerMessage;
use axiom_agent::tick_loop::tick_loop;
use axiom_agent::ws::{serve_ws, AppState};
use axiom_persist::{AutoSaver, PersistenceConfig};
use axiom_runtime::{AxiomEngine, BroadcastSnapshot};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, RwLock};

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_engine() -> AxiomEngine {
    AxiomEngine::new()
}
fn make_saver() -> AutoSaver {
    AutoSaver::new(PersistenceConfig::disabled())
}

/// Запустить полный сервер (WS + REST + tick_loop). Возвращает базовый URL.
async fn spawn_server() -> String {
    let (command_tx, command_rx) = mpsc::channel::<AdapterCommand>(64);
    let (broadcast_tx, _) = broadcast::channel::<ServerMessage>(256);
    let snapshot = Arc::new(RwLock::new(BroadcastSnapshot::default()));

    let ws_state = AppState {
        command_tx,
        broadcast_tx: broadcast_tx.clone(),
        snapshot: Arc::clone(&snapshot),
        next_conn_id: Arc::new(AtomicU64::new(0)),
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(serve_ws(listener, ws_state));

    let mut cfg = AdaptersConfig::from_cli_config(&CliConfig::default());
    cfg.websocket.tick_broadcast_interval = 0;
    cfg.websocket.state_broadcast_interval = 1; // обновлять snapshot каждый тик

    tokio::spawn(tick_loop(
        make_engine(),
        command_rx,
        broadcast_tx,
        snapshot,
        make_saver(),
        None,
        cfg,
        None,
    ));

    format!("http://127.0.0.1:{port}")
}

fn http() -> reqwest::Client {
    reqwest::Client::new()
}

// ── GET /api/status ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_rest_get_status_200() {
    let base = spawn_server().await;
    // Ждём инициализации
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;

    let resp = http()
        .get(format!("{base}/api/status"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let json: serde_json::Value = resp.json().await.unwrap();
    assert!(
        json.get("tick_count").is_some(),
        "status should contain tick_count"
    );
}

#[tokio::test]
async fn test_rest_get_status_no_engine_lock() {
    // Два параллельных GET /api/status не должны блокировать друг друга
    let base = spawn_server().await;
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;

    let (r1, r2) = tokio::join!(
        http().get(format!("{base}/api/status")).send(),
        http().get(format!("{base}/api/status")).send(),
    );
    assert_eq!(r1.unwrap().status(), 200);
    assert_eq!(r2.unwrap().status(), 200);
}

// ── GET /api/domains ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_rest_get_domains_11_entries() {
    let base = spawn_server().await;

    // Дожидаемся первого state-snapshot: посылаем :status и ждём CommandResult
    // Альтернатива: напрямую дать snapshot обновиться через state_broadcast
    // Здесь проще всего сделать один inject чтобы запустить тик
    let _inject = http()
        .post(format!("{base}/api/inject"))
        .json(&serde_json::json!({"text":"init"}))
        .send()
        .await
        .unwrap();

    let resp = http()
        .get(format!("{base}/api/domains"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let json: serde_json::Value = resp.json().await.unwrap();
    let arr = json.as_array().expect("domains should be array");
    assert_eq!(arr.len(), 11, "AXIOM has 11 domains, got {}", arr.len());
}

// ── POST /api/inject ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_rest_post_inject_returns_result() {
    let base = spawn_server().await;

    let resp = http()
        .post(format!("{base}/api/inject"))
        .json(&serde_json::json!({"text":"hello world"}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["type"], "result", "expected ServerMessage::Result");
    assert!(json.get("domain_name").is_some());
    assert!(json.get("coherence").is_some());
}

#[tokio::test]
async fn test_rest_post_invalid_json_returns_400() {
    let base = spawn_server().await;

    let resp = http()
        .post(format!("{base}/api/inject"))
        .header("content-type", "application/json")
        .body("not json")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn test_rest_post_inject_missing_text_returns_400() {
    let base = spawn_server().await;

    let resp = http()
        .post(format!("{base}/api/inject"))
        .json(&serde_json::json!({"wrong_field": "value"}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);
}

// ── POST /api/command ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_rest_post_read_command_status() {
    let base = spawn_server().await;

    let resp = http()
        .post(format!("{base}/api/command"))
        .json(&serde_json::json!({"cmd":":status","type":"read"}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["type"], "command_result");
    assert!(json["output"].as_str().unwrap_or("").contains("tick_count"));
}

#[tokio::test]
async fn test_rest_post_command_default_type_is_read() {
    let base = spawn_server().await;

    // Без поля "type" должно работать как read (default = "")
    let resp = http()
        .post(format!("{base}/api/command"))
        .json(&serde_json::json!({"cmd":":domains"}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["type"], "command_result");
}

// ── GET /api/domain/:id ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_rest_get_domain_valid_id() {
    let base = spawn_server().await;

    // domain_id 100 = SUTRA (всегда существует)
    let resp = http()
        .get(format!("{base}/api/domain/100"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().await.unwrap();
    // ServerMessage::DomainDetail сериализуется как вложенный объект
    assert!(
        json.get("domain_id").is_some() || json.get("DomainDetail").is_some(),
        "expected DomainDetail, got: {json}"
    );
}

#[tokio::test]
async fn test_rest_get_domain_invalid_id_404() {
    let base = spawn_server().await;

    // domain_id 9999 не существует
    let resp = http()
        .get(format!("{base}/api/domain/9999"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}
