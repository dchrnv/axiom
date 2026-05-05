# AXIOM Status

**Обновлено:** 2026-05-05
**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

---

## Текущее состояние

**1174 тестов, 0 failures**

```
AxiomEngine
  ├── Genome (конституция, from_yaml, GenomeIndex O(1))
  ├── AshtiCore — 11 доменов (SUTRA=level*100 .. MAYA=level*100+10)
  │     ├── Arbiter (dual-path routing + Experience + Reflector + SkillSet + Internal Drive)
  │     ├── 11 × Domain (физика поля + CausalFrontier V2.0)
  │     └── 11 × DomainState (токены + связи)
  ├── Guardian (CODEX + GENOME: enforce_access, validate_reflex, ML filters)
  └── Over-Domain Layer:
        ├── OverDomainComponent trait (object-safe, on_tick каждые N тиков через mem::take)
        ├── Weaver trait (type Pattern, scan, propose_to_dream, check_promotion)
        └── FrameWeaver V1.2 ✅ — scan MAYA (0x08 Syntactic) → кристаллизация EXPERIENCE (109)
              scan_state, build_crystallization_commands, ReinforceFrame (lineage_hash dedup),
              build_promotion_commands (→ SUTRA STATE_LOCKED + TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE),
              CycleStrategy::Allow (default); Weaver trait: scan(tick,…), check_promotion(tick,…);
              restore_frame_from_anchor — восстановление участников из графа EXPERIENCE;
              UnfoldFrame handler — разворачивает Frame в произвольный целевой домен;
              встроен в AxiomEngine (on_tick + drain_commands внутри interval-guard);
              GENOME: 5 access rules (MAYA/Read, EXPERIENCE/ReadWrite, SUTRA/Control);
              35+ unit-тестов + integration + e2e smoke; FrameWeaverStats: unfold_requests;
              V1.2: промоция перенесена из on_tick → dream_propose() (вызов при FallingAsleep)

DREAM Phase V1.0 ✅ — когнитивный сон: 4 состояния (Wake/FallingAsleep/Dreaming/Waking)
  ├── DreamScheduler — 3 триггера: Idle (порог idle тиков), Fatigue (0-255, 4 фактора), ExplicitCommand
  ├── FatigueTracker — composite score = Σ(factor × weight) / Σ(weight); отслеживает 4 показателя
  ├── DreamCycle — 3 этапа: Stabilization → Processing → Consolidation; DreamProposal (Promotion/HeavyCrystallization)
  ├── GUARDIAN: check_frame_anchor_sutra_write() — FRAME_ANCHOR в SUTRA только в DREAMING
  ├── GatewayPriority: Normal (игнорируется в DREAMING) / Critical (пробуждение) / Emergency (V2.0=Critical)
  ├── Gateway::with_config() — старт с загрузкой DreamConfig из axiom.yaml
  ├── CLI: :dream-stats / :force-sleep / :wake-up
  └── BroadcastSnapshot расширен: dream_phase, dream_stats (FatigueStats, SchedulerStats, CycleStats)

FractalChain — N уровней AshtiCore (MAYA[n] → SUTRA[n+1], skills exchange)
ConfigWatcher — горячая перезагрузка axiom.yaml (inotify), передаётся в tick_loop
EventBus — pub/sub: типизированные и broadcast подписки
domain_name() — pub fn в axiom-runtime (EA-TD-01 ✅)

axiom-agent:
  ├── TextPerceptor — текст → UclCommand(InjectToken): якорное позиционирование → FNV-1a fallback
  ├── MessageEffector — ProcessingResult → диагностический вывод (DetailLevel: off/min/mid/max)
  ├── MLEngine (mock + ONNX) → VisionPerceptor, AudioPerceptor (VAD)
  ├── CLI Channel: stdin/stdout loop, axiom-cli.yaml, все :команды
  │   CLI Extended V1.0: :domain/:events/:frontier/:guardian/:watch/:config/:trace/:connections
  │   :dream/:multipass/:reflector/:impulses/:schema/:anchors/:match/:help/:perf/:tickrate
  │   Горячая перезагрузка config/axiom.yaml (--hot-reload) через ConfigWatcher → tick_loop
  └── External Adapters (Phase 0–5 + Tech Debt Closure):
      ├── tick_loop — единственный writer AxiomEngine; CliState (PerfTracker, event_log,
      │              watch_fields, multipass_count); адаптивный sleep (EA-TD-03/04 ✅)
      ├── AdapterCommand / AdapterPayload — Inject, MetaRead, MetaMutate, DomainSnapshot,
      │              Subscribe, Unsubscribe; AdapterSource: Cli, WebSocket, Rest, Telegram
      ├── ServerMessage — Result, Tick, State, CommandResult, DomainDetail, Error (serde JSON)
      ├── WebSocket (Phase 1) — axum 0.8/ws, /ws endpoint, подписки ticks/state,
      │              --server / --port флаги; AppState shared через Arc
      ├── REST (Phase 2) — axum Router, 5 handlers (inject/status/domains/traces/domain-detail),
      │              корреляция через broadcast + timeout 5s
      ├── Dashboard (Phase 3) — tools/axiom-dashboard, egui/eframe, sync tungstenite,
      │              4 панели: Status, Space View, Domain List, Input
      ├── Telegram (Phase 4, feature "telegram") — long-poll getUpdates, route_message,
      │              pending HashMap корреляция, --telegram-token / --telegram-allow
      └── OpenSearch (Phase 5, feature "opensearch") — индексирует Result+Tick events,
                     build_result_doc / build_tick_doc, fire-and-forget POST,
                     --opensearch-url / --opensearch-index / --opensearch-tick

axiom-runtime:
  ├── process_and_observe() — обёртка process_command() с диагностикой (ProcessingResult)
  ├── Orchestrator — параллельная маршрутизация + Guardian check + apply_feedback
  ├── AdaptiveTickRate — Variable Tick Rate (min_hz=60, max_hz=1000, cooldown=50)
  ├── domain_name(id: u16) — pub fn, экспортируется без feature-gate
  ├── Over-Domain Layer (over_domain/): OverDomainComponent, Weaver traits; FrameWeaver V1.1
  │   BondTokens + ReinforceFrame + InjectFrameAnchor + UnfoldFrame handlers в engine.rs
  │   restore_frame_from_anchor (pub fn, over_domain::weavers::frame)
  └── Broadcast types (--features adapters): BroadcastSnapshot, DomainSummary,
      DomainDetailSnapshot, TokenSnapshot, ConnectionSnapshot; snapshot_for_broadcast(),
      domain_detail_snapshot(), trace_count(), tension_count(), last_matched()

axiom-config (Config V1.0 + D-07 + Anchor V1.0 + DreamConfig):
  ├── PresetsConfig.heartbeat_file / dream_file → LoadedAxiomConfig.heartbeat / dream (Option<…>)
  ├── DreamConfig: SchedulerConfig + FatigueWeightsConfig + CycleConfig; default/dev/production/validate()
  ├── ConfigWatcher — поллится в tick_loop каждый тик (EA-TD-05 ✅)
  ├── schema — JsonSchema на всех конфигах включая DreamConfig, validate_yaml<T>(), :schema CLI-команда
  └── AnchorSet — якорные токены: axes/layers/domains, YAML-загрузка, match_text(), compute_position/shell/weight

axiom-persist (D-04):
  ├── save/load: Token+Connection+ExperienceTrace → bincode (атомарный rename)
  ├── MemoryManifest (YAML), IMPORT_WEIGHT_FACTOR=0.7
  ├── AutoSaver: интервальное автосохранение, force_save при :quit
  └── exchange: export/import traces+skills (bincode), GUARDIAN-валидация

axiom-space:
  └── apply_gravity_batch — batch-физика, авто-векторизация (feature "simd")

Workstation V1.0 ✅ (2026-05-05):
  axiom-protocol — типы Engine ↔ Workstation: EngineCommand(14), EngineEvent(14),
    EngineMessage/ClientMessage (handshake), SystemSnapshot, ConfigSchema, BenchSpec;
    postcard сериализация; PROTOCOL_VERSION = 0x01_00_00_00
  axiom-broadcasting — WebSocket-сервер (tokio-tungstenite 0.24): BroadcastServer/Handle,
    subscription filter (event_category bits + tick_event_interval), heartbeat ping/pong,
    build_system_snapshot(); BRD-TD-07 (Engine integration) → axiom-node
  axiom-workstation — iced 0.13 desktop-клиент оператора:
    ├── connection.rs — ws_subscription + reconnect backoff [1,2,5,10,30]s
    ├── settings.rs — UiSettings, TOML-персистенция (dirs)
    ├── app.rs — WorkstationApp, AppPhase (Welcome/Main), 8 табов, bidirectional WS,
    │             keyboard shortcuts Ctrl+1–8/,/S/Z, alert overlay, subscription_key hot-reload
    └── ui/ — header, tabs, welcome, system_map(canvas), config(schema-driven), conversation,
              patterns(sparklines L1-L8), dream_state, files, benchmarks, live_field(3D canvas)
```

**Документация:** [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## Crates

| Crate | Тесты | Описание |
|-------|-------|----------|
| axiom-core | 34 | Token, Connection, Event |
| axiom-genome | 26 | Genome V1.0: конституция, GenomeIndex, from_yaml |
| axiom-frontier | 32 | CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection, budget |
| axiom-config | 91 | DomainConfig, ConfigLoader, YAML presets, ConfigWatcher, HeartbeatConfig, DreamConfig, JsonSchema, AnchorSet |
| axiom-space | 110 | SpatialHashGrid, физика, apply_gravity_batch (SIMD-ready, feature "simd") |
| axiom-shell | 48 | Shell V3.0, семантические профили, from_yaml |
| axiom-arbiter | 139 | Arbiter V1.0, Experience, REFLECTOR, SKILLSET, GridHash, AshtiProcessor, COM |
| axiom-heartbeat | 15 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-ucl | 9 | UCL commands |
| axiom-domain | 117 | Domain, DomainState, AshtiCore, CausalHorizon, FractalChain |
| axiom-runtime | 280 | AxiomEngine, Guardian, Over-Domain Layer (OverDomainComponent, Weaver, FrameWeaver V1.2 ✅), DREAM Phase V1.0 (DreamScheduler, FatigueTracker, DreamCycle, DreamProposal), Gateway (with_config, check_config_reload), Channel, EventBus, Adapters, TickSchedule, ProcessingResult, AdaptiveTickRate, Orchestrator, inject_anchor_tokens, domain_name; BroadcastSnapshot (feature "adapters"); FrameWeaverStats; restore_frame_from_anchor; UnfoldFrame handler |
| axiom-agent | 152 (175 all-features) | TextPerceptor (anchor-aware), MessageEffector, CliChannel + CLI Extended V1.0 + Anchor commands, MLEngine; tick_loop (CliState, adaptive sleep, ConfigWatcher), AdapterCommand, ServerMessage; External Adapters Phase 0–5; Telegram (feature), OpenSearch (feature) |
| axiom-persist | 35 | MemoryWriter, MemoryLoader, MemoryManifest, AutoSaver, exchange (bincode) |
| axiom-protocol | 41 | EngineCommand/Event/Message, SystemSnapshot, ConfigSchema, BenchSpec, AdapterInfo; postcard round-trip |
| axiom-broadcasting | 6 | BroadcastServer, BroadcastHandle, subscription filter, heartbeat, build_system_snapshot |
| axiom-workstation | 39 | WorkstationApp (iced 0.13 daemon), 8 вкладок, bidirectional WS, Welcome/Main, alert overlay, keyboard shortcuts |
| axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| tools/axiom-dashboard | — | egui/eframe Desktop GUI — Status, Space View, Domain List, Input panels |
| **Итого** | **1174** | |

---

## Этапы

| Этап | Название | Статус |
|------|----------|--------|
| 1 | GENOME + GUARDIAN | ✅ |
| 2 | Storm Control | ✅ |
| 3 | Configuration System | ✅ |
| 4 | REFLECTOR + SKILLSET | ✅ |
| 5 | GridHash-индекс | ✅ |
| 6 | Адаптивные пороги | ✅ |
| 7 | Causal Horizon + Масштабирование | ✅ |
| 8 | External Integration Layer | ✅ |
| 9 | Tech Debt + EventBus + Config hot reload | ✅ |
| 10 | Agent Layer (CLI/Telegram/Shell) | ✅ |
| 11 | ML Inference | ✅ |
| 12 | FractalChain + SIMD-физика | ✅ |
| 13A | Cognitive Depth — Multi-pass + TensionTrace | ✅ |
| 13B | Cognitive Depth — Heartbeat Internal Drive | ✅ |
| 13C | Cognitive Depth — InternalImpulse + Dominance | ✅ |
| 13D | Cognitive Depth — Goal Persistence + Curiosity | ✅ |
| Cleanup | COM V1.1 — unwrap, Unknown, Event fields, COM, constants, TickSchedule | ✅ |
| Memory | Memory Persistence V1.0 — save/load/autosave/exchange (axiom-persist) | ✅ |
| CLI V1.1 | CLI Channel V1.1 — TextPerceptor, MessageEffector, process_and_observe, axiom-cli.yaml | ✅ |
| Sentinel | Axiom Sentinel V1.0 — Hardware Topology, Parallel Resonance Search, Variable Tick Rate | ✅ |
| CLI Ext | CLI Extended V1.0 (Phase 1-3) — 13 новых команд, detail levels, multipass tracker | ✅ |
| Config | Config V1.0 — HeartbeatConfig load, ConfigWatcher→tick_loop, hot_reload | ✅ |
| D-04 | axiom-persist: bincode вместо serde_json (3-5× меньше, 2-4× быстрее) | ✅ |
| D-07 | JSON-schema валидация конфигов — schemars + jsonschema, :schema CLI-команда | ✅ |
| Anchor | Anchor Tokens V1.0 (Phase 1-3) — AnchorSet, YAML, TextPerceptor, inject_anchor_tokens в SUTRA+домены, :anchors/:match | ✅ |
| Adapters 0A | BroadcastSnapshot + convenience methods (axiom-runtime --features adapters) | ✅ |
| Adapters 0B | Рефактор handle_meta_command → handle_meta_read / handle_meta_mutate | ✅ |
| Adapters 0C | tick_loop, AdapterCommand, ServerMessage, AdaptersConfig; CLI → тонкая обёртка | ✅ |
| Adapters 1 | WebSocket адаптер — axum 0.8, /ws, подписки, --server / --port | ✅ |
| Adapters 2 | REST адаптер — axum Router, 5 handlers, correlation broadcast+timeout | ✅ |
| Adapters 3 | egui Dashboard — tools/axiom-dashboard, sync WS client, 4 панели | ✅ |
| Adapters 4 | Telegram адаптер — long-poll, route_message, pending корреляция | ✅ |
| Adapters 5 | OpenSearch адаптер — Result+Tick indexing, fire-and-forget POST | ✅ |
| Tech Debt | EA-TD-01..06: domain_name, CliState, adaptive tick, ConfigWatcher, DetailLevel | ✅ |
| EA-TD-02 | TokenSnapshot::shell — точный compute_shell через SemanticContributionTable | ✅ |
| FrameWeaver 1–3 | Over-Domain Layer traits + FrameWeaver V1.1 (scan→EXPERIENCE, ReinforceFrame, CycleStrategy::Allow) | ✅ |
| FrameWeaver 4 | Интеграция в AxiomEngine (on_tick + drain_commands), BroadcastSnapshot + FrameWeaverStats, GENOME permissions | ✅ |
| FrameWeaver 5 | 26 unit-тестов: fnv1a, scan, crystallization, reactivation, promotion, stats | ✅ |
| FW Stabilization | E1: restore_frame_from_anchor + UnfoldFrame handler + реальная промоция; E2: tick в Weaver trait; E3: drain_commands оптимизация 311→238 ns; E4 deferred. | ✅ |
| FrameWeaver V1.2 | Промоция перенесена из on_tick (steps 4–5) → dream_propose(); вызов при FallingAsleep; Errata E2–E4 зафиксированы | ✅ |
| DREAM Phase 1–5 | DreamScheduler + FatigueTracker + DreamCycle + DreamProposal + GUARDIAN check_frame_anchor_sutra_write; unit-тесты | ✅ |
| DREAM Phase 6 | CLI :dream-stats / :force-sleep / :wake-up; BroadcastSnapshot расширен; dream_cli_tests (5 тестов) | ✅ |
| DREAM Phase 7 | Smoke-тест 8 тестов: full_cycle, multiple_cycles, interrupted_cycle, scheduler_stats, promotions | ✅ |
| DreamConfig | axiom-config: SchedulerConfig+FatigueWeightsConfig+CycleConfig; apply_dream_config() в engine; Gateway::with_config(); hot-reload; dream.yaml; :schema dream | ✅ |
| WS Stage 0–1 | axiom-protocol (41 тест) + axiom-broadcasting scaffold; postcard сериализация | ✅ |
| WS Stage 2 | axiom-broadcasting: BroadcastServer/Handle, filter, heartbeat, 6 тестов | ✅ |
| WS Stage 3 | axiom-workstation базовая инфраструктура: settings, connection, reconnect backoff, 3 теста | ✅ |
| WS Stage 4 | Multi-window (iced::daemon), tabs, System Map canvas (мандала + анимация) | ✅ |
| WS Stage 5 | Configuration tab: schema-driven UI, bidirectional WS, workstation-секция | ✅ |
| WS Stage 6 | Conversation tab: лента, domain selector, Submit, корреляция с Frame-событиями | ✅ |
| WS Stage 7 | Patterns tab (sparklines L1-L8) + Dream State tab (force sleep / wake up) | ✅ |
| WS Stage 8 | Files tab (import flow) + Benchmarks tab (progress + history) | ✅ |
| WS Stage 9 | Welcome/Main фазы, alert overlay, keyboard shortcuts Ctrl+1–8, hot-reload адреса | ✅ |
| WS Stage 10 | Live Field 3D canvas: орбитальная камера, перспективное проецирование, процедурные токены | ✅ |
| WS Stage 11 | clippy --workspace -D warnings → 0 errors; 1174 тестов; fmt; README + errata | ✅ |
