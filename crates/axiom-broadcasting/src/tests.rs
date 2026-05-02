use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use futures_util::{SinkExt, StreamExt};

use axiom_protocol::messages::{ClientKind, ClientMessage, EngineMessage, ShutdownReason};
use axiom_protocol::PROTOCOL_VERSION;

use crate::{BroadcastingConfig, BroadcastServer};

async fn spawn_test_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let (server, _handle) = BroadcastServer::new(addr, BroadcastingConfig::default());

    // Bind to get actual port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let actual_addr = listener.local_addr().unwrap();
    drop(listener);

    let (server, handle) = BroadcastServer::new(actual_addr, BroadcastingConfig::default());
    let jh = tokio::spawn(async move { server.run().await.ok(); });
    tokio::time::sleep(Duration::from_millis(10)).await; // let server start
    (actual_addr, jh)
}

async fn ws_connect(addr: SocketAddr) -> (
    impl SinkExt<WsMessage, Error = tokio_tungstenite::tungstenite::Error>,
    impl StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>>,
) {
    let url = format!("ws://{}", addr);
    let (ws, _) = connect_async(&url).await.expect("connect");
    ws.split()
}

fn encode(msg: &ClientMessage) -> WsMessage {
    WsMessage::Binary(postcard::to_stdvec(msg).unwrap())
}

fn decode_engine(msg: WsMessage) -> EngineMessage {
    match msg {
        WsMessage::Binary(b) => postcard::from_bytes(&b).unwrap(),
        other => panic!("Expected binary, got {:?}", other),
    }
}

// Test 2.7.a — full handshake
#[tokio::test]
async fn test_handshake_ok() {
    let (addr, _jh) = spawn_test_server().await;
    let (mut sink, mut source) = ws_connect(addr).await;

    sink.send(encode(&ClientMessage::Hello {
        version: PROTOCOL_VERSION,
        client_kind: ClientKind::Workstation,
    })).await.unwrap();

    let reply = source.next().await.unwrap().unwrap();
    let msg = decode_engine(reply);

    assert!(matches!(msg, EngineMessage::Hello { version: PROTOCOL_VERSION, .. }));
}

// Test 2.7.b — version mismatch
#[tokio::test]
async fn test_handshake_version_mismatch() {
    let (addr, _jh) = spawn_test_server().await;
    let (mut sink, mut source) = ws_connect(addr).await;

    // Send a different major version
    sink.send(encode(&ClientMessage::Hello {
        version: 0x02_00_00_00, // major version 2, server is 1
        client_kind: ClientKind::Workstation,
    })).await.unwrap();

    let reply = source.next().await.unwrap().unwrap();
    let msg = decode_engine(reply);
    assert!(matches!(msg, EngineMessage::Bye { reason: ShutdownReason::VersionMismatch }));
}

// Test 2.7.c — multiple clients all receive broadcast
#[tokio::test]
async fn test_multiple_clients_receive_broadcast() {
    let actual_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let actual_addr = listener.local_addr().unwrap();
    drop(listener);

    let (server, handle) = BroadcastServer::new(actual_addr, BroadcastingConfig::default());
    tokio::spawn(async move { server.run().await.ok(); });
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Connect 3 clients
    let mut clients = vec![];
    for _ in 0..3 {
        let (mut sink, mut source) = ws_connect(actual_addr).await;
        sink.send(encode(&ClientMessage::Hello {
            version: PROTOCOL_VERSION,
            client_kind: ClientKind::Workstation,
        })).await.unwrap();
        // Consume Hello reply
        let _ = source.next().await;
        clients.push((sink, source));
    }

    // Engine broadcasts an Alert event
    use axiom_protocol::events::{AlertLevel, EngineEvent};
    handle.publish(EngineMessage::Event(EngineEvent::Alert {
        level: AlertLevel::Info,
        category: "test".into(),
        message: "broadcast test".into(),
    }));

    // All 3 clients should receive it (ALERTS is in DEFAULT subscription)
    for (_sink, source) in &mut clients {
        let msg = tokio::time::timeout(Duration::from_millis(200), source.next())
            .await
            .expect("timeout")
            .unwrap()
            .unwrap();
        let decoded = decode_engine(msg);
        assert!(matches!(decoded, EngineMessage::Event(EngineEvent::Alert { .. })));
    }
}

// Test 2.7.d — subscription filter (Tick not in DEFAULT)
#[tokio::test]
async fn test_subscription_filter() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let (server, handle) = BroadcastServer::new(addr, BroadcastingConfig::default());
    tokio::spawn(async move { server.run().await.ok(); });
    tokio::time::sleep(Duration::from_millis(10)).await;

    let (mut sink, mut source) = ws_connect(addr).await;
    sink.send(encode(&ClientMessage::Hello {
        version: PROTOCOL_VERSION,
        client_kind: ClientKind::Workstation,
    })).await.unwrap();
    let _ = source.next().await; // consume Hello reply

    // Client subscribes to DEFAULT (no Tick)
    use axiom_protocol::event_category;
    sink.send(encode(&ClientMessage::Subscribe {
        event_categories: event_category::DEFAULT,
    })).await.unwrap();

    // Engine publishes a Tick event — should NOT be delivered (filtered)
    use axiom_protocol::events::EngineEvent;
    handle.publish(EngineMessage::Event(EngineEvent::Tick {
        tick: 1,
        event: 0,
        hot_path_ns: 100,
    }));

    // Then publish an Alert — SHOULD be delivered
    use axiom_protocol::events::AlertLevel;
    handle.publish(EngineMessage::Event(EngineEvent::Alert {
        level: AlertLevel::Warning,
        category: "filter_test".into(),
        message: "only this should arrive".into(),
    }));

    // Should receive Alert, not Tick
    let msg = tokio::time::timeout(Duration::from_millis(200), source.next())
        .await
        .expect("timeout waiting for Alert")
        .unwrap()
        .unwrap();
    let decoded = decode_engine(msg);
    assert!(matches!(decoded, EngineMessage::Event(EngineEvent::Alert { .. })));
}
