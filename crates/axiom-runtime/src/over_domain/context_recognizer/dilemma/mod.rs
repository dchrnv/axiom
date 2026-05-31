// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DilemmaDetector V2.0 — под-механизм ContextRecognizer.
// Источник: DilemmaDetector_V2_0.md

pub mod detector;
pub mod tension;

pub use detector::DilemmaDetector;
pub use tension::{compute_tension_score, DILEMMA_THRESHOLD};
