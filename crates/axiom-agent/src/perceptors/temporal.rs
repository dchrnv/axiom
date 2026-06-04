// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// TemporalPerceptor (PRIM-TD-04) — темпоральные маркеры в тексте → time_*-якоря → SUTRA.
//
// Детектирует 7 темпоральных концептов в тексте и инжектирует соответствующие
// time_*-якоря в SUTRA(100) как стабильные токены (temporal_anchor_stable_id).
//
// Паттерн аналогичен L0VisionPerceptor: стабильный sutra_id в reserved[0..4],
// используется движком как proposed_sutra_id (build_token_from_inject).
//
// Источник якорей: AnchorSet.get_subsystem("time") → [time_before..time_horizon].
// Позиции якорей в положительном X-пространстве — не конфликтуют с vision (отриц. X).

use std::collections::VecDeque;

use axiom_config::Anchor;
use axiom_ucl::{OpCode, UclCommand};

/// SUTRA domain на уровне 1.
const SUTRA_DOMAIN_ID: u16 = 100;

/// Масса токена темпорального примитива.
const TEMPORAL_TOKEN_MASS: f32 = 160.0;
/// Температура токена (умеренная активность, дольше живёт в системе).
const TEMPORAL_TOKEN_TEMPERATURE: f32 = 5.0;

/// TemporalPerceptor — детектирует темпоральные маркеры в тексте.
///
/// Принимает time_*-якоря из `AnchorSet.get_subsystem("time")`.
/// Для каждого найденного маркера генерирует `InjectToken` в SUTRA
/// со стабильным `temporal_anchor_stable_id` в `reserved[0..4]`.
pub struct TemporalPerceptor {
    anchors: Vec<Anchor>,
    pending: VecDeque<UclCommand>,
}

impl TemporalPerceptor {
    /// Создать перцептор из time-якорей AnchorSet.
    ///
    /// ```no_run
    /// # use axiom_agent::perceptors::temporal::TemporalPerceptor;
    /// # use axiom_config::AnchorSet;
    /// # let anchor_set = AnchorSet::empty();
    /// let vp = TemporalPerceptor::new(anchor_set.get_subsystem("time"));
    /// ```
    pub fn new(time_anchors: &[Anchor]) -> Self {
        let anchors: Vec<Anchor> = time_anchors
            .iter()
            .filter(|a| a.id.starts_with("time_"))
            .cloned()
            .collect();
        Self {
            anchors,
            pending: VecDeque::new(),
        }
    }

    /// Обнаружить темпоральные маркеры в тексте → поставить InjectToken в очередь.
    ///
    /// Для каждого якоря проверяет `word` и `aliases` (без учёта регистра).
    /// Один и тот же якорь не добавляется дважды за один вызов.
    pub fn perceive(&mut self, text: &str) {
        let text_lower = text.to_lowercase();
        for anchor in &self.anchors {
            if self.matches_anchor(&text_lower, anchor) {
                let cmd = build_inject_cmd(anchor);
                self.pending.push_back(cmd);
            }
        }
    }

    /// Число команд в очереди.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Взять следующую команду из очереди.
    pub fn receive(&mut self) -> Option<UclCommand> {
        self.pending.pop_front()
    }

    fn matches_anchor(&self, text_lower: &str, anchor: &Anchor) -> bool {
        // Проверить основное слово
        let word = anchor.word.to_lowercase();
        if !word.is_empty() && text_lower.contains(word.as_str()) {
            return true;
        }
        // Проверить псевдонимы
        anchor
            .aliases
            .iter()
            .any(|alias| text_lower.contains(alias.to_lowercase().as_str()))
    }
}

/// Построить InjectToken команду для time-якоря.
///
/// Заполняет `reserved[0..4]` стабильным sutra_id (temporal_anchor_stable_id).
/// Это позволяет движку (`build_token_from_inject`) использовать его как `proposed_sutra_id`.
fn build_inject_cmd(anchor: &Anchor) -> UclCommand {
    let mut cmd = UclCommand::new(OpCode::InjectToken, SUTRA_DOMAIN_ID as u32, 100, 0);
    let domain_bytes = SUTRA_DOMAIN_ID.to_le_bytes();
    cmd.payload[0..2].copy_from_slice(&domain_bytes);
    cmd.payload[2] = 0; // token_type = generic
    // [3] padding
    cmd.payload[4..8].copy_from_slice(&TEMPORAL_TOKEN_MASS.to_le_bytes());
    let px = anchor.position[0] as f32;
    let py = anchor.position[1] as f32;
    let pz = anchor.position[2] as f32;
    cmd.payload[8..12].copy_from_slice(&px.to_le_bytes());
    cmd.payload[12..16].copy_from_slice(&py.to_le_bytes());
    cmd.payload[16..20].copy_from_slice(&pz.to_le_bytes());
    // velocity = 0
    cmd.payload[20..32].fill(0);
    cmd.payload[32..36].copy_from_slice(&(1.0f32).to_le_bytes()); // semantic_weight = 1.0
    cmd.payload[36..40].copy_from_slice(&TEMPORAL_TOKEN_TEMPERATURE.to_le_bytes());
    // reserved[0..4] = temporal_anchor_stable_id (proposed_sutra_id)
    let stable_id = temporal_anchor_stable_id(&anchor.id);
    cmd.payload[40..44].copy_from_slice(&stable_id.to_le_bytes());
    cmd
}

/// Детерминированный sutra_id для темпорального якоря (FNV-1a).
///
/// Диапазон: `0x1000_0001..=0x1FFF_FFFF` (бит 28 установлен, биты 29–31 сброшены).
/// Не пересекается с:
///   - sequential event_ids (малые значения)
///   - domain_position_hash (0x0001..0x0FFF_FFFF, 28 бит)
///   - temporal_anchor_stable_id (бит 28: 0x1000_0001..0x1FFF_FFFF)  ← этот диапазон
///   - vision_anchor_stable_id  (бит 29: 0x2000_0001..0x3FFF_FFFF)
///   - text_stable_id           (бит 30: 0x4000_0001..0x7FFF_FFFF)
///   - anchor_sutra_id          (бит 31: 0x8000_0001..0xFFFF_FFFF)
pub fn temporal_anchor_stable_id(anchor_id: &str) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325;
    for byte in anchor_id.bytes() {
        h ^= byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let id = (h as u32) & 0x0FFF_FFFF; // 28 бит без старшего
    (id | 0x1000_0000).max(0x1000_0001) // бит 28 установлен, не ноль
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_config::{Anchor, AnchorLayer};

    fn make_anchor(id: &str, word: &str, aliases: Vec<&str>, pos: [i16; 3]) -> Anchor {
        Anchor {
            id: id.to_string(),
            word: word.to_string(),
            layer: AnchorLayer::L1,
            position: pos,
            shell: [0, 0, 0, 0, 15, 0, 40, 10],
            description: String::new(),
            tags: vec![],
            aliases: aliases.into_iter().map(str::to_string).collect(),
        }
    }

    fn make_time_anchors() -> Vec<Anchor> {
        vec![
            make_anchor("time_before",    "до",           vec!["before", "прежде", "раньше"],    [6000, 2000, 8000]),
            make_anchor("time_after",     "после",        vec!["after", "затем", "позже"],        [6000, 2000, 10000]),
            make_anchor("time_simultaneous", "одновременно", vec!["simultaneous", "в то же время"], [8000, 10000, 9000]),
            make_anchor("time_duration",  "длительность", vec!["duration", "пока", "в течение"],  [5000, 5000, 7000]),
            make_anchor("time_periodic",  "цикл",         vec!["periodic", "снова", "ритм"],      [10000, 7000, 9000]),
            make_anchor("time_irreversible", "необратимо", vec!["irreversible", "навсегда"],      [12000, 1000, 14000]),
            make_anchor("time_horizon",   "горизонт",     vec!["horizon", "в будущем"],           [7000, 6000, 13000]),
        ]
    }

    #[test]
    fn no_tokens_on_neutral_text() {
        let mut vp = TemporalPerceptor::new(&make_time_anchors());
        vp.perceive("Математика — фундамент всего.");
        assert_eq!(vp.pending_count(), 0);
    }

    #[test]
    fn detects_time_before_by_word() {
        let mut vp = TemporalPerceptor::new(&make_time_anchors());
        vp.perceive("Это было до катастрофы.");
        assert_eq!(vp.pending_count(), 1);
        let cmd = vp.receive().unwrap();
        assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    }

    #[test]
    fn detects_time_after_by_alias() {
        let mut vp = TemporalPerceptor::new(&make_time_anchors());
        vp.perceive("After the storm everything changed.");
        assert_eq!(vp.pending_count(), 1);
    }

    #[test]
    fn detects_multiple_markers() {
        let mut vp = TemporalPerceptor::new(&make_time_anchors());
        vp.perceive("Сначала (до) всё было иначе, затем (после) изменилось.");
        // "до" → time_before, "после" → time_after
        assert!(vp.pending_count() >= 2);
    }

    #[test]
    fn case_insensitive() {
        let mut vp = TemporalPerceptor::new(&make_time_anchors());
        vp.perceive("BEFORE the event, AFTER the event.");
        assert!(vp.pending_count() >= 2);
    }

    #[test]
    fn stable_id_deterministic() {
        let id1 = temporal_anchor_stable_id("time_before");
        let id2 = temporal_anchor_stable_id("time_before");
        assert_eq!(id1, id2);
    }

    #[test]
    fn stable_id_different_anchors() {
        let a = temporal_anchor_stable_id("time_before");
        let b = temporal_anchor_stable_id("time_after");
        assert_ne!(a, b);
    }

    #[test]
    fn stable_id_range() {
        for id in &[
            "time_before", "time_after", "time_simultaneous",
            "time_duration", "time_periodic", "time_irreversible", "time_horizon",
        ] {
            let sid = temporal_anchor_stable_id(id);
            assert!(sid >= 0x1000_0001, "id={id}: {sid:#010x} < 0x1000_0001");
            assert!(sid <= 0x1FFF_FFFF, "id={id}: {sid:#010x} > 0x1FFF_FFFF");
        }
    }

    #[test]
    fn stable_id_in_reserved_payload() {
        let anchors = vec![make_anchor("time_before", "до", vec!["before"], [6000, 2000, 8000])];
        let mut vp = TemporalPerceptor::new(&anchors);
        vp.perceive("before");
        let cmd = vp.receive().unwrap();
        let stable = u32::from_le_bytes(cmd.payload[40..44].try_into().unwrap());
        assert_ne!(stable, 0);
        assert!(stable & 0x1000_0000 != 0, "bit 28 must be set");
        assert_eq!(stable, temporal_anchor_stable_id("time_before"));
    }

    #[test]
    fn new_filters_non_time_anchors() {
        let mut anchors = make_time_anchors();
        anchors.push(make_anchor("writing_dot", "точка", vec![], [1000, 1000, 1000]));
        let vp = TemporalPerceptor::new(&anchors);
        // Only time_* anchors should be kept
        assert_eq!(vp.anchors.len(), 7);
    }
}
