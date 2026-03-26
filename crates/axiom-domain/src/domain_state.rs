// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Domain V1.3: DomainState — предвыделённые буферы токенов и связей

use axiom_core::{Token, Connection};
use axiom_config::DomainConfig;

/// Ошибка превышения ёмкости домена.
#[derive(Debug, PartialEq)]
pub struct CapacityExceeded;

/// Рантаймовое состояние домена: предвыделённые буферы токенов и связей.
///
/// Буфер `neighbor_buffer` используется для zero-alloc поиска соседей
/// (передаётся в `Domain::find_neighbors`).
pub struct DomainState {
    pub tokens: Vec<Token>,
    pub connections: Vec<Connection>,
    /// Scratch-буфер для find_neighbors — pre-allocated, zero-alloc в hot path.
    pub neighbor_buffer: Vec<u32>,
    token_capacity: usize,
    connection_capacity: usize,
}

impl DomainState {
    /// Создать DomainState с ёмкостями из конфигурации.
    pub fn new(config: &DomainConfig) -> Self {
        let token_cap = config.token_capacity as usize;
        let conn_cap = config.connection_capacity as usize;
        Self {
            tokens: Vec::with_capacity(token_cap),
            connections: Vec::with_capacity(conn_cap),
            neighbor_buffer: Vec::with_capacity(64),
            token_capacity: token_cap,
            connection_capacity: conn_cap,
        }
    }

    /// Добавить токен. Возвращает его индекс или `CapacityExceeded`.
    pub fn add_token(&mut self, token: Token) -> Result<usize, CapacityExceeded> {
        if self.tokens.len() >= self.token_capacity {
            return Err(CapacityExceeded);
        }
        let idx = self.tokens.len();
        self.tokens.push(token);
        Ok(idx)
    }

    /// Добавить связь. Возвращает её индекс или `CapacityExceeded`.
    pub fn add_connection(&mut self, conn: Connection) -> Result<usize, CapacityExceeded> {
        if self.connections.len() >= self.connection_capacity {
            return Err(CapacityExceeded);
        }
        let idx = self.connections.len();
        self.connections.push(conn);
        Ok(idx)
    }

    /// Количество токенов в буфере.
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    /// Количество связей в буфере.
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}
