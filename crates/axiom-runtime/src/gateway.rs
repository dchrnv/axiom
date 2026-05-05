// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Gateway — единая точка входа для внешних запросов (Этап 8)
//
// Gateway владеет AxiomEngine и добавляет два слоя поверх него:
// 1. Наблюдатели событий (EventObserver) — получают уведомления после каждой команды
// 2. Подключение произвольных адаптеров (RuntimeAdapter) через единый интерфейс

use crate::adapters::{EventBus, EventObserver, RuntimeAdapter};
use crate::channel::{Channel, ChannelBatchResult};
use crate::engine::AxiomEngine;
use axiom_config::{ConfigWatcher, LoadedAxiomConfig};
use axiom_core::Event;
use axiom_ucl::{UclCommand, UclResult};

/// Gateway — единая точка входа для всех внешних взаимодействий с AXIOM.
///
/// Владеет `AxiomEngine` и оркестрирует вызовы:
/// 1. Принимает `UclCommand`
/// 2. Передаёт в Engine
/// 3. Собирает сгенерированные события
/// 4. Уведомляет зарегистрированных наблюдателей
pub struct Gateway {
    engine: AxiomEngine,
    bus: EventBus,
    /// Счётчик обработанных команд
    processed_count: u64,
    /// Наблюдатель за конфигурацией (опционально)
    config_watcher: Option<ConfigWatcher>,
}

impl Gateway {
    /// Создать Gateway с указанным Engine.
    pub fn new(engine: AxiomEngine) -> Self {
        Self {
            engine,
            bus: EventBus::new(),
            processed_count: 0,
            config_watcher: None,
        }
    }

    /// Создать Gateway с Engine по умолчанию.
    pub fn with_default_engine() -> Self {
        Self::new(AxiomEngine::new())
    }

    /// Создать Gateway, загрузив конфигурацию из axiom.yaml.
    ///
    /// Применяет DreamConfig если `presets.dream_file` задан и файл существует.
    /// При ошибках загрузки — fallback на defaults (не паникует).
    pub fn with_config(config_path: &std::path::Path) -> Self {
        let mut gateway = Self::with_default_engine();
        let mut loader = axiom_config::ConfigLoader::new();
        if let Ok(loaded) = loader.load_all(config_path) {
            if let Some(dream_cfg) = loaded.dream {
                gateway.engine.apply_dream_config(&dream_cfg);
            }
        }
        gateway
    }

    /// Зарегистрировать broadcast-наблюдатель (получает все события).
    ///
    /// Эквивалентно `subscribe_all`. Сохранено для обратной совместимости.
    pub fn register_observer(&mut self, observer: Box<dyn EventObserver>) {
        self.bus.subscribe_all(observer);
    }

    /// Подписать наблюдатель на конкретный тип событий.
    ///
    /// Наблюдатель получает только события с matching `event_type`.
    pub fn subscribe(&mut self, event_type: u16, observer: Box<dyn EventObserver>) {
        self.bus.subscribe(event_type, observer);
    }

    /// Доступ к Event Bus для расширенной настройки подписок.
    pub fn event_bus_mut(&mut self) -> &mut EventBus {
        &mut self.bus
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
    pub fn process_with(
        &mut self,
        adapter: &mut dyn RuntimeAdapter,
        cmd: &UclCommand,
    ) -> UclResult {
        let result = adapter.process(&mut self.engine, cmd);
        self.drain_and_notify();
        self.processed_count += 1;
        result
    }

    /// Дренировать события Engine и опубликовать через Event Bus.
    ///
    /// Вызывается автоматически после каждой команды.
    /// Может быть вызван вручную если нужно собрать события без команды.
    pub fn drain_and_notify(&mut self) {
        let events: Vec<Event> = self.engine.drain_events();
        if !events.is_empty() {
            self.bus.publish(&events);
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

    /// Число broadcast-наблюдателей.
    pub fn observer_count(&self) -> usize {
        self.bus.broadcast_count()
    }

    /// Общее число подписчиков (broadcast + typed).
    pub fn total_subscriber_count(&self) -> usize {
        self.bus.total_count()
    }

    /// Подключить наблюдатель за конфигурацией.
    ///
    /// После вызова `check_config_reload()` будет проверять изменения файла
    /// и возвращать новую конфигурацию при необходимости.
    ///
    /// **GENOME-инвариант:** горячая перезагрузка никогда не затрагивает GENOME.
    /// `ConfigLoader::load_all` не читает `genome.yaml` — конституция неизменна.
    pub fn set_config_watcher(&mut self, watcher: ConfigWatcher) {
        self.config_watcher = Some(watcher);
    }

    /// Проверить наличие обновлений конфигурации (неблокирующий вызов).
    ///
    /// Если файл конфигурации изменился — применяет hot-reloadable поля
    /// (DreamConfig) и возвращает новую `LoadedAxiomConfig`.
    /// Возвращает `None` если наблюдатель не подключён или изменений не было.
    pub fn check_config_reload(&mut self) -> Option<LoadedAxiomConfig> {
        let loaded = self.config_watcher.as_ref()?.poll()?;
        // Hot-reload DreamConfig (допустимо менять в WAKE без перезапуска)
        if let Some(ref dream_cfg) = loaded.dream {
            self.engine.apply_dream_config(dream_cfg);
        }
        Some(loaded)
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

            if !events.is_empty() {
                self.bus.publish(&events);
                for event in events {
                    channel.push_event(event);
                }
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
