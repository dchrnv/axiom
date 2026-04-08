// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AXIOM: AshtiCore — архитектурная композиция из 11 доменов (Ashti_Core v2.0)
//
// Собирает все готовые компоненты в один работающий уровень:
//   SUTRA(0) → EXPERIENCE(9) ⇄ ASHTI(1-8) → MAYA(10)
//
// Связанные спецификации:
//   - docs/spec/Ashti_Core_v2_0.md (каноническая)
//   - docs/spec/Arbiter_V1_0.md

use std::collections::HashMap;
use axiom_core::{Token, Event};
use axiom_config::DomainConfig;
use axiom_arbiter::{Arbiter, COM, RoutingResult};
use crate::{Domain, DomainState, CausalHorizon};

/// Один фрактальный уровень Ashti_Core: 11 доменов + маршрутизатор.
///
/// Порядок доменов по structural_role:
/// - 0:  SUTRA      — единственный источник истины
/// - 1:  EXECUTION  — реализация решений
/// - 2:  SHADOW     — симуляция угроз и сценариев
/// - 3:  CODEX      — конституция, пассивный фильтр правил
/// - 4:  MAP        — статичная база фактов
/// - 5:  PROBE      — активное зондирование среды
/// - 6:  LOGIC      — чистое вычисление и дедукция
/// - 7:  DREAM      — фоновая оптимизация и обучение
/// - 8:  VOID       — сбор неопределённостей и аномалий
/// - 9:  EXPERIENCE — ассоциативная память, рефлексы
/// - 10: MAYA       — проекция результата, интерфейс выхода
pub struct AshtiCore {
    /// 11 Domain instances (физика поля каждого домена)
    domains: Vec<Domain>,
    /// 11 DomainState instances (токены и связи каждого домена)
    states: Vec<DomainState>,
    /// Arbiter — над-доменный маршрутизатор (dual-path routing)
    arbiter: Arbiter,
    /// Идентификатор фрактального уровня
    level_id: u16,
    /// Текущий пульс (для handle_heartbeat)
    pulse: u64,
}

impl AshtiCore {
    /// Создать новый уровень Ashti_Core.
    ///
    /// `level_id` определяет пространство domain_id: домены получают ID
    /// `level_id * 100 + role` (0..10), что исключает коллизии между уровнями.
    pub fn new(level_id: u16) -> Self {
        let base = level_id as u32 * 100;
        let sutra_id = base as u16;

        // Конфиги всех 11 доменов в порядке structural_role
        let role_configs: [(u8, DomainConfig); 11] = [
            (0,  DomainConfig::factory_sutra(sutra_id)),
            (1,  DomainConfig::factory_execution(base as u16 + 1,  sutra_id)),
            (2,  DomainConfig::factory_shadow   (base as u16 + 2,  sutra_id)),
            (3,  DomainConfig::factory_codex    (base as u16 + 3,  sutra_id)),
            (4,  DomainConfig::factory_map      (base as u16 + 4,  sutra_id)),
            (5,  DomainConfig::factory_probe    (base as u16 + 5,  sutra_id)),
            (6,  DomainConfig::factory_logic    (base as u16 + 6,  sutra_id)),
            (7,  DomainConfig::factory_dream    (base as u16 + 7,  sutra_id)),
            (8,  DomainConfig::factory_void     (base as u16 + 8,  sutra_id)),
            (9,  DomainConfig::factory_experience(base as u16 + 9, sutra_id)),
            (10, DomainConfig::factory_maya     (base as u16 + 10, sutra_id)),
        ];

        let mut domain_map: HashMap<u32, DomainConfig> = HashMap::with_capacity(11);
        let mut domains: Vec<Domain>      = Vec::with_capacity(11);
        let mut states:  Vec<DomainState> = Vec::with_capacity(11);

        for (_, config) in &role_configs {
            states.push(DomainState::new(config));
            domain_map.insert(config.domain_id as u32, *config);
            domains.push(Domain::new(*config));
        }

        let mut arbiter = Arbiter::new(domain_map, COM::new());

        for (role, config) in &role_configs {
            let _ = arbiter.register_domain(*role, config.domain_id as u32);
        }

        Self { domains, states, arbiter, level_id, pulse: 0 }
    }

    /// Обработать входящий токен — полный dual-path цикл.
    ///
    /// Запускает маршрут SUTRA(0) → EXPERIENCE(9) → ASHTI(1-8) + optional reflex → MAYA(10).
    /// Возвращает `RoutingResult` с reflex, slow_path и consolidated.
    pub fn process(&mut self, token: Token) -> RoutingResult {
        self.arbiter.route_token(token, 0)
    }

    /// Параллельная обработка токена (Axiom Sentinel V1.0, Фаза 2).
    ///
    /// Идентична `process()`, но при наличии достаточного числа traces в Experience
    /// Phase 2 resonance search выполняется параллельно через `pool`.
    /// При `traces.len() < PARALLEL_THRESHOLD` — автоматический fallback на sequential.
    pub fn process_parallel(&mut self, token: Token, pool: &rayon::ThreadPool) -> RoutingResult {
        self.arbiter.route_token_parallel(token, 0, pool)
    }

    /// Применить обратную связь после завершения сравнения.
    ///
    /// Усиляет или ослабляет след в Experience в зависимости от совпадения
    /// рефлекса с результатом ASHTI.
    pub fn apply_feedback(&mut self, event_id: u64) -> Result<(), String> {
        self.arbiter.finalize_comparison(event_id)
    }

    /// Тик физики всех 11 доменов.
    ///
    /// Вызывает `on_event()`, `handle_heartbeat()` и `process_frontier()` для каждого домена.
    /// Возвращает все физические события, сгенерированные за этот тик.
    pub fn tick(&mut self) -> Vec<Event> {
        self.pulse += 1;
        let mut all_events = Vec::new();
        for i in 0..self.domains.len() {
            if let Some(pulse) = self.domains[i].on_event() {
                self.domains[i].handle_heartbeat(pulse);
                let tokens = &self.states[i].tokens;
                let conns  = &self.states[i].connections;
                let mut gen = crate::physics::EventGenerator::new();
                gen.set_pulse_id(pulse);
                let events = self.domains[i].process_frontier(tokens, conns, &mut gen);
                all_events.extend(events);
            }
        }
        all_events
    }

    /// Все 11 доменов зарегистрированы и Arbiter готов к маршрутизации.
    pub fn is_ready(&self) -> bool {
        self.arbiter.is_ready()
    }

    /// Идентификатор фрактального уровня.
    pub fn level_id(&self) -> u16 {
        self.level_id
    }

    /// Иммутабельный доступ к модулю Experience (для диагностики).
    pub fn experience(&self) -> &axiom_arbiter::ExperienceModule {
        self.arbiter.experience()
    }

    /// Mutable доступ к модулю Experience для прямого управления следами.
    pub fn experience_mut(&mut self) -> &mut axiom_arbiter::ExperienceModule {
        self.arbiter.experience_mut()
    }

    /// Доступ к состоянию домена по его позиции (0–10).
    pub fn state(&self, index: usize) -> Option<&DomainState> {
        self.states.get(index)
    }

    /// Mutable доступ к состоянию домена по его позиции (0–10).
    pub fn state_mut(&mut self, index: usize) -> Option<&mut DomainState> {
        self.states.get_mut(index)
    }

    /// domain_id домена по его позиции (0–10).
    ///
    /// Формула: `level_id * 100 + index`.
    pub fn domain_id_at(&self, index: usize) -> Option<u32> {
        if index > 10 { return None; }
        Some(self.level_id as u32 * 100 + index as u32)
    }

    /// Найти позицию домена по domain_id.
    pub fn index_of(&self, domain_id: u32) -> Option<usize> {
        let base = self.level_id as u32 * 100;
        if domain_id < base || domain_id > base + 10 { return None; }
        Some((domain_id - base) as usize)
    }

    /// Число токенов в домене с заданным domain_id.
    pub fn token_count(&self, domain_id: u32) -> usize {
        self.index_of(domain_id)
            .and_then(|i| self.states.get(i))
            .map_or(0, |s| s.token_count())
    }

    /// Впрыснуть токен в домен по domain_id.
    ///
    /// Добавляет токен в DomainState и синхронизирует счётчик active_tokens домена,
    /// чтобы следующий тик heartbeat учитывал реальное число токенов.
    pub fn inject_token(&mut self, domain_id: u32, token: axiom_core::Token) -> Result<usize, crate::CapacityExceeded> {
        let idx = self.index_of(domain_id).ok_or(crate::CapacityExceeded)?;
        let result = self.states[idx].add_token(token)?;
        self.domains[idx].active_tokens = self.states[idx].token_count();
        Ok(result)
    }

    /// Доступ к REFLECTOR — статистика рефлексов для адаптации порогов.
    pub fn reflector(&self) -> &axiom_arbiter::Reflector {
        &self.arbiter.reflector
    }

    /// Mutable доступ к HashMap конфигураций доменов в Arbiter.
    ///
    /// Используется Guardian для адаптации порогов без раскрытия всего Arbiter.
    pub fn arbiter_domain_configs_mut(&mut self) -> &mut std::collections::HashMap<u32, axiom_config::DomainConfig> {
        self.arbiter.domain_configs_mut()
    }

    // ──────────────────────────────────────────────────────────────
    // Этап 7 Шаг 4: Обмен скиллами
    // ──────────────────────────────────────────────────────────────

    /// Экспортировать все кристаллизованные навыки.
    pub fn export_skills(&self) -> Vec<axiom_arbiter::Skill> {
        self.arbiter.skillset.export()
    }

    /// Импортировать пакет навыков из другого экземпляра.
    ///
    /// Возвращает число фактически импортированных (без дублей).
    pub fn import_skills(&mut self, skills: &[axiom_arbiter::Skill]) -> usize {
        self.arbiter.skillset.import_batch(skills)
    }

    /// Импортировать навык с явным weight factor (для knowledge exchange).
    ///
    /// Возвращает true если навык был добавлен (не дубликат).
    pub fn import_skill_exchange(&mut self, skill: axiom_arbiter::Skill, weight_factor: f32) -> bool {
        self.arbiter.skillset.import_skill_exchange(skill, weight_factor)
    }

    /// Применить пороги EXPERIENCE-домена к модулю Experience.
    ///
    /// Вызывать после обновления конфигов через `arbiter_domain_configs_mut`.
    pub fn apply_experience_thresholds(&mut self) {
        self.arbiter.apply_experience_thresholds();
    }

    // ──────────────────────────────────────────────────────────────
    // Этап 7: Causal Horizon
    // ──────────────────────────────────────────────────────────────

    /// Вычислить текущий причинный горизонт — min(token.last_event_id).
    pub fn compute_horizon(&self) -> u64 {
        let state_refs: Vec<&DomainState> = self.states.iter().collect();
        CausalHorizon::compute(&state_refs)
    }

    /// Архивировать следы Experience, каузально устаревшие за горизонтом.
    ///
    /// Возвращает число удалённых следов.
    pub fn run_horizon_gc(&mut self) -> usize {
        let horizon = self.compute_horizon();
        self.arbiter.experience_mut().archive_behind_horizon(horizon)
    }

    /// Конфигурации всех доменов (domain_id, DomainConfig) — для snapshot.
    /// Получить конфиг домена по domain_id.
    pub fn config_of(&self, domain_id: u32) -> Option<axiom_config::DomainConfig> {
        let idx = self.index_of(domain_id)?;
        Some(self.domains[idx].config)
    }

    pub fn all_configs(&self) -> Vec<(u32, axiom_config::DomainConfig)> {
        (0..11).filter_map(|i| {
            let id = self.domain_id_at(i)?;
            Some((id, self.domains[i].config))
        }).collect()
    }

    /// Состояния всех доменов (domain_id, tokens, connections) — для snapshot.
    pub fn all_states(&self) -> Vec<(u32, &DomainState)> {
        (0..11).filter_map(|i| {
            let id = self.domain_id_at(i)?;
            Some((id, &self.states[i]))
        }).collect()
    }

    // ──────────────────────────────────────────────────────────────
    // Этап 12: Фрактальные цепочки
    // ──────────────────────────────────────────────────────────────

    /// Забрать токен из MAYA(10) — выход фрактального уровня.
    ///
    /// Снимает последний токен из домена MAYA и синхронизирует счётчик.
    /// Возвращает `None` если MAYA пуста.
    pub fn take_maya_output(&mut self) -> Option<Token> {
        let maya_idx = 10;
        let token = self.states[maya_idx].tokens.pop()?;
        self.domains[maya_idx].active_tokens = self.states[maya_idx].token_count();
        Some(token)
    }

    /// Впрыснуть токен в SUTRA(0) — вход фрактального уровня.
    ///
    /// Эквивалентно `inject_token(level_id * 100, token)`.
    pub fn set_sutra_input(&mut self, token: Token) -> Result<usize, crate::CapacityExceeded> {
        let sutra_id = self.level_id as u32 * 100;
        self.inject_token(sutra_id, token)
    }

    /// Обработать пульс Heartbeat в Arbiter (Cognitive Depth — tension traces).
    ///
    /// Охлаждает следы напряжения, возвращает список внутренних импульсов
    /// готовых к обработке. `enable_internal_drive = false` — система реактивна.
    pub fn arbiter_heartbeat_pulse(&mut self, pulse_number: u64, enable_internal_drive: bool) -> Vec<axiom_core::Token> {
        self.arbiter.on_heartbeat_pulse(pulse_number, enable_internal_drive)
    }

    /// Сгенерировать goal-импульсы (Cognitive Depth — цели из CODEX).
    pub fn generate_goal_impulses(&self, pulse_number: u64, check_interval: u64) -> Vec<axiom_arbiter::InternalImpulse> {
        self.arbiter.generate_goal_impulses(pulse_number, check_interval)
    }

    /// Согласование семантического пространства всех 11 доменов.
    ///
    /// Выполняет три задачи для каждого домена:
    /// 1. **Spatial grid rebuild** — перестраивает пространственный индекс если
    ///    `events_since_rebuild >= rebuild_frequency` (DomainConfig).
    /// 2. **Prune orphaned connections** — удаляет связи, чьи `source_id`/`target_id`
    ///    ссылаются на несуществующие токены (семантический инвариант).
    /// 3. **Fix domain_id** — исправляет `token.domain_id` если токен оказался
    ///    в домене с другим ID (инвариант принадлежности).
    ///
    /// Возвращает число удалённых осиротевших связей по всем доменам.
    pub fn reconcile_all(&mut self) -> usize {
        let mut pruned_total = 0;

        for i in 0..11 {
            let domain_id = self.domains[i].config.domain_id;

            // 1. Spatial grid rebuild — клонируем токены чтобы избежать двойного заимствования
            if self.domains[i].should_rebuild_spatial_grid() {
                let tokens = self.states[i].tokens.clone();
                self.domains[i].rebuild_spatial_grid(&tokens);
            }

            // 2. Удалить осиротевшие связи
            let live: std::collections::HashSet<u32> = self.states[i].tokens
                .iter()
                .map(|t| t.sutra_id)
                .collect();
            let before = self.states[i].connections.len();
            self.states[i].connections.retain(|c| {
                live.contains(&c.source_id) && live.contains(&c.target_id)
            });
            pruned_total += before - self.states[i].connections.len();

            // 3. Исправить domain_id токенов
            for token in &mut self.states[i].tokens {
                if token.domain_id != domain_id {
                    token.domain_id = domain_id;
                }
            }
        }

        pruned_total
    }
}
