// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-agent — главный бинарник внешней интеграции AXIOM
//
// Запуск: `./axiom-agent [--config <path>]`
// По умолчанию: config/axiom.yaml + config/channels.yaml

use std::path::Path;
use axiom_runtime::{Gateway, Perceptor, Effector};
use axiom_agent::channels::cli::{CliPerceptor, CliEffector};
use axiom_agent::channels::shell::ShellEffector;
use axiom_agent::channels::telegram::{TelegramPerceptor, TelegramConfig};
use axiom_agent::config::AgentConfig;

fn main() {
    eprintln!("AXIOM Agent v{}", env!("CARGO_PKG_VERSION"));

    // ── Конфигурация ──────────────────────────────────────────────────────────
    let channels_path = Path::new("config/channels.yaml");
    let agent_cfg = if channels_path.exists() {
        AgentConfig::from_file(channels_path)
            .unwrap_or_else(|e| {
                eprintln!("Warning: failed to load channels.yaml: {e}");
                AgentConfig::default()
            })
    } else {
        AgentConfig::default()
    };

    // ── Gateway + Engine ──────────────────────────────────────────────────────
    let mut gateway = Gateway::with_default_engine();

    // ── Перцепторы ────────────────────────────────────────────────────────────
    let mut perceptors: Vec<Box<dyn Perceptor>> = Vec::new();

    if agent_cfg.channels.cli {
        eprintln!("[agent] CLI channel active (stdin)");
        perceptors.push(Box::new(CliPerceptor::new()));
    }

    if agent_cfg.channels.telegram.enabled {
        let tg_cfg = TelegramConfig {
            token: agent_cfg.channels.telegram.token.clone(),
            chat_id: agent_cfg.channels.telegram.chat_id,
        };
        eprintln!("[agent] Telegram channel active");
        perceptors.push(Box::new(TelegramPerceptor::new(tg_cfg)));
    }

    // ── Эффекторы ─────────────────────────────────────────────────────────────
    let mut effectors: Vec<Box<dyn Effector>> = Vec::new();

    if agent_cfg.channels.cli {
        effectors.push(Box::new(CliEffector::new()));
    }

    if agent_cfg.channels.shell.enabled {
        let whitelist_path = Path::new(&agent_cfg.channels.shell.whitelist);
        match ShellEffector::from_whitelist_file(whitelist_path) {
            Ok(shell) => {
                eprintln!("[agent] Shell effector active");
                effectors.push(Box::new(shell));
            }
            Err(e) => eprintln!("Warning: shell effector disabled: {e}"),
        }
    }

    if perceptors.is_empty() {
        eprintln!("[agent] No perceptors configured — exiting.");
        return;
    }

    eprintln!("[agent] Ready. Commands: tick | inject <domain_id> | status | quit");

    // ── Главный цикл ──────────────────────────────────────────────────────────
    'main_loop: loop {
        // Опрашиваем все перцепторы
        for perceptor in &mut perceptors {
            if let Some(cmd) = perceptor.receive() {
                let result = gateway.process(&cmd);

                // Уведомляем эффекторы о результате
                for effector in &mut effectors {
                    effector.emit_result(&result);
                }

                // Дренируем события и отправляем эффекторам
                let events = gateway.engine_mut().drain_events();
                for event in &events {
                    for effector in &mut effectors {
                        effector.emit(event);
                    }
                }
            } else if perceptor.name() == "cli" {
                // CLI вернул None — quit или EOF
                break 'main_loop;
            }
        }

        // Горячая перезагрузка конфигурации (если настроена)
        if let Some(_new_cfg) = gateway.check_config_reload() {
            eprintln!("[agent] Config reloaded.");
        }
    }

    eprintln!("[agent] Shutdown.");
}
