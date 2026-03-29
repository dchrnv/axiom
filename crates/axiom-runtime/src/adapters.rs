// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Adapters — trait-границы для подключения внешних адаптеров
//
// Только интерфейсы. Конкретные адаптеры (REST, CLI, WebSocket)
// живут за пределами workspace ядра.

use std::collections::HashMap;
use axiom_core::Event;
use axiom_ucl::{UclCommand, UclResult};
use crate::engine::AxiomEngine;

/// Trait для внешних адаптеров, работающих с Engine.
///
/// Адаптер принимает UCL-команду и возвращает результат.
/// Ядро ничего не знает о транспорте — оно только определяет границу.
pub trait RuntimeAdapter {
    /// Обработать команду через Engine
    fn process(&mut self, engine: &mut AxiomEngine, cmd: &UclCommand) -> UclResult;
}

/// Trait для наблюдателей событий (event bus consumers).
///
/// Вызывается после каждого шага Engine если есть генерированные события.
pub trait EventObserver {
    /// Получить уведомление о событии
    fn on_event(&self, event: &Event);
}

/// Event Bus — подписочная модель поверх EventObserver.
///
/// Позволяет подписываться на конкретные типы событий (`event_type: u16`)
/// или на все события сразу (broadcast).
///
/// # Пример
///
/// ```rust,ignore
/// let mut bus = EventBus::new();
/// bus.subscribe(EventType::TokenCreate as u16, Box::new(my_observer));
/// bus.subscribe_all(Box::new(logger));
/// bus.publish(&events);
/// ```
pub struct EventBus {
    /// Подписчики на конкретный event_type
    subscribers: HashMap<u16, Vec<Box<dyn EventObserver>>>,
    /// Подписчики на все события
    broadcast: Vec<Box<dyn EventObserver>>,
}

impl EventBus {
    /// Создать пустой Event Bus.
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            broadcast: Vec::new(),
        }
    }

    /// Подписаться на конкретный тип событий.
    pub fn subscribe(&mut self, event_type: u16, observer: Box<dyn EventObserver>) {
        self.subscribers.entry(event_type).or_default().push(observer);
    }

    /// Подписаться на все события (broadcast).
    pub fn subscribe_all(&mut self, observer: Box<dyn EventObserver>) {
        self.broadcast.push(observer);
    }

    /// Опубликовать события — рассылает подписчикам.
    ///
    /// Каждое событие доставляется:
    /// 1. Всем broadcast-подписчикам
    /// 2. Подписчикам на конкретный `event_type` этого события
    pub fn publish(&self, events: &[Event]) {
        for event in events {
            for observer in &self.broadcast {
                observer.on_event(event);
            }
            if let Some(typed) = self.subscribers.get(&event.event_type) {
                for observer in typed {
                    observer.on_event(event);
                }
            }
        }
    }

    /// Число broadcast-подписчиков.
    pub fn broadcast_count(&self) -> usize {
        self.broadcast.len()
    }

    /// Число подписчиков на конкретный тип события.
    pub fn typed_count(&self, event_type: u16) -> usize {
        self.subscribers.get(&event_type).map_or(0, |v| v.len())
    }

    /// Общее число подписчиков (broadcast + все typed).
    pub fn total_count(&self) -> usize {
        let typed: usize = self.subscribers.values().map(|v| v.len()).sum();
        self.broadcast.len() + typed
    }

    /// Есть ли хоть один подписчик?
    pub fn is_empty(&self) -> bool {
        self.broadcast.is_empty() && self.subscribers.is_empty()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Простой pass-through адаптер — делегирует прямо в Engine.
///
/// Используется как базовая реализация для тестов и простых клиентов.
pub struct DirectAdapter;

impl RuntimeAdapter for DirectAdapter {
    fn process(&mut self, engine: &mut AxiomEngine, cmd: &UclCommand) -> UclResult {
        engine.process_command(cmd)
    }
}
