// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// TextPerceptor — кодирование текста в UclCommand(InjectToken).
//
// Если задан AnchorSet — позиция вычисляется через якорное совпадение.
// Если якорей нет или совпадений нет — fallback на FNV-1a hash.
//
// Payload layout (сверено с parse_inject_token_payload):
//   [0..2]   target_domain_id  u16 LE
//   [2]      token_type        u8
//   [3]      padding
//   [4..8]   mass              f32 LE
//   [8..12]  position[0]       f32 LE  (x)
//   [12..16] position[1]       f32 LE  (y)
//   [16..20] position[2]       f32 LE  (z)
//   [20..24] velocity[0]       f32 LE
//   [24..28] velocity[1]       f32 LE
//   [28..32] velocity[2]       f32 LE
//   [32..36] semantic_weight   f32 LE
//   [36..40] temperature       f32 LE

use std::sync::Arc;
use axiom_ucl::{UclCommand, OpCode};
use axiom_config::AnchorSet;

/// SUTRA domain_id на уровне 1: level_id * 100 + 0 = 100
const SUTRA_DOMAIN_ID: u16 = 100;

/// Преобразует строку UTF-8 в `UclCommand(InjectToken)` с осмысленным токеном.
///
/// Детерминирован: одинаковый текст → одинаковая команда.
/// Если задан AnchorSet — использует якорное позиционирование.
pub struct TextPerceptor {
    anchor_set: Option<Arc<AnchorSet>>,
}

impl TextPerceptor {
    /// Создать TextPerceptor без якорей (FNV-1a fallback).
    pub fn new() -> Self {
        Self { anchor_set: None }
    }

    /// Создать TextPerceptor с набором якорей для семантического позиционирования.
    pub fn with_anchors(anchors: Arc<AnchorSet>) -> Self {
        Self { anchor_set: Some(anchors) }
    }

    /// Преобразовать текст в UclCommand(InjectToken) для SUTRA(100).
    pub fn perceive(&self, text: &str) -> UclCommand {
        let bytes = text.as_bytes();
        let len   = bytes.len();

        // Mass: зависит от длины текста (50..=250)
        let mass: f32 = (50.0 + (len.min(200) as f32)).min(250.0);

        // Temperature: высокая пластичность для нового ввода (150..=255)
        let temperature: f32 = {
            let base: f32 = 150.0;
            let excl = text.chars().filter(|&c| c == '!').count() as f32;
            let ques = text.chars().filter(|&c| c == '?').count() as f32;
            (base + excl * 15.0 + ques * 10.0).min(255.0)
        };

        // Попытка якорного позиционирования
        if let Some(ref anchors) = self.anchor_set {
            let matches = anchors.match_text(text);
            if !matches.is_empty() {
                let pos = anchors.compute_position(&matches);
                let semantic_weight = anchors.compute_semantic_weight(&matches);
                return build_inject_token_command(
                    SUTRA_DOMAIN_ID,
                    pos[0], pos[1], pos[2],
                    mass,
                    temperature,
                    semantic_weight,
                );
            }
        }

        // Fallback: FNV-1a hash → 3D координаты
        let hash = fnv1a_hash(bytes);
        let x = ((hash >>  0) & 0x7FFF) as f32;
        let y = ((hash >> 16) & 0x7FFF) as f32;
        let z = ((hash >> 32) & 0x7FFF) as f32;
        let semantic_weight: f32 = 0.8;

        build_inject_token_command(
            SUTRA_DOMAIN_ID,
            x, y, z,
            mass,
            temperature,
            semantic_weight,
        )
    }
}

impl Default for TextPerceptor {
    fn default() -> Self {
        Self::new()
    }
}

/// Собрать UclCommand(InjectToken) — зеркало parse_inject_token_payload() в engine.rs.
fn build_inject_token_command(
    target_domain_id: u16,
    x: f32, y: f32, z: f32,
    mass: f32,
    temperature: f32,
    semantic_weight: f32,
) -> UclCommand {
    let mut cmd = UclCommand::new(OpCode::InjectToken, target_domain_id as u32, 100, 0);

    // [0..2] target_domain_id
    cmd.payload[0..2].copy_from_slice(&target_domain_id.to_le_bytes());
    // [2] token_type = 0 (generic)
    cmd.payload[2] = 0;
    // [4..8] mass
    cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
    // [8..12] position[0] = x
    cmd.payload[8..12].copy_from_slice(&x.to_le_bytes());
    // [12..16] position[1] = y
    cmd.payload[12..16].copy_from_slice(&y.to_le_bytes());
    // [16..20] position[2] = z
    cmd.payload[16..20].copy_from_slice(&z.to_le_bytes());
    // [20..32] velocity = [0, 0, 0]
    cmd.payload[20..32].fill(0);
    // [32..36] semantic_weight
    cmd.payload[32..36].copy_from_slice(&semantic_weight.to_le_bytes());
    // [36..40] temperature
    cmd.payload[36..40].copy_from_slice(&temperature.to_le_bytes());

    cmd
}

/// FNV-1a 64-bit hash (детерминированный, без внешних зависимостей).
fn fnv1a_hash(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_ucl::OpCode;

    #[test]
    fn test_deterministic_same_text() {
        let p = TextPerceptor::new();
        let c1 = p.perceive("hello world");
        let c2 = p.perceive("hello world");
        assert_eq!(c1.payload, c2.payload);
    }

    #[test]
    fn test_different_texts_different_position() {
        let p = TextPerceptor::new();
        let c1 = p.perceive("hello");
        let c2 = p.perceive("world");
        // Позиции не должны совпадать (x — первые 4 байта position)
        assert_ne!(&c1.payload[8..12], &c2.payload[8..12]);
    }

    #[test]
    fn test_empty_string_no_panic() {
        let p = TextPerceptor::new();
        let cmd = p.perceive("");
        assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    }

    #[test]
    fn test_unicode_no_panic() {
        let p = TextPerceptor::new();
        let cmd = p.perceive("привет мир 🌍");
        assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    }

    #[test]
    fn test_target_domain_is_sutra() {
        let p = TextPerceptor::new();
        let cmd = p.perceive("test");
        let domain_id = u16::from_le_bytes([cmd.payload[0], cmd.payload[1]]);
        assert_eq!(domain_id, 100, "target domain must be SUTRA=100");
    }

    #[test]
    fn test_temperature_at_least_150() {
        let p = TextPerceptor::new();
        let cmd = p.perceive("simple text");
        let temp = f32::from_le_bytes(cmd.payload[36..40].try_into().unwrap());
        assert!(temp >= 150.0, "temperature should be at least 150.0, got {}", temp);
    }

    #[test]
    fn test_command_goes_through_engine() {
        use axiom_runtime::AxiomEngine;
        let p = TextPerceptor::new();
        let cmd = p.perceive("semantic processing test");
        let mut engine = AxiomEngine::new();
        let result = engine.process_and_observe(&cmd);
        assert_eq!(result.ucl_result.status, 0); // Success
    }

    #[test]
    fn test_with_anchors_exact_match_x_pos() {
        use axiom_config::{Anchor, AnchorSet};
        let mut set = AnchorSet::empty();
        set.axes.push(Anchor {
            id: "ax".to_string(),
            word: "порядок".to_string(),
            aliases: vec!["структура".to_string()],
            tags: vec![],
            position: [30000, 0, 0],
            shell: [0; 8],
            description: String::new(),
        });
        let p = TextPerceptor::with_anchors(Arc::new(set));
        let cmd = p.perceive("порядок");
        // Position X должна быть близка к 30000
        let x = f32::from_le_bytes(cmd.payload[8..12].try_into().unwrap());
        assert!((x - 30000.0).abs() < 1.0, "x={x}");
    }

    #[test]
    fn test_without_anchors_fallback_to_hash() {
        let p = TextPerceptor::new();
        let cmd = p.perceive("порядок");
        let domain_id = u16::from_le_bytes([cmd.payload[0], cmd.payload[1]]);
        assert_eq!(domain_id, 100); // SUTRA
        // С fallback позиции из хэша ≠ 30000
        let x = f32::from_le_bytes(cmd.payload[8..12].try_into().unwrap());
        assert!(x < 32768.0); // в диапазоне хэша
    }
}
