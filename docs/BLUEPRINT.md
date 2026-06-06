# AXIOM — Technical Blueprint

**Назначение:** Плотный технический контекст для AI-ассистента. Не документация для людей.  
**Обновлено:** 2026-06-05  
**Тесты:** 1720, 0 failures

---

## Карта crates

```
axiom-core       — Token, Connection, Event (64B каждый, repr(C, align(64)))
axiom-ucl        — UclCommand, OpCode, UclResult
axiom-genome     — Genome (конституция, frozen в Arc после boot);
                   ModuleId: Sensorium=21, Waves=22; MAX_MODULES=23;
                   EmergentSubsystemRules (V7-D4); CrossModalConfig (CMB-TD-02)
axiom-experience — AxialStore, SutraDepthStore, InterpretationProfileStore, EmergentPrimitiveStore;
                   ModalityStore (sutra_id→Modality; Text/Vision/Internal);
                   FatigueStore + SubsystemFatigue (V7-B2);
                   Octant (8 вариантов), SubsystemId (+Morality=7/Abstractions=8/Dilemmas=9),
                   EvaluationLevel (8 уровней);
                   MetaSubsystemId (0x1001–0x1007), MetaActivation, MetaStore (CR-V6 Фаза C)
axiom-frontier   — CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection
axiom-config     — DomainConfig, AnchorSet (get_subsystem, match_text, perceptual_anchors),
                   ConfigWatcher, HeartbeatConfig, JsonSchema;
                   ВАЖНО: match_text() исключает L0-якоря (и из perceptual, и из subsystems);
                   abstraction_raw — layer: L0 → не матчится в тексте (PRIM-TD-05)
axiom-space      — SpatialHashGrid, apply_gravity_batch (SIMD-ready, feature "simd")
axiom-shell      — ShellProfile=[u8;8], SemanticContributionTable, compute_shell;
                   link_types: 0x08 syntactic, 0x09 composition,
                   0x0A cross-modal (CROSS_MODAL_BOND=0x0A01), 0x0B semantic-anchor
axiom-domain     — Domain, DomainState, AshtiCore (11 доменов), CausalHorizon, FractalChain
axiom-arbiter    — Arbiter (dual-path), Experience (shell_registry, shell_cosine bonus 15%
                   в pattern_similarity, Shell-TD-02), Reflector, SkillSet, GridHash, COM
axiom-heartbeat  — HeartbeatGenerator V2.0
axiom-upo        — UPO v2.2: DynamicTrace, Screen, UPO::compute
axiom-runtime    — AxiomEngine, Guardian, Gateway, Channel, EventBus, TickSchedule,
                   ProcessingResult, AdaptiveTickRate, Orchestrator, domain_name(),
                   SubsystemGravityRule + apply_subsystem_gravity (PRIM-TD-03, subsystem_gravity.rs);
                   BroadcastSnapshot + types (feature "adapters"),
                   Over-Domain Layer: OverDomainComponent, Weaver traits,
                   FrameWeaver V1.3, AxialEvaluator V3.0, ContextRecognizer V6.0+V7,
                   NeuralAdvisor V3.0, OverDomainArbiter V3.0,
                   Waves V1.0 (over_domain/waves/), Sensorium V1.0 (over_domain/sensorium/)
axiom-protocol   — EngineCommand (15 variants), EngineEvent (+ CrossModalBondProposed),
                   EngineState, SystemSnapshot, DomainSnapshot, TokenFieldPoint,
                   FrameWeaverStats, GuardianStats, DreamPhaseStats, BenchSpec/BenchResults
axiom-broadcasting — BroadcastHandle, WebSocket server (axum), broadcast loop,
                   DomainActivity filter, SystemSnapshot publish, Lagged resync;
                   subscribe_events() → broadcast::Receiver<EngineMessage>;
                   latest_snapshot() → Option<SystemSnapshot>;
                   snapshot_live: RwLock<Option<SystemSnapshot>>
axiom-agent      — TextPerceptor (2-path detect_subsystem + perceive_and_bond),
                   text_stable_id (0x4000_0001+, бит 30);
                   L0VisionPerceptor (V7-E2): vision_anchor_stable_id (0x2000_0001+, бит 29);
                   TemporalPerceptor (PRIM-TD-04): temporal_anchor_stable_id (0x1000_0001+, бит 28),
                     7 якорей (time_before..time_horizon), word+aliases case-insensitive,
                     new(anchor_set.get_subsystem("time")) → perceive(text) → InjectToken SUTRA;
                   MessageEffector, CliChannel, meta_commands, tick_loop,
                   AdapterCommand, ServerMessage,
                   External Adapters 0A–5 + telegram (feature), opensearch (feature)
axiom-persist    — MemoryWriter/Loader, AutoSaver, exchange (bincode)
axiom-bench      — Criterion benchmarks
axiom-node       — самостоятельный бинарный узел: tick loop, BroadcastServer :9876,
                   SIGINT/SIGTERM shutdown, axiom.yaml + persistence;
                   HTTP-сервер (axum): GET /api/ws (WS JSON bridge), POST /api/text/submit,
                   POST /api/advisory/confirm|reject/{id}, GET /metrics (Prometheus text);
                   ServeDir(web_dist) для Workstation V2 SPA;
                   NodeCmd channel: unbounded mpsc, HTTP handlers → tick loop;
                   CMB-TD-03: после apply_dream_depth_update публикует CrossModalBondProposed
axiom-observe    — автоматизация OBS-01: Corpus, ObsRunner, TickSnapshot V6, report.md
tools/axiom-web  — React 18 SPA (Vite + Zustand): 8 табов (Overview/Domains/Traces/
                   Internals/Conversation/Phase C/Patterns/Lab);
                   AdvisoryQueue confirm/reject, SVG sparklines, domain grid
tools/grafana    — docker-compose: Grafana :3000 + Prometheus :9090; 3 дашборда
tools/axiom-dashboard — egui/eframe desktop GUI (legacy)
tools/axiom-tray — системный трей (ksni): StatusNotifierItem, poll /metrics, Start/Stop
```

---

## Базовые структуры (axiom-core)

### Token — 64 bytes, repr(C, align(64))

```
sutra_id: u32          — ID потока (> 0)
domain_id: u16         — домен (> 0)
type_flags: u16        — TOKEN_FLAG_GOAL=0x0001, TOKEN_FLAG_IMPULSE=0x0002,
                         TOKEN_FLAG_FRAME_ANCHOR=0x0010,
                         TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE=0x0020,
                         TOKEN_FLAG_DREAM_REPORT=0x0040,
                         TOKEN_FLAG_DILEMMA=0x0080
position: [i16; 3]    — XYZ в семантическом пространстве
velocity: [i16; 3]
target: [i16; 3]
origin: u16            — TOKEN_ORIGIN_LOCAL=0x0000, PERSISTED=0xFE00, EXTERNAL_BASE=0xFF00
valence: i8
mass: u8               — > 0
temperature: u8
state: u8              — STATE_ACTIVE=1, STATE_SLEEPING=2, STATE_LOCKED=3
lineage_hash: u64      — обычный: хэш пути; Frame-анкер: FNV-1a(sorted sutra_id участников)
momentum: [i32; 3]
resonance: u32
last_event_id: u64
```

### Connection — 64 bytes, repr(C, align(64))

```
source_id: u32, target_id: u32, domain_id: u16
link_type: u16         — 0x08XX=syntactic, 0x0901=composition, 0x0A01=cross-modal, 0x0B01=semantic-anchor
flags: u32             — FLAG_ACTIVE=1, INHIBITED=2, TEMPORARY=4, CRITICAL=8
strength: f32, current_stress: f32, ideal_dist: f32, elasticity: f32
density_gate: u8, thermal_gate: u8
reserved_gate: [u8; 14] — [0..1]=origin_domain в Frame-связях
created_at: u64, last_event_id: u64
```

### Event — 64 bytes, repr(C, align(64))

```
event_id: u64, parent_event_id: u64, payload_hash: u64
target_id: u32, source_id: u32
domain_id: u16, event_type: u16
payload_size: u16
priority: u8    — 0=Low, 128=Normal, 200=High, 255=Critical
flags: u8       — REVERSIBLE=1, CRITICAL=2, BATCHED=4, INTERNAL=8
pulse_id: u64, source_domain: u16, event_subtype: u16, snapshot_event_id: u32
payload: [u8; 8]
```

---

## UCL (axiom-ucl)

### UclCommand — 64 bytes

```
payload: [u8; 48]
command_id: u64
target_id: u32
opcode: u16
priority: u8
flags: u8   — Sync=0x01, Force=0x02, Bypass_Membrane=0x04, No_Events=0x08, Critical=0x10, FRAME_ANCHOR=0x20
```

**OpCode (важные):**
```
InjectToken=2000, BondTokens=2003
InjectFrameAnchor=2010, ReinforceFrame=4003
TickForward=3000, ChangeTemperature=3001
NotifyEmergentCandidate=5200, ApproveEmergentCandidate=5201
NotifySubsystemCandidate=5300, ApproveSubsystemCandidate=5301
```

**InjectToken reserved[0..4]** = `proposed_sutra_id` (build_token_from_inject читает):
- TextPerceptor:    `text_stable_id`          бит 30 → 0x4000_0001..0x7FFF_FFFF
- L0VisionPerceptor: `vision_anchor_stable_id` бит 29 → 0x2000_0001..0x3FFF_FFFF
- TemporalPerceptor: `temporal_anchor_stable_id` бит 28 → 0x1000_0001..0x1FFF_FFFF

**Позиционная эвристика модальности** при InjectFrameAnchor в EXPERIENCE:
`position.x < 0 → Modality::Vision, иначе Modality::Text`

---

## Топология доменов

```
100 SUTRA    — точка входа; Frame-промоции из EXPERIENCE
101 EXECUTION – 108 VOID  — когнитивные домены
109 EXPERIENCE — ассоциативная память, Frame-анкеры
110 MAYA       — выходная проекция; FrameWeaver сканирует отсюда
```

`domain_id = level_id * 100 + role_offset`. `domain_name(id: u16)` → `&'static str`.

---

## AxiomEngine (axiom-runtime)

### Struct (ключевые поля)

```
genome: Arc<Genome>
ashti: AshtiCore
guardian: Guardian
frame_weaver: FrameWeaver
axial_evaluator: AxialEvaluator      — tick%5
context_recognizer: ContextRecognizer — tick%7
neural_advisor: NeuralAdvisor         — tick%11
over_domain_arbiter: OverDomainArbiter — tick%13
waves: Waves                           — tick%19, WAKE-only
sensorium: Sensorium                   — каждый тик, последним
dream_phase_state: DreamPhaseState
dream_scheduler: DreamScheduler
pending_cross_modal_bond_events: Vec<(u32, u32, f32)>  — CMB-TD-03, дрейнируется axiom-node
subsystem_gravity_rules: Vec<SubsystemGravityRule>     — PRIM-TD-03, boot-time immutable
```

### Ключевые методы

```rust
fn process_command(&mut self, cmd: &UclCommand) -> UclResult
fn process_and_observe(&mut self, cmd: &UclCommand) -> ProcessingResult
fn snapshot_for_broadcast() -> BroadcastSnapshot   // feature "adapters"
fn inject_anchor_tokens(&AnchorSet) -> usize
fn confirm_pending_advisory(id: u64), reject_pending_advisory(id: u64)
fn drain_cross_modal_bond_events() -> Vec<(u32, u32, f32)>  // CMB-TD-03
```

### Порядок тиков в handle_tick_wake

```
1. AshtiCore::tick() — параллельный (rayon)
2. FrameWeaver (интервал из config)
3. t%5 → AxialEvaluator
4. t%7 → ContextRecognizer
5. t%11 → NeuralAdvisor
6. t%13 → OverDomainArbiter
7. DreamScheduler
8. t%19 → Waves
9. Sensorium::collect() — последним (читает актуальное состояние всех OD)

Cold path (TickSchedule):
  t%200 → reconcile_all()
  t%500 → apply_subsystem_gravity(MAYA, subsystem_gravity_rules)  — PRIM-TD-03
  t%500 → snapshot+prune (snapshot_interval=5000)
```

---

## Over-Domain Layer

### ModuleId (HARD, axiom-genome)

```
Sutra=0 .. Maya=10 (домены)
Arbiter=11, Guardian=12, Heartbeat=13, Shell=14, Adapters=15
FrameWeaver=16, AxialEvaluator=17, ContextRecognizer=18
NeuralAdvisor=19, OverDomainArbiter=20
Sensorium=21, Waves=22
MAX_MODULES=23
```

### Тик-интервалы (простые числа)

```
AxialEvaluator=5, ContextRecognizer=7, NeuralAdvisor=11
OverDomainArbiter=13, Waves=19, Sensorium=1 (каждый тик)
```

### FrameWeaver V1.3 (tick≈100, ModuleId=16)

MAYA → scan (0x08 syntactic) → FrameCandidate → кристаллизация в EXPERIENCE (InjectFrameAnchor) →
промоция в SUTRA (только в DREAMING, через CODEX).

`FrameCandidate { anchor_position, participants, stability_count, lineage_hash, confidence, shell_similarity }`
`iter_candidates()` — pub fn для Waves (Source C).
`proposed_id_from_lineage_hash(hash: u64) → u32` — pub fn для Waves.

### AxialEvaluator V3.0 (tick=5, ModuleId=17)

Frame по осям X/Y/Z (Apollo/Dionysus, Eros/Thanatos, Will/Nothing), 8 EvaluationLevel.
OctantStabilityTracker (ring=10, threshold=0.70), ConflictPersistenceTracker (streak≥5).
NarrativeOctantTracker. AXIAL_EVALUATOR_SOURCE_ID=1.

### ContextRecognizer V6.0+V7 (tick=7, ModuleId=18)

SubsystemEnergy → InterpretationProfile → SutraDepthStore.
ActivityTrace { short=16/mid=64/long=256 } → ActivityDynamics → ActivitySignature.
SubsystemFatigue (FatigueStore, DECAY=0.90, DREAM: load×=0.35).
TransitionMatrix [[f32;16];16], decay=0.995.
CompositeSubsystemProfile + BidirectionalCoupling.
COMPOSITE_DEFS[5]: Calculus, Rhythm, Geometry, Narrative, Ethics(Values+Morality+Dilemmas).
DilemmaStore (max 8 active, ring-64 resolved) + DilemmaDetector V2.1 (Signal A/B/C).
CrossModalDetector: update(frame_ids, modality_store, tick) → pending_dream → drain в DREAM.
ModalityStore: суtra_id → Modality (Text/Vision/Internal); дефолт Text.

### NeuralAdvisor V3.0 (tick=11, ModuleId=19)

5 советников: ReactivationDepth, DepthHistoryBias, PatternLearningResolver,
AnchorVoting, DepthThresholdEmergent (MIN_DEPTH=1000, MIN_REACTIVATIONS=5).
DivergenceLog (ring 256). NeuralAdvisorConfig из genome.yaml.

### OverDomainArbiter V3.0 (tick=13, ModuleId=20)

TrustConfig (Ignore/AutoApply/RequireConfirmation × min_confidence).
CognitiveProfile { octant_weights[8] }, online learning rate=0.05.
PendingQueue → Workstation confirm/reject (HTTP). ArbiterLog ring-500.
TTL=1000 event_id. drain_octant_overrides(), drain_unrouted_feedback().

### Waves V1.0 (tick=19, ModuleId=22)

WAKE-only. Молчит в DREAMING.

```
WavesView { tick, causal_time, had_intake, dream_phase,
            context_recognizer, axial_evaluator, frame_weaver }

Impulse { source: A|B|C, target_sutra_id, pull_strength: u8,
          born_at_event, octant: Option<u8>, raise_count }

internal_dominance_factor: f32 (0..1)
  растёт: тишина + есть impulses + низкий fatigue
  падает: had_intake (DROP_RATE=0.30) / высокий fatigue (0.10)
  DOMINANCE_THRESHOLD=0.25

Источник A — активные дилеммы (intensity × age-factor → pull_strength)
Источник B — SutraDepth (max_depth > 500, не примитивы) → ReinforceFrame UCL
Источник C — FrameWeaver candidates (stability_count ≥ 3) → ReinforceFrame UCL

Защиты: DECAY_RATE=15, MAX_ACTIVE_IMPULSES=4, fatigue-потолок, DREAM: dream_reset(75%)
```

Sensorium читает `waves.internal_dominance_factor` + `waves.active_impulses` через SensoriumView.

### Sensorium V1.0 (каждый тик последним, ModuleId=21)

Только чтение (`&self` инвариант GENOME навсегда). GENOME: Read на все ресурсы.

```
SensoriumView { tick, causal_time, dream_phase, dream_stats,
                context_recognizer, axial_evaluator, frame_weaver,
                over_domain_arbiter, neural_advisor, waves }

SensoriumState (4 группы):
  ВОСПРИЯТИЕ: dominant_subsystem, activity_signature, dominant_octant
  НАПРЯЖЕНИЯ: active_dilemma_count, active_dilemmas, corpus_callosum_active
  ОРГАНИЗМ:   dream_phase_raw, fatigued_subsystems, composite_suspect_count,
              cross_modal_bonds
  ИМПУЛЬС:    internal_dominance_factor, active_impulse_count, impulse_sources

CollectionLevel: Pulse=0 (каждый), State=1 (×8), Full=2 (×32), Memory=3 (после DREAM)
Деградация: в DREAMING → только Pulse
```

Sensorium V2.0: полное поглощение TickSnapshot (DEFERRED SEN-TD-01).

---

## Stable sutra_id диапазоны

```
1..event_id               — sequential tokens
0x0001..0x0FFF_FFFF       — domain_position_hash (28 бит)
0x1000_0001..0x1FFF_FFFF  — temporal_anchor_stable_id (бит 28, TemporalPerceptor)
0x2000_0001..0x3FFF_FFFF  — vision_anchor_stable_id  (бит 29, L0VisionPerceptor)
0x4000_0001..0x7FFF_FFFF  — text_stable_id            (бит 30, TextPerceptor)
0x5000_0000..             — DilemmaDetector Signal B prefix
0x6000_0000..             — DilemmaDetector Signal C prefix
0x8000_0001..0xFFFF_FFFF  — anchor_sutra_id (бит 31, fnv1a_anchor_id)
```

---

## Protocol (axiom-protocol)

### EngineEvent (полный список)

```
Tick, DomainActivity, DreamPhaseTransition
FrameCrystallized, FrameReactivated, FramePromoted
GuardianVeto
AdapterStarted, AdapterProgress, AdapterFinished
BenchStarted, BenchProgress, BenchFinished
Alert { level: AlertLevel, category, message }
CrossModalBondProposed { frame_a, frame_b, modality_a, modality_b, strength }
  — эмитируется из axiom-node/tick.rs после drain_cross_modal_bond_events()
```

### BroadcastSnapshot (feature "adapters")

```
tick_count, com_next_id, trace_count, tension_count
domain_summaries: Vec<DomainSummary>
frame_weaver_stats: Option<FrameWeaverStats>
dream_phase: Option<DreamPhaseSnapshot>
last_crystallization_tick: u64
guardian_vetoes_since_wake: u64
last_dream_summary: Option<LastDreamSummary>
cross_modal_candidates: usize   — активные кандидаты (не достигли порога 50)
cross_modal_bonds: usize        — созданные bonds в EXPERIENCE
```

---

## AnchorSet (axiom-config)

```rust
fn match_text(text: &str) -> Vec<AnchorMatch>
// Исключает L0 якоря:
//   - из perceptual (визуальные, пространственные)
//   - из subsystems (если layer: L0 — например abstraction_raw, PRIM-TD-05)

fn perceptual_anchors() -> &[Anchor]           // только L0 visual
fn get_subsystem(name: &str) -> &[Anchor]      // "time", "writing" и т.д.
fn dominant_subsystem_of(matches) -> Option<SubsystemId>
```

**Подсистемы с YAML-якорями:**
writing, mathematics, logic, time, music, values, morality, abstractions.

**abstractions:** 7 мета-якорей C0–C5+; `abstraction_raw` имеет `layer: L0` →
исключён из match_text (C0 = сырой сигнал, не языковой концепт).

---

## CrossModal Binding (axiom-runtime)

```
MIN_CROSS_MODAL_COACTIVATION=50
CROSS_MODAL_BOND=0x0A01 (категория 0x0A [0,20,0,0,10,0,0,10])

Поток:
  CR::on_tick → cross_modal_detector.update(frame_ids, modality_store, tick)
  → при достижении порога → pending_dream
  → apply_dream_depth_update → drain_cross_modal_bond_commands(109) → BondTokens UCL
  → pending_cross_modal_bond_events → drain в tick.rs → CrossModalBondProposed event

Modality:
  InjectFrameAnchor в EXPERIENCE: position.x<0 → Vision, ≥0 → Text
  register_frame_modality(sutra_id, modality) вызывается явно при кристаллизации
```

---

## Guardian

```
enforce_access(module, resource, op) → bool
validate_reflex(&Token) → Allow | Veto
scan_domain(&DomainState) → Vec<InhibitAction>
dream_propose(&[Token]) → Vec<CodexAction>
reset_wake_stats() при переходе в Wake
```

GENOME on_boot для FrameWeaver: проверяет MayaOutput/Read, ExperienceMemory/ReadWrite, SutraTokens/Control.

---

## TextPerceptor (axiom-agent)

```rust
// Path 1: якорный матч
let matches = anchor_set.match_text(text);
if let Some(s) = anchor_set.dominant_subsystem_of(&matches) { return s; }

// Path 2: AnchorMatchTable (word_signals w=1.0 + char_signals w=0.4)
let s = match_table.dominant_subsystem(text, &decomposition_table);
```

`perceive_and_bond(text)` → InjectToken + BondTokens (SEMANTIC_ANCHOR_BOND=0x0B01).
100% per-text accuracy (OBS-02, 8 корпусов × 30k тиков).

---

## DREAM Phase

```
Wake → FallingAsleep → Dreaming → Waking → Wake
Запись в SUTRA — только в DREAMING (GUARDIAN инвариант)
DreamScheduler триггеры: Idle / Fatigue (composite 0-255) / Explicit
DreamCycle: Stabilization → Processing → Consolidation
apply_dream_depth_update(): SutraDepth updates + cross-modal bond drain
```

---

## Persistence (axiom-persist)

```
bincode (атомарный rename), IMPORT_WEIGHT_FACTOR=0.7
AutoSaver::tick / force_save
ARB-TD-05: TrustConfig calibration roundtrip
ARB-TD-06: CognitiveProfile octant_weights roundtrip
```

---

## Незакрытые задачи

| ID | Суть | Статус |
|----|------|--------|
| SEN-TD-01 | Полное поглощение TickSnapshot → Sensorium V2.0 | DEFERRED |
| FW-TD-02 | Per-pair co-activation (структуру не выбирать до CausalWeaver) | DEFERRED |
| COMP-01 | Vital Signs окно (Companion) | DEFERRED |
| AGENT-TD-01 | TextPerceptor: embeddings вместо lookup | DEFERRED |
| Shell-TD-02 | resonance_search shell bonus (нужен ShellRegistry в Arbiter) | DEFERRED |
| EMERGENT-TD-01 | Калибровка порогов под неоднородный корпус | DEFERRED |
| OBS-TD-03 | delta-energy per-text нерабочий, методы оставлены до embeddings | DEFERRED |
| CMB-TD-01 | Stress-driven revocation cross-modal bonds | DEFERRED |

---

## Критические инварианты

1. **64-byte alignment** — Token, Connection, Event: `repr(C, align(64))`
2. **ID > 0** — sutra_id, domain_id, event_id, created_at
3. **COM монотонность** — event_id > parent_event_id
4. **11 доменов фиксированы** в AshtiCore
5. **Genome frozen** — `Arc<Genome>` после boot
6. **STATE_LOCKED** — якоря не мутируются
7. **Единственный writer** — только tick_loop владеет AxiomEngine
8. **Sensorium только читает** — `&self` инвариант GENOME навсегда
9. **Waves не пишет в SUTRA напрямую** — только через UCL
10. **L0 якоря** — исключены из `match_text()` (и perceptual, и subsystem)
11. **SUTRA write** — только в DREAMING через CODEX
12. **Frame промоция** — только через DreamCycle, не on_tick

---

## Производительность (AMD Ryzen 5 3500U)

| Операция | Время |
|----------|-------|
| Token::new | 17.2 ns |
| TickForward (50 tok, 1M тиков) | **96.5 ns/тик** |
| SpatialHashGrid::rebuild (1K) | 9.50 µs |
| apply_gravity_batch (1K) | 23.4 µs |
| apply_gravity_batch_avx2 (1M) | 6.74 ms |
| resonance_search (1K трейсов) | 12.8 µs |
| AxiomEngine::new | ~440 µs |

Результаты v9: docs/bench/RESULTS.md.
