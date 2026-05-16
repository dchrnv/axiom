// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Синтетическое распознавание октанта — целостное, не аналитическое.
// Источник: AxialEvaluator_V1_0.md §6 (Corpus Callosum)
//
// В V1: синтетический октант аппроксимируется через центр масс позиций участников.
// Позиция центра масс → какой угол пространства занимает Frame.
// Отличается от аналитического октанта (через entropy/density/will метрики).

use axiom_core::Token;
use axiom_experience::{AxialScore, Octant};

/// Синтезировать октант через целостное распознавание позиционного центра Frame.
///
/// Аппроксимация: средняя позиция участников определяет "квадрант пространства".
/// X > 0 → Apollo, X <= 0 → Dionysus (content anchors: X∈[0..32767] → near 0 = Dionysian)
/// Y > threshold → Eros, Y <= threshold → Thanatos
/// Z > threshold → Will, Z <= threshold → Nothing
pub fn synthesize_octant(participants: &[Token], anchor: &Token) -> Octant {
    if participants.is_empty() {
        // Only anchor — use anchor position directly
        return octant_from_position(anchor.position);
    }

    let n = participants.len() as f32;
    let mean_x = participants.iter().map(|t| t.position[0] as f32).sum::<f32>() / n;
    let mean_y = participants.iter().map(|t| t.position[1] as f32).sum::<f32>() / n;
    let mean_z = participants.iter().map(|t| t.position[2] as f32).sum::<f32>() / n;

    // Blend anchor position (weight 0.5) with participant centroid (weight 0.5)
    let blended_x = (anchor.position[0] as f32 * 0.5 + mean_x * 0.5) as i16;
    let blended_y = (anchor.position[1] as f32 * 0.5 + mean_y * 0.5) as i16;
    let blended_z = (anchor.position[2] as f32 * 0.5 + mean_z * 0.5) as i16;

    octant_from_position([blended_x, blended_y, blended_z])
}

/// Производный октант из абсолютной позиции в семантическом пространстве.
///
/// Content anchors [0..32767]³: полюс оси — высокое значение.
/// Axes (±30000) — исключение. Нейтральная точка ≈ 16383/2 = 8191.
fn octant_from_position(pos: [i16; 3]) -> Octant {
    // For content anchors (0..32767): midpoint = 8191
    // For axis anchors (-30000..30000): midpoint = 0
    // Unified: positive = above 0 on axis
    let apollo_score = (pos[0].max(0) as u32 * 255 / 32767) as u8;
    let dionysus_score = ((-pos[0]).max(0) as u32 * 255 / 30000) as u8;
    let eros_score = (pos[1].max(0) as u32 * 255 / 32767) as u8;
    let thanatos_score = ((-pos[1]).max(0) as u32 * 255 / 30000) as u8;
    let will_score = (pos[2].max(0) as u32 * 255 / 32767) as u8;
    let nothing_score = ((-pos[2]).max(0) as u32 * 255 / 30000) as u8;

    let x = AxialScore::new(apollo_score, dionysus_score);
    let y = AxialScore::new(eros_score, thanatos_score);
    let z = AxialScore::new(will_score, nothing_score);
    Octant::from_scores(&x, &y, &z)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tok(pos: [i16; 3]) -> Token {
        Token::new(1, 100, pos, 0)
    }

    #[test]
    fn test_high_x_y_z_is_creative_affirmation() {
        let anchor = tok([30000, 30000, 30000]);
        let octant = synthesize_octant(&[], &anchor);
        assert_eq!(octant, Octant::CreativeAffirmation);
    }

    #[test]
    fn test_low_xyz_is_self_destructive_apathic() {
        let anchor = tok([100, 100, 100]);
        let octant = synthesize_octant(&[], &anchor);
        assert_eq!(octant, Octant::SelfDestructiveApathic);
    }

    #[test]
    fn test_participants_blend_with_anchor() {
        let anchor = tok([20000, 20000, 20000]);
        let participants = vec![tok([20000, 20000, 20000])];
        let octant = synthesize_octant(&participants, &anchor);
        assert_eq!(octant, Octant::CreativeAffirmation);
    }
}
