// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AxiomEngine — главный оркестратор: AshtiCore + Guardian
//
// Архитектура после введения AshtiCore:
//   AxiomEngine
//     ├── AshtiCore (11 доменов + Arbiter + Experience)
//     └── Guardian  (CODEX-валидация рефлексов)

use axiom_core::{Token, Event};
use axiom_config::DomainConfig;
use axiom_domain::AshtiCore;
use axiom_ucl::{UclCommand, UclResult, OpCode, CommandStatus, SpawnDomainPayload, InjectTokenPayload};
use crate::guardian::Guardian;
use crate::snapshot::{EngineSnapshot, DomainSnapshot};
use crate::orchestrator;

/// Коды ошибок UclResult
pub mod error_codes {
    /// Успешное выполнение команды
    pub const OK: u16 = 0;
    /// Домен не найден
    pub const DOMAIN_NOT_FOUND: u16 = 1001;
    /// Превышена ёмкость домена
    pub const CAPACITY_EXCEEDED: u16 = 1002;
    /// Недопустимый payload команды
    pub const INVALID_PAYLOAD: u16 = 1004;
    /// Arbiter не готов к обработке
    pub const ARBITER_NOT_READY: u16 = 2001;
    /// Нарушение правил CODEX Guardian
    pub const GUARDIAN_VIOLATION: u16 = 3001;
    /// Неизвестный OpCode команды
    pub const UNKNOWN_OPCODE: u16 = 9001;
}

/// AxiomEngine — центральный оркестратор всех компонентов AXIOM.
///
/// Содержит один уровень AshtiCore (11 доменов + Arbiter + Experience)
/// и Guardian для CODEX-валидации рефлексов.
pub struct AxiomEngine {
    /// Фрактальный уровень Ashti_Core (11 доменов)
    pub ashti: AshtiCore,
    /// Guardian для проверки CODEX
    pub guardian: Guardian,
    /// Накопленные события текущего шага
    pending_events: Vec<Event>,
}

impl AxiomEngine {
    /// Создать новый Engine с уровнем AshtiCore level_id=1
    pub fn new() -> Self {
        Self {
            ashti: AshtiCore::new(1),
            guardian: Guardian::new(),
            pending_events: Vec::new(),
        }
    }

    /// Число доменов в Engine (всегда 11 — AshtiCore фиксирован)
    pub fn domain_count(&self) -> usize {
        11
    }

    /// Arbiter готов к маршрутизации (все 11 доменов зарегистрированы)
    pub fn arbiter_ready(&self) -> bool {
        self.ashti.is_ready()
    }

    /// Число токенов в домене по domain_id
    pub fn token_count(&self, domain_id: u32) -> usize {
        self.ashti.token_count(domain_id)
    }

    /// Взять накопленные события (очищает буфер)
    pub fn drain_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.pending_events)
    }

    /// Создать snapshot текущего состояния Engine
    pub fn snapshot(&self) -> EngineSnapshot {
        let domains: Vec<DomainSnapshot> = self.ashti.all_states().into_iter()
            .map(|(id, state)| {
                let config = self.ashti.all_configs()
                    .into_iter()
                    .find(|(cid, _)| *cid == id)
                    .map(|(_, c)| c)
                    .unwrap_or_else(|| DomainConfig::factory_void(id as u16, 0));
                DomainSnapshot {
                    domain_id: id,
                    config,
                    tokens: state.tokens.clone(),
                    connections: state.connections.clone(),
                }
            })
            .collect();

        EngineSnapshot {
            domains,
            com_next_id: 0,
            created_at: 0,
        }
    }

    /// Восстановить Engine из snapshot
    pub fn restore_from(snapshot: &EngineSnapshot) -> Self {
        let mut engine = Self::new();

        for ds in &snapshot.domains {
            if let Some(idx) = engine.ashti.index_of(ds.domain_id) {
                if let Some(state) = engine.ashti.state_mut(idx) {
                    for &token in &ds.tokens {
                        let _ = state.add_token(token);
                    }
                    for &conn in &ds.connections {
                        let _ = state.add_connection(conn);
                    }
                }
            }
        }

        engine
    }

    /// Обработать UCL-команду
    pub fn process_command(&mut self, cmd: &UclCommand) -> UclResult {
        let opcode = match opcode_from_u16(cmd.opcode) {
            Some(op) => op,
            None => return make_result(cmd.command_id, CommandStatus::SystemError, error_codes::UNKNOWN_OPCODE, 0),
        };

        match opcode {
            OpCode::SpawnDomain    => self.handle_spawn_domain(cmd),
            OpCode::CollapseDomain => self.handle_collapse_domain(cmd),
            OpCode::InjectToken    => self.handle_inject_token(cmd),
            OpCode::TickForward    => self.handle_tick_forward(cmd),
            OpCode::ProcessTokenDualPath => self.handle_dual_path(cmd),
            OpCode::FinalizeComparison   => self.handle_finalize(cmd),
            OpCode::BackupState    => self.handle_backup(cmd),
            OpCode::RestoreState   => self.handle_restore(cmd),
            OpCode::CoreReset      => self.handle_reset(cmd),
            OpCode::CoreShutdown   => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0),
            _ => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::UNKNOWN_OPCODE, 0),
        }
    }

    // --- Обработчики команд ---

    /// SpawnDomain — в AshtiCore домены фиксированы, команда возвращает Success (no-op)
    fn handle_spawn_domain(&mut self, cmd: &UclCommand) -> UclResult {
        // AshtiCore создаётся с 11 доменами. SpawnDomain принимается без ошибки
        // для обратной совместимости UCL-протокола, но физически не меняет состояние.
        let _ = parse_spawn_domain_payload(&cmd.payload);
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 11)
    }

    /// CollapseDomain — no-op в AshtiCore (домены неудаляемы в рамках уровня)
    fn handle_collapse_domain(&mut self, cmd: &UclCommand) -> UclResult {
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0)
    }

    fn handle_inject_token(&mut self, cmd: &UclCommand) -> UclResult {
        let p = parse_inject_token_payload(&cmd.payload);
        let domain_id = p.target_domain_id as u32;

        // Проверяем что домен существует в AshtiCore
        if self.ashti.index_of(domain_id).is_none() {
            return make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_NOT_FOUND, 0);
        }

        let event_id = self.ashti.domain_id_at(0).unwrap_or(0) as u64; // используем как base для event_id
        let token = build_token_from_inject(&p, p.target_domain_id, event_id);

        match self.ashti.inject_token(domain_id, token) {
            Ok(_)  => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 1),
            Err(_) => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::CAPACITY_EXCEEDED, 0),
        }
    }

    fn handle_tick_forward(&mut self, cmd: &UclCommand) -> UclResult {
        self.ashti.tick();
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 11)
    }

    fn handle_dual_path(&mut self, cmd: &UclCommand) -> UclResult {
        if !self.arbiter_ready() {
            return make_result(cmd.command_id, CommandStatus::SystemError, error_codes::ARBITER_NOT_READY, 0);
        }

        let token_opt = self.find_token_by_sutra_id(cmd.target_id);
        let token = match token_opt {
            Some(t) => t,
            None => return make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_NOT_FOUND, 0),
        };

        let result = orchestrator::route_token(self, token);
        let events = result.routed_events.len() as u16;
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, events)
    }

    fn handle_finalize(&mut self, cmd: &UclCommand) -> UclResult {
        match self.ashti.apply_feedback(cmd.command_id) {
            Ok(())  => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0),
            Err(_)  => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_NOT_FOUND, 0),
        }
    }

    fn handle_backup(&mut self, cmd: &UclCommand) -> UclResult {
        let snap = self.snapshot();
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, snap.domain_count() as u16)
    }

    fn handle_restore(&mut self, cmd: &UclCommand) -> UclResult {
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0)
    }

    fn handle_reset(&mut self, cmd: &UclCommand) -> UclResult {
        self.ashti = AshtiCore::new(1);
        self.guardian = Guardian::new();
        self.pending_events.clear();
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0)
    }

    // --- Внутренние хелперы ---

    /// Найти токен по sutra_id по всем доменам AshtiCore
    fn find_token_by_sutra_id(&self, sutra_id: u32) -> Option<Token> {
        for (_, state) in self.ashti.all_states() {
            if let Some(&token) = state.tokens.iter().find(|t| t.sutra_id == sutra_id) {
                return Some(token);
            }
        }
        None
    }
}

impl Default for AxiomEngine {
    fn default() -> Self {
        Self::new()
    }
}

// --- Утилиты ---

fn make_result(command_id: u64, status: CommandStatus, error_code: u16, events: u16) -> UclResult {
    UclResult {
        command_id,
        execution_time_us: 0,
        consumed_energy: 0.0,
        error_code,
        events_generated: events,
        status: status as u8,
        reserved: [0; 7],
    }
}

fn opcode_from_u16(raw: u16) -> Option<OpCode> {
    match raw {
        1000 => Some(OpCode::SpawnDomain),
        1001 => Some(OpCode::CollapseDomain),
        1002 => Some(OpCode::LockMembrane),
        1003 => Some(OpCode::ReshapeDomain),
        2000 => Some(OpCode::InjectToken),
        2001 => Some(OpCode::ApplyForce),
        2002 => Some(OpCode::AnnihilateToken),
        2003 => Some(OpCode::BondTokens),
        2004 => Some(OpCode::SplitToken),
        3000 => Some(OpCode::TickForward),
        3001 => Some(OpCode::ChangeTemperature),
        3002 => Some(OpCode::ApplyGravity),
        3003 => Some(OpCode::PhaseTransition),
        4000 => Some(OpCode::ProcessTokenDualPath),
        4001 => Some(OpCode::FinalizeComparison),
        9000 => Some(OpCode::CoreShutdown),
        9001 => Some(OpCode::CoreReset),
        9002 => Some(OpCode::BackupState),
        9003 => Some(OpCode::RestoreState),
        _ => None,
    }
}

// --- Парсинг payload (safe, без unsafe) ---

fn read_u16_le(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_f32_le(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
}

/// SpawnDomainPayload из raw payload bytes
fn parse_spawn_domain_payload(payload: &[u8; 48]) -> SpawnDomainPayload {
    SpawnDomainPayload {
        parent_domain_id: read_u16_le(payload, 0),
        factory_preset: payload[2],
        structural_role: payload[3],
        initial_energy: read_f32_le(payload, 4),
        seed: u32::from_le_bytes([payload[8], payload[9], payload[10], payload[11]]),
        reserved: [0; 36],
    }
}

/// InjectTokenPayload из raw payload bytes
fn parse_inject_token_payload(payload: &[u8; 48]) -> InjectTokenPayload {
    InjectTokenPayload {
        target_domain_id: read_u16_le(payload, 0),
        token_type: payload[2],
        mass: read_f32_le(payload, 4),
        position: [
            read_f32_le(payload, 8),
            read_f32_le(payload, 12),
            read_f32_le(payload, 16),
        ],
        velocity: [
            read_f32_le(payload, 20),
            read_f32_le(payload, 24),
            read_f32_le(payload, 28),
        ],
        semantic_weight: read_f32_le(payload, 32),
        temperature: read_f32_le(payload, 36),
        reserved: [0; 6],
    }
}

/// Построить Token из InjectTokenPayload
fn build_token_from_inject(p: &InjectTokenPayload, domain_id: u16, event_id: u64) -> Token {
    let pos = [
        p.position[0].clamp(i16::MIN as f32, i16::MAX as f32) as i16,
        p.position[1].clamp(i16::MIN as f32, i16::MAX as f32) as i16,
        p.position[2].clamp(i16::MIN as f32, i16::MAX as f32) as i16,
    ];
    let vel = [
        p.velocity[0].clamp(i16::MIN as f32, i16::MAX as f32) as i16,
        p.velocity[1].clamp(i16::MIN as f32, i16::MAX as f32) as i16,
        p.velocity[2].clamp(i16::MIN as f32, i16::MAX as f32) as i16,
    ];
    let mass = p.mass.clamp(0.0, 255.0) as u8;
    let temperature = p.temperature.clamp(0.0, 255.0) as u8;

    let mut token = Token::new(event_id as u32, domain_id, pos, event_id);
    token.velocity = vel;
    token.mass = mass;
    token.temperature = temperature;
    token.type_flags = p.token_type as u16;
    token
}
