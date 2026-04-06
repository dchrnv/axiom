// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-cli — первый живой интерфейс к ядру AXIOM.
//
// Boot sequence:
//   1. Разобрать аргументы (--data-dir, --no-load, --tick-hz, ...)
//   2. Найти axiom-data/manifest.yaml → загрузить или чистый старт
//   3. Запустить async tick loop (CliChannel)
//
// Ядро тикает в фоне. Ввод текста — инъекция токена в SUTRA(100).

use axiom_agent::channels::cli::{CliChannel, CliConfig};
use axiom_persist::{load as persist_load, PersistError};
use axiom_runtime::AxiomEngine;
use std::path::{Path, PathBuf};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = CliConfig::from_args_or_default();
    let boot   = BootArgs::from_env();

    let (engine, boot_status) = boot_engine(&boot);

    println!("AXIOM — Cognitive Architecture");
    println!("───────────────────────────────");
    println!("tick_hz: {} Hz  |  domains: 11  |  :help for commands", config.tick_hz);
    println!("{}", boot_status);
    println!();

    let mut cli = CliChannel::new(engine, config);
    cli.run().await;
}

// ─── Boot arguments ───────────────────────────────────────────────────────────

struct BootArgs {
    /// Путь к директории хранилища (--data-dir, default: ./axiom-data)
    data_dir: PathBuf,
    /// Принудительный чистый старт (--no-load)
    no_load: bool,
}

impl BootArgs {
    fn from_env() -> Self {
        let args: Vec<String> = std::env::args().collect();

        let data_dir = args.windows(2)
            .find(|w| w[0] == "--data-dir")
            .map(|w| PathBuf::from(&w[1]))
            .unwrap_or_else(|| PathBuf::from("axiom-data"));

        let no_load = args.iter().any(|a| a == "--no-load");

        Self { data_dir, no_load }
    }
}

// ─── Boot sequence ────────────────────────────────────────────────────────────

/// Возвращает (engine, однострочный статус для banner).
fn boot_engine(boot: &BootArgs) -> (AxiomEngine, String) {
    if boot.no_load {
        return (
            AxiomEngine::new(),
            "  mode: fresh start (--no-load)".to_string(),
        );
    }

    match try_load(&boot.data_dir) {
        Ok((engine, tick, traces, tension)) => {
            let status = format!(
                "  mode: restored from {} (tick={}, traces={}, tension={})",
                boot.data_dir.display(), tick, traces, tension
            );
            (engine, status)
        }
        Err(LoadOutcome::NotFound) => {
            (
                AxiomEngine::new(),
                format!("  mode: fresh start (no data at {})", boot.data_dir.display()),
            )
        }
        Err(LoadOutcome::Failed(msg)) => {
            eprintln!("[axiom-cli] WARNING: load failed — {msg}");
            eprintln!("[axiom-cli] Starting fresh.");
            (
                AxiomEngine::new(),
                format!("  mode: fresh start (load error — see stderr)"),
            )
        }
    }
}

enum LoadOutcome {
    NotFound,
    Failed(String),
}

/// Попробовать загрузить из директории.
/// Возвращает Ok((engine, tick_count, traces_imported, tension_imported)).
fn try_load(dir: &Path) -> Result<(AxiomEngine, u64, u32, u32), LoadOutcome> {
    if !dir.join("manifest.yaml").exists() {
        return Err(LoadOutcome::NotFound);
    }

    match persist_load(dir) {
        Ok(r) => Ok((r.engine, r.manifest.tick_count, r.traces_imported, r.tension_imported)),
        Err(PersistError::NotFound(_)) => Err(LoadOutcome::NotFound),
        Err(e) => Err(LoadOutcome::Failed(e.to_string())),
    }
}
