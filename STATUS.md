# AXIOM Status

**Обновлено:** 2026-05-31
**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

---

## Текущее состояние

**1667 тестов, 0 failures**

Primitive_Nature_and_Connections_V1_0 ✅ (2026-05-30): spatial/causal L0 переведены из якорей-призраков
в Connection link_type определения (config/schema/link_types/); 0x09 Spatial добавлен в
semantic_contributions.yaml; perceptual_anchors() = 8 (только visual); primitives_nature.yaml создан.

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
        ├── FrameWeaver V1.3 ✅ — scan MAYA (0x08 Syntactic) → кристаллизация EXPERIENCE (109);
        │     FrameCandidate.shell_similarity: f32 — средн. косинусное сходство shell участников;
        │     avg_candidate_shell_similarity() → f32 — диагностика для OBS-снимков
        ├── AxialEvaluator V3.0 ✅ (tick=5, ModuleId=17) — Frame по осям X/Y/Z; 8 уровней; Corpus Callosum;
        │     V2: OctantStabilityTracker (ring 10, threshold 70%, min 5), ConflictPersistenceTracker (streak≥5);
        │     subsystem-aware level selection (subsystem_to_level); drain_pending_advisories() → Vec<Advisory>;
        │     AXIAL_EVALUATOR_SOURCE_ID=1; TrustConfig: OctantCorrection(0.70)/ConflictDiagnosis(0.60);
        │     V3: NarrativeOctantTracker (применяет advisory override); adaptive stability threshold;
        │     AxialStore::override_octant(sutra_id, octant) — advisory-override flag, AE уважает на следующем тике
        ├── ContextRecognizer V6.0 ✅ (tick=7, ModuleId=18) — SubsystemEnergy, InterpretationProfile, SutraDepthStore;
        │     V6 A: ActivityTrace (short=16/mid=64/long=256 ring-буферы), ActivityDynamics (entropy_gradient,
        │           oscillation_score, cascade_score, dominant_persistence), ActivitySignature classifier,
        │           ActivityAnalyzer (переименован из TransitionDetector);
        │     V6 B: SubsystemFatigue { activation_load, recovery_debt }, FatigueStore (V7-B2 → axiom-experience);
        │           effective_weight = base*(1-0.5*min(1,load/MAX)); DREAM: activation_load *= 0.35;
        │     TransitionMatrix ✅ (V7-B1) — [[f32; 16]; 16] матрица переходов; record(from, to) при смене
        │           доминанты; decay(0.995) на каждом тике; probability_of(from, to), most_likely_next(from);
        │           Unknown игнорируется; 7 unit-тестов; 3 CR-интеграционных теста
        │     directed_cascade_score ✅ (V7-C1) — ActivityDynamics.directed_cascade_score: f32;
        │           ActivityTrace::directed_cascade_score(matrix, threshold=0.20) → цепочка A→B→C
        │           где prob(A→B)≥T; classify() предпочитает если >0 (fallback на cascade_score); 5 тестов
        │     CompositeSubsystemProfile ✅ (V7-C2) — полный профиль с BidirectionalCoupling;
        │           detect_composite_profiles(recent, sigs, matrix, bi_threshold=0.15);
        │           composite_profiles() accessor в CR; V6 composite_suspects сохранён; 6 тестов
        │     SubsystemVersionStore ✅ (V7-D1) — version в FlatAnchorFile + AnchorSet.subsystem_versions;
        │           init()/check_migration()/drain_stale(); from_anchor_set инициализирует; 8 тестов
        │     SplitMergeDetector ✅ (V7-D2) — Split(load≥0.6·MAX + entropy≥1.5) / Merge(bidirectional≥0.25);
        │           split_merge_candidates() accessor; 9 unit-тестов; on_tick после fatigue.update()
        │     compute_raw_energies(&AshtiCore) → HashMap<SubsystemId, u8> — снимок энергий для OBS
        │     ActivityDynamics fix ✅ (2026-05-30) — CR on_tick: N=1 most-recent MAYA token (by last_event_id)
        │           вместо cumulative compute_energies; dominant_subsystem_confident (threshold 5e-9);
        │           AshtiCore::sleep_oldest_active_token(domain_id) — eviction при переполнении MAYA;
        │           E1-fix: valence=1 + retry on CapacityExceeded → динамика жива весь прогон
        │           (corpus_mixed 60K тиков: Cascade=1.00, Fill=16, Fatigue=4, Signature=Cascading)
        │     Morality detection ✅ (2026-05-30) — SUBSYSTEM_NAMES += "morality"; moral_ prefix в
        │           subsystem_from_anchor_id; word_signals для 7 moral_* якорей (moral_care/harm/fair/
        │           betrayal/loyalty/purity/desecration); SubsystemId::Morality в build_subsystem_refs;
        │           corpus_mixed: config/obs/corpus_mixed.yaml (диагностический корпус 15 текстов,
        │           типы A/Б/В, inject_every=20, stagger=5 тиков/шард)
        │     FrameCompositionStore ✅ (V7-A1) — иерархия Frame-композиций; detect_composed_of() — участники
        │       совпадающие с Frame-анкерами EXPERIENCE = родители; COMPOSITION_BOND (0x0901) в UCL;
        │       composition_level(anchor_id) → FrameComposition (C1Atom..C5Plus);
        │       FrameCandidate.composed_of: Vec<u32> — заполняется перед кристаллизацией; 10 новых тестов
        │     DilemmaStore V1.1 ✅ — хранит дилеммы типов III/IV/V (не I/II); max 8 active, ring-64 resolved;
        │       pending_crystallizations → drain → crystallize_to_experience_commands() → UCL (InjectToken+BondTokens);
        │       кристаллизация в EXPERIENCE domain (level*100+9); lineage_hash FNV-1a; resolution_valence;
        │       DilemmaType: DataConflict/ResourceTradeoff/ValueConflict/OntologicalConflict/Axiogenic
        ├── NeuralAdvisor V3.0 ✅ (tick=11, ModuleId=19) — все 5 слотов заполнены;
        │     depth: ReactivationDepthAdvisor; octant: DepthHistoryBiasAdvisor (DHB_MIN_DEPTH=800,
        │     DHB_MIN_ADVANTAGE=300); conflict: RuleBasedCorpusCallosumResolver (V2) / PatternLearningResolver (V3);
        │     subsystem: AnchorVotingAdvisor (AV_MIN_ENERGY=20, dominance≥0.50, dual-gap<0.15);
        │     emergent: DepthThresholdEmergentDetector; AdvisoryHistory (ring 32 per sutra_id);
        │     OctantAdvisorInput расширен: depth_per_octant[8] + reactivation_count;
        │     implements AdvisorySource → poll_advisories() → Vec<Advisory> с octant_hint;
        │     G1: DivergenceLog (ring 256) — расхождения advisory_octant ↔ analytic_octant (Hamming ≥ 2);
        │     G2: PatternLearningResolver — conflict slot, учится на AdvisoryHistory per-Frame;
        │     G3: NeuralAdvisorConfig — genome.yaml секция neural_advisor → per-advisor enable/disable
        └── OverDomainArbiter V3.0 ✅ (tick=13, ModuleId=20) — координатор advisory-источников;
              TrustConfig (Ignore/AutoApply/RequireConfirmation × min_confidence);
              V2: TrustConfig загружается из genome.yaml секции [arbiter.trust]; TTL ~1000 event_id
              (expires_at_event = created_at_event + 1000 → ArbiterOutcome::Expired + on_feedback);
              CognitiveProfile загружается из yaml (профили: balanced/analytic); AutoApply DepthHint при
              Control в геноме; PendingQueue → Workstation V2 (confirm/reject через HTTP + WS);
              ArbiterLog (ring buffer 500); on_boot читает ExperienceMemory/Control из генома;
              CognitiveProfile { octant_weights[8], init 1.0 }: scale_confidence(octant_idx, raw),
              update(idx, accepted) online learning rate=0.05; Advisory.octant_hint: Option<usize>
              scan_state (confidence из avg connection.strength), build_crystallization_commands,
              ReinforceFrame (lineage_hash dedup), build_promotion_commands (→ SUTRA STATE_LOCKED),
              CycleStrategy::Allow (default); restore_frame_from_anchor; UnfoldFrame handler;
              встроен в AxiomEngine (on_tick + drain_commands); FrameWeaverStats: unfold_requests;
              GENOME: on_boot enforcement (check_access для MAYA/Read, EXPERIENCE/ReadWrite, SUTRA/Control);
              RuleTrigger: StabilityReached, HighConfidence(f32), DreamCycle, RepeatedAssembly{window_ticks};
              min_participant_anchors cross-domain check; check_promotion(tick) — корректный min_age_ticks;
              V1.2: промоция → dream_propose(); V1.3: все RuleTrigger реализованы, GENOME enforcement;
              AxiomEngine: confirm_pending_advisory(advisory_id: u64), reject_pending_advisory(advisory_id: u64);
              V3: drain_octant_overrides() → pending octant overrides для AxialEvaluatorStorage;
              V3: feedback-буфер для незарегистрированных источников (AxialEvaluator source_id)

DREAM Phase V1.1 ✅ — когнитивный сон: 4 состояния (Wake/FallingAsleep/Dreaming/Waking)
  ├── DreamScheduler — 3 триггера: Idle (порог idle тиков), Fatigue (0-255, 4 фактора), ExplicitCommand
  ├── FatigueTracker — composite score = Σ(factor × weight) / Σ(weight); отслеживает 4 показателя
  ├── DreamCycle — 3 этапа: Stabilization → Processing → Consolidation; DreamProposal (Promotion/HeavyCrystallization)
  ├── GUARDIAN: check_frame_anchor_sutra_write() — FRAME_ANCHOR в SUTRA только в DREAMING
  ├── GatewayPriority: Normal (игнорируется в DREAMING) / Critical (пробуждение) / Emergency (V2.0=Critical)
  ├── Gateway::with_config() — старт с загрузкой DreamConfig из axiom.yaml
  ├── CLI: :dream-stats / :force-sleep / :wake-up
  ├── BroadcastSnapshot расширен: dream_phase, dream_stats (FatigueStats, SchedulerStats, CycleStats)
  └── H1/H2: SubsystemCandidate discovery — cluster_emergent_primitives() → SubsystemCandidateStore;
        SubsystemLifecycleState: Proposed→Candidate→InReview→Active→Mature→Deprecated→Archived;
        ApproveSubsystemCandidate (UCL 5301): approve_with_rules(id, genome.emergent_subsystems);
        V7-D4: EmergentSubsystemRules { min_primitives, min_evidence_strength, require_review, max_active_candidates };
        ApproveError: NotFound / InvalidTransition / InsufficientEvidence / TooManyCandidates; 6 тестов

FractalChain — N уровней AshtiCore (MAYA[n] → SUTRA[n+1], skills exchange)
ConfigWatcher — горячая перезагрузка axiom.yaml (inotify), передаётся в tick_loop
EventBus — pub/sub: типизированные и broadcast подписки
domain_name() — pub fn в axiom-runtime (EA-TD-01 ✅)

axiom-agent:
  ├── TextPerceptor — текст → UclCommand(InjectToken): 2-path detect_subsystem()
  │     Path1: AnchorSet.match_text() + dominant_subsystem_of(); Path2: AnchorMatchTable.dominant_subsystem()
  │     (word_signals weight=1.0 + char_signals weight×0.4 → subsystem_from_anchor_id prefix map)
  │     100% per-text subsystem accuracy (OBS-02, 8 корпусов × 30k тиков)
  ├── L0VisionPerceptor ✅ (V7-E2) — RGBA8 → grayscale → Sobel edge detection → stroke classification;
  │     EdgeAnalysis { edge_density, horizontal_fraction, vertical_fraction, diagonal_fraction };
  │     InjectToken в SUTRA(100) для каждого L0 примитива с density ≥ 0.02;
  │     Anchors: visual_edge / stroke_horizontal / stroke_vertical / stroke_diagonal; 10 тестов
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
  ├── AnchorSet — якорные токены: axes/layers/domains, YAML-загрузка, match_text(), compute_position/shell/weight;
  │     SUBSYSTEM_NAMES: [&str; 6], dominant_subsystem_of(matches) → Option<SubsystemId>
  ├── SubsystemDependencies ✅ — загрузчик §2.7 Variant C+ из config/subsystem_dependencies.yaml;
  │     SubsystemDep { builds_on, natural_tensions }, NaturalTension { target, reason };
  │     load_or_empty(config_dir) — graceful degradation; is_natural_tension(a,b) — симметрично;
  │     load_order() → topological sort (DFS), Err(String) при обнаружении цикла
  └── AnchorLayer ✅ (V7-A2) — L0/L1 флаг в Anchor; AnchorSet.perceptual: Vec<Anchor>;
        load_perceptual() из config/anchors/perceptual/ (graceful degradation);
        perceptual_anchors() accessor; L0 НЕ в match_text() (только VisionPerceptor);
        total_count() включает perceptual; 7 новых тестов

axiom-persist (D-04):
  ├── save/load: Token+Connection+ExperienceTrace → bincode (атомарный rename)
  ├── MemoryManifest (YAML), IMPORT_WEIGHT_FACTOR=0.7
  ├── AutoSaver: интервальное автосохранение, force_save при :quit
  ├── exchange: export/import traces+skills (bincode), GUARDIAN-валидация
  ├── ARB-TD-05: StoredTrustEntry в StoredEngineState; save→export_trust_calibration(); load→import_trust_calibration()
  │     TrustConfig: iter_entries() + set_min_confidence() (pub API для сериализации)
  └── ARB-TD-06: octant_weights: Option<[f32;8]> в StoredEngineState; save→cognitive_profile().octant_weights;
        load→CognitiveProfile::with_weights(weights) (с клампингом); 2 новых roundtrip-теста

axiom-space:
  ├── apply_gravity_batch — scalar, детерминировано точный (feature "simd")
  ├── apply_gravity_batch_avx2 — AVX2 f32, Linear, 8 tok/iter; 6.74 ms@1M (S4b ✅)
  └── apply_gravity_batch_chunked + L2_CHUNK_TOKENS — L2-cache-friendly batch для N>1M (S3)

axiom-node HTTP ✅ (2026-05-29):
  axum HTTP-сервер на :8080; маршруты:
    GET  /api/ws                    — WebSocket JSON bridge (snapshot при подключении + EngineEvent)
    POST /api/advisory/confirm/{id} — NodeCmd::AdvisoryConfirm → engine.confirm_pending_advisory()
    POST /api/advisory/reject/{id}  — NodeCmd::AdvisoryReject → engine.reject_pending_advisory()
    POST /api/text/submit           — NodeCmd::SubmitText → perceptor.perceive() → engine
    GET  /metrics                   — Prometheus text format (~30 метрик)
    POST /api/lab/run               — запустить lab job (obs/bench_*/test/showcase)
    POST /api/lab/stop              — остановить текущий job
    GET  /api/lab/status            — статус текущего job (JSON)
    GET  /api/lab/ws/log            — WebSocket stream stdout/stderr текущего job
    GET  *                          — ServeDir(web_dist) статика Workstation V2
  NodeCmd channel: unbounded mpsc HTTP→tick_loop; нет Mutex на AxiomEngine
  BroadcastHandle: subscribe_events() → Receiver<EngineMessage>; latest_snapshot() → Option<SystemSnapshot>;
    snapshot_live: RwLock<Option<SystemSnapshot>> — хранит живой снапшот для /metrics и WS bridge

Workstation V2.0 ✅ (2026-05-24):
  axiom-web — React 18 SPA + Zustand + Vite (tools/axiom-web/):
    8 табов: Overview / Domains / Traces / Internals / Conversation / Phase C / Patterns / Lab
  Advisory Queue: confirm/reject → POST /api/advisory/confirm|reject/{id}, TTL bar
  SVG sparklines (zero-dep, rolling 120 snapshots), domain activity grid
  Авто-переподключение WS каждые 2s; badge на Phase C tab при pending advisories
  Grafana + Prometheus (tools/grafana/docker-compose.yml): 3 provisioned дашборда, scrape 5s

Lab панель ✅ (2026-05-29):
  axiom-node/src/lab.rs: POST /api/lab/run|stop, GET /api/lab/status, GET /api/lab/ws/log
  Запуск OBS / Hot Bench / OverDomain Bench / Stress / Tests / Full Showcase из браузера
  Прогресс-бар OBS (парсинг [observe] N/M (%)), цветной лог, Results panel, история прогонов

Performance & Tooling Sprint ✅ (2026-05-29):
  Token lifecycle: check_decay → TokenDecayed → STATE_SLEEPING (valence=0); scan_region
    пропускает спящие токены; add_token вытесняет спящие при переполнении; eviction hook → Experience
  Parallel ticks: AshtiCore::tick() — sequential heartbeat + parallel process_frontier (rayon)
  Parallel OBS shards: N AxiomEngine на N потоках; corpus_large.yaml: shards=4
  OBS streaming: run_streaming() → snapshots.jsonl + events.jsonl (BufWriter, RAM flat)
  corpus_showcase.yaml: 18 текстов, 9 подсистем, 200K тиков, ~3-5 мин
  corpus_profile.yaml: 4 текста, 50K тиков — для cargo flamegraph профилирования
  INVARIANTS.md v11: правило о жизненном цикле токенов (не удаляются, → STATE_SLEEPING)

```

**Документация:** [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## Crates

| Crate | Тесты | Описание |
|-------|-------|----------|
| axiom-core | 34 | Token, Connection, Event |
| axiom-genome | 26 | Genome V1.0: конституция, GenomeIndex, from_yaml; ModuleId=20 (OverDomainArbiter), MAX_MODULES=21; EmergentSubsystemRules (V7-D4); CrossModalConfig (CMB-TD-02) |
| axiom-frontier | 32 | CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection, budget |
| axiom-config | 115 | DomainConfig, ConfigLoader, YAML presets, ConfigWatcher, HeartbeatConfig, DreamConfig, JsonSchema, AnchorSet; SubsystemDependencies; AnchorLayer L0/L1; perceptual_anchors() |
| axiom-space | 118 | SpatialHashGrid, физика, apply_gravity_batch, apply_gravity_batch_avx2 (AVX2, feature "simd", S4b) |
| axiom-shell | 48 | Shell V3.0, семантические профили, from_yaml; link_types: 0x08 Syntactic, 0x09 Composition, 0x0A CrossModal, 0x0B SemanticAnchor=0x0B01 (AE-TD-08) |
| axiom-arbiter | 139 | Arbiter V1.0, Experience, REFLECTOR, SKILLSET, GridHash, AshtiProcessor, COM |
| axiom-heartbeat | 15 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-ucl | 9 | UCL commands |
| axiom-domain | 126 | Domain, DomainState, AshtiCore, CausalHorizon, FractalChain, Speculative Layer (S6) |
| axiom-experience | 50 | AxialStore, SutraDepthStore, InterpretationProfileStore, EmergentPrimitiveStore, MetaStore; FatigueStore + SubsystemFatigue (V7-B2); ModalityStore + Modality (Text/Vision/Internal); Octant (8), SubsystemId (+Morality/Abstractions/Dilemmas), EvaluationLevel |
| axiom-runtime | 620 (640 features adapters) | AxiomEngine, Guardian, Over-Domain Layer (OverDomainComponent, Weaver, FrameWeaver V1.3, AxialEvaluator V3.0, ContextRecognizer V6.0+V7, NeuralAdvisor V3.0, OverDomainArbiter V3.0), DREAM Phase V1.1, Gateway, Channel, EventBus, Adapters, TickSchedule, ProcessingResult, AdaptiveTickRate, Orchestrator, inject_anchor_tokens, domain_name, apply_domain_config; BroadcastSnapshot (feature "adapters"); FrameWeaverStats; restore_frame_from_anchor; UnfoldFrame handler; AdvisoryHistory, CognitiveProfile; confirm/reject_pending_advisory; DivergenceLog, PatternLearningResolver, NeuralAdvisorConfig; SubsystemCandidateStore, SubsystemLifecycleState, ApproveError; drain_octant_overrides; DilemmaStore V1.1, crystallize_to_experience_commands; TransitionMatrix, FatigueStore→experience, directed_cascade_score, CompositeSubsystemProfile, SubsystemVersionStore, SplitMergeDetector; DilemmaDetector V2.1 (Signal A: coactivation+deps; Signal B: connection stress MEAN_STRESS≥0.35, MIN_STRESSED≥2); CrossModalDetector V1.0 (ModalityStore, CROSS_MODAL_BOND=0x0A01); Ethics composite [Values,Morality,Dilemmas]; MoralSignalDetector (7 moral_* anchors, intensity, antagonistic pairs); ActivityTrace serde; Shell-TD-01: stability_threshold baseline; FrameWeaver: 0x0B semantic anchor participants |
| axiom-agent | 148 (171 telegram,opensearch) | TextPerceptor (2-path detect_subsystem + perceive_and_bond→SEMANTIC_ANCHOR_BOND=0x0B01; text_stable_id 0x4000_0001+; anchor_sutra_id mirror); AnchorMatchTable: domain+layer якоря в id_to_position (P4b); L0VisionPerceptor (V7-E2); MessageEffector, CliChannel + CLI Extended V1.0 + Anchor commands; tick_loop (CliState, adaptive sleep, ConfigWatcher, domain hot-reload, RunBench), AdapterCommand, ServerMessage; External Adapters Phase 0–5; Telegram (feature), OpenSearch (feature) |
| axiom-persist | 37 | MemoryWriter, MemoryLoader, MemoryManifest, AutoSaver, exchange (bincode); ARB-TD-05 TrustConfig calibration roundtrip; ARB-TD-06 CognitiveProfile octant_weights roundtrip |
| axiom-protocol | 41 | EngineCommand(15)/Event/Message, SystemSnapshot+TokenFieldPoint, ConfigSchema, BenchSpec, AdapterInfo, FrameWeaverStats(syntactic_layer_activations); postcard round-trip; WS-5: +PerfSnapshot, TraceSnapshot, TensionTraceSnapshot, ReflectorSnapshot, CognitiveDepthSnapshot, ImpulsesSnapshot; SystemSnapshot: +perf/traces/tension/reflector/cognitive_depth/impulses/skills_count |
| axiom-broadcasting | 6 | BroadcastServer, BroadcastHandle, subscription filter (domain_activity_threshold), heartbeat, snapshot resync при Lagged, build_system_snapshot; subscribe_events() → Receiver<EngineMessage>; latest_snapshot() → Option<SystemSnapshot>; snapshot_live: RwLock; WS-5: build_system_snapshot takes PerfSnapshot, populates all new fields |
| axiom-node | — | HTTP-сервер (axum): WS JSON bridge, advisory confirm/reject, /metrics, ServeDir; NodeCmd channel; tick_loop интеграция; WS-5: NodePerfTracker (window=100) → PerfSnapshot per snapshot |
| tools/axiom-web | — | React 18 SPA: Overview/Conversation/Phase C/Patterns; AdvisoryQueue, Sparklines, Zustand store; WS-5: protocol.ts extended with PerfSnapshot/TraceSnapshot/TensionTraceSnapshot/ReflectorSnapshot/CognitiveDepthSnapshot/ImpulsesSnapshot |

| axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| axiom-corpus | 4 | Corpus loader: 8 текстовых корпусов для OBS-прогонов |
| tools/axiom-dashboard | 6 | egui/eframe Desktop GUI — Status, Space View, Domain List, Input panels |
| tools/axiom-tray | 6 | Системный трей (ksni): StatusNotifierItem, poll /metrics каждые 2s, Start/Stop axiom-node, Open Workstation |
| **Итого** | **1623** | |

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

| Protocol C1–C3 | syntactic_layer_activations [u8;8] в FrameWeaverStats (C1); RunBench в протоколе + tick_loop (C2); TokenFieldPoint + token_field в DomainSnapshot + Live Field real data (C3) | ✅ |
| Engine D1–D6 | tick в check_promotion (D1); min_participant_anchors cross-domain (D2); все RuleTrigger (D3); GENOME on_boot enforcement (D4); domain config hot-reload apply_domain_config (D5); domain_activity_threshold + Lagged resync (D6) | ✅ |
| E2 | MLEngine size check — явная ShapeMismatch вместо silent fallback (D-06 закрыт) | ✅ |
| Phase C1 | axiom-experience: AxialStore, SutraDepthStore, InterpretationProfileStore, EmergentPrimitiveStore; Octant×8 | ✅ |
| Phase C2 | AnchorSet: subsystem architecture, writing/mathematics primitives, FlatAnchorFile YAML | ✅ |
| Phase C3 | AxialEvaluator V1.0 (ModuleId=17, tick=5): X/Y/Z axes, 8 EvaluationLevels, Corpus Callosum conflict | ✅ |
| Phase C6 | AxialEvaluator V2.0: subsystem-aware level selection, OctantStabilityTracker, ConflictPersistenceTracker, drain_pending_advisories, AXIAL_EVALUATOR_SOURCE_ID=1; TrustConfig расширен source=1 | ✅ |
| Phase C4 | ContextRecognizer V1.0 (ModuleId=18, tick=7): ScanningPlan, SubsystemEnergy, InterpretationProfile | ✅ |
| Phase C5 | NeuralAdvisor V1.0 (ModuleId=19, tick=11): advisory-only, 5 трейтов, RuleBasedCorpusCallosumResolver, DepthThresholdEmergentDetector; on_tick → Vec<UclCommand> | ✅ |
| Phase I1 | Engine coordinator: axial_evaluator/context_recognizer/neural_advisor — конкретные поля AxiomEngine, tick % 5/7/11, snapshot sync AE→CR→NA; opcode_from_u16 расширен; 9 тестов | ✅ |
| Phase I4 | ApproveEmergentCandidate (UCL 5201) handler в Engine → neural_advisor.approve_emergent(sutra_id) | ✅ |
| Phase I2 | ContextRecognizer::from_anchor_set(AnchorSet): build_subsystem_refs по именам подсистем; AxiomEngine::apply_anchor_set; axiom-node/startup вызывает при старте; 3 теста | ✅ |
| Phase I3 | Якорный контент: config/anchors/writing/primitives.yaml (7 графических примитивов) + config/anchors/mathematics/primitives.yaml (7 структурных примитивов); ContextRecognizer подхватывает через get_subsystem(); integration test в anchor.rs | ✅ |
| Phase I6 | Workstation Phase C visibility: PhaseCSnapshot в SystemSnapshot (dominant_octant/subsystem, emergent_candidates); ApproveEmergentCandidate в EngineCommand + axiom-node handler; Patterns tab — Phase C panel (октант+подсистема с цветом, emergent candidates с кнопкой Approve) | ✅ |
| Phase I7 | OverDomainArbiter V1.0 (ModuleId=20, tick=13): AdvisorySource трейт, TrustConfig, PendingQueue, ArbiterLog; NeuralAdvisor реализует AdvisorySource; on_boot в try_new; PhaseCSnapshot расширен (octant_depth_avg, pending_advisories); Workstation: octant depth panel + arbiter queue panel; три DepthHint советника: ReactivationDepth, SubsystemAffinity, AgeDecay(DEPTH_FLOOR=50) | ✅ |
| CR-V6 Фаза 0 | SyntacticBridge: bridge_to_maya + domain_position_hash в orchestrator.rs; MAYA получает 8 0x08-связей на каждый routing; FrameWeaver кристаллизует Frame-анкеры; 2 integration-теста | ✅ |
| CR-V6 Фаза A | ActivityTrace (3 кольцевых буфера short=16/mid=64/long=256), ActivityDynamics (4 метрики), ActivitySignature classifier (6 сигнатур, приоритет Steady→Oscillating→Cascading→Converging→Diverging), ActivityAnalyzer (переименован из TransitionDetector); 15 unit-тестов | ✅ |
| CR-V6 Фаза B | SubsystemFatigue { activation_load, recovery_debt }, FatigueStore; decay=0.90/tick, equilibrium=10.0; DREAM: activation_load *= 0.35; apply_to_weights() снижает вес уставших подсистем; 12 unit-тестов + integration | ✅ |
| TextPerceptor 2-path | detect_subsystem(): Path1=AnchorSet.match_text()+dominant_subsystem_of(), Path2=AnchorMatchTable.dominant_subsystem(); word_signals+char_signals×0.4; subsystem_from_anchor_id prefix map; AnchorSet.SUBSYSTEM_NAMES + dominant_subsystem_of() | ✅ |
| OBS-02 | Автоматизированный прогон: 30k тиков, 8 корпусных текстов, 415 инъекций, 100% per-text accuracy (исправлен "каждый" в logic_quantifier). 312 emergent-кандидатов (все Frame). SutraDepthStore reactivation_count: мёртвое поле исправлено (инкремент при apply_evidence с evidence>0). Пороги DepthThresholdEmergentDetector: MIN_DEPTH 8000→1000, MIN_REACTIVATIONS 30→5 (откалибровано по O7 avg_depth=1198, ~10-15 DREAM-циклов за 30k тиков) | ✅ |
| OBS-infra | FrameCandidate.shell_similarity: f32; FrameWeaver.avg_candidate_shell_similarity(); ContextRecognizer.compute_raw_energies(); AxiomEngine.snapshot_subsystem_energies() — диагностическая инфраструктура для OBS-снимков | ✅ |
| NeuralAdvisor V2.0 | Все 5 слотов заполнены: DepthHistoryBiasAdvisor (octant), AnchorVotingAdvisor (subsystem); AdvisoryHistory ring-32; OctantAdvisorInput+depth_per_octant/reactivation_count; CognitiveProfile octant_weights[8] в Arbiter с online learning rate=0.05; Advisory.octant_hint: Option<usize>; engine → with_default_v2() | ✅ |
| AxialEvaluator V3.0 | NarrativeOctantTracker (advisory override), adaptive stability threshold, AxialStore::override_octant(sutra_id, octant) | ✅ |
| OverDomainArbiter V2.0 | TrustConfig from yaml (genome.yaml [arbiter.trust]); TTL 1000 (expires_at_event); CognitiveProfile from yaml (balanced/analytic); confirm/reject_pending_advisory в AxiomEngine | ✅ |
| WS-0 | axiom-node: axum HTTP + WS JSON bridge; React scaffold; axiom-broadcasting: subscribe_events, latest_snapshot | ✅ |
| WS-1 | Advisory Queue UI: confirm/reject кнопки + TTL bar; REST endpoints advisory/confirm|reject/{id} | ✅ |
| WS-2 | Core Tabs: Conversation (feed + textarea), Phase C (octant depth, emergent, advisory), Patterns (sparklines L1–L8, domain grid) | ✅ |
| WS-3 | /metrics Prometheus endpoint (~30 метрик); tools/grafana: docker-compose, 3 provisioned дашборда | ✅ |
| ARB-TD-05/06 | axiom-persist: persist TrustConfig calibration (StoredTrustEntry) + CognitiveProfile octant_weights; TrustConfig: iter_entries()+set_min_confidence(); loader restores both; 2 roundtrip tests | ✅ |
| Phase G1 | NeuralAdvisor V3.0: DivergenceLog (ring 256) — расхождения advisory_octant ↔ analytic_octant (Hamming ≥ 2); octant_hamming_distance() | ✅ |
| Phase G2 | NeuralAdvisor V3.0: PatternLearningResolver (conflict slot) — online learning на AdvisoryHistory per-Frame | ✅ |
| Phase G3 | NeuralAdvisor V3.0: NeuralAdvisorConfig — genome.yaml секция neural_advisor → per-advisor enable/disable | ✅ |
| OverDomainArbiter V3.0 | drain_octant_overrides() → pending overrides для AxialEvaluatorStorage; feedback-буфер для незарегистрированных источников | ✅ |
| WS-5 | axiom-node: NodePerfTracker → PerfSnapshot; SystemSnapshot расширен (traces/tension/reflector/cognitive_depth/impulses/skills); React SPA: Domains, Traces, Internals tabs + расширенный Overview | ✅ |
| Phase H1 | DREAM Phase V1.1: cluster_emergent_primitives() → SubsystemCandidateStore; NotifySubsystemCandidate (UCL 5300) | ✅ |
| Phase H2 | DREAM Phase V1.1: SubsystemLifecycleState (Proposed→Candidate→InReview→Active→Mature→Deprecated→Archived); ApproveSubsystemCandidate (UCL 5301) | ✅ |
| WS-6 | axiom-tray: системный трей (ksni), poll /metrics каждые 2s, Start/Stop axiom-node, Open Workstation в браузере | ✅ |
| Primitive YAMLs | config/anchors/morality/primitives.yaml (7 Haidt: moral_care..moral_desecration, Shell L1/L4/L6); config/anchors/abstractions/primitives.yaml (7 мета-якорей A0–A6, C0→C5+, temp 3–9); config/anchors/time/primitives.yaml (T1–T7: time_before..time_horizon); config/anchors/values/primitives.yaml (V1–V7: val_beneficial..val_forbidden) — выровнены со спецификациями | ✅ |
| config/subsystem_dependencies.yaml | §2.7 Variant C+: 7 подсистем (writing/mathematics/time/morality/values/abstractions/dilemmas), builds_on + natural_tensions | ✅ |
| SubsystemDependencies loader | axiom-config: SubsystemDependencies, SubsystemDep, NaturalTension; load_or_empty, is_natural_tension (симметрично), load_order() топо-сорт с детектированием цикла; 7 тестов | ✅ |
| DilemmaStore V1.1 | axiom-runtime: DilemmaStore (max 8 active, ring-64 resolved), DilemmaType (I–V), DilemmaResolution (5 вариантов); crystallize_to_experience_commands() → UCL InjectToken+BondTokens для EXPERIENCE domain; lineage_hash FNV-1a; 13 тестов | ✅ |
| SubsystemId extension | axiom-experience: SubsystemId += Morality(7), Abstractions(8), Dilemmas(9); subsystem_to_u8, subsystem_to_level, engine.rs string mapping | ✅ |
