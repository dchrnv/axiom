// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AdaptiveTickRate — Variable Tick Rate (Axiom Sentinel V1.0, Фаза 3)
//
// Система адаптирует частоту тиков под когнитивную нагрузку:
//   - Idle (min_hz)  : нет ввода, нет tension traces
//   - High (max_hz)  : tension > threshold / MultiPass / внешний ввод

/// Причина изменения частоты тиков.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickRateReason {
    /// Режим ожидания: нет ввода, нет tension
    Idle,
    /// Обнаружены активные tension traces
    TensionHigh,
    /// Обработка завершилась через MultiPass (низкая когерентность)
    MultiPass,
    /// Получен внешний ввод (пользователь или адаптер)
    ExternalInput,
}

impl std::fmt::Display for TickRateReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TickRateReason::Idle => write!(f, "idle"),
            TickRateReason::TensionHigh => write!(f, "tension_high"),
            TickRateReason::MultiPass => write!(f, "multi_pass"),
            TickRateReason::ExternalInput => write!(f, "external_input"),
        }
    }
}

/// Адаптивная частота тиков (Axiom Sentinel V1.0, Фаза 3).
///
/// Хранится внутри [`TickSchedule`]. Управляет частотой главного цикла CliChannel.
///
/// # Алгоритм
/// - Триггер (tension / multi-pass / ввод) → `current_hz += step_up`, сброс `idle_ticks`
/// - Idle тик (нет триггера) → `idle_ticks += 1`
/// - Когда `idle_ticks >= cooldown` → `current_hz -= step_down` (не ниже `min_hz`)
///
/// # Defaults
/// ```text
/// min_hz = 60    max_hz = 1000
/// step_up = 200  step_down = 20  cooldown = 50
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AdaptiveTickRate {
    /// Минимальная частота тиков, Гц (режим ожидания)
    pub min_hz: u32,
    /// Максимальная частота тиков, Гц (пиковая нагрузка)
    pub max_hz: u32,
    /// Текущая частота тиков, Гц
    pub current_hz: u32,
    /// Шаг увеличения при триггере, Гц
    pub step_up: u32,
    /// Шаг уменьшения за каждый cooldown-цикл, Гц
    pub step_down: u32,
    /// Число idle-тиков до снижения частоты
    pub cooldown: u32,
    /// Внутренний счётчик idle-тиков (сбрасывается при триггере)
    pub idle_ticks: u32,
    /// Последняя причина изменения частоты
    pub last_reason: TickRateReason,
}

impl Default for AdaptiveTickRate {
    fn default() -> Self {
        Self {
            min_hz: 60,
            max_hz: 1000,
            current_hz: 60,
            step_up: 200,
            step_down: 20,
            cooldown: 50,
            idle_ticks: 0,
            last_reason: TickRateReason::Idle,
        }
    }
}

impl AdaptiveTickRate {
    /// Зарегистрировать триггер — повысить частоту, сбросить idle-счётчик.
    pub fn trigger(&mut self, reason: TickRateReason) {
        self.current_hz = (self.current_hz + self.step_up).min(self.max_hz);
        self.idle_ticks = 0;
        self.last_reason = reason;
    }

    /// Зарегистрировать idle-тик. Снижает частоту после `cooldown` тиков без триггера.
    pub fn on_idle_tick(&mut self) {
        self.idle_ticks += 1;
        if self.idle_ticks >= self.cooldown {
            self.current_hz = self
                .current_hz
                .saturating_sub(self.step_down)
                .max(self.min_hz);
            if self.current_hz == self.min_hz {
                self.last_reason = TickRateReason::Idle;
            }
        }
    }

    /// Интервал тика в миллисекундах на основе `current_hz`.
    #[inline]
    pub fn interval_ms(&self) -> u64 {
        1000 / self.current_hz.max(1) as u64
    }

    /// Является ли текущий режим idle (current_hz == min_hz).
    #[inline]
    pub fn is_idle(&self) -> bool {
        self.current_hz <= self.min_hz
    }
}
