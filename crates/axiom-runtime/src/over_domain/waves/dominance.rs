// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

/// Скорость роста internal_dominance_factor в тишине (без внешнего входа).
const RISE_RATE: f32 = 0.05;
/// Скорость падения при внешнем входе (вход важнее — реагируем быстро).
const DROP_RATE_INTAKE: f32 = 0.30;
/// Скорость падения при высоком fatigue (пора в DREAM, не до ветра).
const DROP_RATE_FATIGUE: f32 = 0.10;
/// Порог fatigue_load выше которого ветер стихает.
const FATIGUE_THRESHOLD: f32 = 6.0;
/// Минимум factor ниже которого Waves ничего не делает.
pub const DOMINANCE_THRESHOLD: f32 = 0.25;

/// Пересчитать internal_dominance_factor для текущего тика.
///
/// Плавный переход реактивное↔когнитивное:
/// - Растёт в тишине (нет внешнего входа + усталость невысокая + есть накопленные импульсы).
/// - Падает при внешнем входе (вход всегда перебивает — система не глуха к миру).
/// - Падает при высоком fatigue (пора в DREAM).
pub fn update(
    current: f32,
    had_intake: bool,
    max_fatigue_load: f32,
    active_impulse_count: usize,
) -> f32 {
    let fatigued = max_fatigue_load > FATIGUE_THRESHOLD;

    let new = if had_intake {
        // Внешний вход всегда снижает.
        (current - DROP_RATE_INTAKE).max(0.0)
    } else if fatigued {
        // Высокая усталость — ветер стихает.
        (current - DROP_RATE_FATIGUE).max(0.0)
    } else if active_impulse_count > 0 {
        // Есть что поднимать — растём.
        (current + RISE_RATE).min(1.0)
    } else {
        // Тишина без импульсов — медленно сползаем к нейтрали.
        (current - RISE_RATE * 0.5).max(0.0)
    };

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rises_in_silence_with_impulses() {
        let f = update(0.0, false, 1.0, 3);
        assert!(f > 0.0);
    }

    #[test]
    fn drops_on_intake() {
        let f = update(0.5, true, 1.0, 3);
        assert!(f < 0.5);
    }

    #[test]
    fn drops_on_high_fatigue() {
        let f = update(0.5, false, 8.0, 3);
        assert!(f < 0.5);
    }

    #[test]
    fn clamps_to_zero() {
        let f = update(0.1, true, 1.0, 0);
        assert!(f >= 0.0);
    }

    #[test]
    fn clamps_to_one() {
        let f = update(0.98, false, 1.0, 5);
        assert!(f <= 1.0);
    }
}
