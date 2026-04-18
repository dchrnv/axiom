// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// WebSocket адаптер (Phase 1).
// Клиенты подключаются к /ws, обмениваются JSON-сообщениями.
// Все команды проходят через общий tick_loop (command_tx).
// Ответы доставляются через broadcast + фильтрацию по подпискам.

mod handler;
mod server;

pub use server::{AppState, bind, serve_ws};
