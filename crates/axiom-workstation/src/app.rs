use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime};

use iced::widget::{column, text_editor};
use iced::{window, Element, Subscription, Task};

use axiom_protocol::{
    adapters::AdapterInfo,
    bench::BenchResults,
    commands::{EngineCommand, ImportOptions},
    config::{ConfigSchema, ConfigSection, ConfigValue},
    events::EngineEvent,
    messages::CommandResultData,
    snapshot::{DreamReport, SystemSnapshot},
};

use crate::connection::ws_subscription;
use crate::settings::{is_first_run, load_settings, save_settings, UiSettings};
use crate::ui::{
    benchmarks, config, conversation, dream_state, files, header, live_field, patterns, system_map,
    tabs, welcome,
};

// ── AppPhase ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPhase {
    Welcome,
    Main,
}

// ── AlertEntry ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AlertEntry {
    pub message: String,
    pub timestamp_secs: u64,
}

fn current_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

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

// ── ConversationState ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum SystemMessageKind {
    Acknowledgment,
    FrameCreated,
    FrameReactivated,
    Error,
}

#[derive(Debug, Clone)]
pub enum ConversationMessage {
    User {
        text: String,
        target_domain: u16,
        timestamp_secs: u64,
    },
    System {
        text: String,
        timestamp_secs: u64,
        kind: SystemMessageKind,
    },
}

#[derive(Debug)]
pub struct ConversationState {
    pub messages: Vec<ConversationMessage>,
    pub editor_content: text_editor::Content,
    pub target_domain: u16,
    pub sending: bool,
    pub last_submit_at: Option<Instant>,
    pub pending_submit_id: Option<u64>,
}

impl Default for ConversationState {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            editor_content: text_editor::Content::new(),
            target_domain: 101,
            sending: false,
            last_submit_at: None,
            pending_submit_id: None,
        }
    }
}

impl ConversationState {
    pub fn is_recent_submit(&self) -> bool {
        self.last_submit_at
            .map(|t| t.elapsed().as_secs() < 5)
            .unwrap_or(false)
    }

    pub fn push_system(&mut self, text: String, kind: SystemMessageKind) {
        self.messages.push(ConversationMessage::System {
            text,
            timestamp_secs: current_timestamp_secs(),
            kind,
        });
    }
}

// ── PatternsState ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum FrameEvent {
    Crystallized {
        anchor_id: u32,
        layers_present: u8,
        participant_count: u8,
        timestamp_secs: u64,
    },
    Reactivated {
        anchor_id: u32,
        new_temperature: u8,
        timestamp_secs: u64,
    },
    Vetoed {
        reason: String,
        timestamp_secs: u64,
    },
    Promoted {
        source_anchor_id: u32,
        sutra_anchor_id: u32,
        timestamp_secs: u64,
    },
}

#[derive(Debug)]
pub struct PatternsState {
    pub layer_history: [VecDeque<u8>; 8],
    pub recent_frames: VecDeque<FrameEvent>,
    pub show_all_frames: bool,
}

impl Default for PatternsState {
    fn default() -> Self {
        Self {
            layer_history: std::array::from_fn(|_| VecDeque::with_capacity(30)),
            recent_frames: VecDeque::with_capacity(100),
            show_all_frames: false,
        }
    }
}

impl PatternsState {
    pub fn push_layer_snapshot(&mut self, activations: [u8; 8]) {
        for (i, val) in activations.into_iter().enumerate() {
            if self.layer_history[i].len() >= 30 {
                self.layer_history[i].pop_back();
            }
            self.layer_history[i].push_front(val);
        }
    }

    pub fn push_frame_event(&mut self, ev: FrameEvent) {
        if self.recent_frames.len() >= 100 {
            self.recent_frames.pop_back();
        }
        self.recent_frames.push_front(ev);
    }
}

// ── DreamWindowState ───────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct DreamWindowState {
    pub recent_dreams: VecDeque<DreamReport>,
    pub confirm_force_sleep: bool,
    pub show_all_dreams: bool,
}

impl DreamWindowState {
    pub fn push_dream(&mut self, report: DreamReport) {
        if self.recent_dreams.len() >= 20 {
            self.recent_dreams.pop_back();
        }
        self.recent_dreams.push_front(report);
    }
}

// ── FilesState ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CompletedImport {
    pub adapter_id: String,
    pub source: String,
    pub tokens_added: u32,
    pub errors: u32,
    #[allow(dead_code)]
    pub timestamp_secs: u64,
    pub cancelled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct RunningImport {
    pub adapter_id: String,
    pub source: String,
    pub processed: u64,
    pub total: u64,
}

#[derive(Debug, Default)]
pub struct FilesState {
    pub available_adapters: Vec<AdapterInfo>,
    pub adapters_fetched: bool,
    pub source_path: String,
    pub selected_adapter_id: Option<String>,
    pub running_import: Option<RunningImport>,
    pub completed_imports: VecDeque<CompletedImport>,
    pub cancel_confirm: bool,
}

// ── BenchmarksState ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct RunningBench {
    pub bench_id: String,
    pub run_id: u64,
    pub completed: u32,
    pub total: u32,
}

#[derive(Debug, Default)]
pub struct BenchmarksState {
    pub history: VecDeque<BenchResults>,
    pub running: Option<RunningBench>,
    pub iterations_input: String,
}

// ── LiveFieldState ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OrbitCamera {
    pub azimuth: f32,
    pub elevation: f32,
    pub distance: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            azimuth: 0.3,
            elevation: 0.4,
            distance: 4.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub show_connections: bool,
    pub show_anchors: bool,
    pub layer_color_coding: bool,
    pub highlight_recent: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_connections: true,
            show_anchors: true,
            layer_color_coding: true,
            highlight_recent: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveFieldOption {
    ShowConnections,
    ShowAnchors,
    LayerColorCoding,
    HighlightRecent,
}

#[derive(Debug, Default)]
pub struct LiveFieldState {
    pub selected_domain: Option<u16>,
    pub camera: OrbitCamera,
    pub display: DisplayOptions,
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
    Reconnecting {
        attempt: u32,
        next_retry_secs: u64,
    },
    Connected {
        engine_version: u32,
        connected_at: Instant,
    },
}

// ── Message ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    // WebSocket lifecycle
    WsConnecting,
    WsConnected {
        engine_version: u32,
    },
    WsDisconnected,
    WsReconnecting {
        attempt: u32,
        next_retry_secs: u64,
    },
    WsSnapshot(SystemSnapshot),
    WsEvent(EngineEvent),
    WsCommandResult {
        command_id: u64,
        result: Result<CommandResultData, String>,
    },
    // Command plumbing
    CommandSenderReady(CommandSender),
    #[allow(dead_code)]
    SendCommand(EngineCommand),
    // Window / tab management
    TabSelected(TabKind),
    DetachTab(TabKind),
    WindowCloseRequested(window::Id),
    AnimationTick,
    ToggleViewMenu,
    // Configuration tab
    ConfigSectionSelected(String),
    ConfigFieldChanged {
        section_id: String,
        field_id: String,
        value: ConfigValue,
    },
    ConfigApply {
        section_id: String,
    },
    ConfigDiscard,
    // Conversation tab
    ConversationEditorAction(text_editor::Action),
    ConversationDomainSelected(u16),
    ConversationSubmit,
    // Dream State tab
    ForceSleepRequest,
    ForceSleepConfirm,
    ForceSleepCancel,
    ForceWakeRequest,
    // Files tab
    FilesPathChanged(String),
    FilesBrowse,
    FilesPickPath(Option<String>),
    FilesAdapterSelected(String),
    FilesStartImport,
    FilesCancelRequest,
    FilesConfirmCancel,
    FilesCancelDismiss,
    // Patterns / Dream State pagination
    PatternsShowMore,
    DreamsShowMore,
    // Benchmarks tab
    BenchIterationsChanged(String),
    BenchRun,
    // Welcome screen
    SkipToMain,
    // Connection details popup
    ToggleConnectionDetails,
    // Alert system
    DismissAlert(usize),
    // Live Field tab
    LiveFieldDomainSelected(u16),
    LiveFieldCameraRotate {
        dx: f32,
        dy: f32,
    },
    LiveFieldCameraZoom(f32),
    LiveFieldCameraReset,
    LiveFieldToggleOption(LiveFieldOption),
    // Keyboard
    ConfigApplyActive,
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
    pub conversation: ConversationState,
    pub patterns: PatternsState,
    pub dream_state: DreamWindowState,
    pub command_tx: Option<CommandSender>,
    pub next_command_id: u64,
    pub config: ConfigurationState,
    pub files: FilesState,
    pub benchmarks: BenchmarksState,
    pub live_field: LiveFieldState,
    pub phase: AppPhase,
    pub show_connection_details: bool,
    pub alerts: VecDeque<AlertEntry>,
    pub subscription_key: u64,
    /// Last activity timestamp per ASHTI domain (index = domain_id - 101, range 0-7)
    pub last_domain_active: [Option<std::time::Instant>; 8],
    pub welcome_opacity: f32,
    pub show_view_menu: bool,
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
            conversation: ConversationState::default(),
            patterns: PatternsState::default(),
            dream_state: DreamWindowState::default(),
            files: FilesState::default(),
            benchmarks: BenchmarksState::default(),
            live_field: LiveFieldState::default(),
            phase: if is_first_run() {
                AppPhase::Welcome
            } else {
                AppPhase::Main
            },
            show_connection_details: false,
            alerts: VecDeque::with_capacity(5),
            subscription_key: 0,
            last_domain_active: [None; 8],
            welcome_opacity: 0.0,
            show_view_menu: false,
        }
    }

    #[allow(dead_code)]
    pub fn push_alert(&mut self, message: String) {
        if self.alerts.len() >= 5 {
            self.alerts.pop_front();
        }
        self.alerts.push_back(AlertEntry {
            message,
            timestamp_secs: current_timestamp_secs(),
        });
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
                if self.phase == AppPhase::Welcome {
                    self.phase = AppPhase::Main;
                    save_settings(&self.settings);
                }
            }
            Message::WsDisconnected => {
                self.connection = ConnectionState::Disconnected;
                self.command_tx = None;
            }
            Message::WsReconnecting {
                attempt,
                next_retry_secs,
            } => {
                self.connection = ConnectionState::Reconnecting {
                    attempt,
                    next_retry_secs,
                };
            }
            Message::WsSnapshot(snap) => {
                tracing::debug!("Snapshot: tick={}", snap.current_tick);
                let layer_data = snap
                    .frame_weaver_stats
                    .as_ref()
                    .map(|fw| fw.syntactic_layer_activations)
                    .unwrap_or(snap.over_domain.layer_activations);
                self.patterns.push_layer_snapshot(layer_data);
                // Accumulate DreamReports
                if let Some(report) = &snap.last_dream_report {
                    let is_new = self
                        .dream_state
                        .recent_dreams
                        .front()
                        .map(|r| r.cycle_id != report.cycle_id)
                        .unwrap_or(true);
                    if is_new {
                        self.dream_state.push_dream(report.clone());
                    }
                }
                self.engine_snapshot = Some(snap);
            }
            Message::WsEvent(ev) => {
                tracing::debug!("Event: {:?}", ev);
                if self.recent_events.len() >= 1000 {
                    self.recent_events.pop_front();
                }
                // Patterns feed + conversation correlation
                match &ev {
                    EngineEvent::FrameCrystallized {
                        anchor_id,
                        layers_present,
                        participant_count,
                    } => {
                        self.patterns.push_frame_event(FrameEvent::Crystallized {
                            anchor_id: *anchor_id,
                            layers_present: *layers_present,
                            participant_count: *participant_count,
                            timestamp_secs: current_timestamp_secs(),
                        });
                        if self.conversation.is_recent_submit() {
                            self.conversation.push_system(
                                format!(
                                    "Frame #{} crystallized. Layers: {}. Participants: {}.",
                                    anchor_id, layers_present, participant_count
                                ),
                                SystemMessageKind::FrameCreated,
                            );
                        }
                    }
                    EngineEvent::FrameReactivated {
                        anchor_id,
                        new_temperature,
                    } => {
                        self.patterns.push_frame_event(FrameEvent::Reactivated {
                            anchor_id: *anchor_id,
                            new_temperature: *new_temperature,
                            timestamp_secs: current_timestamp_secs(),
                        });
                        if self.conversation.is_recent_submit() {
                            self.conversation.push_system(
                                format!(
                                    "Frame #{} reactivated. Temperature: {}.",
                                    anchor_id, new_temperature
                                ),
                                SystemMessageKind::FrameReactivated,
                            );
                        }
                    }
                    EngineEvent::FramePromoted {
                        source_anchor_id,
                        sutra_anchor_id,
                    } => {
                        self.patterns.push_frame_event(FrameEvent::Promoted {
                            source_anchor_id: *source_anchor_id,
                            sutra_anchor_id: *sutra_anchor_id,
                            timestamp_secs: current_timestamp_secs(),
                        });
                    }
                    EngineEvent::GuardianVeto { reason, .. } => {
                        self.patterns.push_frame_event(FrameEvent::Vetoed {
                            reason: reason.clone(),
                            timestamp_secs: current_timestamp_secs(),
                        });
                    }
                    EngineEvent::DomainActivity {
                        domain_id,
                        layer_activations,
                        ..
                    } => {
                        self.patterns.push_layer_snapshot(*layer_activations);
                        if *domain_id >= 101 && *domain_id <= 108 {
                            let idx = (*domain_id - 101) as usize;
                            self.last_domain_active[idx] = Some(std::time::Instant::now());
                        }
                    }
                    EngineEvent::AdapterStarted { adapter_id, source } => {
                        self.files.running_import = Some(RunningImport {
                            adapter_id: adapter_id.clone(),
                            source: source.clone(),
                            processed: 0,
                            total: 0,
                        });
                    }
                    EngineEvent::AdapterProgress {
                        adapter_id,
                        processed,
                        total,
                    } => {
                        if let Some(ref mut ri) = self.files.running_import {
                            if &ri.adapter_id == adapter_id {
                                ri.processed = *processed;
                                ri.total = *total;
                            }
                        }
                    }
                    EngineEvent::AdapterFinished {
                        adapter_id: _,
                        tokens_added,
                        errors,
                    } => {
                        if let Some(ri) = self.files.running_import.take() {
                            let imp = CompletedImport {
                                adapter_id: ri.adapter_id,
                                source: ri.source,
                                tokens_added: *tokens_added,
                                errors: *errors,
                                timestamp_secs: current_timestamp_secs(),
                                cancelled: false,
                            };
                            if self.files.completed_imports.len() >= 50 {
                                self.files.completed_imports.pop_back();
                            }
                            self.files.completed_imports.push_front(imp);
                        }
                    }
                    EngineEvent::BenchStarted { bench_id, run_id } => {
                        self.benchmarks.running = Some(RunningBench {
                            bench_id: bench_id.clone(),
                            run_id: *run_id,
                            completed: 0,
                            total: 0,
                        });
                    }
                    EngineEvent::BenchProgress {
                        run_id,
                        completed,
                        total,
                    } => {
                        if let Some(ref mut rb) = self.benchmarks.running {
                            if rb.run_id == *run_id {
                                rb.completed = *completed;
                                rb.total = *total;
                            }
                        }
                    }
                    EngineEvent::BenchFinished { run_id, results } => {
                        if let Some(rb) = self.benchmarks.running.take() {
                            if rb.run_id == *run_id {
                                if self.benchmarks.history.len() >= 20 {
                                    self.benchmarks.history.pop_back();
                                }
                                self.benchmarks.history.push_front(results.clone());
                            }
                        }
                    }
                    _ => {}
                }
                self.recent_events.push_back(ev);
            }
            Message::WsCommandResult { command_id, result } => {
                // SubmitText result check first
                if Some(command_id) == self.conversation.pending_submit_id {
                    self.conversation.pending_submit_id = None;
                    self.conversation.sending = false;
                    match result {
                        Ok(_) => {
                            self.conversation.editor_content = text_editor::Content::new();
                            self.conversation.push_system(
                                "Обработан текст.".to_string(),
                                SystemMessageKind::Acknowledgment,
                            );
                        }
                        Err(e) => {
                            self.conversation
                                .push_system(format!("Ошибка: {}", e), SystemMessageKind::Error);
                        }
                    }
                    return chat_scroll_to_bottom();
                }
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
                    Ok(CommandResultData::AdapterList(adapters)) => {
                        self.files.available_adapters = adapters;
                        self.files.adapters_fetched = true;
                    }
                    _ => {}
                }
            }
            Message::CommandSenderReady(sender) => {
                let id1 = self.next_id();
                let id2 = self.next_id();
                self.command_tx = Some(sender);
                return Task::batch([
                    self.send_command_task(id1, EngineCommand::GetConfigSchema),
                    self.send_command_task(id2, EngineCommand::ListAdapters),
                ]);
            }
            Message::SendCommand(cmd) => {
                let id = self.next_id();
                return self.send_command_task(id, cmd);
            }
            Message::TabSelected(tab) => {
                self.active_tab_in_main = tab;
                self.show_view_menu = false;
            }
            Message::ToggleViewMenu => {
                self.show_view_menu = !self.show_view_menu;
            }
            Message::DetachTab(tab) => {
                self.show_view_menu = false;
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
                self.welcome_opacity = (self.welcome_opacity + 0.04).min(1.0);
                let now = current_timestamp_secs();
                self.alerts
                    .retain(|a| now.saturating_sub(a.timestamp_secs) < 10);
            }
            Message::ConfigSectionSelected(section_id) => {
                self.config.active_section_id = section_id;
            }
            Message::ConfigFieldChanged {
                section_id,
                field_id,
                value,
            } => {
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
                        self.subscription_key += 1;
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
            Message::PatternsShowMore => {
                self.patterns.show_all_frames = true;
            }
            Message::DreamsShowMore => {
                self.dream_state.show_all_dreams = true;
            }
            Message::ConversationEditorAction(action) => {
                self.conversation.editor_content.perform(action);
            }
            Message::ConversationDomainSelected(domain) => {
                self.conversation.target_domain = domain;
            }
            Message::ForceSleepRequest => {
                self.dream_state.confirm_force_sleep = true;
            }
            Message::ForceSleepCancel => {
                self.dream_state.confirm_force_sleep = false;
            }
            Message::ForceSleepConfirm => {
                self.dream_state.confirm_force_sleep = false;
                let id = self.next_id();
                return self.send_command_task(id, EngineCommand::ForceSleep);
            }
            Message::ForceWakeRequest => {
                let id = self.next_id();
                return self.send_command_task(id, EngineCommand::ForceWake);
            }
            Message::FilesPathChanged(path) => {
                self.files.source_path = path;
            }
            Message::FilesBrowse => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_file()
                            .await
                            .map(|h| h.path().to_string_lossy().into_owned())
                    },
                    Message::FilesPickPath,
                );
            }
            Message::FilesPickPath(maybe_path) => {
                if let Some(path) = maybe_path {
                    self.files.source_path = path;
                }
            }
            Message::FilesAdapterSelected(id) => {
                self.files.selected_adapter_id = Some(id);
            }
            Message::FilesStartImport => {
                let Some(adapter_id) = self.files.selected_adapter_id.clone() else {
                    return Task::none();
                };
                if self.files.source_path.is_empty() || self.files.running_import.is_some() {
                    return Task::none();
                }
                let id = self.next_id();
                return self.send_command_task(
                    id,
                    EngineCommand::StartImport {
                        adapter_id,
                        source_path: self.files.source_path.clone(),
                        options: ImportOptions {
                            params: vec![],
                            target_domain: None,
                        },
                    },
                );
            }
            Message::FilesCancelRequest => {
                if self.files.running_import.is_some() {
                    self.files.cancel_confirm = true;
                }
            }
            Message::FilesConfirmCancel => {
                self.files.cancel_confirm = false;
                if let Some(ref ri) = self.files.running_import {
                    let import_id = ri.adapter_id.clone();
                    let id = self.next_id();
                    return self.send_command_task(id, EngineCommand::CancelImport { import_id });
                }
            }
            Message::FilesCancelDismiss => {
                self.files.cancel_confirm = false;
            }
            Message::BenchIterationsChanged(s) => {
                self.benchmarks.iterations_input = s;
            }
            Message::BenchRun => {
                let iterations = self
                    .benchmarks
                    .iterations_input
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(100)
                    .clamp(1, 10_000);
                let id = self.next_id();
                let spec = axiom_protocol::bench::BenchSpec {
                    bench_id: "engine_tick".to_string(),
                    iterations,
                    options: axiom_protocol::bench::BenchOptions::default(),
                };
                self.benchmarks.running = Some(RunningBench {
                    bench_id: spec.bench_id.clone(),
                    run_id: id,
                    completed: 0,
                    total: iterations,
                });
                return self.send_command_task(id, EngineCommand::RunBench { spec });
            }
            Message::SkipToMain => {
                self.phase = AppPhase::Main;
                save_settings(&self.settings);
            }
            Message::ToggleConnectionDetails => {
                self.show_connection_details = !self.show_connection_details;
            }
            Message::DismissAlert(idx) => {
                if idx < self.alerts.len() {
                    self.alerts.remove(idx);
                }
            }
            Message::LiveFieldDomainSelected(id) => {
                self.live_field.selected_domain = Some(id);
            }
            Message::LiveFieldCameraRotate { dx, dy } => {
                self.live_field.camera.azimuth += dx * 3.0;
                self.live_field.camera.elevation =
                    (self.live_field.camera.elevation - dy * 3.0).clamp(-1.4, 1.4);
            }
            Message::LiveFieldCameraZoom(delta) => {
                self.live_field.camera.distance =
                    (self.live_field.camera.distance + delta).clamp(1.5, 15.0);
            }
            Message::LiveFieldCameraReset => {
                self.live_field.camera = OrbitCamera::default();
            }
            Message::LiveFieldToggleOption(opt) => match opt {
                LiveFieldOption::ShowConnections => {
                    self.live_field.display.show_connections =
                        !self.live_field.display.show_connections;
                }
                LiveFieldOption::ShowAnchors => {
                    self.live_field.display.show_anchors = !self.live_field.display.show_anchors;
                }
                LiveFieldOption::LayerColorCoding => {
                    self.live_field.display.layer_color_coding =
                        !self.live_field.display.layer_color_coding;
                }
                LiveFieldOption::HighlightRecent => {
                    self.live_field.display.highlight_recent =
                        !self.live_field.display.highlight_recent;
                }
            },
            Message::ConfigApplyActive => {
                let section_id = self.config.active_section_id.clone();
                return self.update(Message::ConfigApply { section_id });
            }
            Message::ConversationSubmit => {
                let text = self.conversation.editor_content.text().trim().to_string();
                if text.is_empty() || self.conversation.sending {
                    return Task::none();
                }
                let target = self.conversation.target_domain;
                self.conversation.messages.push(ConversationMessage::User {
                    text: text.clone(),
                    target_domain: target,
                    timestamp_secs: current_timestamp_secs(),
                });
                self.conversation.sending = true;
                self.conversation.last_submit_at = Some(Instant::now());
                let id = self.next_id();
                self.conversation.pending_submit_id = Some(id);
                return Task::batch([
                    self.send_command_task(
                        id,
                        EngineCommand::SubmitText {
                            text,
                            target_domain: target,
                        },
                    ),
                    chat_scroll_to_bottom(),
                ]);
            }
        }
        Task::none()
    }

    pub fn next_available_tab(&self, excluded: TabKind) -> TabKind {
        TabKind::all()
            .into_iter()
            .find(|&t| t != excluded && !self.detached_windows.values().any(|&dt| dt == t))
            .unwrap_or(TabKind::SystemMap)
    }

    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        if Some(id) == self.main_window {
            match self.phase {
                AppPhase::Welcome => welcome::welcome_view(&self.connection, self.welcome_opacity),
                AppPhase::Main => self.main_window_view(),
            }
        } else if let Some(&tab) = self.detached_windows.get(&id) {
            self.detached_window_view(tab)
        } else {
            iced::widget::text("Loading...").into()
        }
    }

    fn main_window_view(&self) -> Element<'_, Message> {
        let detached: Vec<TabKind> = self.detached_windows.values().copied().collect();
        let base = column![
            header::header_view(
                &self.connection,
                self.show_connection_details,
                &self.settings.engine_address,
                self.show_view_menu,
                self.active_tab_in_main,
            ),
            tabs::tabs_bar(self.active_tab_in_main, &detached),
            self.tab_content_for(self.active_tab_in_main),
        ];

        if self.alerts.is_empty() {
            base.into()
        } else {
            iced::widget::stack![base, alert_overlay(&self.alerts),].into()
        }
    }

    fn detached_window_view(&self, tab: TabKind) -> Element<'_, Message> {
        column![
            header::header_view(&self.connection, false, &self.settings.engine_address, false, tab),
            self.tab_content_for(tab),
        ]
        .into()
    }

    fn tab_content_for(&self, tab: TabKind) -> Element<'_, Message> {
        match tab {
            TabKind::SystemMap => {
                system_map::system_map_view(
                    &self.engine_snapshot,
                    self.animation_phase,
                    &self.last_domain_active,
                )
            }
            TabKind::Configuration => config::config_view(&self.config, &self.settings),
            TabKind::Conversation => conversation::conversation_view(&self.conversation),
            TabKind::Patterns => patterns::patterns_view(&self.patterns),
            TabKind::DreamState => {
                dream_state::dream_state_view(&self.dream_state, &self.engine_snapshot)
            }
            TabKind::LiveField => {
                live_field::live_field_view(&self.live_field, &self.engine_snapshot)
            }
            TabKind::Files => files::files_view(&self.files),
            TabKind::Benchmarks => benchmarks::benchmarks_view(&self.benchmarks),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            ws_subscription(self.settings.engine_address.clone(), self.subscription_key),
            iced::time::every(Duration::from_millis(33)).map(|_| Message::AnimationTick),
            window::close_requests().map(Message::WindowCloseRequested),
            iced::keyboard::on_key_press(keyboard_shortcut),
        ])
    }
}

// ── Module-level helpers ───────────────────────────────────────────────────

fn chat_scroll_to_bottom() -> Task<Message> {
    use iced::widget::scrollable;
    scrollable::scroll_to(
        scrollable::Id::new("chat_feed"),
        scrollable::AbsoluteOffset {
            x: 0.0,
            y: f32::MAX,
        },
    )
}

fn alert_overlay(alerts: &VecDeque<AlertEntry>) -> Element<'_, Message> {
    use iced::widget::{button, column, container, row, text};
    use iced::{Alignment, Color, Length, Padding};

    let items: Vec<Element<Message>> = alerts
        .iter()
        .enumerate()
        .map(|(i, a)| {
            container(
                row![
                    text(&a.message).size(12).color(Color::WHITE),
                    button(text("✕").size(11))
                        .on_press(Message::DismissAlert(i))
                        .style(button::text),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(Padding {
                top: 6.0,
                right: 10.0,
                bottom: 6.0,
                left: 10.0,
            })
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(
                    0.15, 0.15, 0.15, 0.9,
                ))),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        })
        .collect();

    container(column(items).spacing(4))
        .padding(12)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_bottom(Length::Fill)
        .align_right(Length::Fill)
        .into()
}

fn keyboard_shortcut(
    key: iced::keyboard::Key,
    modifiers: iced::keyboard::Modifiers,
) -> Option<Message> {
    use iced::keyboard::Key;
    if modifiers.control() {
        match key.as_ref() {
            Key::Character("1") => Some(Message::TabSelected(TabKind::SystemMap)),
            Key::Character("2") => Some(Message::TabSelected(TabKind::LiveField)),
            Key::Character("3") => Some(Message::TabSelected(TabKind::Patterns)),
            Key::Character("4") => Some(Message::TabSelected(TabKind::DreamState)),
            Key::Character("5") => Some(Message::TabSelected(TabKind::Conversation)),
            Key::Character("6") => Some(Message::TabSelected(TabKind::Files)),
            Key::Character("7") => Some(Message::TabSelected(TabKind::Configuration)),
            Key::Character("8") => Some(Message::TabSelected(TabKind::Benchmarks)),
            Key::Character(",") => Some(Message::TabSelected(TabKind::Configuration)),
            Key::Character("s") => Some(Message::ConfigApplyActive),
            Key::Character("z") => Some(Message::ConfigDiscard),
            _ => None,
        }
    } else {
        None
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
        app.detached_windows
            .insert(window::Id::unique(), TabKind::LiveField);

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

        let pending = app
            .config
            .pending_changes
            .get("workstation.connection")
            .unwrap();
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

    // Test 6.7.a — empty input → no submit
    #[test]
    fn test_conversation_empty_no_submit() {
        let mut app = WorkstationApp::new();
        assert!(app.conversation.editor_content.text().trim().is_empty());

        let _ = app.update(Message::ConversationSubmit);

        assert!(app.conversation.messages.is_empty());
        assert!(!app.conversation.sending);
    }

    // Test 6.7.b — submit adds user message, sets sending
    #[test]
    fn test_conversation_submit_adds_message() {
        let mut app = WorkstationApp::new();
        app.conversation.editor_content =
            text_editor::Content::with_text("Кошка спит на окне.");
        let _ = app.update(Message::ConversationSubmit);

        assert_eq!(app.conversation.messages.len(), 1);
        assert!(app.conversation.sending);
        assert!(app.conversation.pending_submit_id.is_some());
        assert!(matches!(
            &app.conversation.messages[0],
            ConversationMessage::User { text, target_domain: 101, .. } if text == "Кошка спит на окне."
        ));
    }

    // Test 6.7.c — duplicate submit while sending is no-op
    #[test]
    fn test_conversation_no_double_submit() {
        let mut app = WorkstationApp::new();
        app.conversation.editor_content = text_editor::Content::with_text("text");
        let _ = app.update(Message::ConversationSubmit);
        assert!(app.conversation.sending);

        let _ = app.update(Message::ConversationSubmit);
        // Still only one user message
        assert_eq!(app.conversation.messages.len(), 1);
    }

    // Test 6.7.d — domain selector changes target domain
    #[test]
    fn test_conversation_domain_selector() {
        let mut app = WorkstationApp::new();
        assert_eq!(app.conversation.target_domain, 101);

        let _ = app.update(Message::ConversationDomainSelected(106));
        assert_eq!(app.conversation.target_domain, 106);
    }

    // Test 6.7.e — command result clears sending + adds ack
    #[test]
    fn test_conversation_ack_on_result() {
        let mut app = WorkstationApp::new();
        app.conversation.editor_content = text_editor::Content::with_text("hello");
        let _ = app.update(Message::ConversationSubmit);

        let submit_id = app.conversation.pending_submit_id.unwrap();
        assert!(app.conversation.sending);

        let _ = app.update(Message::WsCommandResult {
            command_id: submit_id,
            result: Ok(CommandResultData::None),
        });

        assert!(!app.conversation.sending);
        assert!(app.conversation.editor_content.text().trim().is_empty());
        assert_eq!(app.conversation.messages.len(), 2); // User + Ack
        assert!(matches!(
            &app.conversation.messages[1],
            ConversationMessage::System {
                kind: SystemMessageKind::Acknowledgment,
                ..
            }
        ));
    }

    // Test 6.7.f — error result adds error message
    #[test]
    fn test_conversation_error_on_result() {
        let mut app = WorkstationApp::new();
        app.conversation.editor_content = text_editor::Content::with_text("hello");
        let _ = app.update(Message::ConversationSubmit);

        let submit_id = app.conversation.pending_submit_id.unwrap();
        let _ = app.update(Message::WsCommandResult {
            command_id: submit_id,
            result: Err("domain is full".to_string()),
        });

        assert!(!app.conversation.sending);
        assert_eq!(app.conversation.messages.len(), 2); // User + Error
        assert!(matches!(
            &app.conversation.messages[1],
            ConversationMessage::System {
                kind: SystemMessageKind::Error,
                ..
            }
        ));
    }

    // Test 7.6.a — FrameCrystallized event populates patterns feed
    #[test]
    fn test_patterns_frame_event_from_ws_event() {
        let mut app = WorkstationApp::new();
        assert!(app.patterns.recent_frames.is_empty());

        let _ = app.update(Message::WsEvent(EngineEvent::FrameCrystallized {
            anchor_id: 1234,
            layers_present: 0b00100001,
            participant_count: 3,
        }));

        assert_eq!(app.patterns.recent_frames.len(), 1);
        assert!(matches!(
            &app.patterns.recent_frames[0],
            FrameEvent::Crystallized {
                anchor_id: 1234,
                participant_count: 3,
                ..
            }
        ));
    }

    // Test 7.6.b — GuardianVeto event adds to patterns feed
    #[test]
    fn test_patterns_veto_event() {
        let mut app = WorkstationApp::new();

        let _ = app.update(Message::WsEvent(EngineEvent::GuardianVeto {
            reason: "missing S1".to_string(),
            command_summary: "cmd".to_string(),
        }));

        assert_eq!(app.patterns.recent_frames.len(), 1);
        assert!(matches!(
            &app.patterns.recent_frames[0],
            FrameEvent::Vetoed { reason, .. } if reason == "missing S1"
        ));
    }

    // Test 7.6.c — force sleep request opens confirmation
    #[test]
    fn test_dream_force_sleep_confirm_flow() {
        let mut app = WorkstationApp::new();
        assert!(!app.dream_state.confirm_force_sleep);

        let _ = app.update(Message::ForceSleepRequest);
        assert!(app.dream_state.confirm_force_sleep);

        let _ = app.update(Message::ForceSleepCancel);
        assert!(!app.dream_state.confirm_force_sleep);
    }

    // Test 7.6.d — layer_history updated on DomainActivity event
    #[test]
    fn test_patterns_layer_history_from_event() {
        let mut app = WorkstationApp::new();
        assert!(app.patterns.layer_history[0].is_empty());

        let _ = app.update(Message::WsEvent(EngineEvent::DomainActivity {
            domain_id: 101,
            recent_activity: 10,
            layer_activations: [50, 100, 30, 0, 200, 10, 5, 150],
        }));

        assert_eq!(app.patterns.layer_history[0].front(), Some(&50));
        assert_eq!(app.patterns.layer_history[4].front(), Some(&200));
    }

    // Test 8.6.a — FilesPathChanged updates source_path
    #[test]
    fn test_files_path_changed() {
        let mut app = WorkstationApp::new();
        assert!(app.files.source_path.is_empty());

        let _ = app.update(Message::FilesPathChanged("/data/corpus.txt".to_string()));
        assert_eq!(app.files.source_path, "/data/corpus.txt");
    }

    // Test 8.6.b — FilesStartImport with no adapter is no-op
    #[test]
    fn test_files_start_import_no_adapter_noop() {
        let mut app = WorkstationApp::new();
        let _ = app.update(Message::FilesPathChanged("/data/f.txt".to_string()));
        let _ = app.update(Message::FilesStartImport);
        assert!(app.files.running_import.is_none());
    }

    // Test 8.6.c — AdapterStarted sets running_import
    #[test]
    fn test_adapter_started_sets_running() {
        let mut app = WorkstationApp::new();
        assert!(app.files.running_import.is_none());

        let _ = app.update(Message::WsEvent(EngineEvent::AdapterStarted {
            adapter_id: "plain_text".to_string(),
            source: "/data/corpus.txt".to_string(),
        }));

        let ri = app.files.running_import.as_ref().unwrap();
        assert_eq!(ri.adapter_id, "plain_text");
        assert_eq!(ri.source, "/data/corpus.txt");
        assert_eq!(ri.processed, 0);
    }

    // Test 8.6.d — AdapterFinished moves to completed
    #[test]
    fn test_adapter_finished_moves_to_completed() {
        let mut app = WorkstationApp::new();
        let _ = app.update(Message::WsEvent(EngineEvent::AdapterStarted {
            adapter_id: "plain_text".to_string(),
            source: "/data/corpus.txt".to_string(),
        }));
        let _ = app.update(Message::WsEvent(EngineEvent::AdapterFinished {
            adapter_id: "plain_text".to_string(),
            tokens_added: 1500,
            errors: 2,
        }));

        assert!(app.files.running_import.is_none());
        assert_eq!(app.files.completed_imports.len(), 1);
        let imp = &app.files.completed_imports[0];
        assert_eq!(imp.tokens_added, 1500);
        assert_eq!(imp.errors, 2);
        assert!(!imp.cancelled);
    }

    // Test 8.6.e — BenchStarted/Progress/Finished flow
    #[test]
    fn test_bench_lifecycle() {
        use axiom_protocol::bench::{BenchEnvironment, BenchResults};

        let mut app = WorkstationApp::new();
        assert!(app.benchmarks.running.is_none());

        let _ = app.update(Message::WsEvent(EngineEvent::BenchStarted {
            bench_id: "memory_recall".to_string(),
            run_id: 42,
        }));
        assert!(app.benchmarks.running.is_some());
        assert_eq!(app.benchmarks.running.as_ref().unwrap().run_id, 42);

        let _ = app.update(Message::WsEvent(EngineEvent::BenchProgress {
            run_id: 42,
            completed: 50,
            total: 100,
        }));
        assert_eq!(app.benchmarks.running.as_ref().unwrap().completed, 50);

        let results = BenchResults {
            bench_id: "memory_recall".to_string(),
            iterations: 100,
            median_ns: 1200.0,
            p50_ns: 1200.0,
            p99_ns: 3500.0,
            std_dev_ns: 200.0,
            environment: BenchEnvironment {
                os: "linux".to_string(),
                arch: "x86_64".to_string(),
                engine_version: 1,
            },
        };
        let _ = app.update(Message::WsEvent(EngineEvent::BenchFinished {
            run_id: 42,
            results,
        }));

        assert!(app.benchmarks.running.is_none());
        assert_eq!(app.benchmarks.history.len(), 1);
        assert_eq!(app.benchmarks.history[0].bench_id, "memory_recall");
    }

    // Test 9.5.a — SkipToMain transitions Welcome → Main
    #[test]
    fn test_skip_to_main() {
        let mut app = WorkstationApp::new();
        app.phase = AppPhase::Welcome;

        let _ = app.update(Message::SkipToMain);
        assert_eq!(app.phase, AppPhase::Main);
    }

    // Test 9.5.b — WsConnected during Welcome → Main
    #[test]
    fn test_ws_connected_transitions_welcome_to_main() {
        let mut app = WorkstationApp::new();
        app.phase = AppPhase::Welcome;

        let _ = app.update(Message::WsConnected { engine_version: 1 });
        assert_eq!(app.phase, AppPhase::Main);
    }

    // Test 9.5.c — WsConnected during Main stays Main
    #[test]
    fn test_ws_connected_main_stays_main() {
        let mut app = WorkstationApp::new();
        app.phase = AppPhase::Main;

        let _ = app.update(Message::WsConnected { engine_version: 1 });
        assert_eq!(app.phase, AppPhase::Main);
    }

    // Test 9.5.d — subscription_key increments on address change (WS5-TD-01)
    #[test]
    fn test_subscription_key_increments_on_address_change() {
        let mut app = WorkstationApp::new();
        assert_eq!(app.subscription_key, 0);

        let _ = app.update(Message::ConfigFieldChanged {
            section_id: "workstation.connection".to_string(),
            field_id: "engine_address".to_string(),
            value: ConfigValue::String("10.0.0.1:9876".to_string()),
        });
        let _ = app.update(Message::ConfigApply {
            section_id: "workstation.connection".to_string(),
        });

        assert_eq!(app.subscription_key, 1);
        assert_eq!(app.settings.engine_address, "10.0.0.1:9876");
    }

    // Test 9.5.e — ToggleConnectionDetails flips flag
    #[test]
    fn test_toggle_connection_details() {
        let mut app = WorkstationApp::new();
        assert!(!app.show_connection_details);

        let _ = app.update(Message::ToggleConnectionDetails);
        assert!(app.show_connection_details);

        let _ = app.update(Message::ToggleConnectionDetails);
        assert!(!app.show_connection_details);
    }

    // Test 9.5.f — DismissAlert removes correct entry
    #[test]
    fn test_dismiss_alert() {
        let mut app = WorkstationApp::new();
        app.push_alert("Alert 1".to_string());
        app.push_alert("Alert 2".to_string());
        assert_eq!(app.alerts.len(), 2);

        let _ = app.update(Message::DismissAlert(0));
        assert_eq!(app.alerts.len(), 1);
        assert_eq!(app.alerts[0].message, "Alert 2");
    }

    // Test 10.5.a — LiveFieldDomainSelected sets selected_domain
    #[test]
    fn test_live_field_domain_selected() {
        let mut app = WorkstationApp::new();
        assert!(app.live_field.selected_domain.is_none());

        let _ = app.update(Message::LiveFieldDomainSelected(102));
        assert_eq!(app.live_field.selected_domain, Some(102));

        let _ = app.update(Message::LiveFieldDomainSelected(105));
        assert_eq!(app.live_field.selected_domain, Some(105));
    }

    // Test 10.5.b — LiveFieldCameraRotate updates azimuth and elevation
    #[test]
    fn test_live_field_camera_rotate() {
        let mut app = WorkstationApp::new();
        let initial_az = app.live_field.camera.azimuth;
        let initial_el = app.live_field.camera.elevation;

        let _ = app.update(Message::LiveFieldCameraRotate { dx: 0.1, dy: 0.1 });

        assert!((app.live_field.camera.azimuth - (initial_az + 0.3)).abs() < 1e-5);
        assert!((app.live_field.camera.elevation - (initial_el - 0.3)).abs() < 1e-5);
    }

    // Test 10.5.c — elevation is clamped to [-1.4, 1.4]
    #[test]
    fn test_live_field_camera_elevation_clamped() {
        let mut app = WorkstationApp::new();
        let _ = app.update(Message::LiveFieldCameraRotate { dx: 0.0, dy: 10.0 });
        assert!(app.live_field.camera.elevation >= -1.4);

        let _ = app.update(Message::LiveFieldCameraRotate { dx: 0.0, dy: -10.0 });
        assert!(app.live_field.camera.elevation <= 1.4);
    }

    // Test 10.5.d — LiveFieldCameraZoom changes distance, clamped to [1.5, 15.0]
    #[test]
    fn test_live_field_camera_zoom_clamped() {
        let mut app = WorkstationApp::new();
        let _ = app.update(Message::LiveFieldCameraZoom(100.0));
        assert!(app.live_field.camera.distance <= 15.0);

        let _ = app.update(Message::LiveFieldCameraZoom(-100.0));
        assert!(app.live_field.camera.distance >= 1.5);
    }

    // Test 10.5.e — LiveFieldCameraReset restores default camera
    #[test]
    fn test_live_field_camera_reset() {
        let mut app = WorkstationApp::new();
        let _ = app.update(Message::LiveFieldCameraRotate { dx: 2.0, dy: 1.0 });
        let _ = app.update(Message::LiveFieldCameraReset);

        let cam = &app.live_field.camera;
        assert!((cam.azimuth - OrbitCamera::default().azimuth).abs() < 1e-5);
        assert!((cam.elevation - OrbitCamera::default().elevation).abs() < 1e-5);
        assert!((cam.distance - OrbitCamera::default().distance).abs() < 1e-5);
    }

    // Test 10.5.f — LiveFieldToggleOption flips display flags
    #[test]
    fn test_live_field_toggle_option() {
        let mut app = WorkstationApp::new();
        assert!(app.live_field.display.show_connections);

        let _ = app.update(Message::LiveFieldToggleOption(
            LiveFieldOption::ShowConnections,
        ));
        assert!(!app.live_field.display.show_connections);

        let _ = app.update(Message::LiveFieldToggleOption(
            LiveFieldOption::ShowConnections,
        ));
        assert!(app.live_field.display.show_connections);

        assert!(app.live_field.display.layer_color_coding);
        let _ = app.update(Message::LiveFieldToggleOption(
            LiveFieldOption::LayerColorCoding,
        ));
        assert!(!app.live_field.display.layer_color_coding);
    }

    // Test 8.6.f — AdapterList result populates available_adapters
    #[test]
    fn test_adapter_list_result() {
        let mut app = WorkstationApp::new();
        assert!(!app.files.adapters_fetched);

        let adapters = vec![AdapterInfo {
            id: "plain_text".to_string(),
            name: "Plain Text".to_string(),
            description: "Imports plain text files".to_string(),
            supported_extensions: vec!["txt".to_string()],
            options_schema: vec![],
        }];
        let _ = app.update(Message::WsCommandResult {
            command_id: 99,
            result: Ok(CommandResultData::AdapterList(adapters)),
        });

        assert!(app.files.adapters_fetched);
        assert_eq!(app.files.available_adapters.len(), 1);
        assert_eq!(app.files.available_adapters[0].id, "plain_text");
    }
}
