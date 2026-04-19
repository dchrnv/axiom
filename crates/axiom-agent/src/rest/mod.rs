// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// REST адаптер (Phase 2).
// Маршруты монтируются на тот же axum router что и WebSocket.
//
// GET  /api/status          — snapshot (не блокирует Engine)
// GET  /api/domains         — список доменов из snapshot
// GET  /api/domain/:id      — детали домена (correlation id через broadcast)
// POST /api/inject          — инъекция текста, ждёт ServerMessage::Result
// POST /api/command         — мета-команда (:status, :save и т.д.), ждёт CommandResult

mod handlers;

pub use handlers::rest_routes;
