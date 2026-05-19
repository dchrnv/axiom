// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ConflictPersistenceTracker — детектирует затяжные конфликты Corpus Callosum.
// Источник: AxialEvaluator_V2_0.md §3

use std::collections::HashMap;

use axiom_experience::{AxialConflict, Octant};

/// После скольких тиков подряд с конфликтом генерируем рекомендацию.
pub const CONFLICT_PERSISTENCE_THRESHOLD: u32 = 5;

/// Информация о зафиксированном затяжном конфликте.
pub struct PersistentConflict {
    pub analytic_octant: Octant,
    pub synthetic_octant: Octant,
    pub streak: u32,
    pub confidence: f32,
}

/// Трекер серийных конфликтов.
#[derive(Debug, Default)]
pub struct ConflictPersistenceTracker {
    streaks: HashMap<u32, (u32, AxialConflict)>,
}

impl ConflictPersistenceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Записать результат оценки Frame. Возвращает `Some(PersistentConflict)` при превышении порога.
    pub fn push(
        &mut self,
        sutra_id: u32,
        conflict: Option<&AxialConflict>,
    ) -> Option<PersistentConflict> {
        match conflict {
            None => {
                self.streaks.remove(&sutra_id);
                None
            }
            Some(c) => {
                let entry = self.streaks.entry(sutra_id).or_insert((0, c.clone()));
                entry.0 += 1;
                entry.1 = c.clone();
                if entry.0 >= CONFLICT_PERSISTENCE_THRESHOLD {
                    let streak = entry.0;
                    let analytic = entry.1.analytic_octant;
                    let synthetic = entry.1.synthetic_octant;
                    entry.0 = 0; // сброс чтобы не спамить
                    Some(PersistentConflict {
                        analytic_octant: analytic,
                        synthetic_octant: synthetic,
                        streak,
                        confidence: (streak as f32 / (CONFLICT_PERSISTENCE_THRESHOLD as f32 * 2.0))
                            .min(0.80),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn remove(&mut self, sutra_id: u32) {
        self.streaks.remove(&sutra_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::ConflictResolution;

    fn make_conflict() -> AxialConflict {
        AxialConflict {
            analytic_octant: Octant::CreativeAffirmation,
            synthetic_octant: Octant::HeroicFatal,
            conflict_strength: 85,
            resolution: ConflictResolution::Unresolved,
        }
    }

    #[test]
    fn test_no_conflict_resets_streak() {
        let mut t = ConflictPersistenceTracker::new();
        let c = make_conflict();
        for _ in 0..CONFLICT_PERSISTENCE_THRESHOLD - 1 {
            assert!(t.push(1, Some(&c)).is_none());
        }
        assert!(t.push(1, None).is_none());
        // Серия сброшена — нужно снова накопить
        for _ in 0..CONFLICT_PERSISTENCE_THRESHOLD - 1 {
            assert!(t.push(1, Some(&c)).is_none());
        }
    }

    #[test]
    fn test_persistent_conflict_fires() {
        let mut t = ConflictPersistenceTracker::new();
        let c = make_conflict();
        for _ in 0..CONFLICT_PERSISTENCE_THRESHOLD - 1 {
            t.push(1, Some(&c));
        }
        let result = t.push(1, Some(&c));
        assert!(result.is_some());
        let pc = result.unwrap();
        assert_eq!(pc.analytic_octant, Octant::CreativeAffirmation);
        assert_eq!(pc.synthetic_octant, Octant::HeroicFatal);
    }

    #[test]
    fn test_fires_once_then_resets() {
        let mut t = ConflictPersistenceTracker::new();
        let c = make_conflict();
        // Ровно THRESHOLD пушей — последний даёт fire, streak сбрасывается в 0
        for _ in 0..CONFLICT_PERSISTENCE_THRESHOLD {
            t.push(1, Some(&c));
        }
        // После fire streak = 0, следующие THRESHOLD-1 пушей не дают сигнала
        for _ in 0..CONFLICT_PERSISTENCE_THRESHOLD - 1 {
            assert!(t.push(1, Some(&c)).is_none());
        }
    }
}
