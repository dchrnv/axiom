// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// TextPerceptor — кодирование текста в UclCommand(InjectToken).
//
// Порядок позиционирования:
//   1. word-level match_text (AnchorSet, exact/alias/substring)
//   2. char/word-level AnchorMatchTable (E1 путь А: OBS-01_Errata_Instructions.md §2)
//   3. FNV-1a fallback
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

use axiom_config::AnchorSet;
use axiom_core::FLAG_ACTIVE;
use axiom_shell::link_types;
use axiom_ucl::{BondTokensPayload, OpCode, UclCommand};
use std::sync::Arc;

use super::anchor_match::AnchorMatchTable;

/// SUTRA domain_id на уровне 1: level_id * 100 + 0 = 100
const SUTRA_DOMAIN_ID: u16 = 100;

/// MAYA domain_id на уровне 1: level_id * 100 + 10 = 110
const MAYA_DOMAIN_ID: u16 = 110;

/// Преобразует строку UTF-8 в `UclCommand(InjectToken)` с осмысленным токеном.
///
/// Детерминирован: одинаковый текст → одинаковая команда.
pub struct TextPerceptor {
    anchor_set: Option<Arc<AnchorSet>>,
    match_table: Option<AnchorMatchTable>,
}

impl TextPerceptor {
    /// Создать TextPerceptor без якорей (FNV-1a fallback).
    pub fn new() -> Self {
        Self { anchor_set: None, match_table: None }
    }

    /// Создать TextPerceptor с набором якорей для семантического позиционирования.
    pub fn with_anchors(anchors: Arc<AnchorSet>) -> Self {
        let match_table = Some(AnchorMatchTable::build(&anchors));
        Self {
            anchor_set: Some(anchors),
            match_table,
        }
    }

    /// Detect the dominant subsystem for a text.
    ///
    /// Path 1: word-level AnchorSet match (exact/alias/substring anchor words).
    /// Path 2: decomposition table (word_signals + char_signals — wider coverage).
    /// Returns None only if both paths produce no signal.
    pub fn detect_subsystem(&self, text: &str) -> Option<String> {
        // Path 1: word-level anchor match
        if let Some(ref anchors) = self.anchor_set {
            let matches = anchors.match_text(text);
            if let Some(sub) = anchors.dominant_subsystem_of(&matches) {
                return Some(sub);
            }
        }
        // Path 2: decomposition table fallback
        self.match_table.as_ref()?.dominant_subsystem(text)
    }

    /// Преобразовать текст в UclCommand(InjectToken) для SUTRA(100).
    pub fn perceive(&self, text: &str) -> UclCommand {
        let bytes = text.as_bytes();
        let len = bytes.len();

        // Mass: зависит от длины текста (50..=250)
        let mass: f32 = (50.0 + (len.min(200) as f32)).min(250.0);

        // Temperature: высокая пластичность для нового ввода (150..=255)
        let temperature: f32 = {
            let base: f32 = 150.0;
            let excl = text.chars().filter(|&c| c == '!').count() as f32;
            let ques = text.chars().filter(|&c| c == '?').count() as f32;
            (base + excl * 15.0 + ques * 10.0).min(255.0)
        };

        // Путь 1: word-level match_text (exact/alias/substring из AnchorSet).
        // Позиция вычисляется ТОЛЬКО по subsystem-якорям — это обеспечивает
        // семантически корректное размещение в пространстве подсистем.
        // Структурные якоря (axes/layers/domains/octants) не загрязняют позицию.
        if let Some(ref anchors) = self.anchor_set {
            let all_matches = anchors.match_text(text);
            if !all_matches.is_empty() {
                // Предпочесть subsystem-позицию; если нет subsystem-хитов — all-anchor
                let sub_matches = anchors.match_subsystem_text(text);
                let pos_matches = if !sub_matches.is_empty() { &sub_matches } else { &all_matches };
                let pos = anchors.compute_position(pos_matches);
                let semantic_weight = anchors.compute_semantic_weight(&all_matches);
                return build_inject_token_command(
                    SUTRA_DOMAIN_ID,
                    pos[0],
                    pos[1],
                    pos[2],
                    mass,
                    temperature,
                    semantic_weight,
                );
            }
        }

        // Путь 2: char/word-level AnchorMatchTable (E1 путь А)
        if let Some(ref table) = self.match_table {
            if let Some(pos) = table.compute_position(text) {
                return build_inject_token_command(
                    SUTRA_DOMAIN_ID,
                    pos[0] as f32,
                    pos[1] as f32,
                    pos[2] as f32,
                    mass,
                    temperature,
                    0.85,
                );
            }
        }

        // Путь 3: FNV-1a fallback
        let hash = fnv1a_hash(bytes);
        let x = (hash & 0x7FFF) as f32;
        let y = ((hash >> 16) & 0x7FFF) as f32;
        let z = ((hash >> 32) & 0x7FFF) as f32;

        build_inject_token_command(SUTRA_DOMAIN_ID, x, y, z, mass, temperature, 0.8)
    }

    /// Преобразовать текст в InjectToken + BondTokens к совпавшим якорям (AE-TD-08).
    ///
    /// InjectToken получает детерминированный `proposed_sutra_id` (записывается в reserved[0..4]).
    /// Для каждого matched anchor-ID — BondTokens в MAYA domain (110).
    /// Возвращает пустой Vec при ошибке (не должна паниковать).
    ///
    /// Если якори недоступны — возвращает один InjectToken без bonds (деградирует к perceive()).
    pub fn perceive_and_bond(&self, text: &str) -> Vec<UclCommand> {
        let inject = self.perceive_with_stable_id(text);
        let stable_id = text_stable_id(text);

        let anchor_ids: Vec<String> = match self.match_table.as_ref() {
            Some(table) => table.matched_anchor_ids(text),
            None => {
                if let Some(ref anchors) = self.anchor_set {
                    anchors.match_subsystem_text(text)
                        .iter()
                        .map(|m| m.anchor.id.clone())
                        .collect()
                } else {
                    vec![]
                }
            }
        };

        let mut cmds = vec![inject];
        for anchor_id in &anchor_ids {
            let target_id = anchor_sutra_id(anchor_id);
            if target_id == stable_id {
                continue; // коллизия
            }
            let bond = BondTokensPayload {
                source_id: stable_id,
                target_id,
                domain_id: MAYA_DOMAIN_ID,
                link_type: link_types::SEMANTIC_ANCHOR_BOND,
                strength: 1.0,
                conn_flags: FLAG_ACTIVE,
                origin_domain: SUTRA_DOMAIN_ID,
                role_id: 0,
                reserved: [0; 24],
            };
            cmds.push(UclCommand::new(OpCode::BondTokens, 0, 10, 0).with_payload(&bond));
        }
        cmds
    }

    /// Как perceive(), но записывает proposed_sutra_id в reserved[0..4] payload.
    fn perceive_with_stable_id(&self, text: &str) -> UclCommand {
        let stable_id = text_stable_id(text);
        let mut cmd = self.perceive(text);
        // reserved начинается с байта 40 (см. build_inject_token_command layout)
        cmd.payload[40..44].copy_from_slice(&stable_id.to_le_bytes());
        cmd
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
    x: f32,
    y: f32,
    z: f32,
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

/// Детерминированный sutra_id для текстового токена (AE-TD-08).
///
/// Диапазон: 0x4000_0001..=0x7FFF_FFFF (бит 30 установлен, бит 31 снят).
/// Не пересекается с:
///   - anchor sutra_ids (бит 31 установлен: 0x8000_0001+)
///   - sequential event_ids (малые значения)
///   - domain_position_hash (28 бит: 0x0001..0x0FFF_FFFF)
fn text_stable_id(text: &str) -> u32 {
    let h = fnv1a_hash(text.as_bytes());
    let id = (h & 0x3FFF_FFFF) as u32;
    // Устанавливаем бит 30 и гарантируем non-zero
    (id | 0x4000_0000).max(0x4000_0001)
}

/// Детерминированный sutra_id примитивного якоря по его строковому ID (зеркало engine.rs).
///
/// Диапазон: 0x8000_0001..=0xFFFF_FFFF (бит 31 установлен).
fn anchor_sutra_id(id: &str) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in id.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let low = (h & 0x7FFF_FFFF) as u32;
    (low | 0x8000_0000).max(0x8000_0001)
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
        assert!(
            temp >= 150.0,
            "temperature should be at least 150.0, got {}",
            temp
        );
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
            layer: axiom_config::AnchorLayer::L1,
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
