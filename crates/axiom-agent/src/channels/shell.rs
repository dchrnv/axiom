// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Shell Effector — выполнение shell-команд при событиях ShellExec
//
// Безопасность: только команды из белого списка могут быть выполнены.
// Guardian должен проверять доступ перед вызовом ShellEffector.

use std::path::Path;
use std::process::Command;
use axiom_core::{Event, EventType};
use axiom_ucl::UclResult;
use axiom_runtime::Effector;

/// ShellEffector — исходящий адаптер для выполнения shell-команд.
///
/// Выполняет команду при получении события типа `ShellExec` (0xE001).
/// Команда извлекается из первых 48 байт `event.payload` (UTF-8 строка).
///
/// # Безопасность
///
/// Только команды из `whitelist` могут быть выполнены.
/// Команды вне списка отклоняются без выполнения.
/// Белый список загружается из YAML при инициализации.
pub struct ShellEffector {
    whitelist: Vec<String>,
    /// Лог выполненных команд (для тестирования и аудита)
    pub executed: Vec<String>,
    /// Лог отклонённых команд
    pub denied: Vec<String>,
}

impl ShellEffector {
    /// Создать эффектор с готовым белым списком.
    pub fn new(whitelist: Vec<String>) -> Self {
        Self {
            whitelist,
            executed: Vec::new(),
            denied: Vec::new(),
        }
    }

    /// Загрузить белый список из YAML файла.
    ///
    /// Формат YAML:
    /// ```yaml
    /// whitelist:
    ///   - "echo hello"
    ///   - "date"
    /// ```
    pub fn from_whitelist_file(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("read whitelist: {e}"))?;
        let val: serde_yaml::Value = serde_yaml::from_str(&content)
            .map_err(|e| format!("parse whitelist: {e}"))?;
        let list = val
            .get("whitelist")
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        Ok(Self::new(list))
    }

    /// Проверить, разрешена ли команда белым списком.
    ///
    /// Сравнение точное (без glob). Команда должна полностью совпадать
    /// с одним из элементов списка.
    pub fn is_allowed(&self, cmd: &str) -> bool {
        self.whitelist.iter().any(|w| w == cmd)
    }

    /// Выполнить команду через `/bin/sh -c`.
    fn execute(&mut self, cmd: &str) {
        self.executed.push(cmd.to_string());
        let _ = Command::new("/bin/sh").arg("-c").arg(cmd).status();
    }

    /// Извлечь команду из payload события (UTF-8 строка, trimmed).
    fn extract_command(event: &Event) -> Option<String> {
        // payload хранится в первых 48 байтах event (но Event не имеет raw payload)
        // Используем token_id как косвенный ключ, команда кодируется в виде строки
        // через event.payload_hash как индекс — либо в будущем через отдельный payload
        // Для текущей архитектуры: команда передаётся через имя события в payload_hash.
        // Упрощённо: используем event_id mod whitelist length для MVP
        // Реальная передача — через UclCommand.payload байты (будущий этап).
        let _ = event;
        None // payload в Event не доступен напрямую — вернуть None для текущего MVP
    }
}

/// Константа типа события ShellExec
pub const SHELL_EXEC_EVENT_TYPE: u16 = EventType::ShellExec as u16;

impl Effector for ShellEffector {
    fn emit(&mut self, event: &Event) {
        if event.event_type != SHELL_EXEC_EVENT_TYPE {
            return;
        }
        // В текущей архитектуре Event не несёт строкового payload.
        // ShellEffector работает через явный вызов execute_command() в тестах.
        // В production payload передаётся через UclCommand → event mapping (следующий этап).
        let _ = Self::extract_command(event);
    }

    fn emit_result(&mut self, _result: &UclResult) {
        // Shell effector не реагирует на результаты команд
    }

    fn name(&self) -> &str {
        "shell"
    }
}

impl ShellEffector {
    /// Выполнить команду напрямую (API для тестов и прямого вызова).
    ///
    /// Проверяет белый список, затем выполняет.
    /// Возвращает `true` если команда выполнена, `false` если отклонена.
    pub fn execute_command(&mut self, cmd: &str) -> bool {
        if self.is_allowed(cmd) {
            self.execute(cmd);
            true
        } else {
            self.denied.push(cmd.to_string());
            false
        }
    }
}
