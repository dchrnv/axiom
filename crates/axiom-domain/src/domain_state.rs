// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Domain V1.4: DomainState — предвыделённые буферы токенов и связей

use axiom_config::DomainConfig;
use axiom_core::{Connection, Token, STATE_LOCKED, STATE_SLEEPING};

/// Ошибка превышения ёмкости домена.
#[derive(Debug, PartialEq)]
pub struct CapacityExceeded;

/// Рантаймовое состояние домена: предвыделённые буферы токенов и связей.
pub struct DomainState {
    pub tokens: Vec<Token>,
    pub connections: Vec<Connection>,
    /// Scratch-буфер для find_neighbors — pre-allocated, zero-alloc в hot path.
    pub neighbor_buffer: Vec<u32>,
    token_capacity: usize,
    connection_capacity: usize,
}

impl DomainState {
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

    /// Добавить токен. Если домен заполнен — сначала вытесняет спящие (STATE_SLEEPING) токены.
    /// Возвращает индекс или `CapacityExceeded` если нет ни места, ни спящих.
    pub fn add_token(&mut self, token: Token) -> Result<usize, CapacityExceeded> {
        if self.tokens.len() >= self.token_capacity {
            // Освобождаем место вытесняя спящие токены (они уже «умерли» функционально)
            self.evict_sleeping(1);
        }
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

    pub fn token_count(&self) -> usize { self.tokens.len() }
    pub fn token_capacity(&self) -> usize { self.token_capacity }
    pub fn connection_count(&self) -> usize { self.connections.len() }

    /// Перевести токен в STATE_SLEEPING и обнулить valence.
    /// Токен остаётся физически — просто становится инертным.
    /// STATE_LOCKED токены (якоря) не затрагиваются.
    pub fn mark_sleeping(&mut self, sutra_id: u32) -> bool {
        for token in &mut self.tokens {
            if token.sutra_id == sutra_id && token.state != STATE_LOCKED {
                token.state = STATE_SLEEPING;
                token.valence = 0;
                return true;
            }
        }
        false
    }

    /// Вытеснить до `n` спящих (STATE_SLEEPING) токенов для освобождения слотов.
    /// Это единственный легальный способ удалить токены — только «мёртвые».
    pub fn evict_sleeping(&mut self, n: usize) -> usize {
        let mut removed = 0;
        self.tokens.retain(|t| {
            if removed < n && t.state == STATE_SLEEPING {
                removed += 1;
                false
            } else {
                true
            }
        });
        removed
    }

    /// True если sutra_id упоминается в любой связи домена.
    pub fn is_connection_referenced(&self, sutra_id: u32) -> bool {
        self.connections
            .iter()
            .any(|c| c.source_id == sutra_id || c.target_id == sutra_id)
    }
}
