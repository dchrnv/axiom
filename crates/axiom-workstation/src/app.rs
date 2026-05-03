use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use iced::widget::column;
use iced::{Element, Subscription, Task, window};

use axiom_protocol::{events::EngineEvent, snapshot::SystemSnapshot};

use crate::connection::ws_subscription;
use crate::settings::{load_settings, UiSettings};
use crate::ui::{header, placeholder, system_map, tabs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabKind {
    SystemMap,
    LiveField,
    Patterns,
    DreamState,
    Conversation,
    Configuration,
    Files,
    Benchmarks,
}

impl TabKind {
    pub fn all() -> [Self; 8] {
        [
            Self::SystemMap,
            Self::LiveField,
            Self::Patterns,
            Self::DreamState,
            Self::Conversation,
            Self::Configuration,
            Self::Files,
            Self::Benchmarks,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::SystemMap => "Map",
            Self::LiveField => "Field",
            Self::Patterns => "Patterns",
            Self::DreamState => "Dream",
            Self::Conversation => "Chat",
            Self::Configuration => "Config",
            Self::Files => "Files",
            Self::Benchmarks => "Bench",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Reconnecting { attempt: u32, next_retry_secs: u64 },
    Connected { engine_version: u32, connected_at: Instant },
}

#[derive(Debug, Clone)]
pub enum Message {
    WsConnecting,
    WsConnected { engine_version: u32 },
    WsDisconnected,
    WsReconnecting { attempt: u32, next_retry_secs: u64 },
    WsSnapshot(SystemSnapshot),
    WsEvent(EngineEvent),
    TabSelected(TabKind),
    #[allow(dead_code)]
    DetachTab(TabKind),
    WindowCloseRequested(window::Id),
    AnimationTick,
}

pub struct WorkstationApp {
    pub connection: ConnectionState,
    pub engine_snapshot: Option<SystemSnapshot>,
    pub recent_events: VecDeque<EngineEvent>,
    pub settings: UiSettings,
    pub main_window: Option<window::Id>,
    pub detached_windows: HashMap<window::Id, TabKind>,
    pub active_tab_in_main: TabKind,
    pub animation_phase: f32,
}

impl WorkstationApp {
    pub fn new() -> Self {
        Self {
            connection: ConnectionState::Disconnected,
            engine_snapshot: None,
            recent_events: VecDeque::with_capacity(1000),
            settings: load_settings(),
            main_window: None,
            detached_windows: HashMap::new(),
            active_tab_in_main: TabKind::SystemMap,
            animation_phase: 0.0,
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
                tracing::debug!("Snapshot: tick={}", snap.current_tick);
                self.engine_snapshot = Some(snap);
            }
            Message::WsEvent(ev) => {
                tracing::debug!("Event: {:?}", ev);
                if self.recent_events.len() >= 1000 {
                    self.recent_events.pop_front();
                }
                self.recent_events.push_back(ev);
            }
            Message::TabSelected(tab) => {
                self.active_tab_in_main = tab;
            }
            Message::DetachTab(tab) => {
                let (id, open_task) = window::open(window::Settings {
                    size: iced::Size::new(900.0, 700.0),
                    ..Default::default()
                });
                self.detached_windows.insert(id, tab);
                if self.active_tab_in_main == tab {
                    self.active_tab_in_main = self.next_available_tab(tab);
                }
                return open_task.map(|_| Message::AnimationTick);
            }
            Message::WindowCloseRequested(id) => {
                if Some(id) == self.main_window {
                    return iced::exit();
                }
                if let Some(tab) = self.detached_windows.remove(&id) {
                    self.active_tab_in_main = tab;
                }
                return window::close(id);
            }
            Message::AnimationTick => {
                self.animation_phase = (self.animation_phase + 0.005) % 1.0;
            }
        }
        Task::none()
    }

    fn next_available_tab(&self, excluded: TabKind) -> TabKind {
        TabKind::all()
            .into_iter()
            .find(|&t| t != excluded && !self.detached_windows.values().any(|&dt| dt == t))
            .unwrap_or(TabKind::SystemMap)
    }

    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        if Some(id) == self.main_window {
            self.main_window_view()
        } else if let Some(&tab) = self.detached_windows.get(&id) {
            self.detached_window_view(tab)
        } else {
            iced::widget::text("Loading...").into()
        }
    }

    fn main_window_view(&self) -> Element<'_, Message> {
        let detached: Vec<TabKind> = self.detached_windows.values().copied().collect();
        column![
            header::header_view(&self.connection),
            tabs::tabs_bar(self.active_tab_in_main, &detached),
            self.tab_content_for(self.active_tab_in_main),
        ]
        .into()
    }

    fn detached_window_view(&self, tab: TabKind) -> Element<'_, Message> {
        column![
            header::header_view(&self.connection),
            self.tab_content_for(tab),
        ]
        .into()
    }

    fn tab_content_for(&self, tab: TabKind) -> Element<'_, Message> {
        match tab {
            TabKind::SystemMap => {
                system_map::system_map_view(&self.engine_snapshot, self.animation_phase)
            }
            _ => placeholder::placeholder_view(tab.label()),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            ws_subscription(self.settings.engine_address.clone()),
            iced::time::every(Duration::from_millis(33)).map(|_| Message::AnimationTick),
            window::close_requests().map(Message::WindowCloseRequested),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 4.6.a — tab switching
    #[test]
    fn test_tab_switching() {
        let mut app = WorkstationApp::new();
        assert_eq!(app.active_tab_in_main, TabKind::SystemMap);

        app.update(Message::TabSelected(TabKind::Patterns));
        assert_eq!(app.active_tab_in_main, TabKind::Patterns);

        app.update(Message::TabSelected(TabKind::Configuration));
        assert_eq!(app.active_tab_in_main, TabKind::Configuration);
    }

    #[test]
    fn test_animation_phase_wraps() {
        let mut app = WorkstationApp::new();
        assert_eq!(app.animation_phase, 0.0);

        for _ in 0..200 {
            app.update(Message::AnimationTick);
        }
        assert!(app.animation_phase >= 0.0);
        assert!(app.animation_phase < 1.0);
    }

    #[test]
    fn test_detach_excludes_tab_from_bar() {
        let mut app = WorkstationApp::new();
        app.detached_windows.insert(window::Id::unique(), TabKind::LiveField);

        let detached: Vec<TabKind> = app.detached_windows.values().copied().collect();
        let visible: Vec<TabKind> = TabKind::all()
            .into_iter()
            .filter(|t| !detached.contains(t))
            .collect();

        assert_eq!(visible.len(), 7);
        assert!(!visible.contains(&TabKind::LiveField));
    }

    #[test]
    fn test_next_available_tab() {
        let mut app = WorkstationApp::new();
        app.active_tab_in_main = TabKind::SystemMap;

        let next = app.next_available_tab(TabKind::SystemMap);
        assert_ne!(next, TabKind::SystemMap);
        assert_eq!(next, TabKind::LiveField);
    }
}
