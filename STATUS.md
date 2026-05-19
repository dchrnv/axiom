# AXIOM Status

**Обновлено:** 2026-05-19
**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

---

## Текущее состояние

**1349 тестов, 0 failures**

```
AxiomEngine
  ├── Genome (конституция, from_yaml, GenomeIndex O(1))
  ├── AshtiCore — 11 доменов (SUTRA=level*100 .. MAYA=level*100+10)
  │     ├── Arbiter (dual-path routing + Experience + Reflector + SkillSet + Internal Drive)
  │     ├── 11 × Domain (физика поля + CausalFrontier V2.0)
  │     └── 11 × DomainState (токены + связи)
  ├── Guardian (CODEX + GENOME: enforce_access, validate_reflex, ML filters)
  └── Over-Domain Layer:
        ├── OverDomainComponent trait (object-safe, on_tick → Result<Vec<UclCommand>, OverDomainError>)
        ├── Weaver trait (type Pattern, scan, propose_to_dream, check_promotion(tick))
        ├── FrameWeaver V1.3 ✅ — scan MAYA (0x08 Syntactic) → кристаллизация EXPERIENCE (109)
        ├── AxialEvaluator V1.0 ✅ (tick=5, ModuleId=17) — Frame по осям X/Y/Z; 8 уровней; Corpus Callosum
        ├── ContextRecognizer V1.0 ✅ (tick=7, ModuleId=18) — SubsystemEnergy, InterpretationProfile, SutraDepthStore
        ├── NeuralAdvisor V1.0 ✅ (tick=11, ModuleId=19) — advisory-only; RuleBasedCorpusCallosumResolver,
        │     DepthThresholdEmergentDetector; on_tick → NotifyEmergentCandidate (UCL 5200);
        │     ReactivationDepthAdvisor + SubsystemAffinityDepthAdvisor + AgeDecayAdvisor (depth.rs);
        │     implements AdvisorySource → poll_advisories() → Vec<Advisory>
        └── OverDomainArbiter V1.0 ✅ (tick=13, ModuleId=20) — координатор advisory-источников;
              TrustConfig (Ignore/AutoApply/RequireConfirmation × min_confidence);
              AutoApply DepthHint при Control в геноме; PendingQueue → Workstation;
              ArbiterLog (ring buffer 500); on_boot читает ExperienceMemory/Control из генома
              scan_state (confidence из avg connection.strength), build_crystallization_commands,
              ReinforceFrame (lineage_hash dedup), build_promotion_commands (→ SUTRA STATE_LOCKED),
              CycleStrategy::Allow (default); restore_frame_from_anchor; UnfoldFrame handler;
              встроен в AxiomEngine (on_tick + drain_commands); FrameWeaverStats: unfold_requests;
              GENOME: on_boot enforcement (check_access для MAYA/Read, EXPERIENCE/ReadWrite, SUTRA/Control);
              RuleTrigger: StabilityReached, HighConfidence(f32), DreamCycle, RepeatedAssembly{window_ticks};
              min_participant_anchors cross-domain check; check_promotion(tick) — корректный min_age_ticks;
              V1.2: промоция → dream_propose(); V1.3: все RuleTrigger реализованы, GENOME enforcement

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
  ├── MLEngine (mock + ONNX) → VisionPerceptor (explicit ShapeMismatch при input_size=0),
  │   AudioPerceptor (VAD)
  ├── CLI Channel: stdin/stdout loop, axiom-cli.yaml, все :команды
  │   CLI Extended V1.0: :domain/:events/:frontier/:guardian/:watch/:config/:trace/:connections
  │   :dream/:multipass/:reflector/:impulses/:schema/:anchors/:match/:help/:perf/:tickrate
  │   Горячая перезагрузка config/axiom.yaml (--hot-reload) через ConfigWatcher → tick_loop
  │   domain config hot-reload: apply_domain_config() при watcher.poll()
  └── External Adapters (Phase 0–5 + Tech Debt Closure):
      ├── tick_loop — единственный writer AxiomEngine; CliState (PerfTracker, event_log,
      │              watch_fields, multipass_count); адаптивный sleep (EA-TD-03/04 ✅)
      │              Workstation commands: handle_wstation_command + RunBench с BenchProgress events
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
  │   route_token_limited (S5): routing через роли 1–N вместо 1–8
  ├── AdaptiveTickRate — Variable Tick Rate (min_hz=60, max_hz=1000, cooldown=50)
  ├── domain_name(id: u16) — pub fn, экспортируется без feature-gate
  ├── Axiom Sentinel V1.1 ✅ (2026-05-12):
  │   S0: thread_pool → Arc<rayon::ThreadPool> в global OnceLock; AxiomEngine::new < 800 µs
  │   S1: inject_token_direct — bypass UCL-парсинга; ~20 ns vs ~35 ns для сенсорных данных
  │   S2: Experience::set_max_traces / should_trigger_export (×5000) / estimate_memory_bytes;
  │       TickSchedule::memory_pressure_threshold_bytes (1.8 GiB) → немедленный horizon GC
  │   S3: apply_gravity_batch_chunked + L2_CHUNK_TOKENS=65536 (512 KB / 8 B per token)
  │   S4: .cargo/config.toml target-cpu=native → авто-векторизация AVX2 в release/bench
  │   S4b: apply_gravity_batch_avx2 — явные AVX2 intrinsics (VSQRTPS+VDIVPS), 8 tok/iter;
  │        6.74 ms @ 1M токенов (цель 8–10 ms ✅); early exit shift≥16; scalar fallback
  │   S5: TickBudget (tick_budget_start / budget_used_fraction); enable_layer_priority gate;
  │       при budget>80% роли 4–8 пропускаются (process_parallel_limited / route_token_limited)
  │   S6: prepare_speculative_grids(pool) — параллельная pre-build SpatialHashGrid для reconcile_all;
  │       speculative_grids[11] + hits/misses счётчики; ~9 µs swap vs ~40 µs rebuild при hit ✅
  ├── TickSchedule: enable_layer_priority, target_tick_ns, memory_pressure_threshold_bytes
  ├── Over-Domain Layer (over_domain/): OverDomainComponent, Weaver traits; FrameWeaver V1.3
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
  ├── apply_gravity_batch — scalar, детерминировано точный (feature "simd")
  ├── apply_gravity_batch_avx2 — AVX2 f32, Linear, 8 tok/iter; 6.74 ms@1M (S4b ✅)
  └── apply_gravity_batch_chunked + L2_CHUNK_TOKENS — L2-cache-friendly batch для N>1M (S3)

Workstation V1.0 ✅ (2026-05-05):
  axiom-protocol — типы Engine ↔ Workstation: EngineCommand(16 incl. RunBench, ApproveEmergentCandidate), EngineEvent(14),
    EngineMessage/ClientMessage (handshake), SystemSnapshot, ConfigSchema, BenchSpec;
    TokenFieldPoint + token_field: Vec<TokenFieldPoint> в DomainSnapshot;
    FrameWeaverStats: syntactic_layer_activations [u8; 8];
    PhaseCSnapshot (dominant_octant/subsystem, emergent_candidates) + EmergentCandidateSnapshot в SystemSnapshot;
    postcard сериализация; PROTOCOL_VERSION = 0x01_00_00_00
  axiom-broadcasting — WebSocket-сервер (tokio-tungstenite 0.24): BroadcastServer/Handle,
    subscription filter (event_category bits + tick_event_interval + domain_activity_threshold),
    heartbeat ping/pong, snapshot resync при RecvError::Lagged,
    build_system_snapshot(); BRD-TD-07 (Engine integration) → axiom-node
  axiom-workstation — iced 0.13 desktop-клиент оператора:
    ├── connection.rs — ws_subscription + reconnect backoff [1,2,5,10,30]s
    ├── settings.rs — UiSettings, TOML-персистенция (dirs)
    ├── app.rs — WorkstationApp, AppPhase (Welcome/Main), 8 табов, bidirectional WS,
    │             keyboard shortcuts Ctrl+1–8/,/S/Z, alert overlay, subscription_key hot-reload;
    │             MenuBar (кастомный dropdown через stack), DetachTab (View → Detach),
    │             RunBench подключён к протоколу (BenchStarted/Progress/Finished events)
    └── ui/ — header, tabs, welcome (fade-in анимация), system_map(canvas::Cache),
              config(schema-driven), conversation(multi-line text_editor, Ctrl+Enter),
              patterns(sparklines L1-L8, Phase C panel: октант+подсистема+emergent candidates, show-more pagination),
              dream_state(show-more pagination), files(rfd AsyncFileDialog Browse button),
              benchmarks, live_field(3D canvas, реальный token_field из DomainSnapshot)
```

**Документация:** [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## Crates

| Crate | Тесты | Описание |
|-------|-------|----------|
| axiom-core | 34 | Token, Connection, Event |
| axiom-genome | 26 | Genome V1.0: конституция, GenomeIndex, from_yaml; ModuleId=20 (OverDomainArbiter), MAX_MODULES=21 |
| axiom-frontier | 32 | CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection, budget |
| axiom-config | 92 | DomainConfig, ConfigLoader, YAML presets, ConfigWatcher, HeartbeatConfig, DreamConfig, JsonSchema, AnchorSet |
| axiom-space | 119 | SpatialHashGrid, физика, apply_gravity_batch, apply_gravity_batch_avx2 (AVX2, feature "simd", S4b) |
| axiom-shell | 48 | Shell V3.0, семантические профили, from_yaml |
| axiom-arbiter | 139 | Arbiter V1.0, Experience, REFLECTOR, SKILLSET, GridHash, AshtiProcessor, COM |
| axiom-heartbeat | 15 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-ucl | 9 | UCL commands |
| axiom-domain | 126 | Domain, DomainState, AshtiCore, CausalHorizon, FractalChain, Speculative Layer (S6) |
| axiom-experience | 28 | AxialStore, SutraDepthStore, InterpretationProfileStore, EmergentPrimitiveStore; Octant (8), SubsystemId, EvaluationLevel |
| axiom-runtime | 397 (features adapters) | AxiomEngine, Guardian, Over-Domain Layer (OverDomainComponent, Weaver, FrameWeaver V1.3, AxialEvaluator V1.0, ContextRecognizer V1.0, NeuralAdvisor V1.0, OverDomainArbiter V1.0), DREAM Phase V1.0, Gateway, Channel, EventBus, Adapters, TickSchedule, ProcessingResult, AdaptiveTickRate, Orchestrator, inject_anchor_tokens, domain_name, apply_domain_config; BroadcastSnapshot (feature "adapters"); FrameWeaverStats; restore_frame_from_anchor; UnfoldFrame handler |
| axiom-agent | 133 (156 telegram,opensearch) | TextPerceptor (anchor-aware), MessageEffector, CliChannel + CLI Extended V1.0 + Anchor commands, MLEngine (explicit ShapeMismatch); tick_loop (CliState, adaptive sleep, ConfigWatcher, domain hot-reload, RunBench), AdapterCommand, ServerMessage; External Adapters Phase 0–5; Telegram (feature), OpenSearch (feature) |
| axiom-persist | 35 | MemoryWriter, MemoryLoader, MemoryManifest, AutoSaver, exchange (bincode) |
| axiom-protocol | 41 | EngineCommand(15)/Event/Message, SystemSnapshot+TokenFieldPoint, ConfigSchema, BenchSpec, AdapterInfo, FrameWeaverStats(syntactic_layer_activations); postcard round-trip |
| axiom-broadcasting | 6 | BroadcastServer, BroadcastHandle, subscription filter (domain_activity_threshold), heartbeat, snapshot resync при Lagged, build_system_snapshot |
| axiom-workstation | 39 | WorkstationApp (iced 0.13 daemon), 8 вкладок, bidirectional WS, Welcome/Main (fade-in), alert overlay, keyboard shortcuts, MenuBar, rfd file picker, multi-line editor, canvas::Cache |
| axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| tools/axiom-dashboard | 6 | egui/eframe Desktop GUI — Status, Space View, Domain List, Input panels |
| **Итого** | **1349** | |

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
| WS B1–B6 | UI-доделки: rfd file picker (B1), multi-line text_editor Ctrl+Enter (B2), show-more pagination Patterns+Dream (B3), canvas::Cache system_map (B4), welcome fade-in (B5), MenuBar + DetachTab (B6) | ✅ |
| Protocol C1–C3 | syntactic_layer_activations [u8;8] в FrameWeaverStats (C1); RunBench в протоколе + tick_loop (C2); TokenFieldPoint + token_field в DomainSnapshot + Live Field real data (C3) | ✅ |
| Engine D1–D6 | tick в check_promotion (D1); min_participant_anchors cross-domain (D2); все RuleTrigger (D3); GENOME on_boot enforcement (D4); domain config hot-reload apply_domain_config (D5); domain_activity_threshold + Lagged resync (D6) | ✅ |
| E2 | MLEngine size check — явная ShapeMismatch вместо silent fallback (D-06 закрыт) | ✅ |
| Phase C1 | axiom-experience: AxialStore, SutraDepthStore, InterpretationProfileStore, EmergentPrimitiveStore; Octant×8 | ✅ |
| Phase C2 | AnchorSet: subsystem architecture, writing/mathematics primitives, FlatAnchorFile YAML | ✅ |
| Phase C3 | AxialEvaluator V1.0 (ModuleId=17, tick=5): X/Y/Z axes, 8 EvaluationLevels, Corpus Callosum conflict | ✅ |
| Phase C4 | ContextRecognizer V1.0 (ModuleId=18, tick=7): ScanningPlan, SubsystemEnergy, InterpretationProfile | ✅ |
| Phase C5 | NeuralAdvisor V1.0 (ModuleId=19, tick=11): advisory-only, 5 трейтов, RuleBasedCorpusCallosumResolver, DepthThresholdEmergentDetector; on_tick → Vec<UclCommand> | ✅ |
| Phase I1 | Engine coordinator: axial_evaluator/context_recognizer/neural_advisor — конкретные поля AxiomEngine, tick % 5/7/11, snapshot sync AE→CR→NA; opcode_from_u16 расширен; 9 тестов | ✅ |
| Phase I4 | ApproveEmergentCandidate (UCL 5201) handler в Engine → neural_advisor.approve_emergent(sutra_id) | ✅ |
| Phase I2 | ContextRecognizer::from_anchor_set(AnchorSet): build_subsystem_refs по именам подсистем; AxiomEngine::apply_anchor_set; axiom-node/startup вызывает при старте; 3 теста | ✅ |
| Phase I3 | Якорный контент: config/anchors/writing/primitives.yaml (7 графических примитивов) + config/anchors/mathematics/primitives.yaml (7 структурных примитивов); ContextRecognizer подхватывает через get_subsystem(); integration test в anchor.rs | ✅ |
| Phase I6 | Workstation Phase C visibility: PhaseCSnapshot в SystemSnapshot (dominant_octant/subsystem, emergent_candidates); ApproveEmergentCandidate в EngineCommand + axiom-node handler; Patterns tab — Phase C panel (октант+подсистема с цветом, emergent candidates с кнопкой Approve) | ✅ |
| Phase I7 | OverDomainArbiter V1.0 (ModuleId=20, tick=13): AdvisorySource трейт, TrustConfig, PendingQueue, ArbiterLog; NeuralAdvisor реализует AdvisorySource; on_boot в try_new; PhaseCSnapshot расширен (octant_depth_avg, pending_advisories); Workstation: octant depth panel + arbiter queue panel; три DepthHint советника: ReactivationDepth, SubsystemAffinity, AgeDecay(DEPTH_FLOOR=50) | ✅ |
