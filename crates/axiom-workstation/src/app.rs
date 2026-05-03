use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use iced::widget::column;
use iced::{Element, Subscription, Task, window};

use axiom_protocol::{
    commands::EngineCommand,
    config::{ConfigSchema, ConfigSection, ConfigValue},
    events::EngineEvent,
    messages::CommandResultData,
    snapshot::SystemSnapshot,
};

use crate::connection::ws_subscription;
use crate::settings::{load_settings, save_settings, UiSettings};
use crate::ui::{config, header, placeholder, system_map, tabs};

// ── CommandSender ──────────────────────────────────────────────────────────

pub struct CommandSender(pub iced::futures::channel::mpsc::Sender<(u64, EngineCommand)>);

impl std::fmt::Debug for CommandSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CommandSender")
    }
}

impl Clone for CommandSender {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// ── ConfigurationState ─────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct ConfigurationState {
    pub engine_schema: Option<ConfigSchema>,
    pub sections: Vec<ConfigSection>,
    pub active_section_id: String,
    pub pending_changes: HashMap<String, HashMap<String, ConfigValue>>,
    pub validation_errors: HashMap<String, String>,
}

// ── TabKind ────────────────────────────────────────────────────────────────

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

// ── ConnectionState ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Reconnecting { attempt: u32, next_retry_secs: u64 },
    Connected { engine_version: u32, connected_at: Instant },
}

// ── Message ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    // WebSocket lifecycle
    WsConnecting,
    WsConnected { engine_version: u32 },
    WsDisconnected,
    WsReconnecting { attempt: u32, next_retry_secs: u64 },
    WsSnapshot(SystemSnapshot),
    WsEvent(EngineEvent),
    WsCommandResult { command_id: u64, result: Result<CommandResultData, String> },
    // Command plumbing
    CommandSenderReady(CommandSender),
    #[allow(dead_code)]
    SendCommand(EngineCommand),
    // Window / tab management
    TabSelected(TabKind),
    #[allow(dead_code)]
    DetachTab(TabKind),
    WindowCloseRequested(window::Id),
    AnimationTick,
    // Configuration tab
    ConfigSectionSelected(String),
    ConfigFieldChanged { section_id: String, field_id: String, value: ConfigValue },
    ConfigApply { section_id: String },
    ConfigDiscard,
}

// ── WorkstationApp ─────────────────────────────────────────────────────────

pub struct WorkstationApp {
    pub connection: ConnectionState,
    pub engine_snapshot: Option<SystemSnapshot>,
    pub recent_events: VecDeque<EngineEvent>,
    pub settings: UiSettings,
    pub main_window: Option<window::Id>,
    pub detached_windows: HashMap<window::Id, TabKind>,
    pub active_tab_in_main: TabKind,
    pub animation_phase: f32,
    pub command_tx: Option<CommandSender>,
    pub next_command_id: u64,
    pub config: ConfigurationState,
}

impl WorkstationApp {
    pub fn new() -> Self {
        let settings = load_settings();
        let ws_section = config::build_workstation_section(&settings);
        let sections = vec![ws_section];
        Self {
            connection: ConnectionState::Disconnected,
            engine_snapshot: None,
            recent_events: VecDeque::with_capacity(1000),
            settings,
            main_window: None,
            detached_windows: HashMap::new(),
            active_tab_in_main: TabKind::SystemMap,
            animation_phase: 0.0,
            command_tx: None,
            next_command_id: 1,
            config: ConfigurationState {
                sections,
                active_section_id: "workstation.connection".to_string(),
                ..Default::default()
            },
        }
    }

    fn rebuild_sections(&mut self) {
        let ws_section = config::build_workstation_section(&self.settings);
        let mut sections = vec![ws_section];
        if let Some(schema) = &self.config.engine_schema {
            sections.extend(schema.sections.iter().cloned());
        }
        self.config.sections = sections;
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_command_id;
        self.next_command_id += 1;
        id
    }

    fn send_command_task(&self, id: u64, cmd: EngineCommand) -> Task<Message> {
        let Some(sender) = self.command_tx.clone() else {
            return Task::none();
        };
        Task::future(async move {
            let mut tx = sender.0;
            tx.try_send((id, cmd)).ok();
            Message::AnimationTick
        })
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
                self.command_tx = None;
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
            Message::WsCommandResult { command_id, result } => {
                tracing::debug!("CommandResult id={}: {:?}", command_id, result);
                match result {
                    Ok(CommandResultData::ConfigSchema(schema)) => {
                        self.config.engine_schema = Some(schema);
                        self.rebuild_sections();
                    }
                    Ok(CommandResultData::ConfigUpdateApplied { .. }) => {}
                    Ok(CommandResultData::ConfigValidationError { field_id, message }) => {
                        self.config.validation_errors.insert(field_id, message);
                    }
                    _ => {}
                }
            }
            Message::CommandSenderReady(sender) => {
                let id = self.next_id();
                self.command_tx = Some(sender);
                return self.send_command_task(id, EngineCommand::GetConfigSchema);
            }
            Message::SendCommand(cmd) => {
                let id = self.next_id();
                return self.send_command_task(id, cmd);
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
            Message::ConfigSectionSelected(section_id) => {
                self.config.active_section_id = section_id;
            }
            Message::ConfigFieldChanged { section_id, field_id, value } => {
                self.config.validation_errors.remove(&field_id);
                self.config
                    .pending_changes
                    .entry(section_id)
                    .or_default()
                    .insert(field_id, value);
            }
            Message::ConfigApply { section_id } => {
                let Some(changes) = self.config.pending_changes.remove(&section_id) else {
                    return Task::none();
                };

                if section_id == "workstation.connection" {
                    if let Some(ConfigValue::String(addr)) = changes.get("engine_address") {
                        self.settings.engine_address = addr.clone();
                        save_settings(&self.settings);
                        self.rebuild_sections();
                    }
                    return Task::none();
                }

                // Engine section: send UpdateConfigField per changed field
                let tasks: Vec<Task<Message>> = changes
                    .into_iter()
                    .map(|(field_id, value)| {
                        let id = self.next_id();
                        self.send_command_task(
                            id,
                            EngineCommand::UpdateConfigField {
                                section_id: section_id.clone(),
                                field_id,
                                value,
                            },
                        )
                    })
                    .collect();
                return Task::batch(tasks);
            }
            Message::ConfigDiscard => {
                self.config
                    .pending_changes
                    .remove(&self.config.active_section_id);
                self.config.validation_errors.clear();
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
            TabKind::Configuration => {
                config::config_view(&self.config, &self.settings)
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

        let _ = app.update(Message::TabSelected(TabKind::Patterns));
        assert_eq!(app.active_tab_in_main, TabKind::Patterns);

        let _ = app.update(Message::TabSelected(TabKind::Configuration));
        assert_eq!(app.active_tab_in_main, TabKind::Configuration);
    }

    #[test]
    fn test_animation_phase_wraps() {
        let mut app = WorkstationApp::new();
        assert_eq!(app.animation_phase, 0.0);

        for _ in 0..200 {
            let _ = app.update(Message::AnimationTick);
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

    // Test 5.7.a — field change marks pending
    #[test]
    fn test_field_change_marks_pending() {
        let mut app = WorkstationApp::new();
        assert!(app.config.pending_changes.is_empty());

        let _ = app.update(Message::ConfigFieldChanged {
            section_id: "workstation.connection".to_string(),
            field_id: "engine_address".to_string(),
            value: ConfigValue::String("192.168.1.1:9876".to_string()),
        });

        let pending = app.config.pending_changes.get("workstation.connection").unwrap();
        assert!(matches!(
            pending.get("engine_address"),
            Some(ConfigValue::String(s)) if s == "192.168.1.1:9876"
        ));
    }

    // Test 5.7.b — discard clears pending for active section
    #[test]
    fn test_discard_clears_pending() {
        let mut app = WorkstationApp::new();
        app.config.active_section_id = "workstation.connection".to_string();
        let _ = app.update(Message::ConfigFieldChanged {
            section_id: "workstation.connection".to_string(),
            field_id: "engine_address".to_string(),
            value: ConfigValue::String("1.2.3.4:9876".to_string()),
        });
        assert!(!app.config.pending_changes.is_empty());

        let _ = app.update(Message::ConfigDiscard);
        assert!(app.config.pending_changes.is_empty());
    }

    // Test 5.7.c — apply workstation section updates settings
    #[test]
    fn test_apply_workstation_updates_settings() {
        let mut app = WorkstationApp::new();
        let _ = app.update(Message::ConfigFieldChanged {
            section_id: "workstation.connection".to_string(),
            field_id: "engine_address".to_string(),
            value: ConfigValue::String("10.0.0.1:1234".to_string()),
        });

        let _ = app.update(Message::ConfigApply {
            section_id: "workstation.connection".to_string(),
        });

        assert_eq!(app.settings.engine_address, "10.0.0.1:1234");
        assert!(app.config.pending_changes.is_empty());
    }

    // Test 5.7.d — section navigation updates active_section_id
    #[test]
    fn test_section_navigation() {
        let mut app = WorkstationApp::new();
        assert_eq!(app.config.active_section_id, "workstation.connection");

        let _ = app.update(Message::ConfigSectionSelected("engine.core".to_string()));
        assert_eq!(app.config.active_section_id, "engine.core");
    }
}
