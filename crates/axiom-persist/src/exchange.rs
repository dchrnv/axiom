// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// exchange.rs — обмен знаниями между экземплярами AXIOM.
//
// `:export traces/skills <path>` → сохранить в JSON
// `:import traces/skills <path>` → загрузить + GUARDIAN валидация + weight factor
//
// Формат файла: ExchangePackage (JSON), один файл.
// Manifest не требуется — exchange package самодостаточен.

use serde::{Deserialize, Serialize};
use std::path::Path;
use axiom_arbiter::{ExperienceTrace, Skill};
use axiom_runtime::{AxiomEngine, guardian::ReflexDecision};
use crate::error::PersistError;
use crate::format::StoredTrace;
use crate::loader::IMPORT_WEIGHT_FACTOR;

// ─── Формат файла ─────────────────────────────────────────────────────────────

/// Тип содержимого пакета обмена.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ExchangeKind {
    /// ExperienceTraces (все веса выше порога)
    Traces,
    /// Кристаллизованные навыки (Skills)
    Skills,
}

/// Заголовок пакета обмена.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeHeader {
    /// Тип содержимого
    pub kind: ExchangeKind,
    /// Версия формата
    pub version: String,
    /// tick_count источника
    pub source_tick: u64,
    /// Дата экспорта
    pub exported_at: String,
    /// Число элементов
    pub count: u32,
}

/// Пакет для обмена ExperienceTraces.
#[derive(Debug, Serialize, Deserialize)]
pub struct TracePackage {
    pub header: ExchangeHeader,
    pub traces: Vec<StoredTrace>,
}

/// Хранимый Skill для exchange (зеркало Skill с serde).
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredSkill {
    pub pattern:          axiom_core::Token,
    pub activation_weight: f32,
    pub created_at:       u64,
    pub success_count:    u32,
    pub pattern_hash:     u64,
}

impl From<&Skill> for StoredSkill {
    fn from(s: &Skill) -> Self {
        Self {
            pattern:           s.pattern,
            activation_weight: s.activation_weight,
            created_at:        s.created_at,
            success_count:     s.success_count,
            pattern_hash:      s.pattern_hash,
        }
    }
}

impl From<StoredSkill> for Skill {
    fn from(s: StoredSkill) -> Self {
        Self {
            pattern:           s.pattern,
            activation_weight: s.activation_weight,
            created_at:        s.created_at,
            success_count:     s.success_count,
            pattern_hash:      s.pattern_hash,
        }
    }
}

/// Пакет для обмена Skills.
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillPackage {
    pub header: ExchangeHeader,
    pub skills: Vec<StoredSkill>,
}

// ─── Отчёт об импорте ─────────────────────────────────────────────────────────

/// Результат импорта.
#[derive(Debug, Default)]
pub struct ImportReport {
    /// Число элементов в пакете
    pub total: u32,
    /// Импортировано
    pub imported: u32,
    /// Отклонено GUARDIAN (CODEX-нарушение)
    pub guardian_rejected: u32,
    /// Пропущено (дубликат)
    pub skipped_duplicate: u32,
}

impl ImportReport {
    pub fn summary_line(&self) -> String {
        format!(
            "imported={} rejected_by_guardian={} skipped_dup={} (total={})",
            self.imported, self.guardian_rejected, self.skipped_duplicate, self.total
        )
    }
}

/// Результат экспорта.
#[derive(Debug)]
pub struct ExportReport {
    pub kind: ExchangeKind,
    pub exported: u32,
    pub path: String,
}

// ─── EXPORT ───────────────────────────────────────────────────────────────────

/// Экспортировать ExperienceTraces с weight ≥ threshold в JSON-файл.
pub fn export_traces(
    engine: &AxiomEngine,
    path: &Path,
    weight_threshold: f32,
) -> Result<ExportReport, PersistError> {
    let traces: Vec<StoredTrace> = engine.ashti.experience()
        .traces()
        .iter()
        .filter(|t| t.weight >= weight_threshold)
        .map(StoredTrace::from)
        .collect();

    let count = traces.len() as u32;
    let pkg = TracePackage {
        header: make_header(ExchangeKind::Traces, engine.tick_count, count),
        traces,
    };

    write_json(path, &pkg)?;
    Ok(ExportReport {
        kind: ExchangeKind::Traces,
        exported: count,
        path: path.display().to_string(),
    })
}

/// Экспортировать Skills (кристаллизованные навыки) в JSON-файл.
pub fn export_skills(engine: &AxiomEngine, path: &Path) -> Result<ExportReport, PersistError> {
    let skills: Vec<StoredSkill> = engine.ashti.export_skills()
        .iter()
        .map(StoredSkill::from)
        .collect();

    let count = skills.len() as u32;
    let pkg = SkillPackage {
        header: make_header(ExchangeKind::Skills, engine.tick_count, count),
        skills,
    };

    write_json(path, &pkg)?;
    Ok(ExportReport {
        kind: ExchangeKind::Skills,
        exported: count,
        path: path.display().to_string(),
    })
}

// ─── IMPORT ───────────────────────────────────────────────────────────────────

/// Импортировать ExperienceTraces из JSON-файла.
///
/// GUARDIAN-валидация: каждый trace.pattern проверяется через `validate_reflex`.
/// Принятые traces получают weight × IMPORT_WEIGHT_FACTOR (0.7).
pub fn import_traces(engine: &mut AxiomEngine, path: &Path) -> Result<ImportReport, PersistError> {
    let bytes = std::fs::read(path)?;
    let pkg: TracePackage = serde_json::from_slice(&bytes)
        .map_err(|e| PersistError::Decode(e.to_string()))?;

    if pkg.header.kind != ExchangeKind::Traces {
        return Err(PersistError::Decode(
            format!("expected traces package, got {:?}", pkg.header.kind)
        ));
    }

    let mut report = ImportReport { total: pkg.header.count, ..Default::default() };

    for stored in pkg.traces {
        // GUARDIAN валидация
        match engine.guardian.validate_reflex(&stored.pattern) {
            ReflexDecision::Veto(_) => {
                report.guardian_rejected += 1;
                continue;
            }
            ReflexDecision::Allow => {}
        }

        let trace = apply_trace_factor(stored);
        engine.ashti.experience_mut().import_trace(trace.into());
        report.imported += 1;
    }

    Ok(report)
}

/// Импортировать Skills из JSON-файла.
///
/// GUARDIAN-валидация: каждый skill.pattern проверяется через `validate_reflex`.
/// Принятые skills получают activation_weight × IMPORT_WEIGHT_FACTOR (0.7).
pub fn import_skills(engine: &mut AxiomEngine, path: &Path) -> Result<ImportReport, PersistError> {
    let bytes = std::fs::read(path)?;
    let pkg: SkillPackage = serde_json::from_slice(&bytes)
        .map_err(|e| PersistError::Decode(e.to_string()))?;

    if pkg.header.kind != ExchangeKind::Skills {
        return Err(PersistError::Decode(
            format!("expected skills package, got {:?}", pkg.header.kind)
        ));
    }

    let mut report = ImportReport { total: pkg.header.count, ..Default::default() };

    for stored in pkg.skills {
        // GUARDIAN валидация
        match engine.guardian.validate_reflex(&stored.pattern) {
            ReflexDecision::Veto(_) => {
                report.guardian_rejected += 1;
                continue;
            }
            ReflexDecision::Allow => {}
        }

        let skill: Skill = stored.into();
        let added = engine.ashti.import_skill_exchange(skill, IMPORT_WEIGHT_FACTOR);
        if added {
            report.imported += 1;
        } else {
            report.skipped_duplicate += 1;
        }
    }

    Ok(report)
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn make_header(kind: ExchangeKind, source_tick: u64, count: u32) -> ExchangeHeader {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs()).unwrap_or(0);
    ExchangeHeader {
        kind,
        version: "axiom-exchange-v1".to_string(),
        source_tick,
        exported_at: format!("{}", secs),
        count,
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), PersistError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let json = serde_json::to_string(value)
        .map_err(|e| PersistError::Encode(e.to_string()))?;
    std::fs::write(path, json.as_bytes())?;
    Ok(())
}

fn apply_trace_factor(mut stored: StoredTrace) -> StoredTrace {
    stored.weight = (stored.weight * IMPORT_WEIGHT_FACTOR).max(0.001);
    stored
}
