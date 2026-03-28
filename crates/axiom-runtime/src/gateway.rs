// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Gateway — единая точка входа для внешних запросов (Этап 8)
//
// Gateway владеет AxiomEngine и добавляет два слоя поверх него:
// 1. Наблюдатели событий (EventObserver) — получают уведомления после каждой команды
// 2. Подключение произвольных адаптеров (RuntimeAdapter) через единый интерфейс

use axiom_core::Event;
use axiom_ucl::{UclCommand, UclResult};
use crate::engine::AxiomEngine;
use crate::adapters::{RuntimeAdapter, EventObserver};
use crate::channel::{Channel, ChannelBatchResult};

/// Gateway — единая точка входа для всех внешних взаимодействий с AXIOM.
///
/// Владеет `AxiomEngine` и оркестрирует вызовы:
/// 1. Принимает `UclCommand`
/// 2. Передаёт в Engine
/// 3. Собирает сгенерированные события
/// 4. Уведомляет зарегистрированных наблюдателей
pub struct Gateway {
    engine: AxiomEngine,
    observers: Vec<Box<dyn EventObserver>>,
    /// Счётчик обработанных команд
    processed_count: u64,
}

impl Gateway {
    /// Создать Gateway с указанным Engine.
    pub fn new(engine: AxiomEngine) -> Self {
        Self {
            engine,
            observers: Vec::new(),
            processed_count: 0,
        }
    }

    /// Создать Gateway с Engine по умолчанию.
    pub fn with_default_engine() -> Self {
        Self::new(AxiomEngine::new())
    }

    /// Зарегистрировать наблюдатель событий.
    ///
    /// Наблюдатель получает уведомление о каждом `Event`,
    /// сгенерированном Engine после выполнения команды.
    pub fn register_observer(&mut self, observer: Box<dyn EventObserver>) {
        self.observers.push(observer);
    }

    /// Обработать команду через Engine.
    ///
    /// После выполнения команды: дренирует события и уведомляет наблюдателей.
    pub fn process(&mut self, cmd: &UclCommand) -> UclResult {
        let result = self.engine.process_command(cmd);
        self.drain_and_notify();
        self.processed_count += 1;
        result
    }

    /// Обработать команду через произвольный адаптер.
    ///
    /// Адаптер может трансформировать команду перед передачей в Engine
    /// (валидация, логирование, rate limiting и т.д.).
    pub fn process_with(&mut self, adapter: &mut dyn RuntimeAdapter, cmd: &UclCommand) -> UclResult {
        let result = adapter.process(&mut self.engine, cmd);
        self.drain_and_notify();
        self.processed_count += 1;
        result
    }

    /// Дренировать события Engine и уведомить всех наблюдателей.
    ///
    /// Вызывается автоматически после каждой команды.
    /// Может быть вызван вручную если нужно собрать события без команды.
    pub fn drain_and_notify(&mut self) {
        let events: Vec<Event> = self.engine.drain_events();
        if !events.is_empty() {
            for event in &events {
                for observer in &self.observers {
                    observer.on_event(event);
                }
            }
        }
    }

    /// Иммутабельный доступ к Engine.
    pub fn engine(&self) -> &AxiomEngine {
        &self.engine
    }

    /// Мутабельный доступ к Engine.
    ///
    /// Используется для прямых вызовов: `run_adaptation`, `snapshot_and_prune` и т.д.
    pub fn engine_mut(&mut self) -> &mut AxiomEngine {
        &mut self.engine
    }

    /// Число обработанных команд с момента создания.
    pub fn processed_count(&self) -> u64 {
        self.processed_count
    }

    /// Число зарегистрированных наблюдателей.
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// Обработать все команды из канала.
    ///
    /// Извлекает все ожидающие команды, передаёт в Engine,
    /// собирает события и помещает их обратно в канал.
    /// Наблюдатели также уведомляются.
    pub fn process_channel(&mut self, channel: &mut Channel) -> ChannelBatchResult {
        let commands = channel.drain_commands();
        let mut result = ChannelBatchResult::new();

        for (i, cmd) in commands.iter().enumerate() {
            let ucl_result = self.engine.process_command(cmd);
            let events: Vec<Event> = self.engine.drain_events();

            for event in &events {
                // Уведомить наблюдателей
                for observer in &self.observers {
                    observer.on_event(event);
                }
                // Поместить в канал для внешнего получателя
                channel.push_event(event.clone());
            }

            self.processed_count += 1;

            if !ucl_result.is_success() {
                result.errors.push((i, ucl_result));
            } else {
                result.processed += 1;
            }
        }

        result
    }
}
