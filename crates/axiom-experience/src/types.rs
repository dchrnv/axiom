// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

/// Восемь октантов семантического пространства.
///
/// Получаются комбинацией трёх философских дихотомий:
///   X: Аполлон (+) / Дионис (-)
///   Y: Эрос (+) / Танатос (-)
///   Z: Воля (+) / Ничто (-)
///
/// Источник: `docs/architecture/AxialEvaluator_V1_0.md §5`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Octant {
    /// +++ Apollo + Eros + Will
    CreativeAffirmation = 0,
    /// -++ Dionysus + Eros + Will
    EcstaticAffirmation = 1,
    /// +-+ Apollo + Thanatos + Will
    HeroicFatal = 2,
    /// --+ Dionysus + Thanatos + Will
    DestructiveActivating = 3,
    /// ++- Apollo + Eros + Nothing
    IdealizedConsoling = 4,
    /// -+- Dionysus + Eros + Nothing
    PassiveSentimental = 5,
    /// +-- Apollo + Thanatos + Nothing
    FormalDenying = 6,
    /// --- Dionysus + Thanatos + Nothing
    SelfDestructiveApathic = 7,
}

impl Octant {
    /// Вычислить октант из трёх осевых оценок.
    pub fn from_scores(x: &AxialScore, y: &AxialScore, z: &AxialScore) -> Self {
        let x_pos = x.dominant.is_positive();
        let y_pos = y.dominant.is_positive();
        let z_pos = z.dominant.is_positive();
        match (x_pos, y_pos, z_pos) {
            (true, true, true) => Octant::CreativeAffirmation,
            (false, true, true) => Octant::EcstaticAffirmation,
            (true, false, true) => Octant::HeroicFatal,
            (false, false, true) => Octant::DestructiveActivating,
            (true, true, false) => Octant::IdealizedConsoling,
            (false, true, false) => Octant::PassiveSentimental,
            (true, false, false) => Octant::FormalDenying,
            (false, false, false) => Octant::SelfDestructiveApathic,
        }
    }

    /// Индекс октанта 0..7 (совпадает с repr(u8)).
    pub fn index(self) -> usize {
        self as usize
    }

    /// Имя октанта для диагностики.
    pub fn name(self) -> &'static str {
        match self {
            Octant::CreativeAffirmation => "CreativeAffirmation",
            Octant::EcstaticAffirmation => "EcstaticAffirmation",
            Octant::HeroicFatal => "HeroicFatal",
            Octant::DestructiveActivating => "DestructiveActivating",
            Octant::IdealizedConsoling => "IdealizedConsoling",
            Octant::PassiveSentimental => "PassiveSentimental",
            Octant::FormalDenying => "FormalDenying",
            Octant::SelfDestructiveApathic => "SelfDestructiveApathic",
        }
    }
}

/// Оценка одной философской оси (0..255 на каждом полюсе).
///
/// Источник: `docs/architecture/AxialEvaluator_V1_0.md §5`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxialScore {
    /// Положительный полюс: Аполлон / Эрос / Воля (0..255)
    pub positive_pole: u8,
    /// Отрицательный полюс: Дионис / Танатос / Ничто (0..255)
    pub negative_pole: u8,
    /// Доминирующее направление
    pub dominant: AxialDominant,
}

impl AxialScore {
    /// Создать оценку, вычислив доминанту автоматически.
    pub fn new(positive_pole: u8, negative_pole: u8) -> Self {
        Self {
            positive_pole,
            negative_pole,
            dominant: AxialDominant::from_diff(positive_pole, negative_pole),
        }
    }

    /// Сбалансированная оценка (Balanced, 128/128).
    pub fn balanced() -> Self {
        Self::new(128, 128)
    }
}

/// Доминирующее направление осевой оценки.
///
/// Пороги: StronglyPositive = diff > 100, LeaningPositive = diff 30..100, Balanced = diff < 30.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AxialDominant {
    StronglyPositive,
    LeaningPositive,
    Balanced,
    LeaningNegative,
    StronglyNegative,
}

impl AxialDominant {
    pub fn from_diff(pos: u8, neg: u8) -> Self {
        let diff = pos as i16 - neg as i16;
        match diff {
            d if d > 100 => AxialDominant::StronglyPositive,
            d if d > 30 => AxialDominant::LeaningPositive,
            d if d >= -30 => AxialDominant::Balanced,
            d if d >= -100 => AxialDominant::LeaningNegative,
            _ => AxialDominant::StronglyNegative,
        }
    }

    pub fn is_positive(self) -> bool {
        matches!(self, AxialDominant::StronglyPositive | AxialDominant::LeaningPositive)
    }

    pub fn is_negative(self) -> bool {
        matches!(self, AxialDominant::StronglyNegative | AxialDominant::LeaningNegative)
    }
}

/// Восемь уровней оценки (слои возрастающей абстракции).
///
/// Источник: `docs/architecture/AxialEvaluator_V1_0.md §3`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum EvaluationLevel {
    Sensory = 1,
    Action = 2,
    Imaginal = 3,
    Conceptual = 4,
    Motivational = 5,
    Social = 6,
    Existential = 7,
    Transcendent = 8,
}

impl EvaluationLevel {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(EvaluationLevel::Sensory),
            2 => Some(EvaluationLevel::Action),
            3 => Some(EvaluationLevel::Imaginal),
            4 => Some(EvaluationLevel::Conceptual),
            5 => Some(EvaluationLevel::Motivational),
            6 => Some(EvaluationLevel::Social),
            7 => Some(EvaluationLevel::Existential),
            8 => Some(EvaluationLevel::Transcendent),
            _ => None,
        }
    }

    /// Shell-слой соответствующий этому уровню (прямое соответствие в V1).
    pub fn shell_layer_index(self) -> usize {
        self as usize - 1
    }
}

/// Подсистемы знания — то, что ContextRecognizer различает.
///
/// Источник: `docs/architecture/ContextRecognizer_V5_0.md`, `Writing_V1_0.md`, `Mathematics_V1_0.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubsystemId {
    Writing,
    Mathematics,
    Music,
    Time,
    Logic,
    Unknown,
}

impl SubsystemId {
    pub fn name(self) -> &'static str {
        match self {
            SubsystemId::Writing => "writing",
            SubsystemId::Mathematics => "mathematics",
            SubsystemId::Music => "music",
            SubsystemId::Time => "time",
            SubsystemId::Logic => "logic",
            SubsystemId::Unknown => "unknown",
        }
    }
}

/// Уровень композиции Frame (глубина структуры, не SutraDepth).
///
/// Источник: `docs/architecture/ContextRecognizer_V5_0.md §9.3`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FrameComposition {
    /// Якорь-примитив в SUTRA (dot, vline, element, …)
    C0Primitive,
    /// Frame из примитивов (буква, символ)
    C1Atom,
    /// Frame из Atoms (слово)
    C2Molecule,
    /// Frame из Molecules (фраза, выражение)
    C3Structure,
    /// Frame из Structures (предложение, формула)
    C4Composition,
    /// Высшие композиции (текст, теорема)
    C5Plus,
}

/// Снимок контекста в момент создания/реактивации Frame.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContextSnapshot {
    pub primary_subsystem: SubsystemId,
    pub primary_octant: Octant,
    pub event_id: u64,
}

impl Default for ContextSnapshot {
    fn default() -> Self {
        Self {
            primary_subsystem: SubsystemId::Unknown,
            primary_octant: Octant::CreativeAffirmation,
            event_id: 0,
        }
    }
}
