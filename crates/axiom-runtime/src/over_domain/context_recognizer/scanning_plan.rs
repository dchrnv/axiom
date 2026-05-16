// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ScanningPlan — управление сканированием по (octant × depth_range × FractalLevel).
// Источник: ContextRecognizer_V5_0.md §4

use axiom_experience::Octant;

/// Уровень фрактальной детализации сканирования.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FractalLevel {
    Symbol = 0,
    Word = 1,
    Phrase = 2,
    Scene = 3,
    Session = 4,
}

/// Диапазон глубин SUTRA для сканирования.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DepthRange {
    pub min: u16,
    pub max: u16,
}

impl DepthRange {
    pub const FULL: Self = Self { min: 0, max: u16::MAX };
    pub const WORKING: Self = Self { min: 100, max: 10000 };
    pub const SURFACE: Self = Self { min: 0, max: 200 };

    pub fn contains(&self, depth: u16) -> bool {
        depth >= self.min && depth <= self.max
    }

    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }
}

/// Активный регион для сканирования: октант × диапазон глубин × уровни.
#[derive(Debug, Clone)]
pub struct ActiveRegion {
    pub octant: Octant,
    pub depth_range: DepthRange,
    /// 0..255 — приоритет при конкуренции за CPU
    pub priority: u8,
    pub fractal_levels: Vec<FractalLevel>,
}

impl ActiveRegion {
    pub fn new(octant: Octant, depth_range: DepthRange, priority: u8) -> Self {
        Self {
            octant,
            depth_range,
            priority,
            fractal_levels: vec![FractalLevel::Word, FractalLevel::Phrase],
        }
    }
}

/// План сканирования для одного тика ContextRecognizer.
#[derive(Debug, Clone)]
pub struct ScanningPlan {
    pub active_regions: Vec<ActiveRegion>,
    pub computed_at_event: u64,
}

impl ScanningPlan {
    /// Пустой план — сканирование пропускается.
    pub fn empty(event_id: u64) -> Self {
        Self {
            active_regions: Vec::new(),
            computed_at_event: event_id,
        }
    }

    /// Простой план на основе набора активных октантов (V1 стратегия).
    ///
    /// Для каждого октанта создаёт регион с WORKING depth_range (100..10000)
    /// и убывающим приоритетом.
    pub fn from_octants(octants: &[Octant], event_id: u64) -> Self {
        let regions = octants
            .iter()
            .copied()
            .enumerate()
            .map(|(i, oct)| {
                let priority = 255u8.saturating_sub((i as u8) * 20);
                ActiveRegion::new(oct, DepthRange::WORKING, priority)
            })
            .collect();
        Self {
            active_regions: regions,
            computed_at_event: event_id,
        }
    }

    /// Добавить поверхностный регион (depth < 200) для необработанных Frame.
    pub fn with_surface_region(mut self, octant: Octant) -> Self {
        self.active_regions.push(ActiveRegion {
            octant,
            depth_range: DepthRange::SURFACE,
            priority: 50,
            fractal_levels: vec![FractalLevel::Symbol, FractalLevel::Word],
        });
        self
    }

    pub fn is_empty(&self) -> bool {
        self.active_regions.is_empty()
    }

    pub fn region_count(&self) -> usize {
        self.active_regions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_range_contains() {
        let r = DepthRange { min: 100, max: 5000 };
        assert!(r.contains(100));
        assert!(r.contains(2500));
        assert!(r.contains(5000));
        assert!(!r.contains(99));
        assert!(!r.contains(5001));
    }

    #[test]
    fn test_empty_plan() {
        let plan = ScanningPlan::empty(0);
        assert!(plan.is_empty());
        assert_eq!(plan.region_count(), 0);
    }

    #[test]
    fn test_from_octants_priority_descending() {
        let octants = vec![
            Octant::CreativeAffirmation,
            Octant::HeroicFatal,
            Octant::FormalDenying,
        ];
        let plan = ScanningPlan::from_octants(&octants, 42);
        assert_eq!(plan.region_count(), 3);
        // First region gets highest priority
        assert!(plan.active_regions[0].priority > plan.active_regions[2].priority);
    }

    #[test]
    fn test_from_octants_uses_working_range() {
        let plan = ScanningPlan::from_octants(&[Octant::CreativeAffirmation], 1);
        assert_eq!(plan.active_regions[0].depth_range.min, 100);
        assert_eq!(plan.active_regions[0].depth_range.max, 10000);
    }

    #[test]
    fn test_with_surface_region() {
        let plan = ScanningPlan::from_octants(&[Octant::CreativeAffirmation], 1)
            .with_surface_region(Octant::CreativeAffirmation);
        assert_eq!(plan.region_count(), 2);
        // Last region is surface
        assert_eq!(plan.active_regions[1].depth_range.max, 200);
    }

    #[test]
    fn test_event_id_stored() {
        let plan = ScanningPlan::empty(99);
        assert_eq!(plan.computed_at_event, 99);
    }
}
