// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-node — самостоятельный когнитивный узел Axiom.
// Запуск: axiom-node [--addr 127.0.0.1:9876] [--data-dir data] ...

mod commands;
mod config;
mod http;
mod shutdown;
mod startup;
mod tick;

use anyhow::{Context, Result};
use clap::Parser;
use tracing::info;

use axiom_broadcasting::{BroadcastingConfig, BroadcastServer};

use crate::config::NodeConfig;
use crate::shutdown::ShutdownSignal;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = NodeConfig::parse();

    init_tracing(&cfg.log_level);

    info!("axiom-node starting up");

    // 1. Инициализация движка, хранилища, якорей
    let state = startup::init(&cfg)?;

    // 2. BroadcastServer — WebSocket для Workstation (binary/postcard)
    let addr = cfg.addr.parse().with_context(|| format!("invalid addr: {}", cfg.addr))?;
    let (server, handle) = BroadcastServer::new(addr, BroadcastingConfig::default());
    info!("WebSocket server listening on {}", cfg.addr);

    tokio::spawn(async move {
        if let Err(e) = server.run().await {
            tracing::error!("BroadcastServer error: {}", e);
        }
    });

    // 3. HTTP server — React SPA + JSON WebSocket bridge + REST API
    let (cmd_tx, cmd_rx) = http::create_cmd_channel();
    let http_addr = cfg.http_addr.parse().with_context(|| format!("invalid http_addr: {}", cfg.http_addr))?;
    let http_handle = handle.clone();
    let web_dist = cfg.web_dist.clone();
    tokio::spawn(async move {
        http::run(http_addr, http_handle, web_dist, cmd_tx).await;
    });

    // 4. Graceful shutdown
    let shutdown = ShutdownSignal::new();
    shutdown.spawn_listener();

    // 5. Tick loop (blocks until shutdown)
    tick::run(
        state.engine,
        state.auto_saver,
        state.anchor_set,
        handle,
        &cfg,
        shutdown,
        cmd_rx,
    )
    .await;

    info!("axiom-node stopped");
    Ok(())
}

fn init_tracing(level: &str) {
    use tracing_subscriber::{fmt, EnvFilter};
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(level)),
        )
        .init();
}
