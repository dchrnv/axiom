// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AxiomEngine — главный цикл: UCL → COM → Frontier → State

use std::collections::HashMap;
use axiom_core::{Token, Event};
use axiom_config::DomainConfig;
use axiom_domain::{Domain, DomainState, CapacityExceeded, EventGenerator};
use axiom_arbiter::{Arbiter, COM};
use axiom_ucl::{UclCommand, UclResult, OpCode, CommandStatus, SpawnDomainPayload, InjectTokenPayload};
use crate::guardian::Guardian;
use crate::snapshot::{EngineSnapshot, DomainSnapshot};
use crate::orchestrator;

/// Коды ошибок UclResult
pub mod error_codes {
    pub const OK: u16 = 0;
    pub const DOMAIN_NOT_FOUND: u16 = 1001;
    pub const CAPACITY_EXCEEDED: u16 = 1002;
    pub const DOMAIN_ALREADY_EXISTS: u16 = 1003;
    pub const INVALID_PAYLOAD: u16 = 1004;
    pub const ARBITER_NOT_READY: u16 = 2001;
    pub const GUARDIAN_VIOLATION: u16 = 3001;
    pub const UNKNOWN_OPCODE: u16 = 9001;
}

/// AxiomEngine — центральный оркестратор всех компонентов AXIOM.
///
/// Содержит все домены, их состояния, Arbiter и Guardian.
/// Принимает UCL-команды и выполняет главный цикл.
pub struct AxiomEngine {
    /// Физика каждого домена (frontier, heartbeat, spatial grid)
    domains: HashMap<u32, Domain>,
    /// Состояния доменов (токены, связи)
    states: HashMap<u32, DomainState>,
    /// Arbiter для маршрутизации
    pub arbiter: Arbiter,
    /// Guardian для проверки CODEX
    pub guardian: Guardian,
    /// Глобальный COM счётчик
    com: COM,
    /// Накопленные события текущего шага
    pending_events: Vec<Event>,
}

impl AxiomEngine {
    /// Создать новый Engine без доменов
    pub fn new() -> Self {
        let com = COM::new();
        let arbiter = Arbiter::new(HashMap::new(), COM::new());
        Self {
            domains: HashMap::new(),
            states: HashMap::new(),
            arbiter,
            guardian: Guardian::new(),
            com,
            pending_events: Vec::new(),
        }
    }

    /// Добавить домен в Engine.
    ///
    /// Регистрирует домен в Arbiter по structural_role и создаёт DomainState.
    pub fn add_domain(&mut self, config: DomainConfig) -> Result<(), String> {
        let domain_id = config.domain_id as u32;

        if self.domains.contains_key(&domain_id) {
            return Err(format!("Domain {} already exists", domain_id));
        }

        let state = DomainState::new(&config);
        let domain = Domain::new(config);

        // Регистрируем в Arbiter
        self.arbiter.add_domain_config(domain_id, config);
        self.arbiter.register_domain(config.structural_role, domain_id)?;

        self.domains.insert(domain_id, domain);
        self.states.insert(domain_id, state);

        Ok(())
    }

    /// Число доменов в Engine
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }

    /// Проверить что Arbiter готов к маршрутизации (все 11 доменов зарегистрированы)
    pub fn arbiter_ready(&self) -> bool {
        self.arbiter.is_ready()
    }

    /// Получить число токенов в домене
    pub fn token_count(&self, domain_id: u32) -> usize {
        self.states.get(&domain_id).map_or(0, |s| s.token_count())
    }

    /// Взять накопленные события (очищает буфер)
    pub fn drain_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.pending_events)
    }

    /// Создать snapshot текущего состояния Engine
    pub fn snapshot(&self) -> EngineSnapshot {
        let domains: Vec<DomainSnapshot> = self.domains.keys().map(|&id| {
            let config = self.domains[&id].config;
            let (tokens, connections) = self.states.get(&id)
                .map(|s| (s.tokens.clone(), s.connections.clone()))
                .unwrap_or_default();
            DomainSnapshot { domain_id: id, config, tokens, connections }
        }).collect();

        EngineSnapshot {
            domains,
            com_next_id: self.com.current_id(),
            created_at: self.com.current_id().saturating_sub(1),
        }
    }

    /// Восстановить Engine из snapshot
    pub fn restore_from(snapshot: &EngineSnapshot) -> Self {
        let mut engine = Self::new();

        for ds in &snapshot.domains {
            // Добавляем домен (игнорируем ошибки — snapshot консистентен)
            let _ = engine.add_domain(ds.config);

            // Восстанавливаем токены и связи
            if let Some(state) = engine.states.get_mut(&ds.domain_id) {
                for &token in &ds.tokens {
                    let _ = state.add_token(token);
                }
                for &conn in &ds.connections {
                    let _ = state.add_connection(conn);
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

    fn handle_spawn_domain(&mut self, cmd: &UclCommand) -> UclResult {
        let p = parse_spawn_domain_payload(&cmd.payload);

        let config = match p.factory_preset {
            0  => DomainConfig::factory_void(cmd.target_id as u16, p.parent_domain_id),
            1  => DomainConfig::factory_sutra(cmd.target_id as u16),
            6  => DomainConfig::factory_logic(cmd.target_id as u16, p.parent_domain_id),
            7  => DomainConfig::factory_dream(cmd.target_id as u16, p.parent_domain_id),
            9  => DomainConfig::factory_experience(cmd.target_id as u16, p.parent_domain_id),
            10 => DomainConfig::factory_maya(cmd.target_id as u16, p.parent_domain_id),
            _  => DomainConfig::factory_void(cmd.target_id as u16, p.parent_domain_id),
        };

        match self.add_domain(config) {
            Ok(()) => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 1),
            Err(_) => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_ALREADY_EXISTS, 0),
        }
    }

    fn handle_collapse_domain(&mut self, cmd: &UclCommand) -> UclResult {
        let id = cmd.target_id;
        if self.domains.remove(&id).is_some() {
            self.states.remove(&id);
            make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0)
        } else {
            make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_NOT_FOUND, 0)
        }
    }

    fn handle_inject_token(&mut self, cmd: &UclCommand) -> UclResult {
        let p = parse_inject_token_payload(&cmd.payload);
        let domain_id = p.target_domain_id as u32;

        let state = match self.states.get_mut(&domain_id) {
            Some(s) => s,
            None => return make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_NOT_FOUND, 0),
        };

        let event_id = self.com.next_event_id(p.target_domain_id);
        let token = build_token_from_inject(&p, domain_id as u16, event_id);

        match state.add_token(token) {
            Ok(_) => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 1),
            Err(CapacityExceeded) => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::CAPACITY_EXCEEDED, 0),
        }
    }

    fn handle_tick_forward(&mut self, cmd: &UclCommand) -> UclResult {
        let mut total_events = 0u16;

        let domain_ids: Vec<u32> = self.domains.keys().copied().collect();
        for id in domain_ids {
            let events = self.tick_domain(id);
            total_events = total_events.saturating_add(events.len() as u16);
            self.pending_events.extend(events);
        }

        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, total_events)
    }

    fn handle_dual_path(&mut self, cmd: &UclCommand) -> UclResult {
        if !self.arbiter_ready() {
            return make_result(cmd.command_id, CommandStatus::SystemError, error_codes::ARBITER_NOT_READY, 0);
        }

        // Берём токен из целевого домена по target_id
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
        let event_id = cmd.command_id; // event_id передаётся в command_id
        match self.arbiter.finalize_comparison(event_id) {
            Ok(()) => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0),
            Err(_)  => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_NOT_FOUND, 0),
        }
    }

    fn handle_backup(&mut self, cmd: &UclCommand) -> UclResult {
        let _snap = self.snapshot();
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, _snap.domain_count() as u16)
    }

    fn handle_restore(&mut self, cmd: &UclCommand) -> UclResult {
        // Без конкретного хранилища snapshot'ов — возвращаем Ok
        // Конкретная реализация живёт в адаптере (см. adapters.rs)
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0)
    }

    fn handle_reset(&mut self, cmd: &UclCommand) -> UclResult {
        self.domains.clear();
        self.states.clear();
        self.pending_events.clear();
        self.com = COM::new();
        self.arbiter = Arbiter::new(HashMap::new(), COM::new());
        self.guardian = Guardian::new();
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0)
    }

    // --- Внутренние хелперы ---

    /// Запустить один шаг физики для домена
    fn tick_domain(&mut self, domain_id: u32) -> Vec<Event> {
        let domain = match self.domains.get_mut(&domain_id) {
            Some(d) => d,
            None => return vec![],
        };
        let state = match self.states.get_mut(&domain_id) {
            Some(s) => s,
            None => return vec![],
        };

        // Шаг heartbeat
        if let Some(pulse) = domain.on_event() {
            domain.handle_heartbeat(pulse);
        }

        // Обновление frontier state
        domain.update_frontier_state();

        // Физика: process_frontier
        let mut event_gen = EventGenerator::new();
        domain.process_frontier(
            &mut state.tokens,
            &state.connections,
            &mut event_gen,
        )
    }

    /// Найти токен по sutra_id по всем доменам
    fn find_token_by_sutra_id(&self, sutra_id: u32) -> Option<Token> {
        for state in self.states.values() {
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
    // SpawnDomainPayload layout (repr(C)):
    // 0: parent_domain_id (u16)
    // 2: factory_preset (u8)
    // 3: structural_role (u8)
    // 4: initial_energy (f32)
    // 8: seed (u32)
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
    // InjectTokenPayload layout (repr(C)):
    // 0: target_domain_id (u16)
    // 2: token_type (u8)
    // 3: pad
    // 4: mass (f32)
    // 8: position[0] (f32), 12: [1], 16: [2]
    // 20: velocity[0] (f32), 24: [1], 28: [2]
    // 32: semantic_weight (f32)
    // 36: temperature (f32)
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
