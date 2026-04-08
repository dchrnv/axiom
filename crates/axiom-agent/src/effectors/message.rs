// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MessageEffector — форматирует ProcessingResult в диагностический текст.
//
// Вывод специально диагностический, не нарратив. Показывает как ядро
// обработало ввод: путь, домен, когерентность, tension.

use axiom_runtime::result::{ProcessingResult, ProcessingPath};

/// Форматирует результаты ядра в читаемый диагностический вывод.
pub struct MessageEffector;

impl MessageEffector {
    /// Создать новый MessageEffector.
    pub fn new() -> Self {
        Self
    }

    /// Форматировать ProcessingResult в строку для stdout.
    pub fn format_result(&self, result: &ProcessingResult) -> String {
        let mut out = String::with_capacity(128);

        let path_str = match &result.path {
            ProcessingPath::Reflex       => "reflex".to_string(),
            ProcessingPath::SlowPath     => "slow-path".to_string(),
            ProcessingPath::MultiPass(n) => format!("multi-pass({})", n),
        };
        out.push_str(&format!("  path:     {}\n", path_str));

        out.push_str(&format!("  domain:   {} ({})\n",
            result.dominant_domain_id,
            domain_name(result.dominant_domain_id)));

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
}

impl Default for MessageEffector {
    fn default() -> Self {
        Self::new()
    }
}

/// Маппинг domain_id → имя домена.
/// domain_id = level_id * 100 + offset; offset 0..=10.
pub fn domain_name(id: u16) -> &'static str {
    match id % 100 {
        0  => "SUTRA",
        1  => "EXECUTION",
        2  => "SHADOW",
        3  => "CODEX",
        4  => "MAP",
        5  => "PROBE",
        6  => "LOGIC",
        7  => "DREAM",
        8  => "ETHICS",
        9  => "EXPERIENCE",
        10 => "MAYA",
        _  => "UNKNOWN",
    }
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
        }
    }

    #[test]
    fn test_format_reflex() {
        let e = MessageEffector::new();
        let r = make_result(ProcessingPath::Reflex, true, 0.95, 110);
        let out = e.format_result(&r);
        assert!(out.contains("reflex"), "output: {}", out);
        assert!(out.contains("MAYA"),   "output: {}", out);
        assert!(out.contains("reflex:   hit"), "output: {}", out);
    }

    #[test]
    fn test_format_multi_pass() {
        let e = MessageEffector::new();
        let r = make_result(ProcessingPath::MultiPass(3), false, 0.45, 106);
        let out = e.format_result(&r);
        assert!(out.contains("multi-pass(3)"), "output: {}", out);
        assert!(out.contains("LOGIC"), "output: {}", out);
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
        let out = e.format_result(&r);
        assert!(out.contains("(10, 20, 30)"), "output: {}", out);
    }
}
