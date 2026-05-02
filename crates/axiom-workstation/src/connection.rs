use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message as WsMessage,
    MaybeTlsStream, WebSocketStream,
};
use tokio::net::TcpStream;
use axiom_protocol::{
    messages::{ClientKind, ClientMessage, EngineMessage},
    PROTOCOL_VERSION,
};

use crate::app::Message;

const BACKOFF_SECS: &[u64] = &[1, 2, 5, 10, 30];

pub fn ws_subscription(address: String) -> iced::Subscription<Message> {
    iced::Subscription::run_with_id(
        address.clone(),
        iced::stream::channel(100, move |mut output| async move {
            let mut attempt: u32 = 0;
            loop {
                output.send(Message::WsConnecting).await.ok();
                let url = format!("ws://{}", address);

                match connect_async(&url).await {
                    Ok((ws, _)) => {
                        attempt = 0;
                        if let Err(e) = run_session(ws, &mut output).await {
                            tracing::debug!("WS session ended: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::debug!("WS connect failed (attempt {}): {}", attempt + 1, e);
                        let secs = BACKOFF_SECS[(attempt as usize).min(BACKOFF_SECS.len() - 1)];
                        attempt += 1;
                        output.send(Message::WsReconnecting {
                            attempt,
                            next_retry_secs: secs,
                        }).await.ok();
                        tokio::time::sleep(Duration::from_secs(secs)).await;
                    }
                }

                output.send(Message::WsDisconnected).await.ok();
            }
        }),
    )
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub(crate) async fn run_session(
    ws: WsStream,
    output: &mut iced::futures::channel::mpsc::Sender<Message>,
) -> Result<(), String> {
    let (mut sink, mut stream) = ws.split();

    // Handshake: отправляем Hello
    let hello_bytes = postcard::to_stdvec(&ClientMessage::Hello {
        version: PROTOCOL_VERSION,
        client_kind: ClientKind::Workstation,
    }).map_err(|e| e.to_string())?;
    sink.send(WsMessage::Binary(hello_bytes)).await.map_err(|e| e.to_string())?;

    // Ожидаем Hello от Engine
    let engine_version = loop {
        match stream.next().await {
            Some(Ok(WsMessage::Binary(b))) => {
                match postcard::from_bytes::<EngineMessage>(&b) {
                    Ok(EngineMessage::Hello { version, .. }) => break version,
                    Ok(EngineMessage::Bye { reason }) => {
                        return Err(format!("Engine rejected: {:?}", reason));
                    }
                    _ => return Err("Unexpected handshake message".into()),
                }
            }
            Some(Ok(WsMessage::Ping(data))) => {
                let _ = sink.send(WsMessage::Pong(data)).await;
            }
            Some(Err(e)) => return Err(e.to_string()),
            _ => return Err("Connection closed during handshake".into()),
        }
    };

    output.send(Message::WsConnected { engine_version }).await
        .map_err(|_| "App closed".to_string())?;

    // Основной цикл чтения
    loop {
        match stream.next().await {
            Some(Ok(WsMessage::Binary(b))) => {
                let app_msg = match postcard::from_bytes::<EngineMessage>(&b) {
                    Ok(EngineMessage::Snapshot(snap)) => Message::WsSnapshot(snap),
                    Ok(EngineMessage::Event(ev))      => Message::WsEvent(ev),
                    Ok(EngineMessage::Bye { .. })      => return Ok(()),
                    _                                  => continue,
                };
                if output.send(app_msg).await.is_err() {
                    return Ok(());
                }
            }
            Some(Ok(WsMessage::Ping(data))) => {
                let _ = sink.send(WsMessage::Pong(data)).await;
            }
            Some(Ok(WsMessage::Close(_))) | None => return Ok(()),
            Some(Err(e))                          => return Err(e.to_string()),
            Some(Ok(_))                           => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use axiom_broadcasting::{BroadcastServer, BroadcastingConfig};

    // Test 3.7.b — WebSocket handshake: Workstation подключается к BroadcastServer
    #[tokio::test]
    async fn test_websocket_handshake() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr: SocketAddr = listener.local_addr().unwrap();
        drop(listener);

        let (server, _handle) = BroadcastServer::new(addr, BroadcastingConfig::default());
        tokio::spawn(async move { server.run().await.ok(); });
        tokio::time::sleep(Duration::from_millis(10)).await;

        let url = format!("ws://{}", addr);
        let (ws, _) = connect_async(&url).await.expect("connect");

        let (mut tx, mut rx) = iced::futures::channel::mpsc::channel(32);
        tokio::spawn(async move {
            run_session(ws, &mut tx).await.ok();
        });

        let msg = tokio::time::timeout(
            Duration::from_millis(500),
            rx.next(),
        ).await.expect("timeout").expect("channel closed");

        assert!(
            matches!(msg, Message::WsConnected { .. }),
            "expected WsConnected, got {:?}", msg
        );
    }
}
