// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CLI Channel — stdin/stdout транспорт для AXIOM Agent
//
// CliPerceptor читает текстовые команды из любого `BufRead` (stdin в prod,
// строка/курсор в тестах). CliEffector пишет результаты в любой `Write`.

use std::io::{BufRead, BufReader, Read, Write};
use std::collections::HashSet;
use axiom_core::Event;
use axiom_ucl::{UclCommand, UclResult, OpCode};
use axiom_runtime::{Perceptor, Effector};

/// Входящий CLI-адаптер: строки из `BufRead` → `UclCommand`.
///
/// Поддерживаемые команды:
/// - `tick` — `TickForward`
/// - `inject <domain_id>` — `InjectToken` в указанный домен
/// - `status` — `CoreReset` (no-op запрос состояния)
/// - `quit` / `exit` — сигнал завершения (возвращает `None`)
pub struct CliPerceptor<R: Read> {
    reader: BufReader<R>,
    done: bool,
}

impl<R: Read> CliPerceptor<R> {
    /// Создать перцептор из любого `Read` источника.
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            done: false,
        }
    }

    /// Был ли получен сигнал завершения (quit/exit/EOF)?
    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl CliPerceptor<std::io::Stdin> {
    /// Создать перцептор из stdin.
    pub fn new() -> Self {
        Self::from_reader(std::io::stdin())
    }
}

impl<R: Read + Send> Perceptor for CliPerceptor<R> {
    fn receive(&mut self) -> Option<UclCommand> {
        if self.done {
            return None;
        }
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => {
                // EOF
                self.done = true;
                None
            }
            Ok(_) => parse_cli_command(line.trim()),
            Err(_) => {
                self.done = true;
                None
            }
        }
    }

    fn name(&self) -> &str {
        "cli"
    }
}

/// Разобрать строку команды CLI → UclCommand.
///
/// Неизвестные команды возвращают `None` (skip).
pub fn parse_cli_command(input: &str) -> Option<UclCommand> {
    let mut parts = input.splitn(2, ' ');
    match parts.next()? {
        "tick" => Some(UclCommand::new(OpCode::TickForward, 0, 0, 0)),
        "inject" => {
            let domain_id: u32 = parts
                .next()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(100); // default: SUTRA
            Some(UclCommand::new(OpCode::InjectToken, domain_id, 100, 0))
        }
        "status" => Some(UclCommand::new(OpCode::TickForward, 0, 0, 0)),
        "quit" | "exit" => None,
        _ => None,
    }
}

/// Исходящий CLI-адаптер: события и результаты → форматированный текст в `Write`.
pub struct CliEffector<W: Write> {
    writer: W,
}

impl<W: Write> CliEffector<W> {
    /// Создать эффектор из любого `Write` стока.
    pub fn from_writer(writer: W) -> Self {
        Self { writer }
    }
}

impl CliEffector<std::io::Stdout> {
    /// Создать эффектор из stdout.
    pub fn new() -> Self {
        Self::from_writer(std::io::stdout())
    }
}

impl<W: Write + Send> Effector for CliEffector<W> {
    fn emit(&mut self, event: &Event) {
        let type_name = format_event_type(event.event_type);
        let _ = writeln!(
            self.writer,
            "[EVENT] type={} domain={} token={}",
            type_name, event.domain_id, event.target_id
        );
    }

    fn emit_result(&mut self, result: &UclResult) {
        let status = if result.is_success() { "OK" } else { "ERR" };
        let _ = writeln!(
            self.writer,
            "[RESULT] {} code={}",
            status, result.error_code
        );
    }

    fn name(&self) -> &str {
        "cli"
    }
}

fn format_event_type(et: u16) -> String {
    match et {
        0x0001 => "TokenCreate".into(),
        0x0002 => "TokenUpdate".into(),
        0x0003 => "TokenDelete".into(),
        0x3001 => "Heartbeat".into(),
        0xE001 => "ShellExec".into(),
        0xE002 => "MayaOutput".into(),
        0xF003 => "SystemShutdown".into(),
        other => format!("{:#06x}", other),
    }
}

// ─── Async CliChannel (CLI Channel V1.1) ─────────────────────────────────────

use axiom_config::{self, ConfigWatcher, AnchorSet};
use axiom_persist::{AutoSaver, PersistenceConfig};
use axiom_runtime::{AxiomEngine, TickSchedule};
use crate::perceptors::text::TextPerceptor;
use crate::effectors::message::{MessageEffector, DetailLevel};
use tokio::sync::mpsc;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::VecDeque;

// ─── PerfTracker — счётчик производительности тиков ──────────────────────────

/// Трекер производительности: замеряет время каждого тика и хранит скользящее окно.
/// Трекер производительности — публичен для тестов.
pub struct PerfTracker {
    start:           std::time::Instant,
    tick_times:      VecDeque<u64>,
    /// Пиковое время тика в наносекундах
    pub peak_ns:      u64,
    window:          usize,
    /// Общее число тиков
    pub total_ticks:  u64,
    /// Тик с пиковым временем
    pub peak_tick_id: u64,
}

impl PerfTracker {
    /// Создать трекер с указанным скользящим окном.
    pub fn new(window: usize) -> Self {
        Self {
            start:        std::time::Instant::now(),
            tick_times:   VecDeque::with_capacity(window),
            peak_ns:      0,
            window,
            total_ticks:  0,
            peak_tick_id: 0,
        }
    }

    /// Записать время одного тика.
    pub fn record(&mut self, ns: u64, tick_id: u64) {
        if self.tick_times.len() >= self.window {
            self.tick_times.pop_front();
        }
        self.tick_times.push_back(ns);
        self.total_ticks += 1;
        if ns > self.peak_ns {
            self.peak_ns      = ns;
            self.peak_tick_id = tick_id;
        }
    }

    pub(crate) fn avg_ns(&self) -> f64 {
        if self.tick_times.is_empty() { return 0.0; }
        self.tick_times.iter().sum::<u64>() as f64 / self.tick_times.len() as f64
    }

    pub(crate) fn actual_hz(&self) -> f64 {
        let elapsed = self.start.elapsed().as_secs_f64();
        if elapsed < 0.001 { return 0.0; }
        self.total_ticks as f64 / elapsed
    }

    pub(crate) fn uptime_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    /// Создать PerfTracker для тестов (window=1).
    pub fn new_for_test() -> Self { Self::new(1) }
}

pub(crate) fn fmt_ns(ns: u64) -> String {
    if ns < 1_000 {
        format!("{} ns", ns)
    } else if ns < 1_000_000 {
        format!("{:.1} µs", ns as f64 / 1_000.0)
    } else {
        format!("{:.2} ms", ns as f64 / 1_000_000.0)
    }
}

// ─── TickSchedule YAML-зеркало ────────────────────────────────────────────────

/// YAML-зеркало TickSchedule для десериализации конфига.
/// Все поля опциональны — отсутствующие берутся из `TickSchedule::default()`.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct TickScheduleConfig {
    #[serde(default)] pub adaptation_interval:    Option<u32>,
    #[serde(default)] pub horizon_gc_interval:    Option<u32>,
    #[serde(default)] pub snapshot_interval:      Option<u32>,
    #[serde(default)] pub dream_interval:         Option<u32>,
    #[serde(default)] pub tension_check_interval: Option<u32>,
    #[serde(default)] pub goal_check_interval:    Option<u32>,
    #[serde(default)] pub reconcile_interval:     Option<u32>,
    #[serde(default)] pub persist_check_interval: Option<u32>,
    /// Минимальная частота тиков, Гц (default: 60)
    #[serde(default)] pub adaptive_min_hz:   Option<u32>,
    /// Максимальная частота тиков, Гц (default: 1000)
    #[serde(default)] pub adaptive_max_hz:   Option<u32>,
    /// Шаг увеличения при триггере, Гц (default: 200)
    #[serde(default)] pub adaptive_step_up:  Option<u32>,
    /// Шаг уменьшения за cooldown-цикл, Гц (default: 20)
    #[serde(default)] pub adaptive_step_down: Option<u32>,
    /// Число idle-тиков до снижения hz (default: 50)
    #[serde(default)] pub adaptive_cooldown: Option<u32>,
}

impl TickScheduleConfig {
    /// Применить значения из конфига поверх дефолтного TickSchedule.
    /// Отсутствующие поля (None) не перезаписываются.
    pub fn apply_to(&self, s: &mut TickSchedule) {
        if let Some(v) = self.adaptation_interval    { s.adaptation_interval    = v; }
        if let Some(v) = self.horizon_gc_interval    { s.horizon_gc_interval    = v; }
        if let Some(v) = self.snapshot_interval      { s.snapshot_interval      = v; }
        if let Some(v) = self.dream_interval         { s.dream_interval         = v; }
        if let Some(v) = self.tension_check_interval { s.tension_check_interval = v; }
        if let Some(v) = self.goal_check_interval    { s.goal_check_interval    = v; }
        if let Some(v) = self.reconcile_interval     { s.reconcile_interval     = v; }
        if let Some(v) = self.persist_check_interval { s.persist_check_interval = v; }
        if let Some(v) = self.adaptive_min_hz        { s.adaptive_tick.min_hz   = v; }
        if let Some(v) = self.adaptive_max_hz        { s.adaptive_tick.max_hz   = v; }
        if let Some(v) = self.adaptive_step_up       { s.adaptive_tick.step_up  = v; }
        if let Some(v) = self.adaptive_step_down     { s.adaptive_tick.step_down = v; }
        if let Some(v) = self.adaptive_cooldown      { s.adaptive_tick.cooldown  = v; }
    }
}

// ─── CliConfigFile — YAML-структура ──────────────────────────────────────────

/// Файл конфигурации CLI Channel (axiom-cli.yaml).
///
/// Приоритет: **default → файл → CLI-флаги**.
/// Все поля опциональны — отсутствующее поле = значение по умолчанию.
///
/// Расположение файла ищется в следующем порядке:
///   1. путь из флага `--config <path>`
///   2. `./axiom-cli.yaml` (рабочая директория)
///   3. `~/.config/axiom/cli.yaml`
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct CliConfigFile {
    /// Частота тиков ядра в Гц (default: 100)
    #[serde(default)]
    pub tick_hz: Option<u32>,
    /// Подробный вывод tension traces (default: false)
    #[serde(default)]
    pub verbose: Option<bool>,
    /// Строка приглашения (default: "axiom> ")
    #[serde(default)]
    pub prompt: Option<String>,
    /// Расписание периодических задач ядра
    #[serde(default)]
    pub tick_schedule: Option<TickScheduleConfig>,
    /// Включить адаптивную частоту тиков (Axiom Sentinel V1.0, Фаза 3)
    #[serde(default)]
    pub adaptive_tick_rate: Option<bool>,
    /// Уровень детализации вывода: off / min / mid / max (default: min)
    #[serde(default)]
    pub detail_level: Option<String>,
    /// Горячая перезагрузка config/axiom.yaml во время работы (default: false)
    #[serde(default)]
    pub hot_reload: Option<bool>,
}

impl CliConfigFile {
    /// Загрузить конфиг из YAML-файла. Возвращает `None` если файл не найден.
    /// Ошибки парсинга выводятся в stderr и считаются как "файл не найден".
    pub fn load(path: &std::path::Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        match serde_yaml::from_str::<Self>(&content) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                eprintln!("[axiom-cli] config parse error in {}: {}", path.display(), e);
                None
            }
        }
    }

    /// Найти и загрузить конфиг из стандартных расположений.
    /// Если `explicit_path` задан — ищем только там.
    pub fn find_and_load(explicit_path: Option<&str>) -> Option<Self> {
        if let Some(p) = explicit_path {
            return Self::load(std::path::Path::new(p));
        }
        // 1. Рабочая директория
        let local = std::path::Path::new("axiom-cli.yaml");
        if local.exists() {
            return Self::load(local);
        }
        // 2. ~/.config/axiom/cli.yaml
        if let Some(home) = std::env::var_os("HOME") {
            let xdg = std::path::PathBuf::from(home).join(".config/axiom/cli.yaml");
            if xdg.exists() {
                return Self::load(&xdg);
            }
        }
        None
    }
}

// ─── CliConfig — рабочая конфигурация ────────────────────────────────────────

/// Рабочая конфигурация CliChannel.
/// Собирается из трёх источников: default → YAML-файл → CLI-флаги.
pub struct CliConfig {
    /// Частота тиков ядра в Гц (default: 100)
    pub tick_hz: u32,
    /// Подробный вывод состояния ядра после каждого тика (default: false)
    pub verbose: bool,
    /// Строка приглашения (default: "axiom> ")
    pub prompt: String,
    /// TickSchedule для применения к Engine при старте
    pub tick_schedule: TickSchedule,
    /// Директория хранилища (default: "./axiom-data"). Используется в :save/:load
    pub data_dir: String,
    /// Включить адаптивную частоту тиков (Axiom Sentinel V1.0, Фаза 3)
    pub adaptive_tick_rate: bool,
    /// Уровень детализации вывода при текстовом вводе (default: Min)
    pub detail_level: DetailLevel,
    /// Горячая перезагрузка config/axiom.yaml во время работы (default: false)
    pub hot_reload: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            tick_hz: 100,
            verbose: false,
            prompt: "axiom> ".to_string(),
            tick_schedule: TickSchedule::default(),
            data_dir: "axiom-data".to_string(),
            adaptive_tick_rate: false,
            detail_level: DetailLevel::Min,
            hot_reload: false,
        }
    }
}

impl CliConfig {
    /// Построить конфиг из трёх источников: default → YAML-файл → CLI-флаги.
    ///
    /// Поиск конфига: `--config <path>` → `./axiom-cli.yaml` → `~/.config/axiom/cli.yaml`
    pub fn from_args_or_default() -> Self {
        let args: Vec<String> = std::env::args().collect();

        // Извлечь --config path до основного парсинга
        let explicit_config = args.windows(2)
            .find(|w| w[0] == "--config")
            .map(|w| w[1].as_str());

        // Слой 1: defaults
        let mut config = Self::default();

        // Слой 2: YAML-файл
        if let Some(file) = CliConfigFile::find_and_load(explicit_config) {
            if let Some(v) = file.tick_hz           { config.tick_hz           = v; }
            if let Some(v) = file.verbose           { config.verbose           = v; }
            if let Some(v) = file.prompt            { config.prompt            = v; }
            if let Some(v) = file.adaptive_tick_rate { config.adaptive_tick_rate = v; }
            if let Some(v) = file.hot_reload        { config.hot_reload         = v; }
            if let Some(ref s) = file.detail_level {
                if let Some(d) = DetailLevel::from_str(s) {
                    config.detail_level = d;
                }
            }
            if let Some(s) = file.tick_schedule {
                s.apply_to(&mut config.tick_schedule);
            }
        }

        // Слой 3: CLI-флаги (перекрывают файл)
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--tick-hz" => {
                    i += 1;
                    if let Some(val) = args.get(i) {
                        config.tick_hz = val.parse().unwrap_or(config.tick_hz);
                    }
                }
                "--verbose" | "-v" => config.verbose = true,
                "--adaptive"       => config.adaptive_tick_rate = true,
                "--hot-reload"     => config.hot_reload = true,
                "--detail" => {
                    i += 1;
                    if let Some(val) = args.get(i) {
                        if let Some(d) = DetailLevel::from_str(val) {
                            config.detail_level = d;
                        }
                    }
                }
                "--config"         => { i += 1; } // уже обработано выше
                "--data-dir" => {
                    i += 1;
                    if let Some(val) = args.get(i) {
                        config.data_dir = val.clone();
                    }
                }
                "--no-load" => {} // обрабатывается в bin/axiom-cli.rs
                "--help" | "-h" => {
                    eprintln!("{}", USAGE);
                    std::process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }
        config
    }
}

const USAGE: &str = "\
Usage: axiom-cli [OPTIONS]

Options:
  --tick-hz N       Tick frequency in Hz (default: 100)
  --verbose, -v     Show internal state (traces, tension) after each input
  --adaptive        Enable adaptive tick rate (Sentinel V1.0 Phase 3)
  --detail <level>  Output detail: off|min|mid|max (default: min)
  --config <path>   Path to axiom-cli.yaml (default: ./axiom-cli.yaml)
  --data-dir <path> Data directory for save/load (default: ./axiom-data)
  --no-load         Skip loading from data directory on startup
  --hot-reload      Watch config/axiom.yaml and reload tick_schedule on change
  --help, -h        Show this message";

/// Максимальный размер лога событий
const EVENT_LOG_CAPACITY: usize = 256;

/// Асинхронный CLI интерфейс к ядру AXIOM.
///
/// Два независимых потока выполнения:
///   1. stdin reader — читает строки и отправляет через mpsc
///   2. tick loop    — тикает ядро по интервалу, принимает ввод из mpsc
///
/// Ядро живёт в tick loop (одном потоке). Все обращения к Engine — через
/// tick loop. tokio не попадает в ядро.
pub struct CliChannel {
    engine:       AxiomEngine,
    perceptor:    TextPerceptor,
    effector:     MessageEffector,
    config:       CliConfig,
    auto_saver:   AutoSaver,
    perf:         PerfTracker,
    /// Кольцевой буфер последних событий COM
    event_log:    VecDeque<Event>,
    /// Активные watch-поля: traces | tension | tps
    watch_fields: HashSet<String>,
    /// Последнее число traces (для :watch traces)
    last_traces:  usize,
    /// Последнее число tension (для :watch tension)
    last_tension: usize,
    /// Тик последнего вывода tps (для :watch tps)
    last_tps_tick: u64,
    /// Счётчик multi-pass событий с момента запуска
    multipass_count: u64,
    /// Последний multi-pass результат (число проходов)
    last_multipass_n: u8,
    /// Горячая перезагрузка config/axiom.yaml (None если hot_reload выключен)
    config_watcher: Option<ConfigWatcher>,
    /// Набор якорных токенов (None если файлы не найдены)
    anchor_set: Option<Arc<AnchorSet>>,
}

impl CliChannel {
    /// Создать новый CliChannel.
    /// TickSchedule из конфига применяется к Engine при создании.
    pub fn new(mut engine: AxiomEngine, config: CliConfig) -> Self {
        engine.tick_schedule = config.tick_schedule;
        let persist_interval = engine.tick_schedule.persist_check_interval;
        let auto_cfg = PersistenceConfig::new(persist_interval);

        let config_watcher = if config.hot_reload {
            match ConfigWatcher::new("config/axiom.yaml") {
                Ok(w) => {
                    eprintln!("[config] hot-reload enabled (watching config/axiom.yaml)");
                    Some(w)
                }
                Err(e) => {
                    eprintln!("[config] hot-reload init failed: {e}");
                    None
                }
            }
        } else {
            None
        };

        // Загружаем якоря из config/anchors/ и инжектируем в движок
        let anchor_set = {
            let set = AnchorSet::load_or_empty(std::path::Path::new("config"));
            if set.is_empty() {
                None
            } else {
                let injected = engine.inject_anchor_tokens(&set);
                eprintln!("[anchors] loaded {} anchors (axes={}, layers={}, domains={}) — injected {} tokens",
                    set.total_count(),
                    set.axes.len(),
                    set.layers.iter().map(|l| l.len()).sum::<usize>(),
                    set.domains.iter().map(|d| d.len()).sum::<usize>(),
                    injected,
                );
                Some(Arc::new(set))
            }
        };

        let perceptor = match &anchor_set {
            Some(a) => TextPerceptor::with_anchors(Arc::clone(a)),
            None    => TextPerceptor::new(),
        };

        Self {
            engine,
            perceptor,
            effector:     MessageEffector::new(),
            auto_saver:   AutoSaver::new(auto_cfg),
            perf:         PerfTracker::new(200),
            event_log:    VecDeque::with_capacity(EVENT_LOG_CAPACITY),
            watch_fields: HashSet::new(),
            last_traces:  0,
            last_tension: 0,
            last_tps_tick: 0,
            multipass_count: 0,
            last_multipass_n: 0,
            config_watcher,
            anchor_set,
            config,
        }
    }

    /// Запустить интерактивный цикл (блокирует до :quit или EOF).
    ///
    /// Phase 0C: thin wrapper над tick_loop. stdin → AdapterCommand → tick_loop.
    /// broadcast_rx → stdout для CLI-вывода.
    ///
    /// EA-TD-03: watch_fields, event_log, verbose, hot_reload, adaptive_tick_rate
    /// временно не перенесены в tick_loop — см. DEFERRED.md.
    pub async fn run(&mut self) {
        use tokio::sync::broadcast;
        use crate::adapter_command::{AdapterCommand, AdapterSource, AdapterPayload};
        use crate::adapters_config::AdaptersConfig;
        use crate::tick_loop::tick_loop;
        use crate::protocol::ServerMessage;
        use axiom_persist::PersistenceConfig;

        let (command_tx, command_rx) = mpsc::channel::<AdapterCommand>(256);
        let (broadcast_tx, mut broadcast_rx) = broadcast::channel::<ServerMessage>(1024);
        let snapshot = Arc::new(tokio::sync::RwLock::new(
            axiom_runtime::BroadcastSnapshot::default()
        ));

        // stdin reader → parse → AdapterCommand
        let tx = command_tx.clone();
        tokio::spawn(async move {
            use tokio::io::{AsyncBufReadExt, BufReader};
            let stdin = tokio::io::stdin();
            let mut reader = BufReader::new(stdin);
            let mut line = String::new();
            let mut seq: u64 = 0;
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let trimmed = line.trim().to_string();
                        if trimmed.is_empty() { continue; }
                        seq += 1;
                        let id = seq.to_string();
                        let payload = if trimmed.starts_with(':') {
                            let cmd = trimmed.splitn(2, ' ').next().unwrap_or("");
                            let is_mutating = matches!(cmd,
                                ":save"|":load"|":autosave"|":tick"|
                                ":export"|":import"|":quit"|":q"
                            );
                            if is_mutating {
                                AdapterPayload::MetaMutate { cmd: trimmed }
                            } else {
                                AdapterPayload::MetaRead { cmd: trimmed }
                            }
                        } else {
                            AdapterPayload::Inject { text: trimmed }
                        };
                        let cmd = AdapterCommand { id, source: AdapterSource::Cli, payload };
                        if tx.send(cmd).await.is_err() { break; }
                    }
                }
            }
        });

        // broadcast_rx → stdout (CLI-подписчик)
        let detail = self.config.detail_level;
        tokio::spawn(async move {
            loop {
                match broadcast_rx.recv().await {
                    Ok(ServerMessage::CommandResult { output, .. }) => {
                        print!("{}", output);
                    }
                    Ok(ServerMessage::Result {
                        domain_name, coherence, traces_matched, position, path, reflex_hit, ..
                    }) => {
                        let [x, y, z] = position;
                        let reflex = if reflex_hit { " ⚡reflex" } else { "" };
                        println!("  [{}{}] → {} | coh={:.2} matched={} pos=({},{},{})",
                            path, reflex, domain_name, coherence, traces_matched, x, y, z);
                        let _ = detail; // используется для будущего полного форматирования
                    }
                    Ok(ServerMessage::Error { message, .. }) => {
                        eprintln!("  error: {}", message);
                    }
                    Ok(_) => {}  // Tick, State — не нужны в CLI stdout
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        eprintln!("[broadcast] lagged by {} messages", n);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });

        // Переносим владение engine и auto_saver в tick_loop
        let auto_saver = std::mem::replace(
            &mut self.auto_saver,
            AutoSaver::new(PersistenceConfig::disabled()),
        );
        let engine = std::mem::replace(&mut self.engine, AxiomEngine::new());
        let adapters_config = AdaptersConfig::from_cli_config(&self.config);

        tick_loop(
            engine, command_rx, broadcast_tx, snapshot,
            auto_saver, self.anchor_set.clone(), adapters_config,
        ).await;
    }

    // ── удалён старый run() (монолитный tick loop) — Phase 0C ────────────────
    // handle_meta_command оставлен для возможного использования в тестах и
    // будущих CLI-только режимах (non-tick_loop path).
    #[allow(dead_code)]
    fn handle_meta_command(&mut self, line: &str) -> bool {
        use crate::meta_commands::{handle_meta_read, handle_meta_mutate, MetaAction};
        let cmd = line.splitn(2, ' ').next().unwrap_or("");

        let is_mutating = matches!(cmd,
            ":save" | ":load" | ":autosave" | ":tick" |
            ":export" | ":import" | ":quit" | ":q"
        );

        if is_mutating {
            let result = handle_meta_mutate(line, &mut self.engine, &mut self.auto_saver, &self.config);
            print!("{}", result.output);
            match result.action {
                MetaAction::Quit => return false,
                MetaAction::EngineReplaced => {
                    self.engine.tick_schedule = self.config.tick_schedule;
                    self.perceptor = match &self.anchor_set {
                        Some(a) => TextPerceptor::with_anchors(std::sync::Arc::clone(a)),
                        None    => TextPerceptor::new(),
                    };
                    self.last_traces  = 0;
                    self.last_tension = 0;
                    self.multipass_count = 0;
                    self.auto_saver.reset_save_tick(self.engine.tick_count);
                }
                MetaAction::AutosaveChanged(n) => {
                    self.engine.tick_schedule.persist_check_interval = n;
                }
                MetaAction::None => {}
            }
        } else {
            let output = handle_meta_read(
                line, &self.engine, self.anchor_set.as_deref(), &self.config,
                &self.watch_fields, &self.event_log, &self.perf,
                self.multipass_count, self.last_multipass_n,
            );
            print!("{}", output);

            // CLI-state mutations — не требуют &mut Engine
            match cmd {
                ":watch" => {
                    let arg = line.splitn(3, ' ').nth(1).unwrap_or("");
                    match arg {
                        "off" => self.watch_fields.clear(),
                        field if !field.is_empty() => {
                            let exp = self.engine.ashti.experience();
                            self.last_traces   = exp.traces().len();
                            self.last_tension  = exp.tension_count();
                            self.last_tps_tick = self.engine.tick_count;
                            self.watch_fields.insert(field.to_string());
                        }
                        _ => {}
                    }
                }
                ":unwatch" => {
                    if let Some(field) = line.splitn(3, ' ').nth(1) {
                        self.watch_fields.remove(field);
                    }
                }
                ":verbose" => {
                    match line.splitn(3, ' ').nth(1) {
                        Some("on")  => self.config.verbose = true,
                        Some("off") => self.config.verbose = false,
                        _ => {}
                    }
                }
                ":detail" => {
                    if let Some(level) = line.splitn(3, ' ').nth(1) {
                        if let Some(d) = DetailLevel::from_str(level) {
                            self.config.detail_level = d;
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }
}
