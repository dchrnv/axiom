// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SubsystemFatigue — накопление усталости подсистем (CR-V6 Фаза B).
//
// Источник: ContextRecognizer_Roadmap_V6_V9.md §1.6, ROADMAP.md Фаза B

use std::collections::HashMap;

use axiom_experience::SubsystemId;

/// Максимальная нагрузка для нормировки в effective_weight.
///
/// Equilibrium activation_load при непрерывной активности:
///   x = x * DECAY + 1.0 → x = 1.0 / (1.0 - DECAY) = 1.0 / 0.10 = 10.0
pub const MAX_ACTIVATION_LOAD: f32 = 10.0;

/// Коэффициент затухания activation_load за один on_tick (каждые 7 тиков).
pub const DECAY_FACTOR: f32 = 0.90;

/// Доля activation_load, конвертируемая в recovery_debt за один on_tick.
pub const DEBT_RATE: f32 = 0.05;

/// Коэффициент затухания recovery_debt (медленный).
pub const DEBT_DECAY: f32 = 0.998;

/// Коэффициент частичного восстановления при DREAM-пробуждении.
pub const DREAM_RECOVERY: f32 = 0.35;

/// Состояние усталости одной подсистемы.
///
/// `activation_load` — текущая активная нагрузка; убывает когда подсистема неактивна.
/// `recovery_debt` — накопленный исторический долг; сохраняется при смене primary-подсистемы.
#[derive(Debug, Clone)]
pub struct SubsystemFatigue {
    pub activation_load: f32,
    pub recovery_debt: f32,
}

impl Default for SubsystemFatigue {
    fn default() -> Self {
        Self { activation_load: 0.0, recovery_debt: 0.0 }
    }
}

impl SubsystemFatigue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Вычислить эффективный вес с учётом усталости.
    ///
    /// `effective_weight = base_weight * (1.0 - 0.5 * min(1.0, activation_load / MAX))`
    pub fn effective_weight(&self, base_weight: f32) -> f32 {
        let penalty = 0.5 * (self.activation_load / MAX_ACTIVATION_LOAD).min(1.0);
        base_weight * (1.0 - penalty)
    }

    /// Частичное восстановление при DREAM-пробуждении: `activation_load *= DREAM_RECOVERY`.
    /// `recovery_debt` остаётся — он линкует независимо от DREAM.
    pub fn dream_recover(&mut self) {
        self.activation_load *= DREAM_RECOVERY;
    }

    /// True если activation_load превышает половину максимума.
    pub fn is_fatigued(&self) -> bool {
        self.activation_load > MAX_ACTIVATION_LOAD * 0.5
    }
}

/// Хранилище усталости по всем подсистемам.
///
/// Хранится в ContextRecognizer (в V6); перенос в axiom-experience — V7.
#[derive(Debug)]
pub struct FatigueStore {
    store: HashMap<SubsystemId, SubsystemFatigue>,
}

impl Default for FatigueStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FatigueStore {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    /// Обновить усталость после каждого on_tick.
    ///
    /// - Все записи: `activation_load *= DECAY_FACTOR`, `recovery_debt *= DEBT_DECAY`
    /// - Доминирующая подсистема: `activation_load += 1.0`, `recovery_debt += DEBT_RATE`
    pub fn update(&mut self, dominant: SubsystemId) {
        for fatigue in self.store.values_mut() {
            fatigue.activation_load *= DECAY_FACTOR;
            fatigue.recovery_debt *= DEBT_DECAY;
        }
        if dominant != SubsystemId::Unknown {
            let entry = self.store.entry(dominant).or_insert_with(SubsystemFatigue::new);
            entry.activation_load += 1.0;
            entry.recovery_debt += DEBT_RATE;
        }
    }

    /// Применить частичное DREAM-восстановление ко всем записям.
    pub fn apply_dream_recovery(&mut self) {
        for fatigue in self.store.values_mut() {
            fatigue.dream_recover();
        }
    }

    /// Получить состояние усталости для подсистемы (None если не встречалась).
    pub fn get(&self, subsystem: SubsystemId) -> Option<&SubsystemFatigue> {
        self.store.get(&subsystem)
    }

    /// Применить усталость к вектору весов подсистем.
    ///
    /// Модифицирует веса in-place; незнакомые подсистемы (без записи) не затрагиваются.
    pub fn apply_to_weights(&self, weights: &mut HashMap<SubsystemId, u8>) {
        for (subsystem, weight) in weights.iter_mut() {
            if let Some(fatigue) = self.store.get(subsystem) {
                let base = *weight as f32;
                *weight = fatigue.effective_weight(base).round() as u8;
            }
        }
    }

    /// Итератор по всем записям (для диагностики).
    pub fn iter(&self) -> impl Iterator<Item = (&SubsystemId, &SubsystemFatigue)> {
        self.store.iter()
    }

    /// Число подсистем с ненулевой усталостью.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

// ── Тесты ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── SubsystemFatigue ────────────────────────────────────────────────────

    #[test]
    fn test_no_fatigue_full_weight() {
        let f = SubsystemFatigue::new();
        assert_eq!(f.effective_weight(200.0), 200.0);
    }

    #[test]
    fn test_max_fatigue_half_weight() {
        let f = SubsystemFatigue {
            activation_load: MAX_ACTIVATION_LOAD,
            recovery_debt: 0.0,
        };
        let w = f.effective_weight(200.0);
        assert!((w - 100.0).abs() < 1.0, "expected ~100, got {w}");
    }

    #[test]
    fn test_half_fatigue_three_quarter_weight() {
        let f = SubsystemFatigue {
            activation_load: MAX_ACTIVATION_LOAD * 0.5,
            recovery_debt: 0.0,
        };
        let w = f.effective_weight(200.0);
        assert!((w - 150.0).abs() < 1.0, "expected ~150, got {w}");
    }

    #[test]
    fn test_dream_recovery_reduces_load() {
        let mut f = SubsystemFatigue {
            activation_load: 8.0,
            recovery_debt: 1.0,
        };
        f.dream_recover();
        assert!((f.activation_load - 8.0 * DREAM_RECOVERY).abs() < 1e-5);
        assert_eq!(f.recovery_debt, 1.0); // долг не меняется
    }

    #[test]
    fn test_is_fatigued_threshold() {
        let mut f = SubsystemFatigue::new();
        assert!(!f.is_fatigued());
        f.activation_load = MAX_ACTIVATION_LOAD * 0.5 + 0.1;
        assert!(f.is_fatigued());
    }

    // ── FatigueStore ────────────────────────────────────────────────────────

    #[test]
    fn test_empty_store_no_modification() {
        let store = FatigueStore::new();
        let mut weights = HashMap::new();
        weights.insert(SubsystemId::Mathematics, 200u8);
        store.apply_to_weights(&mut weights);
        assert_eq!(weights[&SubsystemId::Mathematics], 200);
    }

    #[test]
    fn test_accumulate_increases_load() {
        let mut store = FatigueStore::new();
        for _ in 0..15 {
            store.update(SubsystemId::Mathematics);
        }
        let f = store.get(SubsystemId::Mathematics).unwrap();
        assert!(f.activation_load > 0.0, "expected non-zero load after 15 updates");
    }

    #[test]
    fn test_load_equilibrium_converges() {
        let mut store = FatigueStore::new();
        // Продолжать до равновесия: ~50 итераций
        for _ in 0..100 {
            store.update(SubsystemId::Mathematics);
        }
        let load = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        // Equilibrium = 1.0 / (1.0 - 0.90) = 10.0
        assert!((load - MAX_ACTIVATION_LOAD).abs() < 0.5, "expected ~{MAX_ACTIVATION_LOAD}, got {load}");
    }

    #[test]
    fn test_decay_when_inactive() {
        let mut store = FatigueStore::new();
        // Насытить Mathematics
        for _ in 0..50 {
            store.update(SubsystemId::Mathematics);
        }
        let load_before = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        // Переключиться на другую подсистему
        for _ in 0..5 {
            store.update(SubsystemId::Writing);
        }
        let load_after = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        assert!(load_after < load_before, "load should decay when inactive");
    }

    #[test]
    fn test_recovery_debt_lingers_after_switch() {
        let mut store = FatigueStore::new();
        for _ in 0..20 {
            store.update(SubsystemId::Mathematics);
        }
        let debt_before = store.get(SubsystemId::Mathematics).unwrap().recovery_debt;
        // Переключить primary подсистему
        for _ in 0..5 {
            store.update(SubsystemId::Writing);
        }
        let debt_after = store.get(SubsystemId::Mathematics).unwrap().recovery_debt;
        // Долг убывает медленно (DEBT_DECAY=0.998), а не обнуляется
        assert!(debt_after > 0.0, "debt should linger after switch");
        assert!(debt_after < debt_before, "debt decays slowly");
        // За 5 шагов долг убывает на ~1%, исходный долг должен сохраниться более 95%
        assert!(debt_after > debt_before * 0.95);
    }

    #[test]
    fn test_apply_to_weights_reduces_fatigued_subsystem() {
        let mut store = FatigueStore::new();
        // Насытить Mathematics до максимума
        for _ in 0..100 {
            store.update(SubsystemId::Mathematics);
        }
        let mut weights = HashMap::new();
        weights.insert(SubsystemId::Mathematics, 200u8);
        weights.insert(SubsystemId::Writing, 100u8);
        store.apply_to_weights(&mut weights);
        // Mathematics должна получить сниженный вес (близко к 50% от 200 = 100)
        let math_w = weights[&SubsystemId::Mathematics];
        assert!(math_w < 150, "expected fatigued weight < 150, got {math_w}");
        // Writing без усталости — вес не изменился
        assert_eq!(weights[&SubsystemId::Writing], 100);
    }

    #[test]
    fn test_dream_recovery_reduces_all_fatigues() {
        let mut store = FatigueStore::new();
        for _ in 0..50 {
            store.update(SubsystemId::Mathematics);
        }
        for _ in 0..20 {
            store.update(SubsystemId::Writing);
        }
        let load_math_before = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        let load_wr_before = store.get(SubsystemId::Writing).unwrap().activation_load;
        store.apply_dream_recovery();
        let load_math_after = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        let load_wr_after = store.get(SubsystemId::Writing).unwrap().activation_load;
        assert!((load_math_after - load_math_before * DREAM_RECOVERY).abs() < 1e-4);
        assert!((load_wr_after - load_wr_before * DREAM_RECOVERY).abs() < 1e-4);
    }

    // ── Интеграционный тест: fatigue → DREAM → partial recovery → новый паттерн ──

    #[test]
    fn test_integration_fatigue_dream_recovery_new_pattern() {
        let mut store = FatigueStore::new();

        // Фаза 1: длительная активность Mathematics — формирует усталость
        for _ in 0..60 {
            store.update(SubsystemId::Mathematics);
        }
        let load_after_activity = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        assert!(load_after_activity > 5.0, "should be fatigued after 60 updates");

        // Проверить что weights действительно снижаются
        let mut weights = HashMap::new();
        weights.insert(SubsystemId::Mathematics, 200u8);
        store.apply_to_weights(&mut weights);
        let fatigued_w = weights[&SubsystemId::Mathematics];
        assert!(fatigued_w < 180, "weight should be reduced by fatigue");

        // Фаза 2: DREAM-пробуждение → частичное восстановление
        store.apply_dream_recovery();
        let load_after_dream = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        assert!(
            (load_after_dream - load_after_activity * DREAM_RECOVERY).abs() < 0.1,
            "dream recovery should apply 0.35 factor"
        );

        // Фаза 3: после DREAM вес Mathematics частично восстановлен
        let mut weights2 = HashMap::new();
        weights2.insert(SubsystemId::Mathematics, 200u8);
        store.apply_to_weights(&mut weights2);
        let recovered_w = weights2[&SubsystemId::Mathematics];
        assert!(
            recovered_w > fatigued_w,
            "weight should recover after DREAM: before={fatigued_w}, after={recovered_w}"
        );

        // Фаза 4: новый паттерн — Writing становится активным
        for _ in 0..20 {
            store.update(SubsystemId::Writing);
        }
        let writing_f = store.get(SubsystemId::Writing).unwrap();
        assert!(writing_f.activation_load > 0.0);
        // Mathematics продолжает затухать
        let math_load_final = store.get(SubsystemId::Mathematics).unwrap().activation_load;
        assert!(math_load_final < load_after_dream, "math should keep decaying");
    }
}
