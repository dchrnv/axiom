// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// TextPerceptor — кодирование текста в UclCommand(InjectToken).
//
// Это MVP без ML. Использует детерминированные эвристики:
//   - FNV-1a hash → 3D позиция в семантическом пространстве
//   - Длина и пунктуация → temperature/mass
//   - Маркеры времени, эмоций, абстракции → Shell L4/L7/L8
//
// Единственный источник истины для payload layout —
// parse_inject_token_payload() в axiom-runtime/src/engine.rs.
// Сборка payload здесь — зеркало того парсера.
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

use axiom_ucl::{UclCommand, OpCode};

/// SUTRA domain_id на уровне 1: level_id * 100 + 0 = 100
const SUTRA_DOMAIN_ID: u16 = 100;

/// Преобразует строку UTF-8 в `UclCommand(InjectToken)` с осмысленным токеном.
///
/// Детерминирован: одинаковый текст → одинаковая команда.
pub struct TextPerceptor;

impl TextPerceptor {
    /// Создать новый TextPerceptor.
    pub fn new() -> Self {
        Self
    }

    /// Преобразовать текст в UclCommand(InjectToken) для SUTRA(100).
    pub fn perceive(&self, text: &str) -> UclCommand {
        let bytes = text.as_bytes();
        let len   = bytes.len();

        let hash = fnv1a_hash(bytes);

        // Position: хэш → 3D координаты в i16 диапазоне
        let x = ((hash >>  0) & 0x7FFF) as f32;
        let y = ((hash >> 16) & 0x7FFF) as f32;
        let z = ((hash >> 32) & 0x7FFF) as f32;

        // Mass: зависит от длины текста (50..=250)
        let mass: f32 = (50.0 + (len.min(200) as f32)).min(250.0);

        // Temperature: высокая пластичность для нового ввода (150..=255)
        let temperature: f32 = {
            let base: f32 = 150.0;
            let excl = text.chars().filter(|&c| c == '!').count() as f32;
            let ques = text.chars().filter(|&c| c == '?').count() as f32;
            (base + excl * 15.0 + ques * 10.0).min(255.0)
        };

        // semantic_weight: L5 cognitive высокий (текст всегда когнитивный)
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
}
