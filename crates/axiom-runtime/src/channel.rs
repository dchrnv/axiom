// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Channel — in-process очередь команд и событий (Этап 8)
//
// Channel позволяет внешним компонентам ставить команды в очередь
// и получать события асинхронно через drain.
// Ядро однопоточное — никакой синхронизации не требуется.

use std::collections::VecDeque;
use axiom_core::Event;
use axiom_ucl::{UclCommand, UclResult};

/// In-process канал между внешними компонентами и Gateway.
///
/// Содержит две очереди:
/// - `pending` — команды ожидающие обработки (FIFO)
/// - `events`  — события, сгенерированные после обработки команд
///
/// Использование:
/// 1. Внешний код вызывает `send()` для постановки команд
/// 2. Gateway вызывает `drain_commands()` и обрабатывает их
/// 3. Результаты событий поступают в очередь `events` через `push_event()`
/// 4. Внешний код забирает события через `drain_events()`
pub struct Channel {
    pending: VecDeque<UclCommand>,
    events: VecDeque<Event>,
    /// Число команд, обработанных через этот канал
    processed: u64,
}

impl Channel {
    /// Создать пустой канал.
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            events: VecDeque::new(),
            processed: 0,
        }
    }

    /// Поставить команду в очередь на обработку.
    pub fn send(&mut self, cmd: UclCommand) {
        self.pending.push_back(cmd);
    }

    /// Число команд, ожидающих обработки.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Число событий, ожидающих получения.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Число команд, обработанных через этот канал с момента создания.
    pub fn processed_count(&self) -> u64 {
        self.processed
    }

    /// Извлечь все ожидающие команды (очищает очередь).
    ///
    /// Используется Gateway для пакетной обработки.
    pub fn drain_commands(&mut self) -> Vec<UclCommand> {
        self.processed += self.pending.len() as u64;
        self.pending.drain(..).collect()
    }

    /// Добавить событие в очередь событий канала.
    ///
    /// Вызывается Gateway (или наблюдателем) после обработки команды.
    pub fn push_event(&mut self, event: Event) {
        self.events.push_back(event);
    }

    /// Извлечь все накопленные события (очищает очередь).
    pub fn drain_events(&mut self) -> Vec<Event> {
        self.events.drain(..).collect()
    }

    /// Есть ли ожидающие команды?
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Есть ли события для получения?
    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    /// Очистить обе очереди.
    pub fn clear(&mut self) {
        self.pending.clear();
        self.events.clear();
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::new()
    }
}

/// Результат обработки пакета команд из канала.
pub struct ChannelBatchResult {
    /// Число успешно обработанных команд
    pub processed: usize,
    /// Ошибки по индексу команды
    pub errors: Vec<(usize, UclResult)>,
}

impl ChannelBatchResult {
    pub(crate) fn new() -> Self {
        Self { processed: 0, errors: Vec::new() }
    }

    /// Все команды обработаны без ошибок?
    pub fn all_ok(&self) -> bool {
        self.errors.is_empty()
    }
}
