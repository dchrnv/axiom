// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ASHTI Processor - обработка токенов через ASHTI 1-8 домены

use crate::experience::ExperienceTrace;
use axiom_config::DomainConfig;
use axiom_core::{Token, TOKEN_FLAG_GOAL};

/// ASHTI Processor - обработка токенов через ASHTI 1-8 домены
pub struct AshtiProcessor;

impl AshtiProcessor {
    /// Обработать токен через ASHTI домен.
    ///
    /// Если передана подсказка (hint) из Experience, числовые поля токена
    /// смещаются в сторону паттерна подсказки пропорционально весу следа
    /// и шагу обратной связи домена (feedback_weight_delta).
    ///
    /// Затем применяется специализация домена по structural_role.
    pub fn process_token(
        token: &Token,
        domain: &DomainConfig,
        hint: Option<&ExperienceTrace>,
    ) -> Token {
        let mut out = *token;

        // --- Hint blending (если есть подсказка из Experience) ---
        if let Some(trace) = hint {
            let alpha = blend_alpha(trace.weight, domain.feedback_weight_delta);
            let p = &trace.pattern;

            // Blend temperature
            out.temperature = lerp_u8(token.temperature, p.temperature, alpha);
            // Blend mass
            out.mass = lerp_u8(token.mass, p.mass, alpha);
            // Blend valence
            out.valence = lerp_i8(token.valence, p.valence, alpha);
            // Blend position toward hint pattern
            for i in 0..3 {
                out.position[i] = lerp_i16(token.position[i], p.position[i], alpha);
            }
        }

        // --- Domain-specific transformation by structural_role ---
        match domain.structural_role {
            1 => apply_spatial(domain, &mut out),
            2 => apply_temporal(domain, &mut out),
            3 => apply_logical(domain, &mut out),
            4 => apply_semantic(domain, &mut out),
            5 => apply_thermal(domain, &mut out),
            6 => apply_causal(domain, &mut out),
            7 => apply_resonant(domain, &mut out),
            8 => apply_meta(domain, &mut out),
            _ => {} // Unknown role — pass through
        }

        // Tag with processing domain
        out.domain_id = domain.domain_id;
        out
    }
}

/// Вычислить коэффициент смешивания: trace.weight * (feedback_weight_delta / 255.0)
fn blend_alpha(trace_weight: f32, feedback_delta: u8) -> f32 {
    (trace_weight * (feedback_delta as f32 / 255.0)).clamp(0.0, 0.5)
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

fn lerp_i8(a: i8, b: i8, t: f32) -> i8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as i8
}

fn lerp_i16(a: i16, b: i16, t: f32) -> i16 {
    (a as f32 + (b as f32 - a as f32) * t).round() as i16
}

// --- Специализации по structural_role ---

/// Role 1 — пространственная обработка: движение к target
fn apply_spatial(_domain: &DomainConfig, token: &mut Token) {
    // Nudge position toward target by one step
    for i in 0..3 {
        let diff = token.target[i] - token.position[i];
        token.position[i] = token.position[i].saturating_add(diff.signum());
    }
}

/// Role 2 — временная обработка: сброс last_event_id
fn apply_temporal(domain: &DomainConfig, token: &mut Token) {
    // Stamp the domain's event epoch
    token.last_event_id = domain.created_at;
}

/// Role 3 — логическая обработка (CODEX): маскирование type_flags + goal physics.
///
/// Если токен помечен TOKEN_FLAG_GOAL (Cognitive Depth V1.0 — 13D):
/// - Повышает mass (предотвращает вытеснение из памяти)
/// - Повышает temperature (удерживает токен активным)
fn apply_logical(domain: &DomainConfig, token: &mut Token) {
    // Apply domain arbiter_flags as type_flags mask
    let mask = (domain.arbiter_flags as u16) << 8 | domain.arbiter_flags as u16;
    if mask != 0 {
        token.type_flags |= mask;
    }

    // GOAL physics: цель нельзя забыть — повышаем mass и temperature
    if token.type_flags & TOKEN_FLAG_GOAL != 0 {
        token.mass = token.mass.saturating_add(20);
        token.temperature = token.temperature.saturating_add(15);
    }
}

/// Role 4 — семантическая обработка: масштабирование валентности
fn apply_semantic(domain: &DomainConfig, token: &mut Token) {
    // Scale valence by domain permeability
    let scale = domain.permeability as f32 / 255.0;
    token.valence = (token.valence as f32 * scale).round() as i8;
}

/// Role 5 — термическая обработка: смещение температуры к домену
fn apply_thermal(domain: &DomainConfig, token: &mut Token) {
    let domain_temp = (domain.temperature.clamp(0.0, 255.0)) as u8;
    token.temperature = lerp_u8(token.temperature, domain_temp, 0.1);
}

/// Role 6 — причинная обработка: обновление lineage_hash
fn apply_causal(domain: &DomainConfig, token: &mut Token) {
    // Mix domain_id into lineage_hash for provenance tracking
    let mut h = token.lineage_hash;
    h ^= domain.domain_id as u64;
    h = h.wrapping_mul(0x100000001b3);
    token.lineage_hash = h;
}

/// Role 7 — резонансная обработка: масштабирование resonance
fn apply_resonant(domain: &DomainConfig, token: &mut Token) {
    let scale = domain.permeability as f32 / 255.0;
    token.resonance = (token.resonance as f32 * scale).round() as u32;
}

/// Role 8 — мета-обработка: лёгкое смешивание mass и temperature
fn apply_meta(domain: &DomainConfig, token: &mut Token) {
    let domain_temp = (domain.temperature.clamp(0.0, 255.0)) as u8;
    token.temperature = lerp_u8(token.temperature, domain_temp, 0.05);
    let scale = domain.permeability as f32 / 255.0;
    token.mass = lerp_u8(token.mass, (token.mass as f32 * scale) as u8, 0.05);
}
