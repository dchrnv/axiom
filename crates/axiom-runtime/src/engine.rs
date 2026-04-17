// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AxiomEngine — главный оркестратор: AshtiCore + Guardian
//
// Архитектура после введения AshtiCore:
//   AxiomEngine
//     ├── AshtiCore (11 доменов + Arbiter + Experience)
//     └── Guardian  (CODEX-валидация рефлексов)

use std::sync::Arc;
use axiom_core::{Token, Event};
use axiom_config::DomainConfig;
use axiom_domain::AshtiCore;
use axiom_genome::Genome;
use axiom_ucl::{UclCommand, UclResult, OpCode, CommandStatus, SpawnDomainPayload, InjectTokenPayload, ucl_preset_to_structural_role};
use crate::guardian::{Guardian, RoleStats};
use crate::snapshot::{EngineSnapshot, DomainSnapshot};
use crate::orchestrator;
use crate::adaptive::AdaptiveTickRate;

/// Имя домена по domain_id (используется в диагностике и broadcast-типах).
#[cfg(feature = "adapters")]
fn domain_name(id: u16) -> &'static str {
    match id % 100 {
        0  => "SUTRA",
        1  => "EXECUTION",
        2  => "SHADOW",
        3  => "CODEX",
        4  => "MAP",
        5  => "PROBE",
        6  => "LOGIC",
        7  => "DREAM",
        8  => "ETHICS",
        9  => "EXPERIENCE",
        10 => "MAYA",
        _  => "UNKNOWN",
    }
}

/// Ошибки инициализации AxiomEngine.
#[derive(Debug, Clone, PartialEq)]
pub enum AxiomError {
    /// GENOME не прошёл валидацию
    InvalidGenome(String),
}

impl std::fmt::Display for AxiomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AxiomError::InvalidGenome(msg) => write!(f, "Invalid GENOME: {msg}"),
        }
    }
}

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

/// Расписание периодических задач TickForward.
///
/// Каждое поле — интервал в тиках (0 = задача отключена).
/// Задача выполняется когда `tick_count % interval == 0`.
#[derive(Debug, Clone, Copy)]
pub struct TickSchedule {
    /// Адаптация порогов Guardian (default: 50)
    pub adaptation_interval:    u32,
    /// GC следов Experience по causal horizon (default: 500)
    pub horizon_gc_interval:    u32,
    /// Snapshot + pruning (default: 5000)
    pub snapshot_interval:      u32,
    /// DREAM-предложения CODEX (default: 100)
    pub dream_interval:         u32,
    /// Проверка TensionTrace (Cognitive Depth) (default: 10)
    pub tension_check_interval: u32,
    /// Проверка GoalPersistence (Cognitive Depth) (default: 10)
    pub goal_check_interval:    u32,
    /// Shell reconcile (default: 200)
    pub reconcile_interval:     u32,
    /// Автосохранение состояния на диск (default: 0 = отключено).
    /// При ненулевом значении — сохраняет каждые N тиков.
    pub persist_check_interval: u32,
    /// Адаптивная частота тиков (Axiom Sentinel V1.0, Фаза 3).
    /// Управляет частотой главного цикла CliChannel при включённом adaptive mode.
    pub adaptive_tick: AdaptiveTickRate,
}

impl Default for TickSchedule {
    fn default() -> Self {
        Self {
            adaptation_interval:    50,
            horizon_gc_interval:    500,
            snapshot_interval:      5000,
            dream_interval:         100,
            tension_check_interval: 10,
            goal_check_interval:    10,
            reconcile_interval:     200,
            persist_check_interval: 0,
            adaptive_tick:          AdaptiveTickRate::default(),
        }
    }
}

/// AxiomEngine — центральный оркестратор всех компонентов AXIOM.
///
/// Содержит один уровень AshtiCore (11 доменов + Arbiter + Experience)
/// и Guardian для CODEX+GENOME-валидации рефлексов.
///
/// Axiom Sentinel V1.0: Hardware-Aware Topology.
/// При создании определяет число аппаратных потоков и создаёт `rayon::ThreadPool`
/// с `worker_count - 1` потоками (один резервируется под ОС/Gateway).
pub struct AxiomEngine {
    /// Genome — конституция системы (заморожена в Arc после boot)
    genome: Arc<Genome>,
    /// Фрактальный уровень Ashti_Core (11 доменов)
    pub ashti: AshtiCore,
    /// Guardian для проверки CODEX + GENOME
    pub guardian: Guardian,
    /// Накопленные события текущего шага
    pending_events: Vec<Event>,
    /// Глобальный COM-счётчик: монотонный источник event_id
    pub com_next_id: u64,
    /// Монотонный счётчик тиков (TickForward)
    pub tick_count: u64,
    /// Расписание периодических задач
    pub tick_schedule: TickSchedule,
    /// Число аппаратных потоков, определённых при boot (available_parallelism).
    /// Минимум 1.
    pub worker_count: usize,
    /// Rayon ThreadPool для параллельных операций (фазы 2, 3 Sentinel).
    /// Размер пула = max(1, worker_count - 1).
    pub thread_pool: rayon::ThreadPool,
}

impl AxiomEngine {
    /// Создать Engine с указанным Genome.
    ///
    /// Genome валидируется перед созданием — невалидный Genome → `Err(AxiomError::InvalidGenome)`.
    /// Это единственный путь boot sequence: Genome создаётся первым, замораживается в `Arc`,
    /// затем передаётся в Guardian (и далее по цепочке в Шаге 4).
    pub fn try_new(genome: Arc<Genome>) -> Result<Self, AxiomError> {
        genome.validate().map_err(|e| AxiomError::InvalidGenome(e.to_string()))?;
        let worker_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        Ok(Self {
            genome: Arc::clone(&genome),
            ashti: AshtiCore::new(1),
            guardian: Guardian::new(genome),
            pending_events: Vec::new(),
            com_next_id: 1,
            tick_count: 0,
            tick_schedule: TickSchedule::default(),
            worker_count,
            thread_pool: build_thread_pool(worker_count),
        })
    }

    /// Создать новый Engine с захардкоженным Ashti_Core Genome.
    pub fn new() -> Self {
        Self::try_new(Arc::new(Genome::default_ashti_core()))
            .expect("default_ashti_core genome is always valid")
    }

    /// Монотонный COM event_id. Вызывать при каждом создании события на уровне Engine.
    #[inline]
    pub fn next_event_id(&mut self) -> u64 {
        let id = self.com_next_id;
        self.com_next_id += 1;
        id
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
    pub fn token_count(&self, domain_id: u16) -> usize {
        self.ashti.token_count(domain_id)
    }

    // ── Convenience accessors (используются адаптерами и диагностикой) ──────

    /// Общее число следов опыта.
    pub fn trace_count(&self) -> usize {
        self.ashti.experience().trace_count()
    }

    /// Число активных tension traces.
    pub fn tension_count(&self) -> usize {
        self.ashti.experience().tension_count()
    }

    /// Число трейсов совпавших при последней маршрутизации (last-seen).
    ///
    /// Значение перезаписывается при каждом `route_token`. Если с момента
    /// последнего broadcast не было inject — возвращает то же значение что раньше.
    pub fn last_matched(&self) -> u32 {
        self.ashti.experience().last_traces_matched.get()
    }

    /// Лёгкий snapshot для broadcast (только числа, без клонирования токенов).
    #[cfg(feature = "adapters")]
    pub fn snapshot_for_broadcast(&self) -> crate::broadcast::BroadcastSnapshot {
        crate::broadcast::BroadcastSnapshot {
            tick_count:       self.tick_count,
            com_next_id:      self.com_next_id,
            trace_count:      self.trace_count(),
            tension_count:    self.tension_count(),
            domain_summaries: self.domain_summaries(),
        }
    }

    /// Детальный snapshot одного домена — по явному запросу (dashboard, REST GET /domain/:id).
    #[cfg(feature = "adapters")]
    pub fn domain_detail_snapshot(&self, domain_id: u16) -> Option<crate::broadcast::DomainDetailSnapshot> {
        let idx = self.ashti.index_of(domain_id)?;
        let state = self.ashti.state(idx)?;
        Some(crate::broadcast::DomainDetailSnapshot {
            domain_id,
            tokens:      state.tokens.iter().map(crate::broadcast::TokenSnapshot::from).collect(),
            connections: state.connections.iter().map(crate::broadcast::ConnectionSnapshot::from).collect(),
        })
    }

    /// Краткая сводка по всем 11 доменам для BroadcastSnapshot.
    #[cfg(feature = "adapters")]
    fn domain_summaries(&self) -> Vec<crate::broadcast::DomainSummary> {
        use crate::broadcast::DomainSummary;
        (0u16..=10).map(|offset| {
            let id = 100 + offset;
            let conn_count = self.ashti.index_of(id)
                .and_then(|i| self.ashti.state(i))
                .map_or(0, |s| s.connections.len());
            DomainSummary {
                domain_id:        id,
                name:             domain_name(id).to_string(),
                token_count:      self.ashti.token_count(id),
                connection_count: conn_count,
            }
        }).collect()
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
                    .unwrap_or_else(|| DomainConfig::factory_void(id, 0));
                DomainSnapshot {
                    domain_id: id,
                    config,
                    tokens: state.tokens.clone(),
                    connections: state.connections.clone(),
                }
            })
            .collect();

        let horizon = self.ashti.compute_horizon();

        EngineSnapshot {
            domains,
            com_next_id: self.com_next_id,
            tick_count: self.tick_count,
            created_at: horizon,
        }
    }

    /// Удалить из Experience следы с `last_used < snapshot.created_at`.
    ///
    /// Безопасно: snapshot уже зафиксировал состояние, стары следы больше не нужны.
    /// Возвращает число удалённых следов.
    pub fn prune_after_snapshot(&mut self, snapshot: &EngineSnapshot) -> usize {
        self.ashti
            .experience_mut()
            .archive_behind_horizon(snapshot.created_at)
    }

    /// Сделать snapshot и сразу выполнить pruning Experience.
    ///
    /// Удобная комбинация: фиксируем состояние → удаляем устаревшие следы.
    /// Возвращает (snapshot, число_удалённых_следов).
    pub fn snapshot_and_prune(&mut self) -> (EngineSnapshot, usize) {
        let snap = self.snapshot();
        let pruned = self.prune_after_snapshot(&snap);
        (snap, pruned)
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

        // Восстанавливаем COM-счётчик и tick_count: монотонность гарантирована
        if snapshot.com_next_id > 0 {
            engine.com_next_id = snapshot.com_next_id;
        }
        engine.tick_count = snapshot.tick_count;

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
            // Опкоды протокола, физика которых не реализована — принимаются без ошибки (no-op)
            OpCode::LockMembrane
            | OpCode::ReshapeDomain
            | OpCode::ApplyForce
            | OpCode::AnnihilateToken
            | OpCode::BondTokens
            | OpCode::SplitToken
            | OpCode::ChangeTemperature
            | OpCode::ApplyGravity
            | OpCode::PhaseTransition => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0),
            #[allow(unreachable_patterns)]
            _ => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::UNKNOWN_OPCODE, 0),
        }
    }

    /// Обработать команду с наблюдением когнитивного конвейера.
    ///
    /// Для `InjectToken`: токен немедленно маршрутизируется через SUTRA→EXPERIENCE→ASHTI→MAYA
    /// и возвращается диагностический `ProcessingResult` с данными о пути, когерентности,
    /// рефлексе, tension traces.
    ///
    /// Для остальных команд: вызывает `process_command()` и возвращает минимальный результат.
    ///
    /// **Не заменяет `process_command()`** — это отдельный путь для CLI/адаптеров.
    /// Overhead ≈ 10 ns (чтение полей RoutingResult и Experience).
    pub fn process_and_observe(&mut self, cmd: &UclCommand) -> crate::result::ProcessingResult {
        use crate::result::{ProcessingResult, ProcessingPath};
        use axiom_ucl::OpCode;

        let is_inject = matches!(
            opcode_from_u16(cmd.opcode),
            Some(OpCode::InjectToken)
        );

        if is_inject && self.arbiter_ready() {
            let p = parse_inject_token_payload(&cmd.payload);
            let event_id = self.next_event_id();
            let token = build_token_from_inject(&p, p.target_domain_id, event_id);

            // Захватываем состояние до маршрутизации
            let tension_before   = self.ashti.experience().tension_count();
            let total_traces     = self.ashti.experience().trace_count() as u32;
            let input_position   = token.position;
            let input_shell: [u8; 8] = [0, 0, 0, token.valence.unsigned_abs(), token.temperature, token.mass, 0, 0];
            let input_hash       = fnv1a_token_hash(&token);
            let (max_passes, min_coherence) = self.maya_multipass_params();

            let ucl_result = make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 1);
            let routing = orchestrator::route_token(self, token);

            let tension_created  = self.ashti.experience().tension_count() > tension_before;
            let reflex_hit       = routing.reflex.is_some();
            let confidence       = routing.confidence;
            let passes           = routing.passes;

            let path = if passes > 1 {
                ProcessingPath::MultiPass(passes)
            } else if reflex_hit {
                ProcessingPath::Reflex
            } else {
                ProcessingPath::SlowPath
            };

            let dominant_domain_id = routing.consolidated
                .map(|t| t.domain_id)
                .unwrap_or(110); // MAYA

            let output_position = routing.consolidated
                .map(|t| t.position)
                .unwrap_or([0, 0, 0]);

            // output_shell: диагностическое приближение из полей выходного токена.
            // L4=эмоциональное(valence), L5=когнитивное(temperature), L6=социальное(mass).
            let output_shell: [u8; 8] = if let Some(t) = routing.consolidated {
                [0, 0, 0, t.valence.unsigned_abs(), t.temperature, t.mass, 0, 0]
            } else {
                [0u8; 8]
            };

            let tension_count    = self.ashti.experience().tension_count() as u32;
            let traces_matched   = self.ashti.experience().last_traces_matched.get();

            return ProcessingResult {
                ucl_result,
                path,
                dominant_domain_id,
                coherence_score: Some(confidence),
                tension_count,
                output_shell,
                output_position,
                reflex_hit,
                traces_matched,
                passes,
                max_passes,
                min_coherence,
                total_traces,
                event_id: routing.event_id,
                input_position,
                input_shell,
                input_hash,
                tension_created,
            };
        }

        // Все остальные команды — стандартная обработка без диагностики маршрутизации
        let ucl_result = self.process_command(cmd);
        ProcessingResult::from_ucl(ucl_result)
    }

    /// Получить параметры multi-pass из конфига MAYA (domain 110 для level 1).
    pub fn maya_multipass_params(&self) -> (u8, f32) {
        // MAYA = level_id * 100 + 10; для level 1 → 110
        let maya_id = self.ashti.level_id() * 100 + 10;
        if let Some(cfg) = self.ashti.config_of(maya_id) {
            (cfg.max_passes, cfg.min_coherence as f32 / 255.0)
        } else {
            (0, 0.6)
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
        let domain_id = p.target_domain_id;

        // Проверяем что домен существует в AshtiCore
        if self.ashti.index_of(domain_id).is_none() {
            return make_result(cmd.command_id, CommandStatus::SystemError, error_codes::DOMAIN_NOT_FOUND, 0);
        }

        let event_id = self.next_event_id();
        let token = build_token_from_inject(&p, p.target_domain_id, event_id);

        match self.ashti.inject_token(domain_id, token) {
            Ok(_)  => make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 1),
            Err(_) => make_result(cmd.command_id, CommandStatus::SystemError, error_codes::CAPACITY_EXCEEDED, 0),
        }
    }

    fn handle_tick_forward(&mut self, cmd: &UclCommand) -> UclResult {
        self.tick_count += 1;
        let t = self.tick_count;
        let s = self.tick_schedule;

        // Hot path: физика всех 11 доменов каждый тик
        let events = self.ashti.tick();
        let count = events.len() as u16;
        self.pending_events.extend(events);

        // Warm path: tension traces (Cognitive Depth)
        // Impulse-токены проходят через когнитивный pipeline (SUTRA→EXPERIENCE→ASHTI→MAYA).
        // TOKEN_FLAG_IMPULSE предотвращает петлю: impulse → tension → impulse → ...
        if s.tension_check_interval > 0 && t % s.tension_check_interval as u64 == 0 {
            let impulses = self.ashti.arbiter_heartbeat_pulse(t, true);
            for mut token in impulses {
                token.type_flags |= axiom_core::TOKEN_FLAG_IMPULSE;
                let _ = orchestrator::route_token(self, token);
            }
        }

        // Warm path: goal impulses (Cognitive Depth)
        // Аналогично: goal-импульсы маршрутизируются когнитивно, не физически.
        if s.goal_check_interval > 0 && t % s.goal_check_interval as u64 == 0 {
            let goals = self.ashti.generate_goal_impulses(t, s.goal_check_interval as u64);
            for impulse in goals {
                let mut token = impulse.pattern;
                token.type_flags |= axiom_core::TOKEN_FLAG_IMPULSE;
                let _ = orchestrator::route_token(self, token);
            }
        }

        // Warm path: DREAM
        if s.dream_interval > 0 && t % s.dream_interval as u64 == 0 {
            let _ = self.dream_propose();
        }

        // Cold path: адаптация порогов
        if s.adaptation_interval > 0 && t % s.adaptation_interval as u64 == 0 {
            let _ = self.run_adaptation();
        }

        // Cold path: horizon GC
        if s.horizon_gc_interval > 0 && t % s.horizon_gc_interval as u64 == 0 {
            let _ = self.run_horizon_gc();
        }

        // Cold path: reconcile семантического пространства
        if s.reconcile_interval > 0 && t % s.reconcile_interval as u64 == 0 {
            let _ = self.ashti.reconcile_all();
        }

        // Cold path: snapshot + prune
        if s.snapshot_interval > 0 && t % s.snapshot_interval as u64 == 0 {
            let _ = self.snapshot_and_prune();
        }

        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, count)
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

    // ============================================================
    // Этап 6: Адаптация порогов (GUARDIAN ← REFLECTOR)
    // ============================================================

    /// Запустить цикл адаптации: GUARDIAN читает REFLECTOR, обновляет DomainConfig.
    ///
    /// Шаг 1: адаптирует reflex_threshold.
    /// Шаг 2: адаптирует temperature и resonance_freq.
    /// После обновления конфигов применяет новые пороги к Experience модулю.
    ///
    /// Возвращает список domain_id, чьи конфиги изменились.
    pub fn run_adaptation(&mut self) -> Vec<u16> {
        // Собираем статистику (иммутабельный заём завершается до мутабельного)
        let role_stats: Vec<RoleStats> = (1u8..=8).filter_map(|role| {
            let profile = self.ashti.reflector().domain_profile(role)?;
            let total = profile.total_calls();
            if total == 0 { return None; }
            Some(RoleStats {
                role,
                success_rate: profile.overall_success_rate(),
                total_calls: total,
            })
        }).collect();

        if role_stats.is_empty() {
            return Vec::new();
        }

        // Адаптируем пороги и физику
        let configs = self.ashti.arbiter_domain_configs_mut();
        let mut updated = self.guardian.adapt_thresholds(&role_stats, configs);
        let phys_updated = self.guardian.adapt_domain_physics(&role_stats, configs);
        for id in phys_updated {
            if !updated.contains(&id) {
                updated.push(id);
            }
        }

        // Применяем новые пороги к Experience модулю
        self.ashti.apply_experience_thresholds();

        updated
    }

    // ============================================================
    // Этап 7: Causal Horizon
    // ============================================================

    /// Вычислить текущий причинный горизонт системы.
    ///
    /// `horizon = min(token.last_event_id)` по всем доменам.
    /// Следы с `last_used < horizon` считаются каузально устаревшими.
    pub fn causal_horizon(&self) -> u64 {
        self.ashti.compute_horizon()
    }

    // ============================================================
    // Этап 7 Шаг 4: Обмен скиллами
    // ============================================================

    /// Экспортировать все кристаллизованные навыки из SkillSet.
    pub fn export_skills(&self) -> Vec<axiom_arbiter::Skill> {
        self.ashti.export_skills()
    }

    /// Импортировать навыки из другого экземпляра.
    ///
    /// Дубли пропускаются. Возвращает число фактически импортированных.
    pub fn import_skills(&mut self, skills: &[axiom_arbiter::Skill]) -> usize {
        self.ashti.import_skills(skills)
    }

    /// Запустить сборку мусора Experience по причинному горизонту.
    ///
    /// Удаляет следы с `last_used < horizon`, чистит AssociativeIndex.
    /// Возвращает число удалённых следов.
    pub fn run_horizon_gc(&mut self) -> usize {
        self.ashti.run_horizon_gc()
    }

    /// DREAM(7): проанализировать Experience и предложить изменения CODEX.
    ///
    /// Извлекает высокоактивные паттерны из Experience (weight ≥ 0.9, success_count ≥ 5)
    /// и передаёт их Guardian для генерации CodexAction предложений.
    pub fn dream_propose(&mut self) -> Vec<crate::guardian::CodexAction> {
        let candidates: Vec<_> = self.ashti
            .experience_mut()
            .find_crystallizable(0.9, 5)
            .into_iter()
            .map(|t| t.pattern)
            .collect();

        self.guardian.dream_propose(&candidates)
    }

    fn handle_reset(&mut self, cmd: &UclCommand) -> UclResult {
        self.ashti = AshtiCore::new(1);
        self.guardian = Guardian::new(Arc::clone(&self.genome));
        self.pending_events.clear();
        make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0)
    }

    // --- Внутренние хелперы ---

    /// Инжектировать набор якорных токенов в движок.
    ///
    /// Якоря — конфигурационные объекты: mass=255, temperature=0, state=Locked.
    /// Проходят напрямую, минуя GUARDIAN и UCL-pipeline.
    ///
    /// Возвращает число успешно инжектированных якорей.
    pub fn inject_anchor_tokens(&mut self, anchor_set: &axiom_config::AnchorSet) -> usize {
        let mut injected = 0usize;

        // 1. Оси + слоевые якоря → SUTRA(100)
        let sutra_id: u16 = self.ashti.level_id() * 100;
        let axis_iter = anchor_set.axes.iter();
        let layer_iter = anchor_set.layers.iter().flatten();
        for anchor in axis_iter.chain(layer_iter) {
            let event_id = self.next_event_id();
            let mut token = Token::new(event_id as u32, sutra_id, anchor.position, event_id);
            token.mass        = 255;
            token.temperature = 0;
            token.state       = axiom_core::STATE_LOCKED;
            if self.ashti.inject_token(sutra_id, token).is_ok() {
                injected += 1;
            }
        }

        // 2. Доменные якоря → ASHTI(1-8): domain_id = level*100 + 1..=8
        let level = self.ashti.level_id();
        for (i, domain_anchors) in anchor_set.domains.iter().enumerate() {
            let domain_id: u16 = level * 100 + 1 + i as u16; // 101..=108
            for anchor in domain_anchors {
                let event_id = self.next_event_id();
                let mut token = Token::new(event_id as u32, domain_id, anchor.position, event_id);
                token.mass        = 255;
                token.temperature = 0;
                token.state       = axiom_core::STATE_LOCKED;
                if self.ashti.inject_token(domain_id, token).is_ok() {
                    injected += 1;
                }
            }
        }

        injected
    }

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

/// Создать rayon::ThreadPool с `max(1, worker_count - 1)` потоков.
///
/// Один поток резервируется под ОС/Gateway tick loop.
fn build_thread_pool(worker_count: usize) -> rayon::ThreadPool {
    let threads = worker_count.saturating_sub(1).max(1);
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("rayon ThreadPool build failed")
}

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
///
/// `structural_role` нормализуется через маппинг UCL preset → StructuralRole enum,
/// чтобы числовые значения совпадали с `StructuralRole` (Sutra=0, Void=8).
fn parse_spawn_domain_payload(payload: &[u8; 48]) -> SpawnDomainPayload {
    let factory_preset = payload[2];
    SpawnDomainPayload {
        parent_domain_id: read_u16_le(payload, 0),
        factory_preset,
        structural_role: ucl_preset_to_structural_role(factory_preset),
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

/// FNV-1a хэш ключевых полей токена — для диагностического вывода.
/// Тот же алгоритм что и pattern_hash в experience.rs.
fn fnv1a_token_hash(token: &Token) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    h ^= token.temperature as u64; h = h.wrapping_mul(0x100000001b3);
    h ^= token.mass         as u64; h = h.wrapping_mul(0x100000001b3);
    h ^= token.valence as u8 as u64; h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[0]  as u64; h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[1]  as u64; h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[2]  as u64; h = h.wrapping_mul(0x100000001b3);
    h
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
