// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// TrustConfig — конфигурация доверия OverDomainArbiter.
// Источник: docs/architecture/OverDomainArbiter_V1_0.md §5
//           docs/architecture/OverDomainArbiter_V2_0.md §2

use std::collections::HashMap;
use std::path::Path;

use super::log::ArbiterLog;
use super::source::{AdvisoryType, SourceId};

// ── Константы автокалибровки (ARB-TD-01) ─────────────────────────────────────

pub const CALIBRATION_WINDOW: usize = 20;
pub const CALIBRATION_HIGH: f32 = 0.80;
pub const CALIBRATION_LOW: f32 = 0.40;
pub const CALIBRATION_STEP: f32 = 0.02;
pub const CONFIDENCE_FLOOR: f32 = 0.50;
pub const CONFIDENCE_CEIL: f32 = 0.95;

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

    /// Дефолтная конфигурация V1 для NeuralAdvisor (source_id=0) и AxialEvaluator (source_id=1).
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
        // AxialEvaluator (source_id=1)
        let ae: SourceId = 1;
        cfg.set(ae, AdvisoryType::OctantCorrection,
            TrustEntry::new(TrustMode::RequireConfirmation, 0.70));
        cfg.set(ae, AdvisoryType::ConflictDiagnosis,
            TrustEntry::new(TrustMode::RequireConfirmation, 0.60));
        cfg.set(ae, AdvisoryType::NarrativeShift,
            TrustEntry::new(TrustMode::RequireConfirmation, 0.55));
        cfg
    }

    /// ARB-TD-05: итерировать все записи для сериализации калиброванных значений.
    pub fn iter_entries(&self) -> impl Iterator<Item = (&(SourceId, AdvisoryType), &TrustEntry)> {
        self.entries.iter()
    }

    /// ARB-TD-05: обновить min_confidence для конкретной пары (source, advisory_type).
    pub fn set_min_confidence(&mut self, source: SourceId, advisory_type: AdvisoryType, min_confidence: f32) {
        if let Some(entry) = self.entries.get_mut(&(source, advisory_type)) {
            entry.min_confidence = min_confidence;
        }
    }

    /// V2: автокалибровка min_confidence для пары (source, advisory_type).
    /// Вызывается после confirm_pending / reject_pending.
    pub fn calibrate(
        &mut self,
        source: SourceId,
        advisory_type: AdvisoryType,
        log: &ArbiterLog,
    ) {
        let Some(entry) = self.entries.get_mut(&(source, advisory_type)) else {
            return;
        };
        let Some(quality) = log.quality_window(source, advisory_type, CALIBRATION_WINDOW) else {
            return;
        };
        if quality > CALIBRATION_HIGH && entry.min_confidence > CONFIDENCE_FLOOR {
            entry.min_confidence = (entry.min_confidence - CALIBRATION_STEP).max(CONFIDENCE_FLOOR);
        } else if quality < CALIBRATION_LOW && entry.min_confidence < CONFIDENCE_CEIL {
            entry.min_confidence = (entry.min_confidence + CALIBRATION_STEP).min(CONFIDENCE_CEIL);
        }
    }
}

// ── TrustConfigLoader (ARB-TD-01) ─────────────────────────────────────────────

/// Промежуточные serde-структуры для разбора секции `arbiter.trust` из genome.yaml.
mod yaml_schema {
    use serde::Deserialize;

    #[derive(Deserialize, Default)]
    pub struct GenomeArbiterWrapper {
        pub arbiter: Option<ArbiterSection>,
    }

    #[derive(Deserialize)]
    pub struct ArbiterSection {
        pub trust: Option<Vec<TrustEntryYaml>>,
        pub profile: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct TrustEntryYaml {
        pub source: u8,
        #[serde(rename = "type")]
        pub advisory_type: String,
        pub mode: String,
        pub min_confidence: f32,
    }
}

pub struct TrustConfigLoader;

impl TrustConfigLoader {
    /// Загрузить TrustConfig из секции `arbiter.trust` в genome.yaml.
    /// При ошибке или отсутствии секции → `TrustConfig::default_v1(0)`.
    pub fn from_genome_yaml(path: &Path) -> TrustConfig {
        Self::try_load(path).unwrap_or_else(|| TrustConfig::default_v1(0))
    }

    /// Вернуть имя профиля из `arbiter.profile` в genome.yaml, если задано.
    pub fn profile_name_from_genome_yaml(path: &Path) -> Option<String> {
        let content = std::fs::read_to_string(path).ok()?;
        let wrapper: yaml_schema::GenomeArbiterWrapper =
            serde_yaml::from_str(&content).ok()?;
        wrapper.arbiter?.profile
    }

    fn try_load(path: &Path) -> Option<TrustConfig> {
        let content = std::fs::read_to_string(path).ok()?;
        let wrapper: yaml_schema::GenomeArbiterWrapper =
            serde_yaml::from_str(&content).ok()?;
        let section = wrapper.arbiter?;
        let entries = section.trust?;

        let mut cfg = TrustConfig::new();
        for e in entries {
            let advisory_type = match e.advisory_type.as_str() {
                "DepthHint"             => AdvisoryType::DepthHint,
                "OctantCorrection"      => AdvisoryType::OctantCorrection,
                "ConflictDiagnosis"     => AdvisoryType::ConflictDiagnosis,
                "SubsystemAttribution"  => AdvisoryType::SubsystemAttribution,
                "EmergentCandidate"     => AdvisoryType::EmergentCandidate,
                "NarrativeShift"        => AdvisoryType::NarrativeShift,
                _ => continue,
            };
            let mode = match e.mode.as_str() {
                "AutoApply"           => TrustMode::AutoApply,
                "RequireConfirmation" => TrustMode::RequireConfirmation,
                "Ignore"              => TrustMode::Ignore,
                _ => continue,
            };
            cfg.set(e.source, advisory_type, TrustEntry::new(mode, e.min_confidence));
        }
        Some(cfg)
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

    #[test]
    fn test_calibrate_high_quality_decreases_threshold() {
        use super::super::log::{ArbiterLog, ArbiterLogEntry, ArbiterOutcome};

        let mut cfg = TrustConfig::default_v1(0);
        let initial = cfg.get(0, AdvisoryType::OctantCorrection).unwrap().min_confidence;

        let mut log = ArbiterLog::new();
        // 20 Confirmed → quality = 1.0 > CALIBRATION_HIGH
        for i in 0..20 {
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Confirmed,
            });
        }
        cfg.calibrate(0, AdvisoryType::OctantCorrection, &log);
        let after = cfg.get(0, AdvisoryType::OctantCorrection).unwrap().min_confidence;
        assert!(after < initial, "min_confidence should decrease on high quality");
        assert!((after - (initial - CALIBRATION_STEP)).abs() < 1e-5);
    }

    #[test]
    fn test_calibrate_low_quality_increases_threshold() {
        use super::super::log::{ArbiterLog, ArbiterLogEntry, ArbiterOutcome};

        let mut cfg = TrustConfig::default_v1(0);
        let initial = cfg.get(0, AdvisoryType::OctantCorrection).unwrap().min_confidence;

        let mut log = ArbiterLog::new();
        // 20 Rejected → quality = 0.0 < CALIBRATION_LOW
        for i in 0..20 {
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Rejected,
            });
        }
        cfg.calibrate(0, AdvisoryType::OctantCorrection, &log);
        let after = cfg.get(0, AdvisoryType::OctantCorrection).unwrap().min_confidence;
        assert!(after > initial, "min_confidence should increase on low quality");
        assert!((after - (initial + CALIBRATION_STEP)).abs() < 1e-5);
    }

    #[test]
    fn test_calibrate_medium_quality_no_change() {
        use super::super::log::{ArbiterLog, ArbiterLogEntry, ArbiterOutcome};

        let mut cfg = TrustConfig::default_v1(0);
        let initial = cfg.get(0, AdvisoryType::OctantCorrection).unwrap().min_confidence;

        let mut log = ArbiterLog::new();
        // 12 Confirmed, 8 Rejected → quality = 0.6 (between 0.4 and 0.8)
        for i in 0..12 {
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Confirmed,
            });
        }
        for i in 12..20 {
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Rejected,
            });
        }
        cfg.calibrate(0, AdvisoryType::OctantCorrection, &log);
        let after = cfg.get(0, AdvisoryType::OctantCorrection).unwrap().min_confidence;
        assert!((after - initial).abs() < 1e-5, "mid-range quality should not change threshold");
    }

    #[test]
    fn test_calibrate_floor_not_breached() {
        use super::super::log::{ArbiterLog, ArbiterLogEntry, ArbiterOutcome};

        let mut cfg = TrustConfig::default_v1(0);
        // Force threshold down to floor
        if let Some(entry) = cfg.entries.get_mut(&(0, AdvisoryType::OctantCorrection)) {
            entry.min_confidence = CONFIDENCE_FLOOR;
        }
        let mut log = ArbiterLog::new();
        for i in 0..20 {
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Confirmed,
            });
        }
        cfg.calibrate(0, AdvisoryType::OctantCorrection, &log);
        let after = cfg.get(0, AdvisoryType::OctantCorrection).unwrap().min_confidence;
        assert!((after - CONFIDENCE_FLOOR).abs() < 1e-5, "should not go below CONFIDENCE_FLOOR");
    }

    #[test]
    fn test_loader_fallback_on_missing_file() {
        let cfg = TrustConfigLoader::from_genome_yaml(Path::new("/nonexistent/path.yaml"));
        // Falls back to default_v1
        assert!(cfg.get(0, AdvisoryType::DepthHint).is_some());
    }
}
