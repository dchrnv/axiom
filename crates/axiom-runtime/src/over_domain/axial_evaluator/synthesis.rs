// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Синтетическое распознавание октанта — через физическую подпись токена.
//
// R2 (REPAIR-01 N2): октант ТОЛЬКО из mass/valence/temperature.
// Позиция координат НИКОГДА не несёт природу токена (N1/N2).
//
// Аналитический путь (metrics.rs): entropy/density/will из активности.
// Синтетический путь (этот файл): подпись (mass/valence/temp) участников Frame.
// Corpus Callosum (mod.rs:184): сравнивает обе оценки — оба пути на подписи.

use axiom_core::Token;
use axiom_experience::{AxialScore, Octant};

/// Синтезировать октант Frame через агрегат физических подписей участников.
///
/// N2: природа — только из mass/valence/temperature.
pub fn synthesize_octant(participants: &[Token], anchor: &Token) -> Octant {
    if participants.is_empty() {
        return octant_from_signature(anchor);
    }

    let all_count = participants.len() + 1;
    let n = all_count as f32;

    let mass_sum: f32 = std::iter::once(anchor)
        .chain(participants.iter())
        .map(|t| t.mass as f32)
        .sum();
    let valence_sum: f32 = std::iter::once(anchor)
        .chain(participants.iter())
        .map(|t| t.valence as f32)
        .sum();
    let temp_sum: f32 = std::iter::once(anchor)
        .chain(participants.iter())
        .map(|t| t.temperature as f32)
        .sum();

    let (x, y, z) = axis_scores_from_components(mass_sum / n, valence_sum / n, temp_sum / n);
    Octant::from_scores(&x, &y, &z)
}

fn octant_from_signature(token: &Token) -> Octant {
    let (x, y, z) = axis_scores_from_signature(token);
    Octant::from_scores(&x, &y, &z)
}

/// Вычислить axis scores из физической подписи токена.
///
/// N2 mapping:
///   mass (u8 0..255)     → Apollo (high) / Dionysus (low)
///   valence (i8 -128..127) → Eros (positive) / Thanatos (negative)
///   temperature (u8 0..255) → Will (high) / Nothing (low)
pub fn axis_scores_from_signature(token: &Token) -> (AxialScore, AxialScore, AxialScore) {
    axis_scores_from_components(
        token.mass as f32,
        token.valence as f32,
        token.temperature as f32,
    )
}

fn axis_scores_from_components(mass: f32, valence: f32, temp: f32) -> (AxialScore, AxialScore, AxialScore) {
    let apollo   = mass.clamp(0.0, 255.0) as u8;
    let dionysus = (255.0 - mass).clamp(0.0, 255.0) as u8;

    let eros = if valence > 0.0 {
        (valence * 2.0).clamp(0.0, 255.0) as u8
    } else { 0 };
    let thanatos = if valence < 0.0 {
        ((-valence) * 2.0).clamp(0.0, 255.0) as u8
    } else { 0 };

    let will    = temp.clamp(0.0, 255.0) as u8;
    let nothing = (255.0 - temp).clamp(0.0, 255.0) as u8;

    (
        AxialScore::new(apollo, dionysus),
        AxialScore::new(eros, thanatos),
        AxialScore::new(will, nothing),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tok(mass: u8, valence: i8, temperature: u8) -> Token {
        let mut t = Token::new(1, 100, [16000, 16000, 16000], 0);
        t.mass = mass;
        t.valence = valence;
        t.temperature = temperature;
        t
    }

    #[test]
    fn test_high_signature_is_creative_affirmation() {
        let anchor = tok(220, 100, 220);
        assert_eq!(synthesize_octant(&[], &anchor), Octant::CreativeAffirmation);
    }

    #[test]
    fn test_low_signature_is_self_destructive_apathic() {
        let anchor = tok(30, -80, 30);
        assert_eq!(synthesize_octant(&[], &anchor), Octant::SelfDestructiveApathic);
    }

    #[test]
    fn test_participants_aggregate_signatures() {
        let anchor = tok(200, 80, 200);
        let p = tok(200, 80, 200);
        assert_eq!(synthesize_octant(&[p], &anchor), Octant::CreativeAffirmation);
    }

    #[test]
    fn test_mass_drives_apollo_dionysus() {
        let high = tok(255, 0, 128);
        let (x, _, _) = axis_scores_from_signature(&high);
        assert!(x.dominant.is_positive(), "high mass → Apollo");

        let low = tok(0, 0, 128);
        let (x, _, _) = axis_scores_from_signature(&low);
        assert!(!x.dominant.is_positive(), "low mass → Dionysus");
    }

    #[test]
    fn test_valence_drives_eros_thanatos() {
        let pos = tok(128, 100, 128);
        let (_, y, _) = axis_scores_from_signature(&pos);
        assert!(y.dominant.is_positive(), "positive valence → Eros");

        let neg = tok(128, -100, 128);
        let (_, y, _) = axis_scores_from_signature(&neg);
        assert!(!y.dominant.is_positive(), "negative valence → Thanatos");
    }

    #[test]
    fn test_temperature_drives_will_nothing() {
        let hot = tok(128, 0, 255);
        let (_, _, z) = axis_scores_from_signature(&hot);
        assert!(z.dominant.is_positive(), "high temp → Will");

        let cold = tok(128, 0, 0);
        let (_, _, z) = axis_scores_from_signature(&cold);
        assert!(!z.dominant.is_positive(), "low temp → Nothing");
    }
}
