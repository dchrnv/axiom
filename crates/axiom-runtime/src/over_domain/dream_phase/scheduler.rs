// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DreamScheduler — решает когда системе перейти в DREAMING.
// Спецификация: docs/spec/Dream/DREAM_Phase_V1_0.md, раздел 4.

use super::fatigue::{FatigueTracker, FatigueWeights, FatigueSnapshot, IdleTracker};

/// Конфигурация порогов DreamScheduler.
#[derive(Debug, Clone, Copy)]
pub struct DreamSchedulerConfig {
    /// Минимум тиков в WAKE перед следующим сном (защита от rapid cycling).
    pub min_wake_ticks: u32,
    /// Порог idle_ticks для перехода в FallingAsleep.
    pub idle_threshold: u32,
    /// Порог fatigue_score (0..=255) для перехода в FallingAsleep.
    pub fatigue_threshold: u8,
}

impl Default for DreamSchedulerConfig {
    fn default() -> Self {
        Self {
            min_wake_ticks:    1000,
            idle_threshold:     200,
            fatigue_threshold:  180,
        }
    }
}

/// Решение DreamScheduler по итогам тика.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepDecision {
    /// Оставаться в WAKE.
    StayAwake,
    /// Начать переход в DREAMING (тип триггера прилагается).
    GoToSleep(SleepTriggerKind),
}

/// Причина решения засыпания — для статистики и логирования.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepTriggerKind {
    Idle,
    Fatigue,
    ExplicitCommand,
}

/// Статистика работы DreamScheduler.
#[derive(Debug, Default, Clone, Copy)]
pub struct DreamSchedulerStats {
    /// Сколько раз система приняла решение засыпать.
    pub sleep_decisions: u64,
    /// Из них — по idle триггеру.
    pub idle_triggers: u64,
    /// Из них — по fatigue триггеру.
    pub fatigue_triggers: u64,
    /// Из них — по явной команде.
    pub explicit_triggers: u64,
}

/// DreamScheduler — периодически вызывается из WAKE-тика,
/// решает нужно ли переходить в DREAMING.
pub struct DreamScheduler {
    config:                  DreamSchedulerConfig,
    fatigue:                 FatigueTracker,
    idle:                    IdleTracker,
    /// Тик, когда система последний раз вышла из DREAMING (или 0 — никогда).
    wake_since_tick:         u64,
    /// Ожидающая явная команда перейти в DREAMING (source_id).
    explicit_pending:        Option<u16>,
    pub stats:               DreamSchedulerStats,
}

impl DreamScheduler {
    pub fn new(config: DreamSchedulerConfig, weights: FatigueWeights) -> Self {
        Self {
            config,
            fatigue: FatigueTracker::new(weights),
            idle: IdleTracker::default(),
            wake_since_tick: 0,
            explicit_pending: None,
            stats: DreamSchedulerStats::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(DreamSchedulerConfig::default(), FatigueWeights::default())
    }

    /// Зарегистрировать явную команду перейти в DREAMING (например, CLI :dream).
    pub fn submit_explicit_command(&mut self, source_id: u16) {
        self.explicit_pending = Some(source_id);
    }

    /// Вызывается каждый WAKE-тик. Обновляет трекеры и возвращает решение.
    ///
    /// `tick` — текущий глобальный тик (tick_count).
    /// `snapshot` — свежий снимок метрик.
    /// `intake_present` — был ли внешний ввод в этом тике.
    pub fn on_wake_tick(
        &mut self,
        tick: u64,
        snapshot: FatigueSnapshot,
        intake_present: bool,
    ) -> SleepDecision {
        self.fatigue.update(snapshot);
        self.idle.update(intake_present);

        let ticks_awake = tick.saturating_sub(self.wake_since_tick);
        if ticks_awake < self.config.min_wake_ticks as u64 {
            return SleepDecision::StayAwake;
        }

        if let Some(_src) = self.explicit_pending.take() {
            self.record(SleepTriggerKind::ExplicitCommand);
            return SleepDecision::GoToSleep(SleepTriggerKind::ExplicitCommand);
        }

        if self.idle.idle_ticks() >= self.config.idle_threshold {
            self.record(SleepTriggerKind::Idle);
            return SleepDecision::GoToSleep(SleepTriggerKind::Idle);
        }

        if self.fatigue.score() >= self.config.fatigue_threshold {
            self.record(SleepTriggerKind::Fatigue);
            return SleepDecision::GoToSleep(SleepTriggerKind::Fatigue);
        }

        SleepDecision::StayAwake
    }

    /// Вызывается когда DreamCycle завершился — сбрасывает idle, фиксирует тик возврата.
    pub fn on_dream_finished(&mut self, wake_tick: u64) {
        self.idle.reset();
        self.wake_since_tick = wake_tick;
    }

    /// Текущий fatigue score (последнее вычисленное значение).
    pub fn current_fatigue(&self) -> u8 {
        self.fatigue.score()
    }

    /// Текущий счётчик idle-тиков.
    pub fn current_idle_ticks(&self) -> u32 {
        self.idle.idle_ticks()
    }

    fn record(&mut self, kind: SleepTriggerKind) {
        self.stats.sleep_decisions += 1;
        match kind {
            SleepTriggerKind::Idle            => self.stats.idle_triggers    += 1,
            SleepTriggerKind::Fatigue         => self.stats.fatigue_triggers  += 1,
            SleepTriggerKind::ExplicitCommand => self.stats.explicit_triggers += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scheduler_min0() -> DreamScheduler {
        DreamScheduler::new(
            DreamSchedulerConfig { min_wake_ticks: 0, idle_threshold: 5, fatigue_threshold: 200 },
            FatigueWeights::default(),
        )
    }

    #[test]
    fn stays_awake_below_min_wake_ticks() {
        let mut s = DreamScheduler::new(
            DreamSchedulerConfig { min_wake_ticks: 100, idle_threshold: 1, fatigue_threshold: 1 },
            FatigueWeights::default(),
        );
        // даже с idle=5 — не засыпаем пока ticks_awake < 100
        for i in 0..5u64 {
            let snap = FatigueSnapshot::default();
            assert_eq!(s.on_wake_tick(i, snap, false), SleepDecision::StayAwake);
        }
    }

    #[test]
    fn goes_to_sleep_on_idle_threshold() {
        let mut s = scheduler_min0();
        let snap = FatigueSnapshot::default();
        // 4 тика — ещё не порог
        for i in 0..4u64 {
            s.on_wake_tick(i, snap, false);
        }
        // 5-й тик пересекает idle_threshold=5
        let dec = s.on_wake_tick(4, snap, false);
        assert_eq!(dec, SleepDecision::GoToSleep(SleepTriggerKind::Idle));
        assert_eq!(s.stats.idle_triggers, 1);
    }

    #[test]
    fn explicit_command_takes_priority_over_idle() {
        let mut s = scheduler_min0();
        s.submit_explicit_command(42);
        let snap = FatigueSnapshot::default();
        // сразу на первом тике (min_wake=0)
        let dec = s.on_wake_tick(0, snap, true);
        assert_eq!(dec, SleepDecision::GoToSleep(SleepTriggerKind::ExplicitCommand));
        assert_eq!(s.stats.explicit_triggers, 1);
        // команда consumed — следующий тик: StayAwake (idle не накоплен)
        let dec2 = s.on_wake_tick(1, snap, true);
        assert_eq!(dec2, SleepDecision::StayAwake);
    }

    #[test]
    fn on_dream_finished_resets_idle_and_wake_tick() {
        let mut s = scheduler_min0();
        let snap = FatigueSnapshot::default();
        // накапливаем idle
        for i in 0..5u64 { s.on_wake_tick(i, snap, false); }
        // будит систему на тике 100
        s.on_dream_finished(100);
        // теперь idle = 0, wake_since_tick = 100 → ticks_awake = 0
        let dec = s.on_wake_tick(100, snap, false);
        assert_eq!(dec, SleepDecision::StayAwake);
    }

    #[test]
    fn goes_to_sleep_on_fatigue() {
        let mut s = DreamScheduler::new(
            DreamSchedulerConfig { min_wake_ticks: 0, idle_threshold: 9999, fatigue_threshold: 100 },
            FatigueWeights { experience_pressure: 255, uncrystallized_candidates: 0,
                pending_heavy_proposals: 0, causal_horizon_growth_rate: 0 },
        );
        let snap = FatigueSnapshot {
            experience_token_count: 800,
            experience_capacity:    1000,
            ..FatigueSnapshot::default()
        };
        // fatigue score ≈ 204 → >= threshold 100
        let dec = s.on_wake_tick(0, snap, true);
        assert_eq!(dec, SleepDecision::GoToSleep(SleepTriggerKind::Fatigue));
        assert_eq!(s.stats.fatigue_triggers, 1);
    }

    #[test]
    fn stats_accumulate_across_ticks() {
        let mut s = scheduler_min0();
        let snap = FatigueSnapshot::default();
        // первое засыпание по idle
        for i in 0..=4u64 { s.on_wake_tick(i, snap, false); }
        s.on_dream_finished(10);
        // второе засыпание по idle
        for i in 10..=14u64 { s.on_wake_tick(i, snap, false); }
        assert_eq!(s.stats.sleep_decisions, 2);
        assert_eq!(s.stats.idle_triggers, 2);
    }
}
