//! AXIOM Core — фундаментальные типы
//!
//! Базовые структуры данных: Token, Connection, Event.
//! Не имеет внешних зависимостей (zero dependencies).
//!
//! # Архитектура
//!
//! - `token` — Token структура (64 байта, repr(C, align(64)))
//! - `connection` — Connection структура (64 байта, repr(C, align(64)))
//! - `event` — Event структура и типы событий (64 байта, repr(C, align(64)))
//!
//! Все структуры используют:
//! - Фиксированный размер 64 байта для cache-line оптимизации
//! - repr(C) для FFI совместимости
//! - align(64) для выравнивания на кеш-линию
//! - COM (Causal Order Model) вместо wall-clock времени
//!
//! # Инварианты
//!
//! Каждая структура имеет метод `validate()` для проверки инвариантов.
//! Все ID-поля должны быть > 0 (0 зарезервирован для "отсутствует").

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod token;
pub mod connection;
pub mod event;

// Реэкспорт основных типов
pub use token::{Token, TOKEN_FLAG_GOAL, TOKEN_FLAG_IMPULSE, TOKEN_FLAG_FRAME_ANCHOR, TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE, TOKEN_FLAG_DREAM_REPORT, FRAME_CATEGORY_MASK, FRAME_CATEGORY_SYNTAX, STATE_ACTIVE, STATE_SLEEPING, STATE_LOCKED};
pub use connection::{Connection, FLAG_ACTIVE, FLAG_INHIBITED, FLAG_TEMPORARY, FLAG_CRITICAL};
pub use event::{
    Event, EventType, EventPriority, Snapshot,
    EVENT_REVERSIBLE, EVENT_CRITICAL, EVENT_BATCHED,
};
