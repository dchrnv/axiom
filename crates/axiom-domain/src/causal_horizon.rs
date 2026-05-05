// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CausalHorizon — управление причинным горизонтом (Этап 7)
//
// horizon = min(token.last_event_id) по всем активным сущностям.
// Всё, что "за горизонтом" (last_event_id < horizon), каузально завершено
// и может быть архивировано без потери актуального состояния.

use crate::DomainState;

/// CausalHorizon — вычисляет и хранит текущий причинный горизонт системы.
///
/// Горизонт = минимальный `last_event_id` среди всех активных токенов всех доменов.
/// Следы Experience с `last_used < horizon` каузально устарели — их можно архивировать.
#[derive(Debug, Clone)]
pub struct CausalHorizon {
    /// Текущий горизонт (монотонно растёт)
    pub horizon: u64,
    /// Число архивированных следов с момента создания
    pub archived_count: u64,
}

impl CausalHorizon {
    /// Создать новый CausalHorizon (horizon = 0).
    pub fn new() -> Self {
        Self {
            horizon: 0,
            archived_count: 0,
        }
    }

    /// Вычислить горизонт как min(token.last_event_id) по всем доменам.
    ///
    /// Если в системе нет токенов — возвращает 0.
    pub fn compute(states: &[&DomainState]) -> u64 {
        let mut min_event: Option<u64> = None;

        for state in states {
            for token in &state.tokens {
                let ev = token.last_event_id;
                if ev == 0 {
                    continue;
                } // токены с event_id=0 игнорируем
                min_event = Some(match min_event {
                    None => ev,
                    Some(m) => m.min(ev),
                });
            }
        }

        min_event.unwrap_or(0)
    }

    /// Обновить горизонт по текущему состоянию доменов.
    ///
    /// Горизонт только растёт (монотонный): если новое значение меньше текущего,
    /// оно игнорируется.
    pub fn advance(&mut self, states: &[&DomainState]) {
        let new_horizon = Self::compute(states);
        if new_horizon > self.horizon {
            self.horizon = new_horizon;
        }
    }

    /// Проверить, находится ли событие за горизонтом (каузально устарело).
    #[inline]
    pub fn is_behind(&self, last_event_id: u64) -> bool {
        self.horizon > 0 && last_event_id < self.horizon
    }

    /// Зафиксировать архивированные следы.
    pub fn record_archived(&mut self, count: usize) {
        self.archived_count += count as u64;
    }
}

impl Default for CausalHorizon {
    fn default() -> Self {
        Self::new()
    }
}
