// std::thread WS-клиент — не использует tokio (несовместим с eframe).
//
// Подключается к серверу, подписывается на ticks+state, гоняет loop:
//   recv incoming → обновляет AppData
//   check cmd_rx  → отправляет исходящие команды

use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use std::net::TcpStream;
use tungstenite::{Message, WebSocket};

use crate::protocol::ServerMessage;
use crate::state::AppData;

pub type WsSocket = WebSocket<TcpStream>;

/// Запустить WS-клиент (блокирует поток до разрыва соединения или ошибки).
///
/// `cmd_rx` — исходящие JSON-строки от GUI (inject, meta-команды).
pub fn run_ws_client(
    url: &str,
    data: Arc<Mutex<AppData>>,
    cmd_rx: std::sync::mpsc::Receiver<String>,
) {
    match connect(url) {
        Ok(mut socket) => {
            let _ = socket.get_mut(); // silence unused warning
            {
                let mut d = data.lock().unwrap();
                d.connected = true;
                d.last_error = None;
            }

            // Подписаться на ticks + state
            let sub = r#"{"type":"subscribe","channels":["ticks","state"]}"#;
            if socket.send(Message::Text(sub.to_string())).is_err() {
                mark_disconnected(&data, "subscribe failed");
                return;
            }

            loop {
                // Отправить исходящие команды от GUI
                while let Ok(cmd) = cmd_rx.try_recv() {
                    if socket.send(Message::Text(cmd)).is_err() {
                        mark_disconnected(&data, "send failed");
                        return;
                    }
                }

                // Прочитать входящее сообщение (с коротким таймаутом)
                match socket.read() {
                    Ok(Message::Text(text)) => handle_message(&text, &data),
                    Ok(Message::Close(_)) | Err(tungstenite::Error::ConnectionClosed) => {
                        mark_disconnected(&data, "server closed connection");
                        return;
                    }
                    Err(tungstenite::Error::Io(e))
                        if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut =>
                    {
                        // Нет данных — продолжаем
                    }
                    Err(e) => {
                        mark_disconnected(&data, &e.to_string());
                        return;
                    }
                    Ok(_) => {} // Binary, Ping, Pong — ignore
                }
            }
        }
        Err(e) => {
            mark_disconnected(&data, &e);
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn connect(url: &str) -> Result<WsSocket, String> {
    let addr = parse_addr(url);
    let stream = TcpStream::connect(&addr).map_err(|e| format!("tcp connect: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_millis(50)))
        .ok();

    let (socket, _) = tungstenite::client(url, stream).map_err(|e| format!("ws handshake: {e}"))?;
    Ok(socket)
}

fn parse_addr(url: &str) -> String {
    // ws://host:port/path → "host:port"
    let without_scheme = url.trim_start_matches("ws://").trim_start_matches("wss://");
    let host_port = without_scheme.split('/').next().unwrap_or("127.0.0.1:8765");
    host_port.to_string()
}

fn handle_message(text: &str, data: &Arc<Mutex<AppData>>) {
    let Ok(msg) = serde_json::from_str::<ServerMessage>(text) else {
        return;
    };
    let mut d = data.lock().unwrap();
    match msg {
        ServerMessage::Tick {
            tick_count,
            traces,
            tension,
            last_matched,
        } => {
            d.tick_count = tick_count;
            d.traces = traces;
            d.tension = tension;
            d.last_matched = last_matched;
        }
        ServerMessage::State {
            tick_count,
            snapshot,
        } => {
            d.tick_count = tick_count;
            d.traces = snapshot.trace_count;
            d.tension = snapshot.tension_count;
            d.domains = snapshot.domain_summaries;
        }
        ServerMessage::Result {
            domain_name,
            coherence,
            traces_matched,
            position,
            ..
        } => {
            let [x, y, z] = position;
            d.last_output = format!(
                "→ {} | coh={:.2} matched={} pos=({},{},{})",
                domain_name, coherence, traces_matched, x, y, z
            );
        }
        ServerMessage::CommandResult { output, .. } => {
            d.last_output = output;
        }
        ServerMessage::DomainDetail(snap) => {
            // Обновляем токены для этого домена в Space View
            d.tokens.retain(|(did, _)| *did != snap.domain_id);
            for tok in snap.tokens {
                d.tokens.push((snap.domain_id, tok));
            }
        }
        ServerMessage::Error { message, .. } => {
            d.last_error = Some(message);
        }
    }
}

fn mark_disconnected(data: &Arc<Mutex<AppData>>, reason: &str) {
    let mut d = data.lock().unwrap();
    d.connected = false;
    d.last_error = Some(reason.to_string());
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    #[test]
    fn test_parse_addr() {
        assert_eq!(parse_addr("ws://127.0.0.1:8765/ws"), "127.0.0.1:8765");
        assert_eq!(parse_addr("ws://localhost:9000/ws"), "localhost:9000");
    }

    #[test]
    fn test_ws_client_receives_message() {
        // Простой WS-сервер в отдельном потоке
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut ws = tungstenite::accept(stream).unwrap();
            // Отправить Tick
            let msg = r#"{"type":"tick","tick_count":77,"traces":3,"tension":1,"last_matched":2}"#;
            let _ = ws.send(Message::Text(msg.to_string()));
            // Держать соединение открытым пока клиент не отключится
            loop {
                if ws.read().is_err() {
                    break;
                }
            }
        });

        let data = Arc::new(Mutex::new(AppData::default()));
        let data_clone = Arc::clone(&data);
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let url = format!("ws://127.0.0.1:{port}/ws");

        std::thread::spawn(move || {
            run_ws_client(&url, data_clone, rx);
        });

        // Ждём обновления tick_count
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            if data.lock().unwrap().tick_count == 77 {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "timeout: tick_count never updated"
            );
            std::thread::sleep(Duration::from_millis(10));
        }

        drop(tx);
        assert!(data.lock().unwrap().connected);
    }
}
