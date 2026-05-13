// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// shutdown — обработка SIGINT/SIGTERM для graceful shutdown.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::signal;
use tracing::info;

#[derive(Clone)]
pub struct ShutdownSignal {
    triggered: Arc<AtomicBool>,
}

impl ShutdownSignal {
    pub fn new() -> Self {
        Self {
            triggered: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_triggered(&self) -> bool {
        self.triggered.load(Ordering::Relaxed)
    }

    pub fn trigger(&self) {
        self.triggered.store(true, Ordering::Relaxed);
    }

    /// Spawn a task that listens for SIGINT/SIGTERM and sets the flag.
    pub fn spawn_listener(&self) {
        let flag = self.triggered.clone();
        tokio::spawn(async move {
            wait_for_signal().await;
            info!("OS signal received — initiating graceful shutdown");
            flag.store(true, Ordering::Relaxed);
        });
    }
}

async fn wait_for_signal() {
    #[cfg(unix)]
    {
        use signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("SIGINT handler");
        let mut sigterm = signal(SignalKind::terminate()).expect("SIGTERM handler");
        tokio::select! {
            _ = sigint.recv() => {}
            _ = sigterm.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        signal::ctrl_c().await.expect("ctrl-c handler");
    }
}
