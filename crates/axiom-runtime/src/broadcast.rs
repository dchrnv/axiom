// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Типы для broadcast через внешние адаптеры (WebSocket, REST, Telegram).
//
// Намеренно не клонируют полные Token/Connection — только поля нужные
// для визуализации и диагностики. Доступны только при feature "adapters".

#![cfg(feature = "adapters")]

use axiom_core::STATE_LOCKED;
use serde::Serialize;
use crate::over_domain::{FrameWeaverStats, DreamPhaseState, DreamPhaseStats, CycleStage};

/// Лёгкий snapshot состояния Engine для периодического broadcast.
///
/// Содержит только числа и краткие сводки по доменам — без клонирования
/// токенов и связей. Обновляется tick loop каждые `state_broadcast_interval` тиков.
#[derive(Serialize, Clone, Default)]
pub struct BroadcastSnapshot {
    /// Монотонный счётчик тиков
    pub tick_count: u64,
    /// Следующий COM event_id
    pub com_next_id: u64,
    /// Общее число следов опыта
    pub trace_count: usize,
    /// Число активных tension traces
    pub tension_count: usize,
    /// Краткая сводка по каждому из 11 доменов
    pub domain_summaries: Vec<DomainSummary>,
    /// Статистика FrameWeaver (None до первого тика сканирования)
    pub frame_weaver_stats: Option<FrameWeaverStats>,
    /// Snapshot DREAM Phase (состояние + статистика)
    pub dream_phase: Option<DreamPhaseSnapshot>,
}

/// Snapshot состояния DREAM-фазы для BroadcastSnapshot.
#[derive(Serialize, Clone)]
pub struct DreamPhaseSnapshot {
    pub state:           DreamPhaseState,
    pub current_fatigue: u8,
    pub idle_ticks:      u32,
    pub stats:           DreamPhaseStats,
    pub current_cycle:   Option<ActiveCycleSnapshot>,
}

/// Snapshot активного DreamCycle (только если state == Dreaming).
#[derive(Serialize, Clone)]
pub struct ActiveCycleSnapshot {
    pub stage:      CycleStage,
    pub queue_size: usize,
}

/// Краткая сводка одного домена для BroadcastSnapshot.
#[derive(Serialize, Clone)]
pub struct DomainSummary {
    /// domain_id домена (100–110 для уровня 1)
    pub domain_id: u16,
    /// Имя домена (SUTRA, EXECUTION, ...)
    pub name: String,
    /// Число токенов в домене
    pub token_count: usize,
    /// Число связей в домене
    pub connection_count: usize,
}

/// Детальный snapshot одного домена — только по явному запросу.
///
/// Может быть большим (сотни токенов) — не для периодического broadcast.
/// Отправляется в ответ на `AdapterPayload::DomainSnapshot { domain_id }`.
#[derive(Serialize, Clone)]
pub struct DomainDetailSnapshot {
    /// domain_id запрошенного домена
    pub domain_id: u16,
    /// Все токены домена в компактном формате
    pub tokens: Vec<TokenSnapshot>,
    /// Все связи домена в компактном формате
    pub connections: Vec<ConnectionSnapshot>,
}

/// Компактное представление Token для JSON-передачи.
///
/// Не 64-байтовый `Token` — только поля нужные для визуализации и диагностики.
#[derive(Serialize, Clone)]
pub struct TokenSnapshot {
    /// ID потока (Sutra)
    pub sutra_id: u32,
    /// Позиция в семантическом пространстве [x, y, z]
    pub position: [i16; 3],
    /// Семантический профиль (8 слоёв)
    pub shell: [u8; 8],
    /// Масса токена
    pub mass: u8,
    /// Температура (активность)
    pub temperature: u8,
    /// Валентность (способность формировать связи)
    pub valence: i8,
    /// Происхождение токена
    pub origin: u16,
    /// true если токен — якорный (state == STATE_LOCKED).
    ///
    /// Проверяется через state, а не mass/temperature: обычные токены
    /// могут остыть и набрать массу, но STATE_LOCKED выставляется только
    /// при inject_anchor_tokens (инвариант 9.7).
    pub is_anchor: bool,
}

impl From<&axiom_core::Token> for TokenSnapshot {
    fn from(t: &axiom_core::Token) -> Self {
        Self::from_token_with_shell(t, axiom_shell::EMPTY_SHELL)
    }
}

impl TokenSnapshot {
    /// Точный shell через compute_shell (connections + semantic table).
    pub fn from_token_with_connections(
        t: &axiom_core::Token,
        connections: &[axiom_core::Connection],
        table: &axiom_shell::SemanticContributionTable,
    ) -> Self {
        let shell = axiom_shell::compute_shell(t.sutra_id, connections, table);
        Self::from_token_with_shell(t, shell)
    }

    fn from_token_with_shell(t: &axiom_core::Token, shell: axiom_shell::ShellProfile) -> Self {
        Self {
            sutra_id:    t.sutra_id,
            position:    t.position,
            shell,
            mass:        t.mass,
            temperature: t.temperature,
            valence:     t.valence,
            origin:      t.origin,
            is_anchor:   t.state == STATE_LOCKED,
        }
    }
}

/// Компактное представление Connection для JSON-передачи.
#[derive(Serialize, Clone)]
pub struct ConnectionSnapshot {
    /// ID токена-источника
    pub source_id: u32,
    /// ID токена-цели
    pub target_id: u32,
    /// Сила связи
    pub weight: f32,
}

impl From<&axiom_core::Connection> for ConnectionSnapshot {
    fn from(c: &axiom_core::Connection) -> Self {
        Self {
            source_id: c.source_id,
            target_id: c.target_id,
            weight:    c.strength,
        }
    }
}
