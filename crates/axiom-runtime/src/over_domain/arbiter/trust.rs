// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// TrustConfig — конфигурация доверия OverDomainArbiter.
// Источник: docs/architecture/OverDomainArbiter_V1_0.md §5

use std::collections::HashMap;

use super::source::{AdvisoryType, SourceId};

/// Режим обработки рекомендации.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustMode {
    /// Не обрабатывать — источник зарегистрирован, тип игнорируется.
    Ignore,
    /// Применить автономно если confidence ≥ min_confidence.
    AutoApply,
    /// Поставить в очередь Workstation — chrnv подтверждает вручную.
    RequireConfirmation,
}

/// Запись конфигурации доверия для пары (source × advisory_type).
#[derive(Debug, Clone)]
pub struct TrustEntry {
    /// Минимальный порог confidence — ниже игнорировать.
    pub min_confidence: f32,
    pub mode: TrustMode,
}

impl TrustEntry {
    pub fn new(mode: TrustMode, min_confidence: f32) -> Self {
        Self { min_confidence, mode }
    }
}

/// Конфигурация доверия Arbiter — карта (source × type) → TrustEntry.
#[derive(Debug, Default)]
pub struct TrustConfig {
    entries: HashMap<(SourceId, AdvisoryType), TrustEntry>,
}

impl TrustConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, source: SourceId, advisory_type: AdvisoryType, entry: TrustEntry) {
        self.entries.insert((source, advisory_type), entry);
    }

    /// Получить запись или None если не настроено (=Ignore).
    pub fn get(&self, source: SourceId, advisory_type: AdvisoryType) -> Option<&TrustEntry> {
        self.entries.get(&(source, advisory_type))
    }

    /// Дефолтная конфигурация V1 для NeuralAdvisor (source_id=0).
    /// ARB-TD-01: в V2 вынести в config/genome.yaml.
    pub fn default_v1(neural_advisor_source_id: SourceId) -> Self {
        let mut cfg = Self::new();
        let s = neural_advisor_source_id;
        cfg.set(s, AdvisoryType::DepthHint,
            TrustEntry::new(TrustMode::AutoApply, 0.75));
        cfg.set(s, AdvisoryType::OctantCorrection,
            TrustEntry::new(TrustMode::RequireConfirmation, 0.60));
        cfg.set(s, AdvisoryType::ConflictDiagnosis,
            TrustEntry::new(TrustMode::Ignore, 0.0));
        cfg.set(s, AdvisoryType::SubsystemAttribution,
            TrustEntry::new(TrustMode::Ignore, 0.0));
        cfg.set(s, AdvisoryType::EmergentCandidate,
            TrustEntry::new(TrustMode::RequireConfirmation, 0.60));
        cfg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_v1_depth_hint_autoapply() {
        let cfg = TrustConfig::default_v1(0);
        let entry = cfg.get(0, AdvisoryType::DepthHint).unwrap();
        assert_eq!(entry.mode, TrustMode::AutoApply);
        assert!((entry.min_confidence - 0.75).abs() < 1e-6);
    }

    #[test]
    fn test_default_v1_conflict_ignored() {
        let cfg = TrustConfig::default_v1(0);
        let entry = cfg.get(0, AdvisoryType::ConflictDiagnosis).unwrap();
        assert_eq!(entry.mode, TrustMode::Ignore);
    }

    #[test]
    fn test_unknown_source_returns_none() {
        let cfg = TrustConfig::default_v1(0);
        assert!(cfg.get(99, AdvisoryType::DepthHint).is_none());
    }
}
