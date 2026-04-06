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
use axiom_runtime::{AxiomEngine, TickSchedule};
use crate::perceptors::text::TextPerceptor;
use crate::effectors::message::MessageEffector;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

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
    /// Подробный вывод tension traces после каждого тика (default: false)
    pub verbose: bool,
    /// Строка приглашения (default: "axiom> ")
    pub prompt: String,
    /// TickSchedule для применения к Engine при старте
    pub tick_schedule: TickSchedule,
    /// Директория хранилища (default: "./axiom-data"). Используется в :save/:load
    pub data_dir: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            tick_hz: 100,
            verbose: false,
            prompt: "axiom> ".to_string(),
            tick_schedule: TickSchedule::default(),
            data_dir: "axiom-data".to_string(),
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
            if let Some(v) = file.tick_hz    { config.tick_hz   = v; }
            if let Some(v) = file.verbose    { config.verbose   = v; }
            if let Some(v) = file.prompt     { config.prompt    = v; }
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
  --verbose, -v     Show tension traces after each tick
  --config <path>   Path to axiom-cli.yaml (default: ./axiom-cli.yaml)
  --data-dir <path> Data directory for save/load (default: ./axiom-data)
  --no-load         Skip loading from data directory on startup
  --help, -h        Show this message";

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
}

impl CliChannel {
    /// Создать новый CliChannel.
    /// TickSchedule из конфига применяется к Engine при создании.
    pub fn new(mut engine: AxiomEngine, config: CliConfig) -> Self {
        engine.tick_schedule = config.tick_schedule;
        let persist_interval = engine.tick_schedule.persist_check_interval;
        let auto_cfg = PersistenceConfig::new(config.data_dir.clone(), persist_interval);
        Self {
            engine,
            perceptor:  TextPerceptor::new(),
            effector:   MessageEffector::new(),
            auto_saver: AutoSaver::new(auto_cfg),
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

        let tick_ms = 1000 / self.config.tick_hz.max(1);
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_millis(tick_ms as u64)
        );
        let tick_cmd = axiom_ucl::UclCommand::new(axiom_ucl::OpCode::TickForward, 0, 100, 0);

        loop {
            interval.tick().await;

            // Обработать все сообщения из stdin (non-blocking)
            loop {
                match rx.try_recv() {
                    Ok(line) if line.is_empty() => {}

                    Ok(line) if line.starts_with(':') => {
                        if !self.handle_meta_command(&line) {
                            return; // :quit
                        }
                    }

                    Ok(line) => {
                        // Пользовательский ввод → TextPerceptor → process_and_observe
                        let cmd = self.perceptor.perceive(&line);
                        let result = self.engine.process_and_observe(&cmd);
                        let out = self.effector.format_result(&result);
                        print!("{}", out);
                    }

                    Err(mpsc::error::TryRecvError::Empty) => break,
                    Err(mpsc::error::TryRecvError::Disconnected) => return,
                }
            }

            // Ядро тикает каждый интервал
            self.engine.process_command(&tick_cmd);

            // Автосохранение по интервалу
            if self.auto_saver.tick(&self.engine) {
                if self.config.verbose {
                    println!("  [autosave: tick={}]", self.engine.tick_count);
                }
            } else if let Some(e) = &self.auto_saver.last_error {
                eprintln!("[autosave] error: {e}");
                self.auto_saver.last_error = None; // показываем ошибку один раз
            }

            // Verbose: показать tension traces
            if self.config.verbose {
                let tension = self.engine.ashti.experience().tension_count();
                if tension > 0 {
                    println!("  [tension: {} active]", tension);
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
                    let dir = self.auto_saver.config.data_dir.clone();
                    match self.auto_saver.force_save(&self.engine) {
                        Ok(_) => println!("  autosaved to {}", dir.display()),
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
                println!("  tick_count: {}", self.engine.tick_count);
                let tension = self.engine.ashti.experience().tension_count();
                println!("  tension:    {}", tension);
            }
            ":domains" => {
                for offset in 0..=10u16 {
                    let id = 100 + offset;
                    let count = self.engine.token_count(id as u32);
                    println!("  {} ({}) — {} tokens", id,
                        crate::effectors::message::domain_name(id), count);
                }
            }
            ":tokens" => {
                if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse::<u32>() {
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
                        println!("{}", self.auto_saver.status_line());
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
                let tension = self.engine.ashti.experience().tension_count();
                println!("  tension traces: {}", tension);
            }
            ":tension" => {
                let tension = self.engine.ashti.experience().tension_count();
                println!("  active tension: {}", tension);
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
  :quit / :q          — завершить
  :status             — tick_count, tension
  :domains            — список доменов с числом токенов
  :tokens <domain_id> — токены в домене
  :traces             — tension traces
  :tension            — активные tension traces
  :verbose [on/off]   — переключить подробный вывод
  :tick [N]           — прокрутить N тиков
  :snapshot           — показать info снапшота
  :schedule           — текущий TickSchedule
  :save [path]              — сохранить состояние (default: axiom-data)
  :load [path]              — загрузить состояние (default: axiom-data)
  :memory                   — показать статистику памяти
  :autosave [on N|off]      — вкл/выкл автосохранение (N = интервал тиков)
  :export traces|skills [p] — экспортировать знания в JSON-файл
  :import traces|skills [p] — импортировать с GUARDIAN-валидацией (weight×0.7)
  :help               — этот список
  Любой другой ввод   → InjectToken в SUTRA(100) → результат";
