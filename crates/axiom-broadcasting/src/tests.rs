use std::net::SocketAddr;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};

use axiom_protocol::messages::{ClientKind, ClientMessage, EngineMessage, ShutdownReason};
use axiom_protocol::PROTOCOL_VERSION;

use crate::{BroadcastServer, BroadcastingConfig};

async fn spawn_test_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let actual_addr = listener.local_addr().unwrap();
    drop(listener);

    let (server, _handle) = BroadcastServer::new(actual_addr, BroadcastingConfig::default());
    let jh = tokio::spawn(async move {
        server.run().await.ok();
    });
    tokio::time::sleep(Duration::from_millis(10)).await;
    (actual_addr, jh)
}

async fn ws_connect(
    addr: SocketAddr,
) -> (
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
    }))
    .await
    .unwrap();

    let reply = source.next().await.unwrap().unwrap();
    let msg = decode_engine(reply);

    assert!(matches!(
        msg,
        EngineMessage::Hello {
            version: PROTOCOL_VERSION,
            ..
        }
    ));
}

// Test 2.7.b — version mismatch
#[tokio::test]
async fn test_handshake_version_mismatch() {
    let (addr, _jh) = spawn_test_server().await;
    let (mut sink, mut source) = ws_connect(addr).await;

    sink.send(encode(&ClientMessage::Hello {
        version: 0x02_00_00_00, // major version 2, server is 1
        client_kind: ClientKind::Workstation,
    }))
    .await
    .unwrap();

    let reply = source.next().await.unwrap().unwrap();
    let msg = decode_engine(reply);
    assert!(matches!(
        msg,
        EngineMessage::Bye {
            reason: ShutdownReason::VersionMismatch
        }
    ));
}

// Test 2.7.c — multiple clients all receive broadcast
#[tokio::test]
async fn test_multiple_clients_receive_broadcast() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let actual_addr = listener.local_addr().unwrap();
    drop(listener);

    let (server, handle) = BroadcastServer::new(actual_addr, BroadcastingConfig::default());
    tokio::spawn(async move {
        server.run().await.ok();
    });
    tokio::time::sleep(Duration::from_millis(10)).await;

    let mut clients = vec![];
    for _ in 0..3 {
        let (mut sink, mut source) = ws_connect(actual_addr).await;
        sink.send(encode(&ClientMessage::Hello {
            version: PROTOCOL_VERSION,
            client_kind: ClientKind::Workstation,
        }))
        .await
        .unwrap();
        let _ = source.next().await; // consume Hello reply
        clients.push((sink, source));
    }

    use axiom_protocol::events::{AlertLevel, EngineEvent};
    handle.publish(EngineMessage::Event(EngineEvent::Alert {
        level: AlertLevel::Info,
        category: "test".into(),
        message: "broadcast test".into(),
    }));

    for (_sink, source) in &mut clients {
        let msg = tokio::time::timeout(Duration::from_millis(200), source.next())
            .await
            .expect("timeout")
            .unwrap()
            .unwrap();
        let decoded = decode_engine(msg);
        assert!(matches!(
            decoded,
            EngineMessage::Event(EngineEvent::Alert { .. })
        ));
    }
}

// Test 2.7.d — subscription filter (Tick not in DEFAULT)
#[tokio::test]
async fn test_subscription_filter() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let (server, handle) = BroadcastServer::new(addr, BroadcastingConfig::default());
    tokio::spawn(async move {
        server.run().await.ok();
    });
    tokio::time::sleep(Duration::from_millis(10)).await;

    let (mut sink, mut source) = ws_connect(addr).await;
    sink.send(encode(&ClientMessage::Hello {
        version: PROTOCOL_VERSION,
        client_kind: ClientKind::Workstation,
    }))
    .await
    .unwrap();
    let _ = source.next().await; // consume Hello reply

    use axiom_protocol::event_category;
    sink.send(encode(&ClientMessage::Subscribe {
        event_categories: event_category::DEFAULT,
    }))
    .await
    .unwrap();

    use axiom_protocol::events::EngineEvent;
    handle.publish(EngineMessage::Event(EngineEvent::Tick {
        tick: 1,
        event: 0,
        hot_path_ns: 100,
    }));

    use axiom_protocol::events::AlertLevel;
    handle.publish(EngineMessage::Event(EngineEvent::Alert {
        level: AlertLevel::Warning,
        category: "filter_test".into(),
        message: "only this should arrive".into(),
    }));

    let msg = tokio::time::timeout(Duration::from_millis(200), source.next())
        .await
        .expect("timeout waiting for Alert")
        .unwrap()
        .unwrap();
    let decoded = decode_engine(msg);
    assert!(matches!(
        decoded,
        EngineMessage::Event(EngineEvent::Alert { .. })
    ));
}

// Test 2.7.e — server sends heartbeat pings at configured interval.
// Note: tungstenite clients auto-respond to pings, so pong_timeout disconnect
// cannot be triggered via a normal WebSocket client — tested at code-review level.
#[tokio::test]
async fn test_heartbeat() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let config = BroadcastingConfig {
        heartbeat_interval: Duration::from_millis(50),
        pong_timeout: Duration::from_millis(500),
        ..BroadcastingConfig::default()
    };
    let (server, _handle) = BroadcastServer::new(addr, config);
    tokio::spawn(async move {
        server.run().await.ok();
    });
    tokio::time::sleep(Duration::from_millis(10)).await;

    let (mut sink, mut source) = ws_connect(addr).await;
    sink.send(encode(&ClientMessage::Hello {
        version: PROTOCOL_VERSION,
        client_kind: ClientKind::Workstation,
    }))
    .await
    .unwrap();
    let _ = source.next().await; // consume Hello reply

    // Server must send a Ping within heartbeat_interval (50ms) + 150ms margin
    let found = tokio::time::timeout(Duration::from_millis(200), async move {
        while let Some(Ok(msg)) = source.next().await {
            match msg {
                WsMessage::Ping(_) => return true,
                WsMessage::Binary(_) => continue, // skip any event messages
                _ => return false,
            }
        }
        false
    })
    .await;
    assert!(
        matches!(found, Ok(true)),
        "server must send heartbeat Ping within 200ms"
    );
}

// Test 2.7.f — server survives message flood (handles RecvError::Lagged gracefully)
#[tokio::test]
async fn test_dropping_under_load() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let config = BroadcastingConfig {
        max_event_queue_per_client: 4, // tiny queue to force RecvError::Lagged
        ..BroadcastingConfig::default()
    };
    let (server, handle) = BroadcastServer::new(addr, config);
    tokio::spawn(async move {
        server.run().await.ok();
    });
    tokio::time::sleep(Duration::from_millis(10)).await;

    let (mut sink, mut source) = ws_connect(addr).await;
    sink.send(encode(&ClientMessage::Hello {
        version: PROTOCOL_VERSION,
        client_kind: ClientKind::Workstation,
    }))
    .await
    .unwrap();
    let _ = source.next().await; // consume Hello reply

    // Flood with 50 events (queue = 4 → Lagged guaranteed)
    use axiom_protocol::events::{AlertLevel, EngineEvent};
    for i in 0..50u64 {
        handle.publish(EngineMessage::Event(EngineEvent::Alert {
            level: AlertLevel::Info,
            category: "flood".into(),
            message: format!("msg-{}", i),
        }));
    }

    // Give the event loop time to process lagged messages before publishing sentinel
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Sentinel proves server is still alive and delivering messages after the flood
    handle.publish(EngineMessage::Event(EngineEvent::Alert {
        level: AlertLevel::Warning,
        category: "sentinel".into(),
        message: "alive".into(),
    }));

    let mut found_sentinel = false;
    loop {
        match tokio::time::timeout(Duration::from_millis(300), source.next()).await {
            Ok(Some(Ok(WsMessage::Binary(b)))) => {
                if let Ok(EngineMessage::Event(EngineEvent::Alert { message, .. })) =
                    postcard::from_bytes::<EngineMessage>(&b)
                {
                    if message == "alive" {
                        found_sentinel = true;
                        break;
                    }
                }
            }
            Ok(Some(Ok(WsMessage::Ping(_)))) => {} // ignore heartbeat pings
            _ => break,
        }
    }
    assert!(
        found_sentinel,
        "server must survive message flood and still deliver messages"
    );
}
