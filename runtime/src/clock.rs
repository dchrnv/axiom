// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// COM: CausalClock — docs/spec/CAUSAL ORDER MODEL (COM).md

use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Глобальный причинный счётчик. event_id строго монотонно возрастает.
#[derive(Debug, Default)]
pub struct CausalClock;

impl CausalClock {
    /// Возвращает следующий event_id (атомарно).
    #[inline]
    pub fn next() -> u64 {
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    /// Текущее значение счётчика (без инкремента).
    #[inline]
    pub fn current() -> u64 {
        COUNTER.load(Ordering::SeqCst)
    }

    /// Сброс счётчика (только для тестов).
    #[doc(hidden)]
    pub fn reset_for_test() {
        COUNTER.store(0, Ordering::SeqCst);
    }
}
