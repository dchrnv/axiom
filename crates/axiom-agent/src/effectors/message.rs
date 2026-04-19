// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MessageEffector — форматирует ProcessingResult в диагностический текст.
//
// Четыре уровня детализации:
//   off — path + domain (2 строки)
//   min — текущий краткий вывод (5 строк, по умолчанию)
//   mid — routing + output без input-секции
//   max — полный вывод: input / routing / output

use axiom_runtime::result::{ProcessingResult, ProcessingPath};
pub use axiom_runtime::domain_name;

/// Уровень детализации вывода при обработке текстового ввода.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetailLevel {
    /// Только path + domain (2 строки)
    Off,
    /// Краткий вывод: path, domain, coherence, traces, position
    #[default]
    Min,
    /// Routing + output без секции input
    Mid,
    /// Полный вывод: input / routing / output
    Max,
}

impl DetailLevel {
    /// Разобрать уровень детализации из строки ("off" / "min" / "mid" / "max").
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "off"  => Some(Self::Off),
            "min"  => Some(Self::Min),
            "mid"  => Some(Self::Mid),
            "max"  => Some(Self::Max),
            _      => None,
        }
    }

    /// Вернуть строковое представление уровня.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Min => "min",
            Self::Mid => "mid",
            Self::Max => "max",
        }
    }
}

/// Форматирует результаты ядра в читаемый диагностический вывод.
pub struct MessageEffector;

impl MessageEffector {
    /// Создать новый `MessageEffector`.
    pub fn new() -> Self { Self }

    /// Форматировать ProcessingResult в строку для stdout.
    ///
    /// `detail` — уровень детализации.
    /// `input_text` — исходный текст ввода (для секции input в режиме max).
    pub fn format_result(
        &self,
        result: &ProcessingResult,
        detail: DetailLevel,
        input_text: Option<&str>,
    ) -> String {
        match detail {
            DetailLevel::Off => self.fmt_off(result),
            DetailLevel::Min => self.fmt_min(result),
            DetailLevel::Mid => self.fmt_mid(result),
            DetailLevel::Max => self.fmt_max(result, input_text),
        }
    }

    // ── off: 2 строки ────────────────────────────────────────────────────────

    fn fmt_off(&self, result: &ProcessingResult) -> String {
        let path = path_str(&result.path);
        format!(
            "  path:     {}\n  domain:   {} ({})\n",
            path,
            result.dominant_domain_id,
            domain_name(result.dominant_domain_id),
        )
    }

    // ── min: краткий (5-6 строк) ─────────────────────────────────────────────

    fn fmt_min(&self, result: &ProcessingResult) -> String {
        let mut out = String::with_capacity(160);
        out.push_str(&format!("  path:     {}\n", path_str(&result.path)));
        out.push_str(&format!("  domain:   {} ({})\n",
            result.dominant_domain_id, domain_name(result.dominant_domain_id)));
        if let Some(c) = result.coherence_score {
            out.push_str(&format!("  coherence:{:.2}\n", c));
        }
        if result.reflex_hit {
            out.push_str("  reflex:   hit\n");
        }
        if result.traces_matched > 0 {
            out.push_str(&format!("  traces:   {} matched\n", result.traces_matched));
        }
        if result.tension_count > 0 {
            out.push_str(&format!("  tension:  {} active\n", result.tension_count));
        }
        let [x, y, z] = result.output_position;
        out.push_str(&format!("  position: ({}, {}, {})\n", x, y, z));
        out
    }

    // ── mid: routing + output ────────────────────────────────────────────────

    fn fmt_mid(&self, result: &ProcessingResult) -> String {
        let mut out = String::with_capacity(320);

        // ── routing
        out.push_str("  ── routing ──────────────────────────\n");
        out.push_str(&format!("  path:       {}\n", path_str(&result.path)));
        if result.reflex_hit {
            out.push_str("  reflex:     hit\n");
        }
        if let Some(c) = result.coherence_score {
            out.push_str(&format!("  confidence: {:.2}\n", c));
        }
        out.push_str(&format!("  traces:     {} matched (of {} total)\n",
            result.traces_matched, result.total_traces));
        out.push_str(&format!("  passes:     {} (max: {})\n",
            result.passes.max(1), result.max_passes));

        // ── output
        out.push_str("  ── output ───────────────────────────\n");
        out.push_str(&format!("  domain:     {} ({})\n",
            result.dominant_domain_id, domain_name(result.dominant_domain_id)));
        if let Some(c) = result.coherence_score {
            out.push_str(&format!("  coherence:  {:.2} (threshold: {:.2})\n", c, result.min_coherence));
        }
        let [x, y, z] = result.output_position;
        out.push_str(&format!("  position:   ({}, {}, {})\n", x, y, z));
        out.push_str(&format!("  tension:    created={}\n",
            if result.tension_created { "true" } else { "false" }));

        out
    }

    // ── max: полный ──────────────────────────────────────────────────────────

    fn fmt_max(&self, result: &ProcessingResult, input_text: Option<&str>) -> String {
        let mut out = String::with_capacity(512);

        // ── input
        out.push_str("  ── input ────────────────────────────\n");
        if let Some(text) = input_text {
            out.push_str(&format!("  text:       {:?}\n", text));
        }
        out.push_str(&format!("  hash:       {:#018x}\n", result.input_hash));
        let [ix, iy, iz] = result.input_position;
        let [_, _, _, valence, temp, mass, _, _] = result.input_shell;
        out.push_str(&format!("  token:      pos=({},{},{}) mass={} temp={} valence={}\n",
            ix, iy, iz, mass, temp, valence));
        out.push_str(&format!("  shell:      {:?}\n", result.input_shell));

        // ── routing
        out.push_str("  ── routing ──────────────────────────\n");
        let path_icon = match &result.path {
            ProcessingPath::Reflex      => "⚡ reflex",
            ProcessingPath::SlowPath    => "slow-path",
            ProcessingPath::MultiPass(_)=> "multi-pass",
        };
        out.push_str(&format!("  path:       {}\n", path_icon));
        out.push_str(&format!("  reflex_hit: {}\n", result.reflex_hit));
        if let Some(c) = result.coherence_score {
            out.push_str(&format!("  confidence: {:.2}\n", c));
        }
        out.push_str(&format!("  traces:     {} matched (of {} total)\n",
            result.traces_matched, result.total_traces));
        out.push_str(&format!("  passes:     {} (max: {})\n",
            result.passes.max(1), result.max_passes));

        // ── output
        out.push_str("  ── output ───────────────────────────\n");
        out.push_str(&format!("  dominant:   {} ({})\n",
            result.dominant_domain_id, domain_name(result.dominant_domain_id)));
        if let Some(c) = result.coherence_score {
            out.push_str(&format!("  coherence:  {:.2} (threshold: {:.2})\n", c, result.min_coherence));
        }
        let [ox, oy, oz] = result.output_position;
        out.push_str(&format!("  position:   ({}, {}, {})\n", ox, oy, oz));
        out.push_str(&format!("  shell:      {:?}\n", result.output_shell));

        // Δpos
        let dx = ox as i32 - ix as i32;
        let dy = oy as i32 - iy as i32;
        let dz = oz as i32 - iz as i32;
        out.push_str(&format!("  Δpos:       ({:+}, {:+}, {:+})\n", dx, dy, dz));

        out.push_str(&format!("  event_id:   {}\n", result.event_id));
        out.push_str(&format!("  tension:    created={}\n",
            if result.tension_created { "true" } else { "false" }));

        out
    }
}

/// Форматировать ProcessingPath в строку.
fn path_str(path: &ProcessingPath) -> String {
    match path {
        ProcessingPath::Reflex       => "reflex".to_string(),
        ProcessingPath::SlowPath     => "slow-path".to_string(),
        ProcessingPath::MultiPass(n) => format!("multi-pass({})", n),
    }
}

impl Default for MessageEffector {
    fn default() -> Self { Self::new() }
}


#[cfg(test)]
mod tests {
    use super::*;
    use axiom_runtime::result::{ProcessingResult, ProcessingPath};
    use axiom_ucl::UclResult;

    fn make_result(path: ProcessingPath, reflex: bool, coherence: f32, domain: u16) -> ProcessingResult {
        ProcessingResult {
            ucl_result: UclResult::success(0),
            path,
            dominant_domain_id: domain,
            coherence_score: Some(coherence),
            tension_count: 0,
            output_shell: [0u8; 8],
            output_position: [10, 20, 30],
            reflex_hit: reflex,
            traces_matched: 0,
            passes: 1,
            max_passes: 3,
            min_coherence: 0.6,
            total_traces: 47,
            event_id: 12345,
            input_position: [5, 10, 15],
            input_shell: [0, 0, 0, 0, 200, 100, 0, 0],
            input_hash: 0xA7F3B2C1E4D5F6A8,
            tension_created: false,
        }
    }

    #[test]
    fn test_format_reflex_min() {
        let e = MessageEffector::new();
        let r = make_result(ProcessingPath::Reflex, true, 0.95, 110);
        let out = e.format_result(&r, DetailLevel::Min, None);
        assert!(out.contains("reflex"), "output: {}", out);
        assert!(out.contains("MAYA"),   "output: {}", out);
        assert!(out.contains("reflex:   hit"), "output: {}", out);
    }

    #[test]
    fn test_format_multi_pass_min() {
        let e = MessageEffector::new();
        let r = make_result(ProcessingPath::MultiPass(3), false, 0.45, 106);
        let out = e.format_result(&r, DetailLevel::Min, None);
        assert!(out.contains("multi-pass(3)"), "output: {}", out);
        assert!(out.contains("LOGIC"), "output: {}", out);
    }

    #[test]
    fn test_format_max_has_sections() {
        let e = MessageEffector::new();
        let r = make_result(ProcessingPath::SlowPath, false, 0.7, 101);
        let out = e.format_result(&r, DetailLevel::Max, Some("hello"));
        assert!(out.contains("── input"), "output: {}", out);
        assert!(out.contains("── routing"), "output: {}", out);
        assert!(out.contains("── output"), "output: {}", out);
        assert!(out.contains("\"hello\""), "output: {}", out);
        assert!(out.contains("Δpos:"), "output: {}", out);
        assert!(out.contains("event_id:"), "output: {}", out);
    }

    #[test]
    fn test_format_off() {
        let e = MessageEffector::new();
        let r = make_result(ProcessingPath::Reflex, true, 1.0, 110);
        let out = e.format_result(&r, DetailLevel::Off, None);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2, "off should be 2 lines: {:?}", lines);
    }

    #[test]
    fn test_domain_name_logic() {
        assert_eq!(domain_name(106), "LOGIC");
    }

    #[test]
    fn test_domain_name_maya() {
        assert_eq!(domain_name(110), "MAYA");
    }

    #[test]
    fn test_domain_name_unknown() {
        assert_eq!(domain_name(999), "UNKNOWN");
    }

    #[test]
    fn test_position_in_output() {
        let e = MessageEffector::new();
        let r = make_result(ProcessingPath::SlowPath, false, 0.7, 110);
        let out = e.format_result(&r, DetailLevel::Min, None);
        assert!(out.contains("(10, 20, 30)"), "output: {}", out);
    }
}
