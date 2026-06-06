use std::net::SocketAddr;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

// ── BRD-TD-06: pong timeout disconnects unresponsive client ──────────────────
//
// tungstenite auto-responds to Pings — нельзя протестировать через обычный WS клиент.
// Решение: raw TCP + ручной WebSocket handshake. Получаем Ping, игнорируем,
// ждём pong_timeout, проверяем что сервер закрыл соединение (Close frame или EOF).

/// Сделать masked WebSocket фрейм (клиент → сервер).
/// Маска из нулей: XOR с 0 = payload без изменений (валидно по спеке).
fn ws_client_frame(opcode: u8, payload: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();
    f.push(0x80 | opcode); // FIN=1
    let len = payload.len();
    if len < 126 {
        f.push(0x80 | len as u8); // MASK=1
    } else {
        f.push(0xFE); // MASK=1 | 126
        f.push((len >> 8) as u8);
        f.push(len as u8);
    }
    f.extend_from_slice(&[0u8; 4]); // нулевая маска → payload не меняется
    f.extend_from_slice(payload);
    f
}

/// Прочитать один WS фрейм с сервера (немаскированный). Возвращает opcode + payload.
async fn ws_read_frame(stream: &mut tokio::net::TcpStream) -> std::io::Result<(u8, Vec<u8>)> {
    let mut hdr = [0u8; 2];
    stream.read_exact(&mut hdr).await?;
    let opcode = hdr[0] & 0x0F;
    let len_byte = hdr[1] & 0x7F; // сервер не маскирует
    let payload_len: usize = if len_byte < 126 {
        len_byte as usize
    } else if len_byte == 126 {
        let mut l = [0u8; 2];
        stream.read_exact(&mut l).await?;
        u16::from_be_bytes(l) as usize
    } else {
        let mut l = [0u8; 8];
        stream.read_exact(&mut l).await?;
        u64::from_be_bytes(l) as usize
    };
    let mut payload = vec![0u8; payload_len];
    if payload_len > 0 {
        stream.read_exact(&mut payload).await?;
    }
    Ok((opcode, payload))
}

// Test 2.7.g — BRD-TD-06: server disconnects client that ignores pings.
#[tokio::test]
async fn test_pong_timeout_disconnects_silent_client() {
    use axiom_protocol::messages::ClientKind;

    let config = BroadcastingConfig {
        heartbeat_interval: Duration::from_millis(50),
        pong_timeout: Duration::from_millis(80),
        ..BroadcastingConfig::default()
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let (server, _handle) = BroadcastServer::new(addr, config);
    tokio::spawn(async move { server.run().await.ok() });
    tokio::time::sleep(Duration::from_millis(10)).await;

    // 1. Подключаемся raw TCP
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();

    // 2. HTTP WebSocket upgrade (static key — сервер проверяет только формат)
    let request = format!(
        "GET / HTTP/1.1\r\nHost: {addr}\r\nUpgrade: websocket\r\n\
         Connection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
         Sec-WebSocket-Version: 13\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).await.unwrap();

    // 3. Читаем HTTP 101 (до \r\n\r\n)
    let mut http_buf = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        stream.read_exact(&mut byte).await.unwrap();
        http_buf.push(byte[0]);
        if http_buf.ends_with(b"\r\n\r\n") { break; }
    }
    assert!(
        http_buf.starts_with(b"HTTP/1.1 101"),
        "expected 101 Switching Protocols"
    );

    // 4. Отправляем ClientMessage::Hello как masked binary WS фрейм
    let hello = ClientMessage::Hello {
        version: PROTOCOL_VERSION,
        client_kind: ClientKind::Workstation,
    };
    let payload = postcard::to_stdvec(&hello).unwrap();
    stream.write_all(&ws_client_frame(2, &payload)).await.unwrap();

    // 5. Читаем фреймы НЕ отвечая на Ping.
    //    heartbeat=50ms → первый Ping через ~50ms.
    //    pong_timeout=80ms → сервер закроет ~130ms после старта.
    //    Ждём максимум 400ms.
    let deadline = tokio::time::Instant::now() + Duration::from_millis(400);
    let mut got_close = false;

    loop {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() { break; }

        let result = tokio::time::timeout(remaining, ws_read_frame(&mut stream)).await;
        match result {
            Ok(Ok((opcode, _))) => {
                if opcode == 8 { // Close frame
                    got_close = true;
                    break;
                }
                // opcode 9=Ping, 10=Pong, 2=Binary → игнорируем, не отвечаем
            }
            Ok(Err(_)) => {
                // EOF или ошибка чтения — сервер закрыл соединение
                got_close = true;
                break;
            }
            Err(_) => break, // таймаут всего теста
        }
    }

    assert!(
        got_close,
        "server must disconnect client that ignores pings within pong_timeout"
    );
}
