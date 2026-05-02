use std::collections::VecDeque;
use std::time::Instant;

use iced::widget::{column, text};
use iced::{Element, Subscription, Task};

use axiom_protocol::{events::EngineEvent, snapshot::SystemSnapshot};

use crate::connection::ws_subscription;
use crate::settings::{load_settings, UiSettings};

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Reconnecting { attempt: u32, next_retry_secs: u64 },
    Connected { engine_version: u32, connected_at: Instant },
}

#[derive(Debug)]
pub enum Message {
    WsConnecting,
    WsConnected { engine_version: u32 },
    WsDisconnected,
    WsReconnecting { attempt: u32, next_retry_secs: u64 },
    WsSnapshot(SystemSnapshot),
    WsEvent(EngineEvent),
}

pub struct WorkstationApp {
    pub connection: ConnectionState,
    pub engine_snapshot: Option<SystemSnapshot>,
    pub recent_events: VecDeque<EngineEvent>,
    pub settings: UiSettings,
}

impl WorkstationApp {
    pub fn new() -> Self {
        Self {
            connection: ConnectionState::Disconnected,
            engine_snapshot: None,
            recent_events: VecDeque::with_capacity(1000),
            settings: load_settings(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WsConnecting => {
                self.connection = ConnectionState::Connecting;
            }
            Message::WsConnected { engine_version } => {
                tracing::info!("Connected to Engine v{:#010x}", engine_version);
                self.connection = ConnectionState::Connected {
                    engine_version,
                    connected_at: Instant::now(),
                };
            }
            Message::WsDisconnected => {
                self.connection = ConnectionState::Disconnected;
            }
            Message::WsReconnecting { attempt, next_retry_secs } => {
                self.connection = ConnectionState::Reconnecting { attempt, next_retry_secs };
            }
            Message::WsSnapshot(snap) => {
                tracing::debug!("Snapshot received: tick={}", snap.current_tick);
                self.engine_snapshot = Some(snap);
            }
            Message::WsEvent(ev) => {
                tracing::debug!("Event: {:?}", ev);
                if self.recent_events.len() >= 1000 {
                    self.recent_events.pop_front();
                }
                self.recent_events.push_back(ev);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let conn_line = match &self.connection {
            ConnectionState::Disconnected => "● Disconnected".to_string(),
            ConnectionState::Connecting   => "● Connecting...".to_string(),
            ConnectionState::Reconnecting { attempt, next_retry_secs } => {
                format!("● Reconnecting  (attempt {}, retry in {}s)", attempt, next_retry_secs)
            }
            ConnectionState::Connected { engine_version, connected_at } => {
                format!(
                    "● Connected  v{:#010x}  ({}s)",
                    engine_version,
                    connected_at.elapsed().as_secs()
                )
            }
        };

        let tick_line = match &self.engine_snapshot {
            Some(snap) => format!("tick {}  |  {} events buffered", snap.current_tick, self.recent_events.len()),
            None       => format!("{} events buffered", self.recent_events.len()),
        };

        column![
            text("AXIOM Workstation").size(24),
            text(conn_line),
            text(tick_line),
        ]
        .padding(20)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        ws_subscription(self.settings.engine_address.clone())
    }
}
