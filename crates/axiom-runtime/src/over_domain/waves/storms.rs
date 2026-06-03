// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Защиты от штормов — §6 спеки Waves_Internal_Drive_V1_0.md.
//
// Без этих защит Waves превратит субъекта в невротика, застрявшего
// в своих мыслях. С ними — здоровая внутренняя жизнь, знающая меру.

use super::impulse::Impulse;

/// Скорость затухания pull_strength за одно поднятие без результата.
pub const DECAY_RATE: u8 = 15;
/// Максимальное число одновременных активных импульсов.
pub const MAX_ACTIVE_IMPULSES: usize = 4;

/// Применить затухание повтора: импульсы поднятые много раз без изменения слабеют.
///
/// Анти-руминация: покрутил трижды, ничего не сдвинулось — отпускает.
pub fn apply_decay(impulses: &mut Vec<Impulse>) {
    for imp in impulses.iter_mut() {
        if imp.raise_count > 0 {
            imp.decay(DECAY_RATE);
        }
    }
    // Удалить истощённые.
    impulses.retain(|imp| !imp.is_exhausted());
}

/// Оставить только MAX_ACTIVE_IMPULSES самых сильных.
///
/// Дно не поднимает всё разом — только самое тянущее.
pub fn limit(impulses: &mut Vec<Impulse>) {
    if impulses.len() <= MAX_ACTIVE_IMPULSES {
        return;
    }
    impulses.sort_by(|a, b| b.pull_strength.cmp(&a.pull_strength));
    impulses.truncate(MAX_ACTIVE_IMPULSES);
}

/// Сбросить raise_count после DREAM (утро вечера мудренее).
///
/// Что-то отпускает само, что-то сохраняет тягу — через pull_strength.
pub fn dream_reset(impulses: &mut Vec<Impulse>) {
    for imp in impulses.iter_mut() {
        imp.raise_count = 0;
        // Небольшое затухание самой тяги при DREAM-переосмыслении.
        imp.pull_strength = (imp.pull_strength as u16 * 75 / 100) as u8;
    }
    impulses.retain(|imp| !imp.is_exhausted());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::over_domain::waves::impulse::ImpulseSource;

    fn make_impulse(strength: u8, raises: u32) -> Impulse {
        let mut imp = Impulse::new(ImpulseSource::Dilemma, 1, strength, 0, None);
        imp.raise_count = raises;
        imp
    }

    #[test]
    fn decay_weakens_raised_impulses() {
        let mut impulses = vec![make_impulse(100, 2)];
        apply_decay(&mut impulses);
        assert!(impulses[0].pull_strength < 100);
    }

    #[test]
    fn decay_removes_exhausted() {
        let mut impulses = vec![make_impulse(5, 3)];
        apply_decay(&mut impulses);
        // 5 - 15 = 0 → exhausted → removed
        assert!(impulses.is_empty());
    }

    #[test]
    fn limit_keeps_strongest() {
        let mut impulses = vec![
            make_impulse(10, 0),
            make_impulse(200, 0),
            make_impulse(150, 0),
            make_impulse(80, 0),
            make_impulse(50, 0),
        ];
        limit(&mut impulses);
        assert_eq!(impulses.len(), MAX_ACTIVE_IMPULSES);
        assert_eq!(impulses[0].pull_strength, 200);
    }

    #[test]
    fn dream_reset_reduces_strength() {
        let mut impulses = vec![make_impulse(100, 5)];
        dream_reset(&mut impulses);
        assert_eq!(impulses[0].raise_count, 0);
        assert!(impulses[0].pull_strength < 100);
    }
}
