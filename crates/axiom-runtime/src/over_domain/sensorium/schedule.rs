// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use crate::over_domain::sensorium::levels::CollectionLevel;

/// Расписание сборки Sensorium — большой цикл.
///
/// Детерминированный часовой механизм: разные шестерни, разная скорость, один ход.
/// Нагрузка вычислима из расписания наперёд — нет пиков "все разом".
///
/// Большой цикл = 32 тика (= FULL_INTERVAL).
/// Внутри: уровень 0 каждый тик, уровень 1 каждые 8, уровень 2 на 32-м.
#[derive(Debug, Clone)]
pub struct SensoriumSchedule {
    /// Флаг: нужно ли собрать Memory-уровень при следующем collect (устанавливается после DREAM).
    pub collect_memory_next: bool,
}

impl Default for SensoriumSchedule {
    fn default() -> Self {
        Self {
            collect_memory_next: false,
        }
    }
}

impl SensoriumSchedule {
    pub fn new() -> Self {
        Self::default()
    }

    /// Определить уровень сборки для данного тика и обновить позицию.
    pub fn advance(&mut self, tick: u64) -> CollectionLevel {
        if self.collect_memory_next {
            self.collect_memory_next = false;
            return CollectionLevel::Memory;
        }
        CollectionLevel::for_tick(tick)
    }

    /// Посмотреть уровень для тика без изменения состояния (для engine pre-compute).
    pub fn peek_level(&self, tick: u64) -> CollectionLevel {
        if self.collect_memory_next {
            return CollectionLevel::Memory;
        }
        CollectionLevel::for_tick(tick)
    }

    /// Пометить: при следующем collect собрать Memory-уровень (вызывается после DREAM Waking).
    pub fn schedule_memory_collection(&mut self) {
        self.collect_memory_next = true;
    }
}
