// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use crate::over_domain::sensorium::state::SensoriumState;

/// Структурированное выражение из среза — V1: детерминированная функция.
///
/// Строится напрямую из полей SensoriumState — каждое слово проверяемо.
/// Честно и суховато. Архитектурно открыта вверх: в V2.0 эту структуру
/// можно передать в языковой слой / маленькую обученную модель (не меняя Sensorium).
///
/// Граница (GENOME): выражение — это всё что Sensorium делает наружу.
/// Увидел → сказал. Не переключил, не вызвал, не погасил.
#[derive(Clone, Debug, Default)]
pub struct SensoriumExpression {
    /// Основная тема (доминирующая подсистема, если есть).
    pub dominant_theme: Option<String>,
    /// Есть ли напряжение (дилемма или Corpus Callosum).
    pub has_tension: bool,
    /// Описание усталости (если есть).
    pub fatigue_signal: Option<String>,
    /// Фаза DREAM в текстовом виде.
    pub phase_label: &'static str,
    /// Суммарная строка: что сейчас происходит.
    pub summary: String,
}

/// Построить выражение из текущего среза — детерминированная функция §11.
pub fn express(state: &SensoriumState) -> SensoriumExpression {
    let dominant_theme = state.dominant_subsystem.map(|s| format!("{s:?}"));
    let has_tension = state.active_dilemma_count > 0 || state.corpus_callosum_active;

    let fatigue_signal = if !state.fatigued_subsystems.is_empty() {
        let names: Vec<_> = state
            .fatigued_subsystems
            .iter()
            .map(|s| format!("{s:?}"))
            .collect();
        Some(names.join(", "))
    } else {
        None
    };

    let phase_label = match state.dream_phase_raw {
        0 => "Wake",
        1 => "FallingAsleep",
        2 => "Dreaming",
        3 => "Waking",
        _ => "Unknown",
    };

    let summary = build_summary(
        &dominant_theme,
        has_tension,
        state.active_dilemma_count,
        &fatigue_signal,
        state.candidates_count,
        phase_label,
    );

    SensoriumExpression {
        dominant_theme,
        has_tension,
        fatigue_signal,
        phase_label,
        summary,
    }
}

fn build_summary(
    theme: &Option<String>,
    has_tension: bool,
    dilemma_count: usize,
    fatigue: &Option<String>,
    candidates: usize,
    phase: &str,
) -> String {
    let mut parts = Vec::new();

    if phase != "Wake" {
        parts.push(format!("[{phase}]"));
    }

    match theme {
        Some(t) if has_tension && dilemma_count > 0 => {
            parts.push(format!("tension in {t} ({dilemma_count} dilemma(s))"));
        }
        Some(t) if has_tension => {
            parts.push(format!("conflict in {t}"));
        }
        Some(t) => {
            parts.push(t.clone());
        }
        None if has_tension => {
            parts.push("unresolved tension".to_string());
        }
        None => {
            parts.push("idle".to_string());
        }
    }

    if candidates > 0 {
        parts.push(format!("{candidates} frame candidate(s)"));
    }

    if let Some(f) = fatigue {
        parts.push(format!("fatigue: {f}"));
    }

    parts.join(" | ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn express_idle_state() {
        let state = SensoriumState::default();
        let expr = express(&state);
        assert_eq!(expr.has_tension, false);
        assert!(expr.dominant_theme.is_none());
        assert!(expr.summary.contains("idle"));
    }

    #[test]
    fn express_tension_with_theme() {
        use axiom_experience::SubsystemId;
        let mut state = SensoriumState::default();
        state.dominant_subsystem = Some(SubsystemId::Mathematics);
        state.active_dilemma_count = 2;
        let expr = express(&state);
        assert!(expr.has_tension);
        assert!(expr.summary.contains("tension"));
        assert!(expr.summary.contains("Mathematics"));
    }

    #[test]
    fn express_dreaming_phase() {
        let mut state = SensoriumState::default();
        state.dream_phase_raw = 2;
        let expr = express(&state);
        assert_eq!(expr.phase_label, "Dreaming");
        assert!(expr.summary.contains("[Dreaming]"));
    }
}
