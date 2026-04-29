// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Standalone-функции для мета-команд CLI (Phase 0B).
//
// handle_meta_read  — read-only запросы к Engine (возвращает String).
// handle_meta_mutate — команды с мутацией Engine/AutoSaver (возвращает MetaMutateResult).
//
// CliChannel::handle_meta_command — тонкая обёртка над этими двумя функциями.

use std::collections::HashSet;
use std::fmt::Write;
use std::collections::VecDeque;

use axiom_core::Event;
use axiom_config::AnchorSet;
use axiom_persist::{
    save as persist_save, load as persist_load, WriteOptions, AutoSaver,
    export_traces, export_skills, import_traces, import_skills,
};
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

use crate::channels::cli::{CliConfig, CliConfigFile, PerfTracker, fmt_ns};
use axiom_runtime::domain_name;

/// Результат мутирующей мета-команды.
/// Результат мутирующей мета-команды.
pub struct MetaMutateResult {
    /// Строка для вывода
    pub output: String,
    /// Побочный эффект требующий реакции tick loop
    pub action: MetaAction,
}

/// Побочный эффект мутирующей команды — нужен tick loop чтобы реагировать.
pub enum MetaAction {
    /// Нет побочного эффекта
    None,
    /// :quit — завершить
    Quit,
    /// :load — Engine заменён целиком
    EngineReplaced,
    /// :autosave on N — изменился интервал
    AutosaveChanged(u32),
}

// ── handle_meta_read ──────────────────────────────────────────────────────────

/// Команды только для чтения — не мутируют Engine.
///
/// Принимает `&AxiomEngine`. Возвращает строку готовую к печати или отправке
/// через любой транспорт. Вызывается из `CliChannel::handle_meta_command`
/// и в будущем из tick_loop (Phase 0C).
pub fn handle_meta_read(
    line:             &str,
    engine:           &AxiomEngine,
    anchor_set:       Option<&AnchorSet>,
    config:           &CliConfig,
    watch_fields:     &HashSet<String>,
    event_log:        &VecDeque<Event>,
    perf:             &PerfTracker,
    multipass_count:  u64,
    last_multipass_n: u8,
) -> String {
    let mut out = String::new();
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    match parts[0] {

        ":help" => {
            match parts.get(1).copied() {
                None => out.push_str(HELP_TEXT),
                Some(cmd) => {
                    let topic = if cmd.starts_with(':') { cmd } else { line };
                    match topic {
                        ":trace"       => out.push_str(HELP_TRACE),
                        ":connections" => out.push_str(HELP_CONNECTIONS),
                        ":dream"       => out.push_str(HELP_DREAM),
                        ":multipass"   => out.push_str(HELP_MULTIPASS),
                        ":reflector"   => out.push_str(HELP_REFLECTOR),
                        ":impulses"    => out.push_str(HELP_IMPULSES),
                        ":traces"   => writeln!(out, "  :traces — experience traces top-20 по weight. Колонки: #, Weight, tmp/mss/val, (x,y,z), Age, Hash.").unwrap(),
                        ":tension"  => writeln!(out, "  :tension — активные tension traces с temperature и возрастом.").unwrap(),
                        ":depth"    => writeln!(out, "  :depth — параметры Cognitive Depth: max_passes, min_coherence, internal_dominance.").unwrap(),
                        ":arbiter"  => writeln!(out, "  :arbiter — thresholds per domain + reflector stats.").unwrap(),
                        ":guardian" => writeln!(out, "  :guardian — GUARDIAN stats: reflex_allowed/vetoed, access_denied, etc.").unwrap(),
                        ":frontier" => writeln!(out, "  :frontier — Causal Frontier size + mem% по всем доменам.").unwrap(),
                        ":domain"   => writeln!(out, "  :domain <id> — полные детали домена: capacity, physics, arbiter, membrane.").unwrap(),
                        ":events"   => writeln!(out, "  :events [N] — последние N COM-событий из кольцевого буфера (max 256).").unwrap(),
                        ":perf"     => writeln!(out, "  :perf — avg/peak ns/тик, actual Hz, процент бюджета, periodic task counters.").unwrap(),
                        ":status"   => writeln!(out, "  :status — tick_count, com_next_id, uptime, Hz, memory summary, cognitive params.").unwrap(),
                        ":watch"    => writeln!(out, "  :watch <traces|tension|tps> — следить за изменениями в реальном времени.").unwrap(),
                        _           => writeln!(out, "  No help for '{}'. Type :help for full list.", cmd).unwrap(),
                    }
                }
            }
        }

        ":status" => {
            let exp    = engine.ashti.experience();
            let traces = exp.traces().len();
            let tension = exp.tension_count();
            let snap   = engine.snapshot();
            let tokens: usize = snap.domains.iter().map(|d| d.tokens.len()).sum();
            let conns:  usize = snap.domains.iter().map(|d| d.connections.len()).sum();
            let skills = engine.ashti.export_skills().len();
            let (max_passes, min_coh) = engine.maya_multipass_params();
            writeln!(out, "  ══ Engine Status ══════════════════════").unwrap();
            writeln!(out, "  tick_count:    {}", engine.tick_count).unwrap();
            writeln!(out, "  com_next_id:   {}", engine.com_next_id).unwrap();
            writeln!(out, "  uptime:        {:.1}s", perf.uptime_secs()).unwrap();
            writeln!(out, "  tick_rate:     {} Hz (actual: {:.1} Hz)", config.tick_hz, perf.actual_hz()).unwrap();
            writeln!(out, "  ── memory ─────────────────────────────").unwrap();
            writeln!(out, "  tokens:        {}", tokens).unwrap();
            writeln!(out, "  connections:   {}", conns).unwrap();
            writeln!(out, "  traces:        {}", traces).unwrap();
            writeln!(out, "  skills:        {}", skills).unwrap();
            writeln!(out, "  tension:       {}", tension).unwrap();
            writeln!(out, "  ── cognitive ──────────────────────────").unwrap();
            writeln!(out, "  max_passes:    {}", max_passes).unwrap();
            writeln!(out, "  min_coherence: {:.2}", min_coh).unwrap();
        }

        ":memory" => {
            let exp = engine.ashti.experience();
            let traces    = exp.traces().len();
            let tension   = exp.tension_count();
            let snap      = engine.snapshot();
            let tokens: usize  = snap.domains.iter().map(|d| d.tokens.len()).sum();
            let conns: usize   = snap.domains.iter().map(|d| d.connections.len()).sum();
            writeln!(out, "  tick_count:  {}", engine.tick_count).unwrap();
            writeln!(out, "  tokens:      {}", tokens).unwrap();
            writeln!(out, "  connections: {}", conns).unwrap();
            writeln!(out, "  traces:      {}", traces).unwrap();
            writeln!(out, "  tension:     {}", tension).unwrap();
        }

        ":domains" => {
            for offset in 0..=10u16 {
                let id = 100 + offset;
                let count = engine.token_count(id);
                writeln!(out, "  {} ({}) — {} tokens", id, domain_name(id), count).unwrap();
            }
        }

        ":tokens" => {
            if let Some(id_str) = parts.get(1) {
                if let Ok(id) = id_str.parse::<u16>() {
                    writeln!(out, "  domain {}: {} tokens", id, engine.token_count(id)).unwrap();
                } else {
                    writeln!(out, "  Usage: :tokens <domain_id>").unwrap();
                }
            }
        }

        ":verbose" => {
            writeln!(out, "  verbose: {}", if config.verbose { "on" } else { "off" }).unwrap();
        }

        ":snapshot" => {
            let snap = engine.snapshot();
            writeln!(out, "  snapshot: tick_count={} domains={}", snap.tick_count, snap.domains.len()).unwrap();
        }

        ":schedule" => {
            let s = engine.tick_schedule.clone();
            writeln!(out, "  adaptation:    {}", s.adaptation_interval).unwrap();
            writeln!(out, "  horizon_gc:    {}", s.horizon_gc_interval).unwrap();
            writeln!(out, "  snapshot:      {}", s.snapshot_interval).unwrap();
            writeln!(out, "  dream:         {}", s.dream_interval).unwrap();
            writeln!(out, "  tension_check: {}", s.tension_check_interval).unwrap();
            writeln!(out, "  goal_check:    {}", s.goal_check_interval).unwrap();
            writeln!(out, "  reconcile:     {}", s.reconcile_interval).unwrap();
        }

        ":traces" => {
            let exp    = engine.ashti.experience();
            let traces = exp.traces();
            let tick   = engine.tick_count;
            if traces.is_empty() {
                writeln!(out, "  no experience traces").unwrap();
            } else {
                let avg_w = traces.iter().map(|t| t.weight).sum::<f32>() / traces.len() as f32;
                let max_w = traces.iter().map(|t| t.weight).fold(0f32, f32::max);
                writeln!(out, "  ══ Experience Traces ══════════════════").unwrap();
                writeln!(out, "  Total: {}  |  Avg weight: {:.2}  |  Max weight: {:.2}", traces.len(), avg_w, max_w).unwrap();
                writeln!(out, "  {:>3}  {:>6}  {:>3}/{:>3}/{:>3}  {:>6}  {:>8}", "#", "Weight", "tmp", "mss", "val", "Age", "Hash").unwrap();
                let mut sorted: Vec<_> = traces.iter().enumerate().collect();
                sorted.sort_by(|a, b| b.1.weight.total_cmp(&a.1.weight));
                for (i, (_, t)) in sorted.iter().take(20).enumerate() {
                    let age = tick.saturating_sub(t.created_at);
                    let [x, y, z] = t.pattern.position;
                    writeln!(out, "  {:>3}  {:.4}  {:>3}/{:>3}/{:>3}  ({},{},{})  {:>6}  {:>8}  {:#010x}",
                        i + 1, t.weight,
                        t.pattern.temperature, t.pattern.mass, t.pattern.valence,
                        x, y, z,
                        age, t.success_count,
                        t.pattern_hash & 0xFFFFFFFF,
                    ).unwrap();
                }
                if traces.len() > 20 {
                    writeln!(out, "  ... и ещё {} traces", traces.len() - 20).unwrap();
                }
            }
        }

        ":tension" => {
            let exp    = engine.ashti.experience();
            let tt     = exp.tension_traces();
            let tick   = engine.tick_count;
            if tt.is_empty() {
                writeln!(out, "  no active tension traces").unwrap();
            } else {
                writeln!(out, "  ══ Tension Traces ═════════════════════").unwrap();
                writeln!(out, "  Active: {}", tt.len()).unwrap();
                writeln!(out, "  {:>3}  {:>4}  {:>10}  {:>12}", "#", "Temp", "Hash", "Age (ticks)").unwrap();
                for (i, t) in tt.iter().enumerate() {
                    let age = tick.saturating_sub(t.created_at);
                    let ph = t.pattern.temperature as u64 ^ t.pattern.mass as u64;
                    writeln!(out, "  {:>3}  {:>4}  {:#010x}   {:>12}", i + 1, t.temperature, ph, age).unwrap();
                }
            }
        }

        ":detail" => {
            writeln!(out, "  detail: {}  (off|min|mid|max)", config.detail_level.as_str()).unwrap();
        }

        ":depth" => {
            let (max_passes, min_coh) = engine.maya_multipass_params();
            let maya_id = engine.ashti.level_id() * 100 + 10;
            let dom_factor = engine.ashti.config_of(maya_id)
                .map(|c| c.internal_dominance_factor as f32 / 128.0)
                .unwrap_or(0.0);
            let exp = engine.ashti.experience();
            writeln!(out, "  ══ Cognitive Depth ════════════════════").unwrap();
            writeln!(out, "  max_passes:          {}", max_passes).unwrap();
            writeln!(out, "  min_coherence:       {:.2}", min_coh).unwrap();
            writeln!(out, "  internal_dominance:  {:.2}", dom_factor).unwrap();
            writeln!(out, "  tension_threshold:   128  (drain at 50% heat)").unwrap();
            writeln!(out, "  ── current state ──────────────────────").unwrap();
            writeln!(out, "  traces:              {}", exp.traces().len()).unwrap();
            writeln!(out, "  tension_active:      {}", exp.tension_count()).unwrap();
        }

        ":arbiter" => {
            writeln!(out, "  ══ Arbiter — Domain Thresholds ════════").unwrap();
            writeln!(out, "  {:>5}  {:>10}  {:>8}  {:>7}  {:>8}  {:>8}",
                "ID", "Name", "Reflex-T", "Assoc-T", "Cooldown", "MaxPass").unwrap();
            let mut configs = engine.ashti.all_configs();
            configs.sort_by_key(|(id, _)| *id);
            for (id, cfg) in &configs {
                if cfg.structural_role == 0 { continue; }
                writeln!(out, "  {:>5}  {:>10}  {:>8}  {:>7}  {:>8}  {:>8}",
                    id, domain_name(*id),
                    cfg.reflex_threshold, cfg.association_threshold,
                    cfg.reflex_cooldown, cfg.max_passes,
                ).unwrap();
            }
            let reflector = engine.ashti.reflector();
            writeln!(out, "  ── reflector ──────────────────────────").unwrap();
            writeln!(out, "  patterns tracked:  {}", reflector.tracked_patterns()).unwrap();
            writeln!(out, "  reflex success:    {}  fail: {}", reflector.total_success(), reflector.total_fail()).unwrap();
        }

        ":perf" => {
            let avg = perf.avg_ns();
            let hz  = perf.actual_hz();
            writeln!(out, "  ══ Performance ════════════════════════").unwrap();
            writeln!(out, "  uptime:       {:.1}s", perf.uptime_secs()).unwrap();
            writeln!(out, "  total ticks:  {}", perf.total_ticks).unwrap();
            writeln!(out, "  actual rate:  {:.1} Hz (target: {} Hz)", hz, config.tick_hz).unwrap();
            writeln!(out, "  ── tick breakdown ─────────────────────").unwrap();
            writeln!(out, "  avg tick:     {}", fmt_ns(avg as u64)).unwrap();
            if perf.peak_ns > 0 {
                writeln!(out, "  peak tick:    {}  (tick #{})", fmt_ns(perf.peak_ns), perf.peak_tick_id).unwrap();
            }
            let budget_ns = 1_000_000u64 / config.tick_hz.max(1) as u64 * 1000;
            if budget_ns > 0 {
                writeln!(out, "  budget used:  {:.2}%", avg / budget_ns as f64 * 100.0).unwrap();
            }
            let s = engine.tick_schedule.clone();
            let t = perf.total_ticks;
            writeln!(out, "  ── periodic tasks (calls) ─────────────").unwrap();
            if s.adaptation_interval > 0 {
                writeln!(out, "  adaptation:   {} calls (every {} ticks)", t / s.adaptation_interval as u64, s.adaptation_interval).unwrap();
            }
            if s.horizon_gc_interval > 0 {
                writeln!(out, "  horizon_gc:   {} calls (every {} ticks)", t / s.horizon_gc_interval as u64, s.horizon_gc_interval).unwrap();
            }
            if s.dream_interval > 0 {
                writeln!(out, "  dream:        {} calls (every {} ticks)", t / s.dream_interval as u64, s.dream_interval).unwrap();
            }
            if s.tension_check_interval > 0 {
                writeln!(out, "  tension_chk:  {} calls (every {} ticks)", t / s.tension_check_interval as u64, s.tension_check_interval).unwrap();
            }
        }

        ":tickrate" => {
            let a = &engine.tick_schedule.adaptive_tick;
            writeln!(out, "  current_hz:  {} Hz", a.current_hz).unwrap();
            writeln!(out, "  reason:      {}", a.last_reason).unwrap();
            writeln!(out, "  idle_ticks:  {}", a.idle_ticks).unwrap();
            writeln!(out, "  range:       {}..{} Hz", a.min_hz, a.max_hz).unwrap();
            writeln!(out, "  adaptive:    {}", if config.adaptive_tick_rate { "on" } else { "off" }).unwrap();
        }

        ":events" => {
            let n: usize = parts.get(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(10)
                .min(event_log.len().max(1));
            if event_log.is_empty() {
                writeln!(out, "  no events captured yet").unwrap();
            } else {
                let take = n.min(event_log.len());
                writeln!(out, "  ══ Last {} COM Events ══════════════════", take).unwrap();
                writeln!(out, "  {:>10}  {:>6}  {:>6}  {:>6}", "ID", "Type", "Domain", "Target").unwrap();
                for ev in event_log.iter().rev().take(take) {
                    writeln!(out, "  {:>10}  {:#06x}  {:>6}  {:>6}", ev.event_id, ev.event_type, ev.domain_id, ev.target_id).unwrap();
                }
            }
        }

        ":frontier" => {
            let stats = engine.ashti.frontier_stats();
            let total_size: usize = stats.iter().map(|(_, sz, _)| sz).sum();
            writeln!(out, "  ══ Causal Frontier ════════════════════").unwrap();
            writeln!(out, "  {:>5}  {:>10}  {:>8}  {:>8}", "ID", "Name", "Size", "Mem%").unwrap();
            for (id, size, mem) in &stats {
                writeln!(out, "  {:>5}  {:>10}  {:>8}  {:>7.1}%", id, domain_name(*id), size, mem * 100.0).unwrap();
            }
            writeln!(out, "  ── total frontier size: {}", total_size).unwrap();
        }

        ":guardian" => {
            let s = engine.guardian.stats();
            writeln!(out, "  ══ GUARDIAN ════════════════════════════").unwrap();
            writeln!(out, "  reflexes approved:    {}", s.reflex_allowed).unwrap();
            writeln!(out, "  reflexes vetoed:      {}", s.reflex_vetoed).unwrap();
            writeln!(out, "  access denied:        {}", s.access_denied).unwrap();
            writeln!(out, "  protocol denied:      {}", s.protocol_denied).unwrap();
            writeln!(out, "  domains scanned:      {}", s.domains_scanned).unwrap();
            writeln!(out, "  thresholds adapted:   {}", s.thresholds_adapted).unwrap();
            writeln!(out, "  dream proposals:      {}", s.dream_proposals).unwrap();
        }

        ":trace" => {
            match parts.get(1).and_then(|s| s.parse::<usize>().ok()) {
                None => writeln!(out, "  Usage: :trace <index>  (1-based, same as :traces output)").unwrap(),
                Some(idx) if idx == 0 => writeln!(out, "  index must be ≥ 1").unwrap(),
                Some(idx) => {
                    let exp   = engine.ashti.experience();
                    let traces = exp.traces();
                    let mut sorted: Vec<_> = traces.iter().enumerate().collect();
                    sorted.sort_by(|a, b| b.1.weight.total_cmp(&a.1.weight));
                    match sorted.get(idx - 1) {
                        None => writeln!(out, "  trace #{} not found (total: {})", idx, traces.len()).unwrap(),
                        Some((_, t)) => {
                            let tick = engine.tick_count;
                            let age  = tick.saturating_sub(t.created_at);
                            let [x, y, z] = t.pattern.position;
                            writeln!(out, "  ══ Experience Trace #{} ════════════════", idx).unwrap();
                            writeln!(out, "  weight:        {:.4}", t.weight).unwrap();
                            writeln!(out, "  success_count: {}", t.success_count).unwrap();
                            writeln!(out, "  pattern_hash:  {:#018x}", t.pattern_hash).unwrap();
                            writeln!(out, "  created_at:    {} (age: {} ticks)", t.created_at, age).unwrap();
                            writeln!(out, "  last_used:     {}", t.last_used).unwrap();
                            writeln!(out, "  ── pattern (token) ────────────────────").unwrap();
                            writeln!(out, "  position:      ({}, {}, {})", x, y, z).unwrap();
                            writeln!(out, "  temperature:   {}", t.pattern.temperature).unwrap();
                            writeln!(out, "  mass:          {}", t.pattern.mass).unwrap();
                            writeln!(out, "  valence:       {}", t.pattern.valence).unwrap();
                            writeln!(out, "  velocity:      ({}, {}, {})",
                                t.pattern.velocity[0], t.pattern.velocity[1], t.pattern.velocity[2]).unwrap();
                            writeln!(out, "  type_flags:    {:#04x}", t.pattern.type_flags).unwrap();
                        }
                    }
                }
            }
        }

        ":connections" => {
            let states = engine.ashti.all_states();
            let filter_id = parts.get(1).and_then(|s| s.parse::<u16>().ok());
            let mut total = 0usize;
            writeln!(out, "  ══ Connections ════════════════════════").unwrap();
            for (id, state) in &states {
                if let Some(fid) = filter_id {
                    if *id != fid { continue; }
                }
                if state.connections.is_empty() { continue; }
                writeln!(out, "  ── domain {} ({}) ── {} connections",
                    id, domain_name(*id), state.connections.len()).unwrap();
                for (i, c) in state.connections.iter().take(10).enumerate() {
                    writeln!(out, "  {:>3}  {:>8}→{:<8}  strength={:.2}  stress={:.2}  type={:#06x}",
                        i + 1, c.source_id, c.target_id,
                        c.strength, c.current_stress, c.link_type).unwrap();
                }
                if state.connections.len() > 10 {
                    writeln!(out, "  ... и ещё {}", state.connections.len() - 10).unwrap();
                }
                total += state.connections.len();
            }
            if total == 0 {
                writeln!(out, "  no connections").unwrap();
            } else {
                writeln!(out, "  ── total: {} connections", total).unwrap();
            }
        }

        ":dream" => {
            let exp         = engine.ashti.experience();
            let candidates  = exp.find_crystallizable(0.9, 5);
            let gs          = engine.guardian.stats();
            writeln!(out, "  ══ DREAM ═══════════════════════════════").unwrap();
            writeln!(out, "  dream_proposals:  {}", gs.dream_proposals).unwrap();
            writeln!(out, "  crystallizable:   {} (weight≥0.9, success≥5)", candidates.len()).unwrap();
            if candidates.is_empty() {
                writeln!(out, "  (no candidates — more experience needed)").unwrap();
            } else {
                writeln!(out, "  {:>3}  {:>6}  {:>5}  {:>10}", "#", "Weight", "Succ", "Hash").unwrap();
                for (i, t) in candidates.iter().enumerate() {
                    writeln!(out, "  {:>3}  {:.4}  {:>5}  {:#010x}",
                        i + 1, t.weight, t.success_count,
                        t.pattern_hash & 0xFFFFFFFF).unwrap();
                }
            }
        }

        ":dream-stats" => {
            let state   = engine.dream_phase_state;
            let fatigue = engine.dream_scheduler.current_fatigue();
            let idle    = engine.dream_scheduler.current_idle_ticks();
            let stats   = &engine.dream_phase_stats;
            let sched   = &engine.dream_scheduler.stats;
            writeln!(out, "  ══ DREAM Phase ══════════════════════════").unwrap();
            writeln!(out, "  Current state:      {:?}", state).unwrap();
            writeln!(out, "  Current fatigue:    {}/255 ({:.0}%)", fatigue, fatigue as f32 / 2.55).unwrap();
            writeln!(out, "  Idle ticks:         {}", idle).unwrap();
            writeln!(out, "  Total sleeps:       {}", stats.total_sleeps).unwrap();
            writeln!(out, "  Total dream ticks:  {}", stats.total_dream_ticks).unwrap();
            writeln!(out, "  Interrupted dreams: {}", stats.interrupted_dreams).unwrap();
            writeln!(out, "  By trigger:  Idle={}, Fatigue={}, Explicit={}",
                sched.idle_triggers, sched.fatigue_triggers, sched.explicit_triggers).unwrap();
            let cycle = &engine.dream_cycle.stats;
            writeln!(out, "  Cycles:  total={}, complete={}, timeout={}, approved={}, vetoed={}",
                cycle.total_cycles, cycle.completed_cycles, cycle.timed_out_cycles,
                cycle.total_approved, cycle.total_vetoed).unwrap();
        }

        ":multipass" => {
            writeln!(out, "  ══ Multi-Pass Statistics ══════════════").unwrap();
            writeln!(out, "  total events:     {}", engine.com_next_id).unwrap();
            writeln!(out, "  multipass count:  {}", multipass_count).unwrap();
            let rate = if engine.com_next_id > 0 {
                multipass_count as f64 / engine.com_next_id as f64 * 100.0
            } else { 0.0 };
            writeln!(out, "  multipass rate:   {:.2}%", rate).unwrap();
            if multipass_count > 0 {
                writeln!(out, "  last passes:      {}", last_multipass_n).unwrap();
            }
            let (max_passes, min_coh) = engine.maya_multipass_params();
            writeln!(out, "  ── config ─────────────────────────────").unwrap();
            writeln!(out, "  max_passes:       {}", max_passes).unwrap();
            writeln!(out, "  min_coherence:    {:.2}", min_coh).unwrap();
        }

        ":reflector" => {
            let reflector = engine.ashti.reflector();
            writeln!(out, "  ══ REFLECTOR — Per-Domain ═════════════").unwrap();
            writeln!(out, "  {:>5}  {:>10}  {:>8}  {:>8}  {:>8}", "Role", "Name", "Success", "Total", "Rate").unwrap();
            let mut has_data = false;
            for role in 1u8..=8 {
                if let Some(profile) = reflector.domain_profile(role) {
                    let total = profile.total_calls();
                    if total == 0 { continue; }
                    has_data = true;
                    let rate = profile.overall_success_rate();
                    let domain_id = engine.ashti.level_id() * 100 + role as u16;
                    writeln!(out, "  {:>5}  {:>10}  {:>8}  {:>8}  {:>7.1}%",
                        role, domain_name(domain_id),
                        (rate * total as f32) as u32, total, rate * 100.0).unwrap();
                }
            }
            if !has_data {
                writeln!(out, "  no reflector data yet").unwrap();
            }
            writeln!(out, "  ── global ─────────────────────────────").unwrap();
            writeln!(out, "  patterns tracked: {}", reflector.tracked_patterns()).unwrap();
            writeln!(out, "  reflex success:   {}  fail: {}", reflector.total_success(), reflector.total_fail()).unwrap();
        }

        ":impulses" => {
            let tick     = engine.tick_count;
            let interval = engine.tick_schedule.goal_check_interval;
            let goals = engine.ashti.generate_goal_impulses(tick, interval as u64);
            let exp         = engine.ashti.experience();
            let curiosity   = exp.find_crystallizable(0.72, 2);
            let tension_n   = exp.tension_count();
            writeln!(out, "  ══ Pending Impulses ═══════════════════").unwrap();
            writeln!(out, "  tension traces:  {} (each may generate impulse)", tension_n).unwrap();
            writeln!(out, "  goal impulses:   {}", goals.len()).unwrap();
            writeln!(out, "  curiosity cands: {}", curiosity.len()).unwrap();
            if !goals.is_empty() {
                writeln!(out, "  ── goal ───────────────────────────────").unwrap();
                for (i, imp) in goals.iter().enumerate() {
                    let [x, y, z] = imp.pattern.position;
                    writeln!(out, "  {:>3}  weight={:.2}  pos=({},{},{})", i + 1, imp.weight, x, y, z).unwrap();
                }
            }
            if !curiosity.is_empty() {
                writeln!(out, "  ── curiosity (near crystallization) ───").unwrap();
                for (i, t) in curiosity.iter().take(5).enumerate() {
                    writeln!(out, "  {:>3}  weight={:.4}  success={}  {:#010x}",
                        i + 1, t.weight, t.success_count, t.pattern_hash & 0xFFFFFFFF).unwrap();
                }
            }
        }

        ":watch" => {
            if watch_fields.is_empty() {
                writeln!(out, "  watching: nothing").unwrap();
            } else {
                let fields: Vec<_> = watch_fields.iter().map(|s| s.as_str()).collect();
                writeln!(out, "  watching: {}", fields.join(", ")).unwrap();
            }
        }

        ":unwatch" => {
            match parts.get(1).copied() {
                None => writeln!(out, "  Usage: :unwatch <field>").unwrap(),
                Some(field) => writeln!(out, "  unwatched: {}", field).unwrap(),
            }
        }

        ":config" => {
            let s = &engine.tick_schedule;
            writeln!(out, "  ══ Configuration ══════════════════════").unwrap();
            writeln!(out, "  tick_hz:          {}", config.tick_hz).unwrap();
            writeln!(out, "  detail_level:     {}", config.detail_level.as_str()).unwrap();
            writeln!(out, "  verbose:          {}", config.verbose).unwrap();
            writeln!(out, "  adaptive:         {}", config.adaptive_tick_rate).unwrap();
            writeln!(out, "  data_dir:         {}", config.data_dir).unwrap();
            writeln!(out, "  ── tick schedule ──────────────────────").unwrap();
            writeln!(out, "  tension_check:    {}", s.tension_check_interval).unwrap();
            writeln!(out, "  adaptation:       {}", s.adaptation_interval).unwrap();
            writeln!(out, "  dream:            {}", s.dream_interval).unwrap();
            writeln!(out, "  horizon_gc:       {}", s.horizon_gc_interval).unwrap();
            writeln!(out, "  reconcile:        {}", s.reconcile_interval).unwrap();
            writeln!(out, "  persist_check:    {}", s.persist_check_interval).unwrap();
            writeln!(out, "  ── adaptive tick ──────────────────────").unwrap();
            writeln!(out, "  min_hz:           {}", s.adaptive_tick.min_hz).unwrap();
            writeln!(out, "  max_hz:           {}", s.adaptive_tick.max_hz).unwrap();
            writeln!(out, "  step_up:          {}", s.adaptive_tick.step_up).unwrap();
            writeln!(out, "  step_down:        {}", s.adaptive_tick.step_down).unwrap();
            writeln!(out, "  cooldown:         {}", s.adaptive_tick.cooldown).unwrap();
        }

        ":schema" => {
            let kind = parts.get(1).copied().unwrap_or("axiom");
            match kind {
                "axiom" => writeln!(out, "{}", axiom_config::schema::axiom_schema_json()).unwrap(),
                "domain" => writeln!(out, "{}", axiom_config::schema::domain_schema_json()).unwrap(),
                "heartbeat" => writeln!(out, "{}", axiom_config::schema::heartbeat_schema_json()).unwrap(),
                "dream" => writeln!(out, "{}", axiom_config::schema::dream_schema_json()).unwrap(),
                "cli" => {
                    let schema = schemars::schema_for!(CliConfigFile);
                    match serde_json::to_string_pretty(&schema) {
                        Ok(s) => writeln!(out, "{s}").unwrap(),
                        Err(e) => writeln!(out, "  error: {e}").unwrap(),
                    }
                }
                _ => {
                    writeln!(out, "  Usage: :schema [axiom|domain|heartbeat|dream|cli]").unwrap();
                    writeln!(out, "    axiom     — JSON-схема axiom.yaml (корневой конфиг)").unwrap();
                    writeln!(out, "    domain    — JSON-схема доменного конфига (presets/domains/)").unwrap();
                    writeln!(out, "    heartbeat — JSON-схема heartbeat.yaml").unwrap();
                    writeln!(out, "    dream     — JSON-схема dream.yaml (DREAM Phase)").unwrap();
                    writeln!(out, "    cli       — JSON-схема axiom-cli.yaml").unwrap();
                }
            }
        }

        ":anchors" => {
            match anchor_set {
                None => writeln!(out, "  no anchors loaded (config/anchors/ not found or empty)").unwrap(),
                Some(set) => {
                    let sub = parts.get(1).copied();
                    match sub {
                        None => {
                            writeln!(out, "  ══ Anchor Set ═════════════════════════").unwrap();
                            writeln!(out, "  total:   {}", set.total_count()).unwrap();
                            writeln!(out, "  axes:    {}", set.axes.len()).unwrap();
                            writeln!(out, "  layers:  {} total ({})",
                                set.layers.iter().map(|l| l.len()).sum::<usize>(),
                                (1..=8).filter(|&i| !set.layers[i-1].is_empty())
                                    .map(|i| format!("L{}", i))
                                    .collect::<Vec<_>>().join(", ")).unwrap();
                            writeln!(out, "  domains: {} total ({})",
                                set.domains.iter().map(|d| d.len()).sum::<usize>(),
                                (1..=8).filter(|&i| !set.domains[i-1].is_empty())
                                    .map(|i| format!("D{}", i))
                                    .collect::<Vec<_>>().join(", ")).unwrap();
                        }
                        Some("axes") => {
                            writeln!(out, "  ══ Axes ({}) ════════════════════════════", set.axes.len()).unwrap();
                            for a in &set.axes {
                                let [x, y, z] = a.position;
                                writeln!(out, "  {:20}  pos=({:7},{:7},{:7})  aliases={}", a.word, x, y, z, a.aliases.len()).unwrap();
                            }
                        }
                        Some("layer") => {
                            let layer_arg = parts.get(2).copied().unwrap_or("");
                            let idx = layer_arg.trim_start_matches('L').parse::<usize>().unwrap_or(0);
                            if idx == 0 || idx > 8 {
                                writeln!(out, "  Usage: :anchors layer L<1..8>").unwrap();
                            } else {
                                let anchors = &set.layers[idx - 1];
                                writeln!(out, "  ══ Layer L{} ({} anchors) ════════════════", idx, anchors.len()).unwrap();
                                for a in anchors {
                                    let [x, y, z] = a.position;
                                    writeln!(out, "  {:20}  pos=({:7},{:7},{:7})", a.word, x, y, z).unwrap();
                                }
                            }
                        }
                        Some("domain") => {
                            let dom_arg = parts.get(2).copied().unwrap_or("");
                            let idx = dom_arg.trim_start_matches('D').parse::<usize>().unwrap_or(0);
                            if idx == 0 || idx > 8 {
                                writeln!(out, "  Usage: :anchors domain D<1..8>").unwrap();
                            } else {
                                let anchors = &set.domains[idx - 1];
                                writeln!(out, "  ══ Domain D{} ({} anchors) ══════════════", idx, anchors.len()).unwrap();
                                for a in anchors {
                                    let [x, y, z] = a.position;
                                    writeln!(out, "  {:20}  pos=({:7},{:7},{:7})", a.word, x, y, z).unwrap();
                                }
                            }
                        }
                        Some(word) => {
                            let word_lower = word.to_lowercase();
                            let mut found = false;
                            for a in set.axes.iter()
                                .chain(set.layers.iter().flatten())
                                .chain(set.domains.iter().flatten())
                            {
                                if a.word.to_lowercase() == word_lower
                                    || a.aliases.iter().any(|al| al.to_lowercase() == word_lower)
                                {
                                    let [x, y, z] = a.position;
                                    writeln!(out, "  ══ Anchor: {} ════════════════════════", a.word).unwrap();
                                    writeln!(out, "  id:          {}", a.id).unwrap();
                                    writeln!(out, "  aliases:     {}", a.aliases.join(", ")).unwrap();
                                    writeln!(out, "  tags:        {}", a.tags.join(", ")).unwrap();
                                    writeln!(out, "  position:    ({}, {}, {})", x, y, z).unwrap();
                                    writeln!(out, "  shell:       {:?}", a.shell).unwrap();
                                    if !a.description.is_empty() {
                                        writeln!(out, "  description: {}", a.description).unwrap();
                                    }
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                writeln!(out, "  anchor '{}' not found", word).unwrap();
                            }
                        }
                    }
                }
            }
        }

        ":match" => {
            let text = parts.get(1).map(|s| s.trim()).unwrap_or("");
            if text.is_empty() {
                writeln!(out, "  Usage: :match \"<text>\"").unwrap();
            } else {
                let text = text.trim_matches('"');
                match anchor_set {
                    None => writeln!(out, "  no anchors loaded — would use FNV-1a hash fallback").unwrap(),
                    Some(set) => {
                        let matches = set.match_text(text);
                        if matches.is_empty() {
                            writeln!(out, "  no anchor matches — FNV-1a hash fallback").unwrap();
                            let bytes = text.as_bytes();
                            let mut h: u64 = 0xcbf29ce484222325;
                            for &b in bytes { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); }
                            let x = ((h >>  0) & 0x7FFF) as f32;
                            let y = ((h >> 16) & 0x7FFF) as f32;
                            let z = ((h >> 32) & 0x7FFF) as f32;
                            writeln!(out, "  hash position: ({:.0}, {:.0}, {:.0})", x, y, z).unwrap();
                        } else {
                            let pos = set.compute_position(&matches);
                            let sw  = set.compute_semantic_weight(&matches);
                            let shell = set.compute_shell(&matches);
                            writeln!(out, "  ══ Anchor Match: \"{}\" ═══════════════════", text).unwrap();
                            writeln!(out, "  matches:         {}", matches.len()).unwrap();
                            writeln!(out, "  position:        ({:.0}, {:.0}, {:.0})", pos[0], pos[1], pos[2]).unwrap();
                            writeln!(out, "  semantic_weight: {:.3}", sw).unwrap();
                            writeln!(out, "  shell:           {:?}", shell).unwrap();
                            writeln!(out, "  ── matched anchors ────────────────────").unwrap();
                            for m in &matches {
                                writeln!(out, "  {:20}  score={:.2}  type={}  matched='{}'",
                                    m.anchor.word, m.score, m.match_type.as_str(), m.matched_word).unwrap();
                            }
                        }
                    }
                }
            }
        }

        ":domain" => {
            match parts.get(1).and_then(|s| s.parse::<u16>().ok()) {
                None => writeln!(out, "  Usage: :domain <domain_id>").unwrap(),
                Some(id) => match engine.ashti.config_of(id) {
                    None => writeln!(out, "  Domain {} not found", id).unwrap(),
                    Some(cfg) => {
                        let state = engine.ashti.all_states()
                            .into_iter()
                            .find(|(did, _)| *did == id);
                        let (tokens, conns) = state
                            .map(|(_, s)| (s.tokens.len(), s.connections.len()))
                            .unwrap_or((0, 0));
                        writeln!(out, "  ══ Domain {} ({}) ════════════════════", id, domain_name(id)).unwrap();
                        writeln!(out, "  structural_role:  {}", cfg.structural_role).unwrap();
                        writeln!(out, "  token_capacity:   {} (used: {})", cfg.token_capacity, tokens).unwrap();
                        writeln!(out, "  connection_cap:   {} (used: {})", cfg.connection_capacity, conns).unwrap();
                        writeln!(out, "  temperature:      {:.1}", cfg.temperature).unwrap();
                        writeln!(out, "  gravity_strength: {:.2}", cfg.gravity_strength).unwrap();
                        writeln!(out, "  ── arbiter ────────────────────────────").unwrap();
                        writeln!(out, "  reflex_threshold: {}", cfg.reflex_threshold).unwrap();
                        writeln!(out, "  assoc_threshold:  {}", cfg.association_threshold).unwrap();
                        writeln!(out, "  reflex_cooldown:  {}", cfg.reflex_cooldown).unwrap();
                        writeln!(out, "  max_passes:       {}", cfg.max_passes).unwrap();
                        writeln!(out, "  min_coherence:    {:.2}", cfg.min_coherence as f32 / 255.0).unwrap();
                        writeln!(out, "  ── membrane ───────────────────────────").unwrap();
                        writeln!(out, "  permeability:     {:.2}", cfg.permeability as f32 / 255.0).unwrap();
                        writeln!(out, "  threshold_mass:   {}", cfg.threshold_mass).unwrap();
                        writeln!(out, "  threshold_temp:   {}", cfg.threshold_temp).unwrap();
                    }
                },
            }
        }

        _ => {
            writeln!(out, "  Unknown command '{}'. Type :help for list.", parts[0]).unwrap();
        }
    }
    out
}

// ── handle_meta_mutate ────────────────────────────────────────────────────────

/// Команды с мутацией Engine/AutoSaver — вызываются только из tick loop.
pub fn handle_meta_mutate(
    line:       &str,
    engine:     &mut AxiomEngine,
    auto_saver: &mut AutoSaver,
    config:     &CliConfig,
) -> MetaMutateResult {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    let mut output = String::new();
    let action = match parts[0] {

        ":quit" | ":q" => {
            if auto_saver.config.enabled {
                let data_dir = std::path::Path::new(&config.data_dir);
                match auto_saver.force_save(engine, data_dir) {
                    Ok(_)  => writeln!(output, "  autosaved to {}", data_dir.display()).unwrap(),
                    Err(e) => writeln!(output, "  autosave on quit failed: {e}").unwrap(),
                }
            }
            writeln!(output, "Завершение.").unwrap();
            MetaAction::Quit
        }

        ":tick" => {
            let n: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
            let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
            for _ in 0..n { engine.process_command(&tick_cmd); }
            writeln!(output, "  ticked {} times. tick_count={}", n, engine.tick_count).unwrap();
            MetaAction::None
        }

        ":force-sleep" => {
            engine.dream_scheduler.submit_explicit_command(0);
            writeln!(output, "  Submitted explicit sleep command. System will fall asleep on next tick.").unwrap();
            MetaAction::None
        }

        ":wake-up" => {
            use axiom_runtime::GatewayPriority;
            use axiom_ucl::{UclCommand, OpCode};
            let wake_cmd = UclCommand::new(OpCode::TickForward, 0, 255, 0);
            engine.submit_priority_command(wake_cmd, GatewayPriority::Critical);
            writeln!(output, "  Critical wake signal sent. State: {:?}", engine.dream_phase_state).unwrap();
            MetaAction::None
        }

        ":save" => {
            let dir_str = parts.get(1).copied().unwrap_or(config.data_dir.as_str());
            let dir = std::path::Path::new(dir_str);
            match persist_save(engine, dir, &WriteOptions::default()) {
                Ok(m) => writeln!(output,
                    "  saved to {dir_str} (tick={}, traces={}, tokens={})",
                    m.tick_count, m.contents.traces, m.contents.tokens).unwrap(),
                Err(e) => writeln!(output, "  save failed: {e}").unwrap(),
            }
            MetaAction::None
        }

        ":load" => {
            let dir_str = parts.get(1).copied().unwrap_or(config.data_dir.as_str());
            let dir = std::path::Path::new(dir_str);
            match persist_load(dir) {
                Ok(r) => {
                    writeln!(output,
                        "  loaded from {dir_str} (tick={}, traces={}, tension={})",
                        r.engine.tick_count, r.traces_imported, r.tension_imported).unwrap();
                    *engine = r.engine;
                    MetaAction::EngineReplaced
                }
                Err(e) => {
                    writeln!(output, "  load failed: {e}").unwrap();
                    MetaAction::None
                }
            }
        }

        ":autosave" => {
            match parts.get(1).copied() {
                Some("on") => {
                    let interval = parts.get(2)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1000u32);
                    auto_saver.set_interval(interval);
                    writeln!(output, "  autosave: on  interval={interval} ticks").unwrap();
                    MetaAction::AutosaveChanged(interval)
                }
                Some("off") => {
                    auto_saver.set_enabled(false);
                    writeln!(output, "  autosave: off").unwrap();
                    MetaAction::AutosaveChanged(0)
                }
                _ => {
                    let data_dir = std::path::Path::new(&config.data_dir);
                    writeln!(output, "{}", auto_saver.status_line(data_dir)).unwrap();
                    MetaAction::None
                }
            }
        }

        ":export" => {
            let what = parts.get(1).copied().unwrap_or("traces");
            let path_str = parts.get(2).copied().unwrap_or(match what {
                "skills" => "axiom-export-skills.json",
                _        => "axiom-export-traces.json",
            });
            let path = std::path::Path::new(path_str);
            match what {
                "skills" => match export_skills(engine, path) {
                    Ok(r)  => writeln!(output, "  exported {} skills → {}", r.exported, r.path).unwrap(),
                    Err(e) => writeln!(output, "  export failed: {e}").unwrap(),
                },
                "traces" => match export_traces(engine, path, 0.0) {
                    Ok(r)  => writeln!(output, "  exported {} traces → {}", r.exported, r.path).unwrap(),
                    Err(e) => writeln!(output, "  export failed: {e}").unwrap(),
                },
                _ => writeln!(output, "  Usage: :export traces|skills [path]").unwrap(),
            }
            MetaAction::None
        }

        ":import" => {
            let what = parts.get(1).copied().unwrap_or("traces");
            let path_str = parts.get(2).copied().unwrap_or(match what {
                "skills" => "axiom-export-skills.json",
                _        => "axiom-export-traces.json",
            });
            let path = std::path::Path::new(path_str);
            match what {
                "skills" => match import_skills(engine, path) {
                    Ok(r)  => writeln!(output, "  {}", r.summary_line()).unwrap(),
                    Err(e) => writeln!(output, "  import failed: {e}").unwrap(),
                },
                "traces" => match import_traces(engine, path) {
                    Ok(r)  => writeln!(output, "  {}", r.summary_line()).unwrap(),
                    Err(e) => writeln!(output, "  import failed: {e}").unwrap(),
                },
                _ => writeln!(output, "  Usage: :import traces|skills [path]").unwrap(),
            }
            MetaAction::None
        }

        _ => MetaAction::None,
    };
    MetaMutateResult { output, action }
}

// ── Help texts ────────────────────────────────────────────────────────────────

pub(crate) const HELP_TRACE: &str = "\
  :trace <index>
  Детали одного experience trace. Индекс — тот же что в :traces (1-based, top by weight).
  Показывает: weight, success_count, pattern_hash, created_at/age, last_used,
  а также поля Token паттерна (position, temperature, mass, valence, velocity).";

pub(crate) const HELP_CONNECTIONS: &str = "\
  :connections [domain_id]
  Связи в домене: source→target, strength, current_stress, link_type.
  Без аргумента — все домены. С аргументом — только указанный домен. Top-10 per domain.";

pub(crate) const HELP_DREAM: &str = "\
  :dream
  Состояние DREAM-цикла: число кристаллизуемых паттернов (weight≥0.9, success≥5)
  и суммарное число DREAM-proposals от GUARDIAN с момента запуска.";

pub(crate) const HELP_MULTIPASS: &str = "\
  :multipass
  Статистика multi-pass обработки: сколько событий вызвали повторные проходы,
  процент от всех COM-событий, и число проходов в последнем multipass.";

pub(crate) const HELP_REFLECTOR: &str = "\
  :reflector
  Per-domain точность REFLECTOR: success/total/rate для ролей 1–8.
  Показывает только домены у которых есть данные. Также global stats.";

pub(crate) const HELP_IMPULSES: &str = "\
  :impulses
  Диагностика очереди внутренних импульсов:
  - tension traces (каждый активный может генерировать impulse)
  - goal impulses (traces с GOAL-флагом, weight < достигнуто)
  - curiosity candidates (traces near crystallization threshold)";

pub(crate) const HELP_TEXT: &str = "\
  ── состояние ──────────────────────────────────────────────
  :status               — расширенный статус ядра
  :memory               — токены, связи, traces, tension
  :domains              — список доменов с числом токенов
  :domain <id>          — детали одного домена
  :tokens <domain_id>   — токены в домене
  :schedule             — текущий TickSchedule
  :snapshot             — info снапшота
  :tickrate             — адаптивная частота (Sentinel Phase 3)
  :config               — текущая конфигурация CLI
  ── опыт ───────────────────────────────────────────────────
  :traces               — experience traces top-20 по weight
  :trace <n>            — детали одного trace
  :tension              — активные tension traces
  :depth                — параметры Cognitive Depth
  :dream                — DREAM-цикл и кандидаты на кристаллизацию
  :dream-stats          — DREAM Phase: состояние, fatigue, статистика циклов
  :multipass            — статистика multi-pass обработки
  :reflector            — per-domain точность REFLECTOR
  :impulses             — диагностика очереди импульсов
  ── связи ──────────────────────────────────────────────────
  :connections [id]     — связи в домене (top-10 per domain)
  :frontier             — Causal Frontier size + mem%
  :events [N]           — последние N COM-событий
  ── системное ──────────────────────────────────────────────
  :guardian             — GUARDIAN stats
  :arbiter              — Arbiter thresholds per domain
  :perf                 — производительность тиков
  :schema [kind]        — JSON-схема конфига (axiom|domain|heartbeat|dream|cli)
  :anchors [sub]        — якорные токены (axes|layer L<n>|domain D<n>|<word>)
  :match \"<text>\"       — тест маппинга текста → позиция
  ── управление ─────────────────────────────────────────────
  :tick [N]             — ручной тик (N раз)
  :force-sleep          — принудительное засыпание на следующем тике
  :wake-up              — Critical-сигнал пробуждения из DREAMING
  :verbose [on|off]     — подробный вывод после тика
  :detail [off|min|mid|max] — уровень детализации
  :watch <field>        — следить за traces|tension|tps
  :unwatch <field>      — снять слежение
  :save [dir]           — сохранить состояние
  :load [dir]           — загрузить состояние
  :autosave [on N|off]  — автосохранение
  :export [traces|skills] [path]
  :import [traces|skills] [path]
  :help [cmd]           — справка по команде
  :quit                 — завершить";
