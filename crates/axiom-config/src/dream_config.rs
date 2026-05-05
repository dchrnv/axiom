//! DREAM Phase Configuration
//!
//! DreamConfig — параметры DreamScheduler, DreamCycle и FatigueTracker.
//! Спецификация: docs/spec/Dream/DREAM_Phase_V1_0.md, разделы 3–4.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Конфигурация DreamScheduler — пороги принятия решения о засыпании.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct SchedulerConfig {
    /// Минимум тиков в WAKE перед следующим сном (защита от rapid cycling).
    pub min_wake_ticks: u32,
    /// Число подряд идущих тиков без внешнего intake → засыпание.
    pub idle_threshold: u32,
    /// Fatigue score (0..=255) → засыпание.
    pub fatigue_threshold: u8,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            min_wake_ticks: 1000,
            idle_threshold: 200,
            fatigue_threshold: 180,
        }
    }
}

impl SchedulerConfig {
    /// Валидация.
    pub fn validate(&self) -> Result<(), String> {
        if self.idle_threshold == 0 {
            return Err("idle_threshold must be > 0".to_string());
        }
        Ok(())
    }
}

/// Веса четырёх факторов FatigueTracker (0..=255 каждый).
///
/// Сумма весов не обязана быть 255 — нормировка происходит делением на total.
/// Нулевой вес отключает соответствующий фактор.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct FatigueWeightsConfig {
    /// Кандидаты FrameWeaver без кристаллизации.
    pub uncrystallized_candidates: u8,
    /// Заполненность EXPERIENCE (tokens / capacity).
    pub experience_pressure: u8,
    /// Тяжёлые предложения в очереди DreamCycle.
    pub pending_heavy_proposals: u8,
    /// Скорость роста causal horizon.
    pub causal_horizon_growth_rate: u8,
}

impl Default for FatigueWeightsConfig {
    fn default() -> Self {
        Self {
            uncrystallized_candidates: 80,
            experience_pressure: 100,
            pending_heavy_proposals: 60,
            causal_horizon_growth_rate: 30,
        }
    }
}

impl FatigueWeightsConfig {
    /// Валидация: хотя бы один вес должен быть ненулевым.
    pub fn validate(&self) -> Result<(), String> {
        let total = self.uncrystallized_candidates as u32
            + self.experience_pressure as u32
            + self.pending_heavy_proposals as u32
            + self.causal_horizon_growth_rate as u32;
        if total == 0 {
            return Err(
                "fatigue_weights: all weights are zero — FatigueTracker will always return 0"
                    .to_string(),
            );
        }
        Ok(())
    }
}

/// Конфигурация DreamCycle — параметры выполнения цикла во сне.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct CycleConfig {
    /// Максимальная длительность одного DreamCycle в тиках (защита от зависания).
    pub max_dream_duration_ticks: u32,
    /// Максимум предложений, принятых в один цикл.
    pub max_proposals_per_cycle: u32,
    /// Предложений, обрабатываемых за один тик стадии Processing.
    pub batch_size: u32,
}

impl Default for CycleConfig {
    fn default() -> Self {
        Self {
            max_dream_duration_ticks: 50_000,
            max_proposals_per_cycle: 100,
            batch_size: 8,
        }
    }
}

impl CycleConfig {
    /// Валидация.
    pub fn validate(&self) -> Result<(), String> {
        if self.max_dream_duration_ticks == 0 {
            return Err("max_dream_duration_ticks must be > 0".to_string());
        }
        if self.batch_size == 0 {
            return Err("batch_size must be > 0".to_string());
        }
        Ok(())
    }
}

/// Полная конфигурация DREAM Phase.
///
/// Загружается опционально через `presets.dream_file` в axiom.yaml.
/// При отсутствии файла engine использует дефолты из кода.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct DreamConfig {
    /// Параметры DreamScheduler (пороги засыпания).
    #[serde(default)]
    pub scheduler: SchedulerConfig,
    /// Веса факторов FatigueTracker.
    #[serde(default)]
    pub fatigue_weights: FatigueWeightsConfig,
    /// Параметры DreamCycle (длительность, батч).
    #[serde(default)]
    pub cycle: CycleConfig,
}

impl DreamConfig {
    /// Пресет для разработки и тестирования: низкие пороги, быстрые циклы.
    pub fn dev() -> Self {
        Self {
            scheduler: SchedulerConfig {
                min_wake_ticks: 0,
                idle_threshold: 10,
                fatigue_threshold: 200,
            },
            fatigue_weights: FatigueWeightsConfig::default(),
            cycle: CycleConfig {
                max_dream_duration_ticks: 1_000,
                max_proposals_per_cycle: 50,
                batch_size: 4,
            },
        }
    }

    /// Пресет для продакшена: консервативные пороги, большие батчи.
    pub fn production() -> Self {
        Self {
            scheduler: SchedulerConfig {
                min_wake_ticks: 2000,
                idle_threshold: 500,
                fatigue_threshold: 200,
            },
            fatigue_weights: FatigueWeightsConfig::default(),
            cycle: CycleConfig {
                max_dream_duration_ticks: 100_000,
                max_proposals_per_cycle: 500,
                batch_size: 32,
            },
        }
    }

    /// Валидация всех вложенных конфигов.
    pub fn validate(&self) -> Result<(), String> {
        self.scheduler.validate()?;
        self.fatigue_weights.validate()?;
        self.cycle.validate()?;
        Ok(())
    }
}
