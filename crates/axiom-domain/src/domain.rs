// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Domain V1.3: Domain runtime struct
//
// Causal Frontier V1, раздел 12: Domain isolation
// Heartbeat V2.0, раздел 9: каждый домен имеет свой HeartbeatGenerator
// SPACE V6.0, раздел 8: каждый домен имеет свой SpatialHashGrid

use axiom_core::{Token, Connection, Event};
use axiom_config::DomainConfig;
use axiom_frontier::{CausalFrontier, FrontierConfig, FrontierEntity};
use axiom_heartbeat::{HeartbeatConfig, HeartbeatGenerator};
use axiom_space::SpatialHashGrid;

use crate::physics::{EventGenerator, DEFAULT_DECAY_RATE, DEFAULT_STRESS_THRESHOLD, DEFAULT_COLLISION_RADIUS};

/// Domain — runtime-структура управляющая состоянием и причинным фронтиром.
///
/// Использует `axiom_config::DomainConfig` напрямую — без дублирования и без unsafe.
pub struct Domain {
    /// Конфигурация домена (из axiom-config, без дублирования)
    pub config: DomainConfig,

    /// Причинный фронтир для управления активными вычислениями
    pub frontier: CausalFrontier,

    /// Heartbeat generator для периодической активации
    pub heartbeat: HeartbeatGenerator,

    /// Конфигурация heartbeat
    pub heartbeat_config: HeartbeatConfig,

    /// Spatial Hash Grid для пространственной индексации токенов
    /// SPACE V6.0: быстрый поиск соседей через хеш-сетку
    pub spatial_grid: SpatialHashGrid,

    /// Текущее количество активных токенов
    pub active_tokens: usize,

    /// Текущее количество активных связей
    pub active_connections: usize,

    /// Счётчик событий с последней перестройки spatial grid
    pub events_since_rebuild: usize,

    /// Shell Cache для семантических профилей токенов
    pub shell_cache: axiom_shell::DomainShellCache,

    /// Semantic Contribution Table для Shell вычислений
    pub semantic_table: axiom_shell::SemanticContributionTable,
}

impl Domain {
    /// Создать домен из конфигурации с heartbeat по умолчанию.
    pub fn new(config: DomainConfig) -> Self {
        Self::with_heartbeat(config, HeartbeatConfig::default())
    }

    /// Создать домен из конфигурации с кастомным heartbeat.
    pub fn with_heartbeat(config: DomainConfig, heartbeat_config: HeartbeatConfig) -> Self {
        let shell_capacity = config.token_capacity as usize;
        let shell_cache = axiom_shell::DomainShellCache::new(shell_capacity);
        let semantic_table = axiom_shell::SemanticContributionTable::default_ashti_core();

        let frontier_config = FrontierConfig {
            max_frontier_size: (config.token_capacity / 5).max(100),
            max_events_per_cycle: 1000,
            storm_threshold: (config.token_capacity / 10).max(50),
            enable_batch_events: true,
            token_capacity: config.token_capacity,
            connection_capacity: config.connection_capacity,
        };

        Self {
            heartbeat: HeartbeatGenerator::new(config.domain_id, heartbeat_config.interval),
            heartbeat_config,
            config,
            frontier: CausalFrontier::new(frontier_config),
            spatial_grid: SpatialHashGrid::new(),
            active_tokens: 0,
            active_connections: 0,
            events_since_rebuild: 0,
            shell_cache,
            semantic_table,
        }
    }

    /// Обновить состояние frontier на основе текущих метрик.
    pub fn update_frontier_state(&mut self) {
        self.frontier.update_state();
    }

    /// Проверить, достигнут ли лимит ёмкости.
    pub fn is_at_capacity(&self) -> bool {
        self.active_tokens >= self.config.token_capacity as usize
            || self.active_connections >= self.config.connection_capacity as usize
    }

    /// Получить текущее использование памяти frontier (%).
    pub fn frontier_memory_usage(&self) -> f32 {
        self.frontier.memory_usage()
    }

    /// Обработать событие и проверить нужен ли Heartbeat.
    ///
    /// Возвращает pulse_number если пора генерировать Heartbeat.
    pub fn on_event(&mut self) -> Option<u64> {
        self.heartbeat.on_event()
    }

    /// Обработать Heartbeat — добавить сущности в frontier.
    pub fn handle_heartbeat(&mut self, pulse_number: u64) {
        axiom_heartbeat::handle_heartbeat(
            &mut self.frontier,
            pulse_number,
            &self.heartbeat_config,
            self.active_tokens,
            self.active_connections,
        );
    }

    /// Получить текущий номер пульса heartbeat.
    pub fn current_pulse(&self) -> u64 {
        self.heartbeat.current_pulse()
    }

    /// Обработать frontier — извлечь сущности и сгенерировать события.
    ///
    /// Causal Frontier V1, раздел 7: Processing frontier entities
    /// Event-Driven V1, раздел 6: Event generation from state changes
    /// Heartbeat V2.0, раздел 6: Background processes через Frontier
    pub fn process_frontier(
        &mut self,
        tokens: &[Token],
        connections: &[Connection],
        event_generator: &mut EventGenerator,
    ) -> Vec<Event> {
        let mut generated_events = Vec::new();

        self.frontier.begin_cycle();
        while let Some(entity) = self.frontier.pop() {
            match entity {
                FrontierEntity::Token(token_idx) => {
                    if let Some(token) = tokens.get(token_idx as usize) {
                        // Затухание
                        if self.heartbeat_config.enable_decay {
                            if let Some(event) = event_generator.check_decay(token, DEFAULT_DECAY_RATE) {
                                generated_events.push(event);
                            }
                        }

                        // Гравитация
                        if self.heartbeat_config.enable_gravity
                            && self.config.gravity_strength.abs() > 0.01
                        {
                            let event = event_generator.generate_gravity_update(token);
                            generated_events.push(event);
                        }

                        // SPACE V6.0: столкновения через spatial hash
                        if self.heartbeat_config.enable_spatial_collision
                            && self.spatial_grid.entry_count > 0
                        {
                            let collisions = axiom_space::detect_collisions(
                                token_idx,
                                (token.position[0], token.position[1], token.position[2]),
                                DEFAULT_COLLISION_RADIUS,
                                |idx| {
                                    tokens
                                        .get(idx as usize)
                                        .map(|t| (t.position[0], t.position[1], t.position[2]))
                                        .unwrap_or((0, 0, 0))
                                },
                                &self.spatial_grid,
                            );

                            for collision_idx in collisions {
                                if let Some(other_token) = tokens.get(collision_idx as usize) {
                                    let event = event_generator.generate_collision(token, other_token);
                                    generated_events.push(event);

                                    self.frontier.push_token(collision_idx);

                                    // Shell V3.0: помечаем связанные токены как dirty
                                    for conn in connections {
                                        let involves = (conn.source_id == token.sutra_id
                                            && conn.target_id == other_token.sutra_id)
                                            || (conn.source_id == other_token.sutra_id
                                                && conn.target_id == token.sutra_id);
                                        if involves {
                                            axiom_shell::process_connection_event(
                                                &mut self.shell_cache,
                                                conn,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                FrontierEntity::Connection(conn_idx) => {
                    if self.heartbeat_config.enable_connection_maintenance {
                        if let Some(connection) = connections.get(conn_idx as usize) {
                            if let Some(event) = event_generator
                                .check_connection_stress(connection, DEFAULT_STRESS_THRESHOLD)
                            {
                                generated_events.push(event);
                            }
                            axiom_shell::process_connection_event(&mut self.shell_cache, connection);
                        }
                    }
                }
            }
        }
        self.frontier.end_cycle();

        self.update_frontier_state();
        generated_events
    }

    /// Перестроить spatial hash grid.
    ///
    /// SPACE V6.0, раздел 4.5: перестройка после применения событий.
    pub fn rebuild_spatial_grid(&mut self, tokens: &[Token]) {
        self.spatial_grid.rebuild(self.active_tokens, |token_index| {
            tokens
                .get(token_index)
                .map(|t| (t.position[0], t.position[1], t.position[2]))
                .unwrap_or((0, 0, 0))
        });
        self.events_since_rebuild = 0;
    }

    /// Нужна ли перестройка spatial grid?
    ///
    /// Возвращает true, если:
    /// - rebuild_frequency > 0 (перестройка включена)
    /// - events_since_rebuild >= rebuild_frequency
    pub fn should_rebuild_spatial_grid(&self) -> bool {
        self.config.rebuild_frequency > 0
            && self.events_since_rebuild >= self.config.rebuild_frequency as usize
    }

    /// Инкрементировать счётчик событий с последней перестройки.
    pub fn increment_events_since_rebuild(&mut self) {
        self.events_since_rebuild = self.events_since_rebuild.saturating_add(1);
    }

    /// Найти соседей токена через spatial grid.
    ///
    /// SPACE V6.0, раздел 4.6: поиск соседей.
    pub fn find_neighbors(&self, token: &Token, radius: i16, tokens: &[Token]) -> Vec<u32> {
        self.spatial_grid.find_neighbors(
            token.position[0],
            token.position[1],
            token.position[2],
            radius,
            |token_index| {
                tokens
                    .get(token_index as usize)
                    .map(|t| (t.position[0], t.position[1], t.position[2]))
                    .unwrap_or((0, 0, 0))
            },
        )
    }
}
