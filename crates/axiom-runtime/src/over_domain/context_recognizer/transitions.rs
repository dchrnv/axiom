// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ActivityAnalyzer (бывший TransitionDetector) — лёгкий детектор смены подсистемы.
// CR-V6: переименован; остаётся как lightweight компонент рядом с ActivityTrace.

use axiom_experience::SubsystemId;

/// Событие переключения между подсистемами.
#[derive(Debug, Clone)]
pub struct SubsystemTransition {
    pub from: SubsystemId,
    pub to: SubsystemId,
    pub at_event: u64,
}

/// Лёгкий анализатор переключений между подсистемами.
///
/// Фиксирует факт смены доминирующей подсистемы.
/// Для анализа паттернов активности использовать `ActivityTrace`.
#[derive(Debug)]
pub struct ActivityAnalyzer {
    last_primary: SubsystemId,
    last_event: u64,
}

/// Совместимый псевдоним для кода, использующего старое имя.
pub type TransitionDetector = ActivityAnalyzer;

impl Default for ActivityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityAnalyzer {
    pub fn new() -> Self {
        Self {
            last_primary: SubsystemId::Unknown,
            last_event: 0,
        }
    }

    /// Обновить состояние. Возвращает событие переключения если подсистема сменилась.
    pub fn update(&mut self, new_primary: SubsystemId, event_id: u64) -> Option<SubsystemTransition> {
        if self.last_primary == new_primary {
            return None;
        }
        let from = self.last_primary;
        self.last_primary = new_primary;
        self.last_event = event_id;
        if from == SubsystemId::Unknown {
            return None; // первое обнаружение — не переключение
        }
        Some(SubsystemTransition { from, to: new_primary, at_event: event_id })
    }

    pub fn current(&self) -> SubsystemId {
        self.last_primary
    }

    pub fn last_event(&self) -> u64 {
        self.last_event
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_update_no_transition() {
        let mut d = ActivityAnalyzer::new();
        assert!(d.update(SubsystemId::Writing, 1).is_none());
    }

    #[test]
    fn test_same_subsystem_no_transition() {
        let mut d = ActivityAnalyzer::new();
        d.update(SubsystemId::Writing, 1);
        assert!(d.update(SubsystemId::Writing, 2).is_none());
    }

    #[test]
    fn test_different_subsystem_returns_transition() {
        let mut d = ActivityAnalyzer::new();
        d.update(SubsystemId::Writing, 1);
        let t = d.update(SubsystemId::Mathematics, 2);
        assert!(t.is_some());
        let t = t.unwrap();
        assert_eq!(t.from, SubsystemId::Writing);
        assert_eq!(t.to, SubsystemId::Mathematics);
        assert_eq!(t.at_event, 2);
    }

    #[test]
    fn test_compat_alias_transition_detector() {
        let mut d = TransitionDetector::new();
        d.update(SubsystemId::Writing, 1);
        assert!(d.update(SubsystemId::Mathematics, 2).is_some());
    }
}
