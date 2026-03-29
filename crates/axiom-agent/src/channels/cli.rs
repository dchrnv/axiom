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
