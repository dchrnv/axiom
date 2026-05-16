// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Сканирование MAYA по ScanningPlan (octant × depth_range × FractalLevel).
// Источник: ContextRecognizer_V5_0.md §4

use axiom_core::{Token, STATE_ACTIVE};
use axiom_domain::DomainState;
use axiom_experience::{Octant, SutraDepthEntry};
use std::collections::HashMap;

use crate::over_domain::context_recognizer::scanning_plan::{ActiveRegion, DepthRange};

/// Результат сканирования одного региона.
#[derive(Debug)]
pub struct ScanResult {
    pub octant: Octant,
    pub tokens: Vec<Token>,
    pub token_count: usize,
    pub total_mass: u32,
}

/// Сканировать домен по активному региону.
///
/// Фильтрует активные токены MAYA у которых глубина sutra_id попадает в depth_range региона.
/// Если sutra_id не найден в depth_cache — токен включается (нет данных = включаем).
pub fn scan_region(
    state: &DomainState,
    region: &ActiveRegion,
    depth_cache: &HashMap<u32, SutraDepthEntry>,
) -> ScanResult {
    let tokens: Vec<Token> = state
        .tokens
        .iter()
        .filter(|t| {
            t.state == STATE_ACTIVE && is_in_depth_range(t.sutra_id, &region.depth_range, depth_cache)
        })
        .cloned()
        .collect();

    let total_mass: u32 = tokens.iter().map(|t| t.mass as u32).sum();
    let token_count = tokens.len();

    ScanResult {
        octant: region.octant,
        tokens,
        token_count,
        total_mass,
    }
}

/// Проверить входит ли sutra_id в depth_range.
///
/// Если sutra_id не в cache → True (токен не классифицирован — включаем для анализа).
fn is_in_depth_range(
    sutra_id: u32,
    range: &DepthRange,
    depth_cache: &HashMap<u32, SutraDepthEntry>,
) -> bool {
    match depth_cache.get(&sutra_id) {
        None => true, // неизвестная глубина → включить
        Some(entry) => {
            // Проверяем максимальную глубину по всем октантам
            let max_depth = entry.depth_per_octant.iter().copied().max().unwrap_or(0);
            range.contains(max_depth)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_domain::DomainState;
    use axiom_experience::SutraDepthEntry;

    use crate::over_domain::context_recognizer::scanning_plan::{ActiveRegion, DepthRange};
    use axiom_experience::Octant;

    fn make_state_with_tokens(tokens: Vec<Token>) -> DomainState {
        let mut config = axiom_config::DomainConfig::default_void();
        config.token_capacity = 256;
        let mut state = DomainState::new(&config);
        for t in tokens {
            let _ = state.add_token(t);
        }
        state
    }

    fn tok(sutra_id: u32) -> Token {
        Token::new(sutra_id, 10, [0, 0, 0], 0)
    }

    fn region(min: u16, max: u16) -> ActiveRegion {
        ActiveRegion::new(Octant::CreativeAffirmation, DepthRange { min, max }, 200)
    }

    fn depth_entry(sutra_id: u32, depth: u16) -> (u32, SutraDepthEntry) {
        let mut entry = SutraDepthEntry {
            sutra_id,
            depth_per_octant: [0u16; 8],
            last_settle_event: 0,
            reactivation_count: 0,
        };
        entry.depth_per_octant[0] = depth;
        (sutra_id, entry)
    }

    #[test]
    fn test_scan_empty_domain() {
        let state = make_state_with_tokens(vec![]);
        let r = region(0, 10000);
        let result = scan_region(&state, &r, &HashMap::new());
        assert_eq!(result.token_count, 0);
        assert_eq!(result.total_mass, 0);
    }

    #[test]
    fn test_scan_includes_unknown_depth_tokens() {
        let state = make_state_with_tokens(vec![tok(1), tok(2)]);
        let r = region(1000, 5000);
        // Empty depth_cache → all tokens included
        let result = scan_region(&state, &r, &HashMap::new());
        assert_eq!(result.token_count, 2);
    }

    #[test]
    fn test_scan_filters_by_depth_range() {
        let deep_tok = tok(1);
        let shallow_tok = tok(2);
        let state = make_state_with_tokens(vec![deep_tok, shallow_tok]);

        let mut cache = HashMap::new();
        cache.insert(1u32, depth_entry(1, 5000).1); // depth 5000 — in [1000..10000]
        cache.insert(2u32, depth_entry(2, 50).1);   // depth 50 — NOT in [1000..10000]

        let r = region(1000, 10000);
        let result = scan_region(&state, &r, &cache);
        // Only sutra_id=1 should be included
        assert_eq!(result.token_count, 1);
        assert_eq!(result.tokens[0].sutra_id, 1);
    }

    #[test]
    fn test_scan_result_has_correct_octant() {
        let state = make_state_with_tokens(vec![]);
        let r = region(0, u16::MAX);
        let result = scan_region(&state, &r, &HashMap::new());
        assert_eq!(result.octant, Octant::CreativeAffirmation);
    }
}
