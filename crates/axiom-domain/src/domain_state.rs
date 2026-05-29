// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Domain V1.3: DomainState — предвыделённые буферы токенов и связей

use axiom_config::DomainConfig;
use axiom_core::{Connection, Token, STATE_LOCKED};

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

    /// Максимальное число токенов, допустимое в домене.
    pub fn token_capacity(&self) -> usize {
        self.token_capacity
    }

    /// Количество связей в буфере.
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// True если токен защищён от eviction/decay.
    /// Защищены: токены с protected_flags ИЛИ с state==STATE_LOCKED (якоря — temp=0 изначально).
    fn is_protected(token: &Token, protected_flags: u16) -> bool {
        token.type_flags & protected_flags != 0 || token.state == STATE_LOCKED
    }

    /// Уменьшить temperature всех незащищённых токенов на `rate`.
    /// STATE_LOCKED токены (anchor-токены, temperature=0 изначально) не затрагиваются.
    /// Возвращает количество токенов достигших temperature == 0.
    pub fn decay_temperatures(&mut self, rate: u8, protected_flags: u16) -> usize {
        let mut dead = 0;
        for token in &mut self.tokens {
            if Self::is_protected(token, protected_flags) {
                continue;
            }
            token.temperature = token.temperature.saturating_sub(rate);
            if token.temperature == 0 {
                dead += 1;
            }
        }
        dead
    }

    /// Удалить мёртвые токены (temperature == 0) которые не защищены.
    /// STATE_LOCKED токены никогда не удаляются (они являются якорями).
    /// Возвращает удалённые токены (для eviction hook).
    pub fn evict_dead(&mut self, protected_flags: u16) -> Vec<Token> {
        let mut evicted = Vec::new();
        self.tokens.retain(|t| {
            if Self::is_protected(t, protected_flags) {
                return true;
            }
            if t.temperature == 0 {
                evicted.push(*t);
                false
            } else {
                true
            }
        });
        evicted
    }

    /// Evict excess tokens down to `max_keep`, removing coldest (lowest temperature)
    /// unprotected tokens first. Protected (flags or STATE_LOCKED) tokens are never evicted.
    /// Returns the evicted tokens (for eviction hook processing by caller).
    pub fn evict_excess(&mut self, max_keep: usize, protected_flags: u16) -> Vec<Token> {
        if self.tokens.len() <= max_keep {
            return vec![];
        }
        let (mut keep, mut evictable): (Vec<_>, Vec<_>) = std::mem::take(&mut self.tokens)
            .into_iter()
            .partition(|t| Self::is_protected(t, protected_flags));

        let slots = max_keep.saturating_sub(keep.len());
        evictable.sort_unstable_by(|a, b| b.temperature.cmp(&a.temperature));
        let evicted = if evictable.len() > slots {
            evictable.split_off(slots)
        } else {
            vec![]
        };
        keep.extend(evictable);
        self.tokens = keep;
        evicted
    }

    /// True если sutra_id упоминается как source или target в любой связи домена.
    /// Используется eviction hook для определения «структурно важных» токенов.
    pub fn is_connection_referenced(&self, sutra_id: u32) -> bool {
        self.connections
            .iter()
            .any(|c| c.source_id == sutra_id || c.target_id == sutra_id)
    }
}
