use std::time::Duration;

use axiom_protocol::{
    commands::EngineCommand,
    messages::{ClientKind, ClientMessage, EngineMessage},
    PROTOCOL_VERSION,
};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::Message as WsMessage, MaybeTlsStream, WebSocketStream,
};

use crate::app::{CommandSender, Message};

const BACKOFF_SECS: &[u64] = &[1, 2, 5, 10, 30];

pub fn ws_subscription(address: String, key: u64) -> iced::Subscription<Message> {
    iced::Subscription::run_with_id(
        (address.clone(), key),
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
                        output
                            .send(Message::WsReconnecting {
                                attempt,
                                next_retry_secs: secs,
                            })
                            .await
                            .ok();
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
    })
    .map_err(|e| e.to_string())?;
    sink.send(WsMessage::Binary(hello_bytes))
        .await
        .map_err(|e| e.to_string())?;

    // Ожидаем Hello от Engine
    let engine_version = loop {
        match stream.next().await {
            Some(Ok(WsMessage::Binary(b))) => match postcard::from_bytes::<EngineMessage>(&b) {
                Ok(EngineMessage::Hello { version, .. }) => break version,
                Ok(EngineMessage::Bye { reason }) => {
                    return Err(format!("Engine rejected: {:?}", reason));
                }
                _ => return Err("Unexpected handshake message".into()),
            },
            Some(Ok(WsMessage::Ping(data))) => {
                let _ = sink.send(WsMessage::Pong(data)).await;
            }
            Some(Err(e)) => return Err(e.to_string()),
            _ => return Err("Connection closed during handshake".into()),
        }
    };

    // Создаём канал для команд App → Engine
    let (cmd_tx, mut cmd_rx) = iced::futures::channel::mpsc::channel::<(u64, EngineCommand)>(32);

    output
        .send(Message::WsConnected { engine_version })
        .await
        .map_err(|_| "App closed".to_string())?;

    output
        .send(Message::CommandSenderReady(CommandSender(cmd_tx)))
        .await
        .map_err(|_| "App closed".to_string())?;

    // Основной цикл: читаем WS и команды от App
    loop {
        tokio::select! {
            ws_msg = stream.next() => {
                match ws_msg {
                    Some(Ok(WsMessage::Binary(b))) => {
                        let app_msg = match postcard::from_bytes::<EngineMessage>(&b) {
                            Ok(EngineMessage::Snapshot(snap)) => Message::WsSnapshot(snap),
                            Ok(EngineMessage::Event(ev)) => Message::WsEvent(ev),
                            Ok(EngineMessage::CommandResult { command_id, result }) => {
                                Message::WsCommandResult { command_id, result }
                            }
                            Ok(EngineMessage::Bye { .. }) => return Ok(()),
                            _ => continue,
                        };
                        if output.send(app_msg).await.is_err() {
                            return Ok(());
                        }
                    }
                    Some(Ok(WsMessage::Ping(data))) => {
                        let _ = sink.send(WsMessage::Pong(data)).await;
                    }
                    Some(Ok(WsMessage::Close(_))) | None => return Ok(()),
                    Some(Err(e)) => return Err(e.to_string()),
                    Some(Ok(_)) => {}
                }
            }
            cmd = cmd_rx.next() => {
                match cmd {
                    Some((id, command)) => {
                        let msg = ClientMessage::Command { command_id: id, command };
                        let bytes = postcard::to_stdvec(&msg).map_err(|e| e.to_string())?;
                        sink.send(WsMessage::Binary(bytes)).await
                            .map_err(|e| e.to_string())?;
                    }
                    None => return Ok(()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_broadcasting::{BroadcastServer, BroadcastingConfig};
    use std::net::SocketAddr;

    // Test 3.7.b — WebSocket handshake: Workstation подключается к BroadcastServer
    #[tokio::test]
    async fn test_websocket_handshake() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr: SocketAddr = listener.local_addr().unwrap();
        drop(listener);

        let (server, _handle) = BroadcastServer::new(addr, BroadcastingConfig::default());
        tokio::spawn(async move {
            server.run().await.ok();
        });
        tokio::time::sleep(Duration::from_millis(10)).await;

        let url = format!("ws://{}", addr);
        let (ws, _) = connect_async(&url).await.expect("connect");

        let (mut tx, mut rx) = iced::futures::channel::mpsc::channel(32);
        tokio::spawn(async move {
            run_session(ws, &mut tx).await.ok();
        });

        // Skip CommandSenderReady, find WsConnected
        let deadline = tokio::time::Instant::now() + Duration::from_millis(500);
        loop {
            let msg = tokio::time::timeout_at(deadline, rx.next())
                .await
                .expect("timeout")
                .expect("channel closed");
            if matches!(msg, Message::WsConnected { .. }) {
                return;
            }
        }
    }
}
