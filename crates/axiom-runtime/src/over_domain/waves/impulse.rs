// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

/// Источник импульса (какое дно поднял ветер).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpulseSource {
    /// Источник A: незавершённая дилемма (напряжение, не отпускает).
    Dilemma,
    /// Источник B: глубоко укоренённая область (резонирует, тянет вернуться).
    Resonance,
    /// Источник C: почти-кристаллизованный Frame-кандидат (хочется достроить).
    Unfinished,
}

/// Единица внутреннего ветра — то что Waves поднял.
///
/// `pull_strength` — функция от остроты × длительности × частоты возврата.
/// Чем дольше тянет и чем острее — тем сильнее импульс.
#[derive(Clone, Debug)]
pub struct Impulse {
    /// Источник: что подняло дно.
    pub source: ImpulseSource,
    /// sutra_id цели (Frame-анкер в EXPERIENCE, или кандидат, или якорь).
    pub target_sutra_id: u32,
    /// Сила тяги: 0..255. Используется для приоритизации и деградации.
    pub pull_strength: u8,
    /// COM event_id когда импульс впервые возник (для age-множителя).
    pub born_at_event: u64,
    /// Октант цели (если известен) — окраска ветра.
    pub octant: Option<u8>,
    /// Сколько раз поднимался без изменения результата (для затухания §6).
    pub raise_count: u32,
}

impl Impulse {
    pub fn new(
        source: ImpulseSource,
        target_sutra_id: u32,
        pull_strength: u8,
        born_at_event: u64,
        octant: Option<u8>,
    ) -> Self {
        Self {
            source,
            target_sutra_id,
            pull_strength,
            born_at_event,
            octant,
            raise_count: 0,
        }
    }

    /// Ослабить импульс на один raise без результата.
    pub fn decay(&mut self, rate: u8) {
        self.pull_strength = self.pull_strength.saturating_sub(rate);
        self.raise_count += 1;
    }

    /// Мёртвый ли импульс (слишком слабый чтобы поднимать).
    pub fn is_exhausted(&self) -> bool {
        self.pull_strength == 0
    }
}
