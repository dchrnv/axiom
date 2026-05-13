// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// tick — чистый tick loop axiom-node.
// Без CLI-состояния. Единственный writer AxiomEngine.

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use tokio::time::Duration;
use tracing::{debug, info};

use axiom_agent::perceptors::text::TextPerceptor;
use axiom_broadcasting::{build_system_snapshot, BroadcastHandle};
use axiom_config::AnchorSet;
use axiom_persist::AutoSaver;
use axiom_protocol::{events::EngineEvent, messages::EngineMessage};
use axiom_runtime::{AxiomEngine, TickRateReason};
use axiom_ucl::{OpCode, UclCommand};

use crate::commands::handle_engine_command;
use crate::config::NodeConfig;
use crate::shutdown::ShutdownSignal;

pub async fn run(
    mut engine: AxiomEngine,
    mut auto_saver: AutoSaver,
    anchor_set: Option<Arc<AnchorSet>>,
    handle: Arc<BroadcastHandle>,
    cfg: &NodeConfig,
    shutdown: ShutdownSignal,
) {
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let base_tick_ms = 1000u64 / cfg.tick_hz.max(1) as u64;

    let mut perceptor = match &anchor_set {
        Some(arc) => TextPerceptor::with_anchors(arc.clone()),
        None => TextPerceptor::new(),
    };

    let mut last_tick_ns: u64 = 0;

    info!(
        "tick loop started — {tick_hz} Hz, addr {addr}",
        tick_hz = cfg.tick_hz,
        addr = cfg.addr,
    );

    loop {
        // Выход по сигналу
        if shutdown.is_triggered() {
            info!("shutdown signal received — saving state");
            let _ = auto_saver.force_save(&engine, Path::new(&cfg.data_dir));
            break;
        }

        // Sleep
        let sleep_ms = if cfg.adaptive_tick() {
            engine.tick_schedule.adaptive_tick.interval_ms()
        } else {
            base_tick_ms
        };
        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

        // 1. Drain EngineCommand от Workstation
        let mut had_commands = false;
        loop {
            match handle.try_recv_command().await {
                Some((cmd_id, cmd)) => {
                    had_commands = true;
                    handle_engine_command(
                        cmd_id, cmd, &mut engine, &handle,
                        &mut perceptor, &anchor_set, last_tick_ns,
                        &shutdown,
                    );
                }
                None => break,
            }
        }

        // 2. Speculative grids (S6) — параллельная предсборка до reconcile
        {
            let pool = engine.thread_pool.clone();
            engine.ashti.prepare_speculative_grids(&pool);
        }

        // 3. Tick ядра
        let t0 = Instant::now();
        engine.process_command(&tick_cmd);
        last_tick_ns = t0.elapsed().as_nanos() as u64;
        let tick = engine.tick_count;

        debug!(
            "tick={tick} ns={last_tick_ns} spec_hits={} spec_misses={}",
            engine.ashti.speculative_stats().0,
            engine.ashti.speculative_stats().1,
        );

        // Адаптивный tick rate
        if cfg.adaptive_tick() {
            if had_commands || engine.tension_count() > 0 {
                let reason = if had_commands {
                    TickRateReason::ExternalInput
                } else {
                    TickRateReason::TensionHigh
                };
                engine.tick_schedule.adaptive_tick.trigger(reason);
            } else {
                engine.tick_schedule.adaptive_tick.on_idle_tick();
            }
        }

        // 3. Tick-событие → Workstation
        if cfg.tick_interval > 0 && tick % cfg.tick_interval as u64 == 0 {
            handle.publish(EngineMessage::Event(EngineEvent::Tick {
                tick,
                event: engine.com_next_id,
                hot_path_ns: last_tick_ns,
            }));
        }

        // 4. Snapshot → Workstation
        if cfg.snapshot_interval > 0 && tick % cfg.snapshot_interval as u64 == 0 {
            let snap = build_system_snapshot(&engine, last_tick_ns);
            handle.update_snapshot(snap.clone());
            handle.publish(EngineMessage::Snapshot(snap));
        }

        // 5. Автосохранение
        auto_saver.tick(&engine, Path::new(&cfg.data_dir));
    }

    info!("tick loop stopped at tick {}", engine.tick_count);
}
