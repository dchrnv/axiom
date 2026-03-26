// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Adapters — trait-границы для подключения внешних адаптеров
//
// Только интерфейсы. Конкретные адаптеры (REST, CLI, WebSocket)
// живут за пределами workspace ядра.

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

/// Простой pass-through адаптер — делегирует прямо в Engine.
///
/// Используется как базовая реализация для тестов и простых клиентов.
pub struct DirectAdapter;

impl RuntimeAdapter for DirectAdapter {
    fn process(&mut self, engine: &mut AxiomEngine, cmd: &UclCommand) -> UclResult {
        engine.process_command(cmd)
    }
}
