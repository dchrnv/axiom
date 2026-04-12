// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CLI Channel — stdin/stdout транспорт для AXIOM Agent
//
// CliPerceptor читает текстовые команды из любого `BufRead` (stdin в prod,
// строка/курсор в тестах). CliEffector пишет результаты в любой `Write`.

use std::io::{BufRead, BufReader, Read, Write};
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

use axiom_persist::{
    save as persist_save, load as persist_load, WriteOptions, AutoSaver, PersistenceConfig,
    export_traces, export_skills, import_traces, import_skills,
};
use axiom_runtime::{AxiomEngine, TickSchedule, AdaptiveTickRate, TickRateReason, ProcessingPath};
use crate::perceptors::text::TextPerceptor;
use crate::effectors::message::{MessageEffector, DetailLevel, domain_name};
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// ─── PerfTracker — счётчик производительности тиков ──────────────────────────

/// Трекер производительности: замеряет время каждого тика и хранит скользящее окно.
struct PerfTracker {
    start:        std::time::Instant,
    tick_times:   VecDeque<u64>,   // последние N времён тиков в наносекундах
    peak_ns:      u64,
    window:       usize,
    total_ticks:  u64,
    peak_tick_id: u64,
}

impl PerfTracker {
    fn new(window: usize) -> Self {
        Self {
            start:        std::time::Instant::now(),
            tick_times:   VecDeque::with_capacity(window),
            peak_ns:      0,
            window,
            total_ticks:  0,
            peak_tick_id: 0,
        }
    }

    fn record(&mut self, ns: u64, tick_id: u64) {
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

    fn avg_ns(&self) -> f64 {
        if self.tick_times.is_empty() { return 0.0; }
        self.tick_times.iter().sum::<u64>() as f64 / self.tick_times.len() as f64
    }

    fn actual_hz(&self) -> f64 {
        let elapsed = self.start.elapsed().as_secs_f64();
        if elapsed < 0.001 { return 0.0; }
        self.total_ticks as f64 / elapsed
    }

    fn uptime_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

fn fmt_ns(ns: u64) -> String {
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
  --help, -h        Show this message";

/// Минимальное число активных tension traces для повышения hz тика.
const ADAPTIVE_TENSION_THRESHOLD: usize = 3;

/// Асинхронный CLI интерфейс к ядру AXIOM.
///
/// Два независимых потока выполнения:
///   1. stdin reader — читает строки и отправляет через mpsc
///   2. tick loop    — тикает ядро по интервалу, принимает ввод из mpsc
///
/// Ядро живёт в tick loop (одном потоке). Все обращения к Engine — через
/// tick loop. tokio не попадает в ядро.
pub struct CliChannel {
    engine:     AxiomEngine,
    perceptor:  TextPerceptor,
    effector:   MessageEffector,
    config:     CliConfig,
    auto_saver: AutoSaver,
    perf:       PerfTracker,
}

impl CliChannel {
    /// Создать новый CliChannel.
    /// TickSchedule из конфига применяется к Engine при создании.
    pub fn new(mut engine: AxiomEngine, config: CliConfig) -> Self {
        engine.tick_schedule = config.tick_schedule;
        let persist_interval = engine.tick_schedule.persist_check_interval;
        let auto_cfg = PersistenceConfig::new(persist_interval);
        Self {
            engine,
            perceptor:  TextPerceptor::new(),
            effector:   MessageEffector::new(),
            auto_saver: AutoSaver::new(auto_cfg),
            perf:       PerfTracker::new(200),
            config,
        }
    }

    /// Запустить интерактивный цикл (блокирует до :quit или EOF).
    pub async fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel::<String>(64);

        // stdin reader task
        tokio::spawn(async move {
            use tokio::io::{AsyncBufReadExt, BufReader};
            let stdin = tokio::io::stdin();
            let mut reader = BufReader::new(stdin);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) | Err(_) => break, // EOF или ошибка
                    Ok(_) => {
                        let trimmed = line.trim().to_string();
                        if tx.send(trimmed).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        // При адаптивном режиме начальный hz берётся из adaptive_tick.current_hz,
        // иначе — из tick_hz конфига.
        let tick_ms = 1000u64 / self.config.tick_hz.max(1) as u64;
        let mut current_interval_ms = tick_ms;
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_millis(tick_ms)
        );
        let tick_cmd = axiom_ucl::UclCommand::new(axiom_ucl::OpCode::TickForward, 0, 100, 0);

        loop {
            interval.tick().await;

            // Флаги состояния итерации для адаптивного тика
            let mut had_input    = false;
            let mut had_multipass = false;

            // Обработать все сообщения из stdin (non-blocking)
            loop {
                match rx.try_recv() {
                    Ok(line) if line.is_empty() => {}

                    Ok(line) if line.starts_with(':') => {
                        had_input = true;
                        if !self.handle_meta_command(&line) {
                            return; // :quit
                        }
                    }

                    Ok(line) => {
                        // Пользовательский ввод → TextPerceptor → process_and_observe
                        had_input = true;
                        let cmd = self.perceptor.perceive(&line);
                        let result = self.engine.process_and_observe(&cmd);
                        if matches!(result.path, ProcessingPath::MultiPass(_)) {
                            had_multipass = true;
                        }
                        let out = self.effector.format_result(
                            &result,
                            self.config.detail_level,
                            Some(&line),
                        );
                        print!("{}", out);
                    }

                    Err(mpsc::error::TryRecvError::Empty) => break,
                    Err(mpsc::error::TryRecvError::Disconnected) => return,
                }
            }

            // Ядро тикает каждый интервал — замеряем время
            let t0 = std::time::Instant::now();
            self.engine.process_command(&tick_cmd);
            self.perf.record(t0.elapsed().as_nanos() as u64, self.engine.tick_count);

            // Автосохранение по интервалу
            let data_dir = std::path::Path::new(&self.config.data_dir);
            if self.auto_saver.tick(&self.engine, data_dir) {
                if self.config.verbose {
                    println!("  [autosave: tick={}]", self.engine.tick_count);
                }
            } else if let Some(e) = &self.auto_saver.last_error {
                eprintln!("[autosave] error: {e}");
                self.auto_saver.last_error = None; // показываем ошибку один раз
            }

            // Verbose: статус ядра — только после пользовательского ввода,
            // чтобы не перекрывать набираемый текст (курсор в одной строке)
            if had_input && self.config.verbose {
                let exp     = self.engine.ashti.experience();
                let traces  = exp.traces().len();
                let matched = exp.last_traces_matched.get();
                let tension = exp.tension_count();
                println!("  [tick={} traces={} matched={} tension={}]",
                    self.engine.tick_count, traces, matched, tension);
            }

            // Адаптивная частота тиков (Axiom Sentinel V1.0, Фаза 3)
            if self.config.adaptive_tick_rate {
                let tension = self.engine.ashti.experience().tension_count();
                let adaptive = &mut self.engine.tick_schedule.adaptive_tick;
                if had_input {
                    adaptive.trigger(TickRateReason::ExternalInput);
                } else if had_multipass {
                    adaptive.trigger(TickRateReason::MultiPass);
                } else if tension >= ADAPTIVE_TENSION_THRESHOLD {
                    adaptive.trigger(TickRateReason::TensionHigh);
                } else {
                    adaptive.on_idle_tick();
                }

                let new_ms = adaptive.interval_ms();
                if new_ms != current_interval_ms {
                    current_interval_ms = new_ms;
                    interval = tokio::time::interval(
                        tokio::time::Duration::from_millis(new_ms)
                    );
                }
            }
        }
    }

    /// Обработать служебную команду (:status, :quit, ...).
    /// Возвращает false если нужно завершить.
    fn handle_meta_command(&mut self, line: &str) -> bool {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        match parts[0] {
            ":quit" | ":q" => {
                // Автосохранение при выходе если включено
                if self.auto_saver.config.enabled {
                    let data_dir = std::path::Path::new(&self.config.data_dir);
                    match self.auto_saver.force_save(&self.engine, data_dir) {
                        Ok(_) => println!("  autosaved to {}", data_dir.display()),
                        Err(e) => eprintln!("  autosave on quit failed: {e}"),
                    }
                }
                println!("Завершение.");
                return false;
            }
            ":help" => {
                println!("{}", HELP_TEXT);
            }
            ":status" => {
                let exp    = self.engine.ashti.experience();
                let traces = exp.traces().len();
                let tension = exp.tension_count();
                let snap   = self.engine.snapshot();
                let tokens: usize = snap.domains.iter().map(|d| d.tokens.len()).sum();
                let conns:  usize = snap.domains.iter().map(|d| d.connections.len()).sum();
                let skills = self.engine.ashti.export_skills().len();
                let (max_passes, min_coh) = self.engine.maya_multipass_params();
                println!("  ══ Engine Status ══════════════════════");
                println!("  tick_count:    {}", self.engine.tick_count);
                println!("  com_next_id:   {}", self.engine.com_next_id);
                println!("  uptime:        {:.1}s", self.perf.uptime_secs());
                println!("  tick_rate:     {} Hz (actual: {:.1} Hz)",
                    self.config.tick_hz, self.perf.actual_hz());
                println!("  ── memory ─────────────────────────────");
                println!("  tokens:        {}", tokens);
                println!("  connections:   {}", conns);
                println!("  traces:        {}", traces);
                println!("  skills:        {}", skills);
                println!("  tension:       {}", tension);
                println!("  ── cognitive ──────────────────────────");
                println!("  max_passes:    {}", max_passes);
                println!("  min_coherence: {:.2}", min_coh);
            }
            ":domains" => {
                for offset in 0..=10u16 {
                    let id = 100 + offset;
                    let count = self.engine.token_count(id);
                    println!("  {} ({}) — {} tokens", id,
                        crate::effectors::message::domain_name(id), count);
                }
            }
            ":tokens" => {
                if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse::<u16>() {
                        let count = self.engine.token_count(id);
                        println!("  domain {}: {} tokens", id, count);
                    } else {
                        println!("  Usage: :tokens <domain_id>");
                    }
                }
            }
            ":verbose" => {
                match parts.get(1).copied() {
                    Some("on")  => { self.config.verbose = true;  println!("  verbose: on"); }
                    Some("off") => { self.config.verbose = false; println!("  verbose: off"); }
                    _ => println!("  verbose: {}", if self.config.verbose { "on" } else { "off" }),
                }
            }
            ":tick" => {
                let n: u64 = parts.get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                let tick_cmd = axiom_ucl::UclCommand::new(axiom_ucl::OpCode::TickForward, 0, 100, 0);
                for _ in 0..n {
                    self.engine.process_command(&tick_cmd);
                }
                println!("  ticked {} times. tick_count={}", n, self.engine.tick_count);
            }
            ":snapshot" => {
                let snap = self.engine.snapshot();
                println!("  snapshot: tick_count={} domains={}", snap.tick_count, snap.domains.len());
            }
            ":save" => {
                let dir_str = parts.get(1).copied()
                    .unwrap_or(self.config.data_dir.as_str());
                let dir = std::path::Path::new(dir_str);
                match persist_save(&self.engine, dir, &WriteOptions::default()) {
                    Ok(m) => println!(
                        "  saved to {dir_str} (tick={}, traces={}, tokens={})",
                        m.tick_count, m.contents.traces, m.contents.tokens
                    ),
                    Err(e) => println!("  save failed: {e}"),
                }
            }
            ":load" => {
                let dir_str = parts.get(1).copied()
                    .unwrap_or(self.config.data_dir.as_str());
                let dir = std::path::Path::new(dir_str);
                match persist_load(dir) {
                    Ok(r) => {
                        println!(
                            "  loaded from {dir_str} (tick={}, traces={}, tension={})",
                            r.engine.tick_count, r.traces_imported, r.tension_imported
                        );
                        self.engine = r.engine;
                        // TickSchedule сохраняем из конфига
                        self.engine.tick_schedule = self.config.tick_schedule;
                        // D-07: сбрасываем last_save_tick чтобы autosave не завис
                        self.auto_saver.reset_save_tick(self.engine.tick_count);
                    }
                    Err(e) => println!("  load failed: {e}"),
                }
            }
            ":autosave" => {
                match parts.get(1).copied() {
                    Some("on") => {
                        let interval = parts.get(2)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(1000u32);
                        self.auto_saver.set_interval(interval);
                        // Синхронизируем TickSchedule
                        self.engine.tick_schedule.persist_check_interval = interval;
                        println!("  autosave: on  interval={interval} ticks");
                    }
                    Some("off") => {
                        self.auto_saver.set_enabled(false);
                        self.engine.tick_schedule.persist_check_interval = 0;
                        println!("  autosave: off");
                    }
                    _ => {
                        let data_dir = std::path::Path::new(&self.config.data_dir);
                        println!("{}", self.auto_saver.status_line(data_dir));
                    }
                }
            }
            ":memory" => {
                let exp = self.engine.ashti.experience();
                let traces    = exp.traces().len();
                let tension   = exp.tension_count();
                let snap      = self.engine.snapshot();
                let tokens: usize  = snap.domains.iter().map(|d| d.tokens.len()).sum();
                let conns: usize   = snap.domains.iter().map(|d| d.connections.len()).sum();
                println!("  tick_count:  {}", self.engine.tick_count);
                println!("  tokens:      {}", tokens);
                println!("  connections: {}", conns);
                println!("  traces:      {}", traces);
                println!("  tension:     {}", tension);
            }
            ":schedule" => {
                let s = self.engine.tick_schedule;
                println!("  adaptation:    {}", s.adaptation_interval);
                println!("  horizon_gc:    {}", s.horizon_gc_interval);
                println!("  snapshot:      {}", s.snapshot_interval);
                println!("  dream:         {}", s.dream_interval);
                println!("  tension_check: {}", s.tension_check_interval);
                println!("  goal_check:    {}", s.goal_check_interval);
                println!("  reconcile:     {}", s.reconcile_interval);
            }
            ":traces" => {
                let exp    = self.engine.ashti.experience();
                let traces = exp.traces();
                let tick   = self.engine.tick_count;
                if traces.is_empty() {
                    println!("  no experience traces");
                } else {
                    let avg_w = traces.iter().map(|t| t.weight).sum::<f32>() / traces.len() as f32;
                    let max_w = traces.iter().map(|t| t.weight).fold(0f32, f32::max);
                    println!("  ══ Experience Traces ══════════════════");
                    println!("  Total: {}  |  Avg weight: {:.2}  |  Max weight: {:.2}",
                        traces.len(), avg_w, max_w);
                    println!("  {:>3}  {:>6}  {:>3}/{:>3}/{:>3}  {:>6}  {:>8}",
                        "#", "Weight", "tmp", "mss", "val", "Age", "Hash");
                    // Сортируем по weight desc, показываем top-20
                    let mut sorted: Vec<_> = traces.iter().enumerate().collect();
                    sorted.sort_by(|a, b| b.1.weight.total_cmp(&a.1.weight));
                    for (i, (_, t)) in sorted.iter().take(20).enumerate() {
                        let age = tick.saturating_sub(t.created_at);
                        let [x, y, z] = t.pattern.position;
                        println!("  {:>3}  {:.4}  {:>3}/{:>3}/{:>3}  ({},{},{})  {:>6}  {:>8}  {:#010x}",
                            i + 1, t.weight,
                            t.pattern.temperature, t.pattern.mass, t.pattern.valence,
                            x, y, z,
                            age, t.success_count,
                            t.pattern_hash & 0xFFFFFFFF,
                        );
                    }
                    if traces.len() > 20 {
                        println!("  ... и ещё {} traces", traces.len() - 20);
                    }
                }
            }
            ":tension" => {
                let exp    = self.engine.ashti.experience();
                let tt     = exp.tension_traces();
                let tick   = self.engine.tick_count;
                if tt.is_empty() {
                    println!("  no active tension traces");
                } else {
                    println!("  ══ Tension Traces ═════════════════════");
                    println!("  Active: {}", tt.len());
                    println!("  {:>3}  {:>4}  {:>10}  {:>12}",
                        "#", "Temp", "Hash", "Age (ticks)");
                    for (i, t) in tt.iter().enumerate() {
                        let age = tick.saturating_sub(t.created_at);
                        // Compute hash of pattern for display
                        let ph = t.pattern.temperature as u64 ^ t.pattern.mass as u64;
                        println!("  {:>3}  {:>4}  {:#010x}   {:>12}",
                            i + 1, t.temperature, ph, age);
                    }
                }
            }
            ":detail" => {
                match parts.get(1).copied() {
                    Some(level) => {
                        if let Some(d) = DetailLevel::from_str(level) {
                            self.config.detail_level = d;
                            println!("  detail: {}", d.as_str());
                        } else {
                            println!("  Unknown level. Use: off | min | mid | max");
                        }
                    }
                    None => {
                        println!("  detail: {}  (off|min|mid|max)", self.config.detail_level.as_str());
                    }
                }
            }
            ":depth" => {
                let (max_passes, min_coh) = self.engine.maya_multipass_params();
                // MAYA = level_id * 100 + 10
                let maya_id = self.engine.ashti.level_id() * 100 + 10;
                let dom_factor = self.engine.ashti.config_of(maya_id)
                    .map(|c| c.internal_dominance_factor as f32 / 128.0)
                    .unwrap_or(0.0);
                let exp = self.engine.ashti.experience();
                println!("  ══ Cognitive Depth ════════════════════");
                println!("  max_passes:          {}", max_passes);
                println!("  min_coherence:       {:.2}", min_coh);
                println!("  internal_dominance:  {:.2}", dom_factor);
                println!("  tension_threshold:   128  (drain at 50% heat)");
                println!("  ── current state ──────────────────────");
                println!("  traces:              {}", exp.traces().len());
                println!("  tension_active:      {}", exp.tension_count());
            }
            ":arbiter" => {
                println!("  ══ Arbiter — Domain Thresholds ════════");
                println!("  {:>5}  {:>10}  {:>8}  {:>7}  {:>8}  {:>8}",
                    "ID", "Name", "Reflex-T", "Assoc-T", "Cooldown", "MaxPass");
                let mut configs = self.engine.ashti.all_configs();
                configs.sort_by_key(|(id, _)| *id);
                for (id, cfg) in &configs {
                    if cfg.structural_role == 0 { continue; } // SUTRA — нет рефлекса
                    println!("  {:>5}  {:>10}  {:>8}  {:>7}  {:>8}  {:>8}",
                        id,
                        domain_name(*id),
                        cfg.reflex_threshold,
                        cfg.association_threshold,
                        cfg.reflex_cooldown,
                        cfg.max_passes,
                    );
                }
                let reflector = self.engine.ashti.reflector();
                println!("  ── reflector ──────────────────────────");
                println!("  patterns tracked:  {}", reflector.tracked_patterns());
                println!("  reflex success:    {}  fail: {}",
                    reflector.total_success(), reflector.total_fail());
            }
            ":perf" => {
                let avg = self.perf.avg_ns();
                let hz  = self.perf.actual_hz();
                println!("  ══ Performance ════════════════════════");
                println!("  uptime:       {:.1}s", self.perf.uptime_secs());
                println!("  total ticks:  {}", self.perf.total_ticks);
                println!("  actual rate:  {:.1} Hz (target: {} Hz)",
                    hz, self.config.tick_hz);
                println!("  ── tick breakdown ─────────────────────");
                println!("  avg tick:     {}", fmt_ns(avg as u64));
                if self.perf.peak_ns > 0 {
                    println!("  peak tick:    {}  (tick #{})",
                        fmt_ns(self.perf.peak_ns), self.perf.peak_tick_id);
                }
                let budget_ns = 1_000_000u64 / self.config.tick_hz.max(1) as u64 * 1000;
                if budget_ns > 0 {
                    println!("  budget used:  {:.2}%",
                        avg / budget_ns as f64 * 100.0);
                }
                // Periodic tasks call counts
                let s = self.engine.tick_schedule;
                let t = self.perf.total_ticks;
                println!("  ── periodic tasks (calls) ─────────────");
                if s.adaptation_interval > 0 {
                    println!("  adaptation:   {} calls (every {} ticks)",
                        t / s.adaptation_interval as u64, s.adaptation_interval);
                }
                if s.horizon_gc_interval > 0 {
                    println!("  horizon_gc:   {} calls (every {} ticks)",
                        t / s.horizon_gc_interval as u64, s.horizon_gc_interval);
                }
                if s.dream_interval > 0 {
                    println!("  dream:        {} calls (every {} ticks)",
                        t / s.dream_interval as u64, s.dream_interval);
                }
                if s.tension_check_interval > 0 {
                    println!("  tension_chk:  {} calls (every {} ticks)",
                        t / s.tension_check_interval as u64, s.tension_check_interval);
                }
            }
            ":tickrate" => {
                let a = &self.engine.tick_schedule.adaptive_tick;
                println!("  current_hz:  {} Hz", a.current_hz);
                println!("  reason:      {}", a.last_reason);
                println!("  idle_ticks:  {}", a.idle_ticks);
                println!("  range:       {}..{} Hz", a.min_hz, a.max_hz);
                println!("  adaptive:    {}", if self.config.adaptive_tick_rate { "on" } else { "off" });
            }
            ":export" => {
                // :export traces [path] | :export skills [path]
                let what = parts.get(1).copied().unwrap_or("traces");
                let path_str = parts.get(2).copied()
                    .unwrap_or(match what {
                        "skills" => "axiom-export-skills.json",
                        _        => "axiom-export-traces.json",
                    });
                let path = std::path::Path::new(path_str);
                match what {
                    "skills" => match export_skills(&self.engine, path) {
                        Ok(r) => println!("  exported {} skills → {}", r.exported, r.path),
                        Err(e) => println!("  export failed: {e}"),
                    },
                    "traces" => {
                        let threshold: f32 = 0.0; // экспортировать все
                        match export_traces(&self.engine, path, threshold) {
                            Ok(r) => println!("  exported {} traces → {}", r.exported, r.path),
                            Err(e) => println!("  export failed: {e}"),
                        }
                    }
                    _ => println!("  Usage: :export traces|skills [path]"),
                }
            }
            ":import" => {
                // :import traces [path] | :import skills [path]
                let what = parts.get(1).copied().unwrap_or("traces");
                let path_str = parts.get(2).copied()
                    .unwrap_or(match what {
                        "skills" => "axiom-export-skills.json",
                        _        => "axiom-export-traces.json",
                    });
                let path = std::path::Path::new(path_str);
                match what {
                    "skills" => match import_skills(&mut self.engine, path) {
                        Ok(r) => println!("  {}", r.summary_line()),
                        Err(e) => println!("  import failed: {e}"),
                    },
                    "traces" => match import_traces(&mut self.engine, path) {
                        Ok(r) => println!("  {}", r.summary_line()),
                        Err(e) => println!("  import failed: {e}"),
                    },
                    _ => println!("  Usage: :import traces|skills [path]"),
                }
            }
            _ => {
                println!("  Unknown command. Type :help for list.");
            }
        }
        true
    }
}

const HELP_TEXT: &str = "\
  ── состояние ──────────────────────────────────────────────
  :status             — расширенный статус ядра
  :memory             — токены, связи, traces, tension
  :domains            — список доменов с числом токенов
  :tokens <domain_id> — токены в домене
  :schedule           — текущий TickSchedule
  :snapshot           — info снапшота
  :tickrate           — адаптивная частота (Sentinel Phase 3)

  ── experience / когнитивный слой ──────────────────────────
  :traces             — experience traces (top-20 по weight)
  :tension            — активные tension traces
  :depth              — Cognitive Depth: параметры и состояние
  :arbiter            — пороги Arbiter по доменам + Reflector

  ── производительность ─────────────────────────────────────
  :perf               — ns/тик, пик, actual Hz, периодические задачи

  ── управление выводом ─────────────────────────────────────
  :detail [off|min|mid|max]  — уровень детализации вывода
  :verbose [on|off]          — verbose после каждого ввода

  ── управление тиками ──────────────────────────────────────
  :tick [N]           — прокрутить N тиков вручную

  ── persistence ────────────────────────────────────────────
  :save [path]              — сохранить состояние
  :load [path]              — загрузить состояние
  :autosave [on N|off]      — автосохранение каждые N тиков
  :export traces|skills [p] — экспорт знаний в JSON
  :import traces|skills [p] — импорт с GUARDIAN-валидацией (weight×0.7)

  ── прочее ─────────────────────────────────────────────────
  :help               — этот список
  :quit / :q          — выход (с автосохранением)
  Любой другой ввод   → InjectToken в SUTRA(100) → результат";
