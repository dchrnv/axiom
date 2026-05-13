# AXIOM — Technical Blueprint

**Назначение:** Плотный технический контекст для AI-ассистента. Не документация для людей.  
**Обновлено:** 2026-05-13  
**Тесты:** 1200, 0 failures  
**Последний коммит:** 7f07e72

---

## Карта crates

```
axiom-core       — Token, Connection, Event (64B каждый, repr(C, align(64)))
axiom-ucl        — UclCommand, OpCode, UclResult
axiom-genome     — Genome (конституция, frozen в Arc после boot)
axiom-frontier   — CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection
axiom-config     — DomainConfig, AnchorSet, ConfigWatcher, HeartbeatConfig, JsonSchema
axiom-space      — SpatialHashGrid, apply_gravity_batch (SIMD-ready, feature "simd")
axiom-shell      — ShellProfile=[u8;8], SemanticContributionTable, compute_shell
axiom-domain     — Domain, DomainState, AshtiCore (11 доменов), CausalHorizon, FractalChain
axiom-arbiter    — Arbiter (dual-path), Experience, Reflector, SkillSet, GridHash, COM
axiom-heartbeat  — HeartbeatGenerator V2.0
axiom-upo        — UPO v2.2: DynamicTrace, Screen, UPO::compute
axiom-runtime    — AxiomEngine, Guardian, Gateway, Channel, EventBus, TickSchedule,
                   ProcessingResult, AdaptiveTickRate, Orchestrator, domain_name(),
                   BroadcastSnapshot + types (feature "adapters"),
                   Over-Domain Layer: OverDomainComponent, Weaver traits,
                   FrameWeaver V1.3 (crates/axiom-runtime/src/over_domain/weavers/frame.rs)
axiom-protocol   — EngineCommand (15 variants), EngineEvent, EngineState, SystemSnapshot,
                   DomainSnapshot, TokenFieldPoint, FrameWeaverStats, GuardianStats,
                   DreamPhaseStats, BenchSpec/BenchResults (serde JSON)
axiom-broadcasting — BroadcastHandle, WebSocket server (axum), broadcast loop,
                   DomainActivity filter, SystemSnapshot publish, Lagged resync
axiom-agent      — TextPerceptor, MessageEffector, CliChannel, meta_commands,
                   tick_loop (9 params), AdapterCommand, ServerMessage,
                   External Adapters 0A–5 + telegram (feature), opensearch (feature)
axiom-persist    — MemoryWriter/Loader, AutoSaver, exchange (bincode)
axiom-bench      — Criterion benchmarks
axiom-workstation — egui/eframe desktop GUI V1.0 (async tungstenite)
tools/axiom-dashboard — egui/eframe desktop GUI (sync tungstenite, legacy)
```

---

## Базовые структуры (axiom-core)

### Token — 64 bytes, repr(C, align(64))

```
sutra_id: u32          — ID потока (> 0, инвариант)
domain_id: u16         — домен (> 0)
type_flags: u16        — TOKEN_FLAG_GOAL=0x0001, TOKEN_FLAG_IMPULSE=0x0002,
                         TOKEN_FLAG_FRAME_ANCHOR=0x0010,
                         TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE=0x0020
position: [i16; 3]    — XYZ в семантическом пространстве
velocity: [i16; 3]
target: [i16; 3]
origin: u16            — TOKEN_ORIGIN_LOCAL=0x0000, PERSISTED=0xFE00, EXTERNAL_BASE=0xFF00
valence: i8            — способность к связям
mass: u8               — масса (> 0)
temperature: u8        — активность
state: u8              — STATE_ACTIVE=1, STATE_SLEEPING=2, STATE_LOCKED=3
lineage_hash: u64      — хэш предков (используется FrameWeaver для Frame-дедупликации)
momentum: [i32; 3]
resonance: u32
last_event_id: u64     — COM-timestamp (>= created_at); FrameWeaver использует как proxy для времени создания
```

### Connection — 64 bytes, repr(C, align(64))

```
source_id: u32         — (> 0)
target_id: u32         — (> 0)
domain_id: u16         — (> 0)
link_type: u16         — тип связи; 0x08XX = синтаксическая категория (FrameWeaver)
                         биты [7:4] = слой (S1=1…S8=8), биты [3:0] = роль
flags: u32             — FLAG_ACTIVE=1, INHIBITED=2, TEMPORARY=4, CRITICAL=8
strength: f32          — (> 0.0); используется FrameWeaver как confidence per-connection
current_stress: f32
ideal_dist: f32
elasticity: f32
density_gate: u8
thermal_gate: u8
reserved_gate: [u8; 14] — [0..1] = origin_domain (BE u16) в Frame-связях
created_at: u64
last_event_id: u64
```

### Event — 64 bytes, repr(C, align(64))

```
event_id: u64
parent_event_id: u64
payload_hash: u64
target_id: u32
source_id: u32
domain_id: u16
event_type: u16        — EventType enum
payload_size: u16
priority: u8           — 0=Low, 128=Normal, 200=High, 255=Critical
flags: u8              — REVERSIBLE=1, CRITICAL=2, BATCHED=4, INTERNAL=8
pulse_id: u64
source_domain: u16
event_subtype: u16     — NONE=0, GRAVITY=1, MANUAL=2, COLLISION=3, IMPULSE=4
snapshot_event_id: u32
payload: [u8; 8]
```

**EventType major ranges:**
- Token: 0x0001–0x000C (Create, Update, Delete, Move, Decay, Merge, Split, Activate, Deactivate, Frozen, Thawed)
- Space: 0x0010–0x0012 (Moved, Collision, EnteredCell)
- Connection: 0x1001–0x1008
- Domain: 0x2001–0x2003
- Physics: 0x3001–0x3005 (Heartbeat, GravityUpdate, CollisionDetected, ResonanceTriggered, Thermodynamics)
- Agent: 0xE001–0xE002 (ShellExec, MayaOutput)
- System: 0xF001–0xF003 (Checkpoint, Rollback, Shutdown)

---

## UCL (axiom-ucl)

### UclCommand — 64 bytes, repr(C, align(64))

```
payload: [u8; 48]
command_id: u64
target_id: u32         — домен или токен
opcode: u16
priority: u8
flags: u8              — Sync=0x01, Force=0x02, Bypass_Membrane=0x04, No_Events=0x08, Critical=0x10,
                         FRAME_ANCHOR=0x20
```

**OpCode enum:**
```
SpawnDomain=1000, CollapseDomain=1001, LockMembrane=1002, ReshapeDomain=1003
InjectToken=2000, ApplyForce=2001, AnnihilateToken=2002, BondTokens=2003, SplitToken=2004
InjectFrameAnchor=2010, ReinforceFrame=2011
TickForward=3000, ChangeTemperature=3001, ApplyGravity=3002, PhaseTransition=3003
ProcessTokenDualPath=4000, FinalizeComparison=4001
CoreShutdown=9000, CoreReset=9001, BackupState=9002, RestoreState=9003
```

**Frame-специфичные payload'ы:**
- `InjectFrameAnchorPayload` — lineage_hash, proposed_sutra_id, target_domain_id, type_flags, position, state, mass, temperature
- `BondTokensPayload` — source_id, target_id, domain_id, link_type, strength, origin_domain (stored in reserved_gate[0..2])
- `ReinforceFramePayload` — anchor_id, delta_mass, delta_temperature

---

## Топология доменов (AshtiCore)

**Формула:** `domain_id = level_id * 100 + role_offset`  
При `level_id=1` (стандартный уровень):

```
100 — SUTRA      (role 0)  — точка входа потока; Frame-промоции из EXPERIENCE
101 — EXECUTION  (role 1)  — реализация решений
102 — SHADOW     (role 2)  — симуляция угроз
103 — CODEX      (role 3)  — конституционный фильтр
104 — MAP        (role 4)  — статическая база фактов
105 — PROBE      (role 5)  — активное зондирование
106 — LOGIC      (role 6)  — чистая дедукция
107 — DREAM      (role 7)  — фоновая оптимизация
108 — ETHICS     (role 8)  — коллектор аномалий
109 — EXPERIENCE (role 9)  — ассоциативная память, рефлексы, Frame-анкеры
110 — MAYA       (role 10) — выходная проекция; FrameWeaver сканирует отсюда
```

**domain_name(id: u16) → &'static str** — pub fn в axiom-runtime, по `id % 100`.

### Фундаментальная архитектура: трёхчастная онтология

AXIOM построен на разделении трёх онтологических слоёв:

**SUTRA (100) — нить, вечная истина.** Первичные сущности: anchor-токены, факты,
семантические оси, аксиомы. SUTRA не обрабатывает. Хранит неизменное.
Промотированные Frame (STATE_LOCKED, TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE) живут здесь.

**EXPERIENCE (109) — накопленный опыт.** Удачные узоры, кристаллизованные скиллы,
Frame-анкеры. Растёт, стареет. Межсистемный обмен идёт через EXPERIENCE.
Frame здесь: STATE_ACTIVE, нарастающий mass/temperature при реактивации.

**MAYA (110) — проявление, "сейчас".** Сборка узоров, генерация ответа.
FrameWeaver сканирует синтаксические связи именно здесь.

**ASHTI 101–108 — зеркала.** Специализированные линзы, через которые узоры преломляются.

```
SUTRA (истина)  ──────┐
                      ├──► ASHTI 101-108 ─► MAYA (проявление)
EXPERIENCE (опыт) ────┘                           │
                                                   │
EXPERIENCE (опыт) ◄──── Weavers ◄──────── MAYA (живые узоры)

                         [редкий путь]
EXPERIENCE ──► (GUARDIAN/CODEX) ──► SUTRA (промоция)
```

**TokenGraph не существует как тип.** Граф неявный: `DomainState.tokens` + `DomainState.connections`.

---

## Domain & DomainState (axiom-domain)

### AshtiCore struct

```
domains: Vec<Domain>       — 11 экземпляров физики
states: Vec<DomainState>   — 11 буферов данных
arbiter: Arbiter
level_id: u16
pulse: u64
```

**Ключевые методы:**
```rust
fn domain(&self, index: usize) -> Option<&Domain>
fn state(&self, index: usize) -> Option<&DomainState>
fn state_mut(&self, index: usize) -> Option<&mut DomainState>
fn index_of(&self, domain_id: u16) -> Option<usize>
fn domain_id_at(&self, index: usize) -> Option<u16>
fn level_id(&self) -> u16
fn inject_token(domain_id: u16, token: Token) -> Result<usize, CapacityExceeded>
fn inject_connection(domain_id: u16, conn: Connection) -> Result<usize, CapacityExceeded>
fn process(token: Token) -> RoutingResult
fn tick() -> Vec<Event>
fn reconcile_all() -> usize
fn experience_mut() -> &mut ExperienceModule
```

---

## Arbiter & Routing (axiom-arbiter)

### Dual-Path Routing Logic

```
Входящий Token
    ↓
[Fast Path] experience.resonance_search(token)
    → confidence ≥ reflex_threshold?
        YES → ReflexHit (возврат cached token)
        NO  → [Slow Path] ASHTI 1→8→MAYA pipeline
                ↓
            confidence < 0.6 → создать TensionTrace
            confidence ≥ 0.6 → обновить Experience
```

### RoutingResult

```
event_id: u64
reflex: Option<Token>
slow_path: Vec<Token>
consolidated: Option<Token>
confidence: f32            — 0.0..=1.0
passes: u8
routed_events: Vec<u64>
```

---

## Shell & Семантика (axiom-shell)

### ShellProfile = [u8; 8]

```
[0] L1: Physical   (материальность)
[1] L2: Sensory    (ощущения)
[2] L3: Motor      (действие)
[3] L4: Emotional  (валентность)
[4] L5: Cognitive  (знание)
[5] L6: Social     (отношения)
[6] L7: Temporal   (ритм)
[7] L8: Abstract   (символы)
```

Синтаксические слои FrameWeaver используют ту же нумерацию: S1=L1 … S8=L8.
В link_type: биты [7:4] = слой (1..=8), биты [3:0] = роль.

---

## AxiomEngine (axiom-runtime)

### Struct

```
genome: Arc<Genome>
ashti: AshtiCore
guardian: Guardian
frame_weaver: FrameWeaver          — Over-Domain компонент V1.3
pending_events: Vec<Event>
com_next_id: u64
tick_count: u64
tick_schedule: TickSchedule
worker_count: usize
thread_pool: rayon::ThreadPool
```

### TickSchedule

```
adaptation_interval: u32    — 50
horizon_gc_interval: u32    — 500
snapshot_interval: u32      — 5000
dream_interval: u32         — 100
tension_check_interval: u32 — 10
goal_check_interval: u32    — 10
reconcile_interval: u32     — 200
persist_check_interval: u32 — 0 = disabled
weaver_scan_intervals: HashMap<WeaverId, u32>
adaptive_tick: AdaptiveTickRate
domain_activity_threshold: u32     — фильтр: DomainActivity публикуется только при recent_activity > threshold
```

### Ключевые методы

```rust
fn new() -> Self
fn process_command(&mut self, cmd: &UclCommand) -> UclResult
fn process_and_observe(&mut self, cmd: &UclCommand) -> ProcessingResult
fn snapshot() -> EngineSnapshot
fn apply_domain_config(domain_id: u16, config: &DomainConfig)  // hot-reload
fn snapshot_for_broadcast() -> BroadcastSnapshot               // feature "adapters"
fn domain_detail_snapshot(domain_id: u16) -> Option<DomainDetailSnapshot>
fn drain_events() -> Vec<Event>
fn inject_anchor_tokens(&AnchorSet) -> usize
fn trace_count() -> usize
pub fn domain_name(id: u16) -> &'static str
```

### ProcessingResult

```
path: ProcessingPath       — Direct | Reflex | MultiPass(n) | Rejected
dominant_domain_id: u16
coherence_score: Option<f32>
reflex_hit: bool
traces_matched: usize
position: [i16; 3]
event_id: u64
```

---

## Over-Domain Layer (axiom-runtime::over_domain)

Архитектурный слой компонентов **над** доменами. Две категории:

**Guardians** — контроль допустимости, veto-логика. GUARDIAN V1.0 — существующий.

**Weavers** — сборка реляционных структур, кристаллизация узоров в EXPERIENCE,
промоция в SUTRA через CODEX.
- FrameWeaver V1.3 — синтаксические/реляционные Frame ✅
- Deferred: CausalWeaver, SpatialWeaver, TemporalWeaver, AnalogyWeaver, NarrativeWeaver

### Инварианты Over-Domain

- Нет собственного хранилища (пишут в EXPERIENCE/SUTRA через UCL)
- Чтение только через AshtiCore (передаётся в on_tick)
- Авторизация через GENOME (ModuleId::FrameWeaver)
- Подчинены GUARDIAN

### Traits (актуальные сигнатуры)

```rust
pub trait OverDomainComponent: Send {
    fn name(&self) -> &'static str;
    fn module_id(&self) -> ModuleId;
    fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError>;
    fn on_tick_interval(&self) -> u32 { 1 }
    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<(), OverDomainError>;
    fn on_shutdown(&mut self) -> Vec<UclCommand>;
}

pub trait Weaver: OverDomainComponent {
    type Pattern: Send;
    fn scan(&mut self, tick: u64, maya_state: &DomainState) -> Vec<Self::Pattern>;
    fn propose_to_dream(&self, patterns: &[Self::Pattern]) -> Vec<CrystallizationProposal>;
    fn check_promotion(
        &self,
        tick: u64,
        experience_state: &DomainState,
        anchors: &[&Token],
    ) -> Vec<PromotionProposal>;
    fn weaver_id(&self) -> WeaverId;
    fn target_domain(&self) -> u16 { 109 }  // EXPERIENCE
}
```

**OverDomainError:** `BootFailed(String)` | `TickFailed(String)` | `GenomeDenied`

**OverDomainComponent не object-safe** для Weaver (из-за `type Pattern`).
FrameWeaver хранится по значению в AxiomEngine, не как `Box<dyn>`.

---

## FrameWeaver V1.3 (axiom-runtime)

Полное описание реализации: [docs/guides/FrameWeaver_Implementation_V1_3.md](guides/FrameWeaver_Implementation_V1_3.md)

### Цикл

```
MAYA → scan_state() → FrameCandidate[] → update_candidates() → (stable) →
    evaluate_crystallization_rules() →
        CrystallizeFull → build_crystallization_commands() → EXPERIENCE
        или → build_reinforce_command() → EXPERIENCE (если Frame уже существует)

FallingAsleep → dream_propose(ashti, tick) → Vec<DreamProposal> (→ SUTRA через CODEX)
```

### Ключевые типы

```
FrameCandidate {
    anchor_position: [i16; 3]   — центроид участников
    participants: Vec<Participant>
    detected_at_tick: u64
    stability_count: u32
    category: u16               — FRAME_CATEGORY_SYNTAX
    lineage_hash: u64           — FNV-1a по sutra_id участников (order-independent)
    confidence: f32             — среднее strength синтаксических связей
}

Participant { sutra_id, origin_domain_id, role_link_type, layer: u8 }

RuleTrigger:
    StabilityReached(n: u32)       — stability_count >= n
    HighConfidence(threshold: f32) — confidence >= threshold
    DreamCycle                      — dream_cycle_completed флаг (on_dream_wake())
    RepeatedAssembly { window_ticks } — stability_count * scan_interval >= window_ticks

PromotionRule {
    min_age_ticks, min_reactivations, min_temperature, min_mass,
    min_participant_anchors: usize,  — cross-domain: число участников-анкеров в SUTRA
    requires_codex_approval
}
```

### on_boot — GENOME enforcement

```rust
fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError> {
    let index = GenomeIndex::build(genome);
    // Требует три разрешения:
    index.check_access(FrameWeaver, MayaOutput, Read)          // сканирование
    index.check_access(FrameWeaver, ExperienceMemory, ReadWrite) // кристаллизация
    index.check_access(FrameWeaver, SutraTokens, Control)       // промоция
    // Нет → Err(OverDomainError::GenomeDenied)
}
```

### dream_propose — промоция в SUTRA

```rust
fn dream_propose(&self, ashti: &AshtiCore, tick: u64) -> Vec<DreamProposal>
// Вызывается AxiomEngine при FallingAsleep → Dreaming
// Итерирует EXPERIENCE Frame-анкеры (type_flags & TOKEN_FLAG_FRAME_ANCHOR != 0,
//   не TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE)
// Cross-domain min_participant_anchors: подсчитывает участников в SUTRA-стейте
// Возвращает DreamProposal::Promotion { anchor_id, source=EXPERIENCE, target=SUTRA }
```

---

## Protocol (axiom-protocol)

Независимый crate. Serde JSON. Используется axiom-workstation + axiom-broadcasting.

### EngineCommand (15 вариантов)

```
ForceSleep, ForceWake
GetConfigSchema, GetConfigSection { id }, UpdateConfigField { section_id, field_id, value }
ListAdapters, StartImport { adapter_id, source_path, options }, CancelImport { import_id }
SubmitText { text, target_domain }
InjectToken { domain_id, layer, content }, InjectConnection { from_domain, to_domain }
GracefulShutdown, ForceShutdown
RequestFullSnapshot, RequestFrameDetails { anchor_id }
RunBench { spec: BenchSpec }
```

### EngineEvent

```
Tick { tick, event, hot_path_ns }
DomainActivity { domain_id, recent_activity, layer_activations: [u8;8] }
DreamPhaseTransition { from, to, trigger: SleepTrigger }
FrameCrystallized { anchor_id, layers_present, participant_count }
FrameReactivated { anchor_id, new_temperature }
FramePromoted { source_anchor_id, sutra_anchor_id }
GuardianVeto { reason, command_summary }
AdapterStarted, AdapterProgress, AdapterFinished
BenchStarted { bench_id, run_id }, BenchProgress { run_id, completed, total },
BenchFinished { run_id, results }
Alert { level: AlertLevel, category, message }
```

### EngineState

`Wake | FallingAsleep | Dreaming | Waking`

### SystemSnapshot (полный snapshot по RequestFullSnapshot)

```
engine_state: EngineState
current_tick: u64
current_event: u64
hot_path_ns: u64              — последний tick ns
domains: Vec<DomainSnapshot>
over_domain: OverDomainSnapshot
fatigue: FatigueSnapshot
last_dream_report: Option<DreamReport>
frame_weaver_stats: Option<FrameWeaverStats>
guardian_stats: GuardianStats
dream_phase_stats: DreamPhaseStats
adapter_progress: Vec<AdapterProgress>
```

### DomainSnapshot

```
id, name, config_summary, token_count, connection_count
temperature_avg, recent_activity
layer_activations: [u8; 8]       — активность по слоям
token_field: Vec<TokenFieldPoint> — до 300 точек, для Live Field
```

### TokenFieldPoint

```
position: [f32; 3], layer: u8, temperature: u8, anchor_membership: Option<u32>
```

### FrameWeaverStats (в протоколе)

```
total_frames, frames_in_sutra, promotions_since_wake
last_crystallization_tick
syntactic_layer_activations: [u8; 8]
```

---

## Broadcasting (axiom-broadcasting)

```
BroadcastHandle {
    command_tx: mpsc::Sender<EngineCommand>
    event_rx: broadcast::Receiver<EngineEvent>
}

BroadcastServer — WebSocket (axum), публикует EngineEvent подписчикам
```

**domain_activity_threshold** — фильтр в tick_loop: `DomainActivity` публикуется только
если `recent_activity > threshold`. Предотвращает flood при idle.

**Lagged snapshot resync** — при broadcast::error::RecvError::Lagged клиент получает
полный SystemSnapshot для ресинхронизации.

---

## Guardian (axiom-runtime)

```
genome: Arc<Genome>
genome_index: GenomeIndex
stats: GuardianStats
```

**Ключевые методы:**
```rust
fn enforce_access(module, resource, op) -> bool
fn validate_reflex(&Token) -> ReflexDecision   // Allow | Veto(VetoReason)
fn scan_domain(&DomainState) -> Vec<InhibitAction>
fn update_codex(&mut DomainState, CodexAction) -> Result<(), GuardianError>
fn adapt_thresholds(...) -> Vec<u16>
fn dream_propose(&[Token]) -> Vec<CodexAction>   // до 5 за вызов
```

---

## tick_loop & External Adapters (axiom-agent)

### tick_loop — 9 параметров

```rust
pub async fn tick_loop(
    mut engine: AxiomEngine,
    mut command_rx: mpsc::Receiver<AdapterCommand>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
    snapshot: Arc<RwLock<BroadcastSnapshot>>,
    mut auto_saver: AutoSaver,
    anchor_set: Option<Arc<AnchorSet>>,
    config: AdaptersConfig,
    config_watcher: Option<ConfigWatcher>,
    wstation_handle: Option<Arc<BroadcastHandle>>,  // axiom-workstation интеграция
)
```

**Итерация цикла (в порядке):**
1. `sleep(adaptive_interval или base_tick_ms)`
2. `config_watcher.poll()` — hot-reload axiom.yaml, применяется через `apply_domain_config()`
3. `command_rx.try_recv()` — drain всех команд
4. `engine.process_command(&tick_cmd)` — TickForward
5. `engine.drain_events()` → `cli_state.event_log`
6. Adaptive tick: ExternalInput/TensionHigh или on_idle_tick()
7. TickSchedule периодические задачи
8. Broadcast
9. Если `wstation_handle`: publish EngineEvents через axiom-broadcasting

### AdapterCommand

```
id: String
source: AdapterSource     — Cli | WebSocket(u64) | Rest | Telegram(i64)
payload: AdapterPayload
priority: GatewayPriority — Normal | High | Critical (добавлено в B1-E2)
```

### AdapterPayload

```
Inject { text }
MetaRead { cmd }     — :status, :domains, :traces, ...
MetaMutate { cmd }   — :save, :load, :quit, :tick N, ...
Subscribe { channels }, Unsubscribe { channels }
DomainSnapshot { domain_id }
```

### ServerMessage (axiom-agent, serde tag = "type")

```
Result { command_id, path, domain_id, domain_name, coherence, reflex_hit,
         traces_matched, position, shell, event_id }
Tick { tick_count, traces, tension, last_matched }
State { tick_count, snapshot: BroadcastSnapshot }
CommandResult { command_id, output: String }
DomainDetail(DomainDetailSnapshot)
Error { command_id, message }
```

---

## External Adapters

### Telegram (Phase 4, feature "telegram")

```
route_message(text) -> Option<AdapterPayload>:
    /start         → None (welcome)
    /status, /domains, /traces → MetaRead
    :save/:quit/... → MetaMutate
    :* остальные   → MetaRead
    plain text     → Inject

AdapterCommand { priority: GatewayPriority::Normal }  ← обязательное поле
```

Задачи: `poll_task` + `notify_task`. `pending: HashMap<String, i64>` — command_id → chat_id.

### RunBench flow (C2, tick_loop)

```
AdapterCommand { payload: MetaMutate { cmd: ":bench <spec>" } }
  → tick_loop → EngineCommand::RunBench { spec }
  → engine runs bench
  → ServerMessage::CommandResult + BenchProgress events через broadcast
```

---

## Persistence (axiom-persist)

```
AutoSaver::tick(&engine, path)      — условное сохранение
AutoSaver::force_save(&engine, path)
save/load: Token + Connection + ExperienceTrace → bincode (атомарный rename)
export/import traces+skills: GUARDIAN-валидация при импорте (weight × 0.7)
```

---

## Configuration (axiom-config)

### DomainConfig (128 bytes)

Фабрики: `factory_sutra`, `factory_execution`, ..., `factory_maya`.

**apply_domain_config(domain_id, &DomainConfig)** — горячий перезапуск конфигурации
без перезагрузки Engine. Реализован в AxiomEngine. Вызывается tick_loop при изменении axiom.yaml.

### AnchorSet

```
axes: Vec<Anchor>       — 6 осевых → SUTRA
layers: Vec<Vec<Anchor>> — L1–L8 → SUTRA
domains: Vec<Vec<Anchor>> — D1–D8 → ASHTI[1..=8]
```

**Загруженные якоря (сейчас):**
- `config/anchors/axes.yaml` — 6
- `config/anchors/layers/L5_cognitive.yaml` — 10
- `config/anchors/domains/D1_execution.yaml` — 6

Остальные 14 файлов — Anchor-Fill (DEFERRED.md). FNV-1a fallback активен.

---

## Workstation V1.0 (axiom-workstation)

Desktop GUI на egui/eframe + async tungstenite.

### Вкладки

```
Conversation  — чат с Engine (SubmitText), multi-line, Ctrl+Enter для отправки
Files         — импорт файлов (rfd picker), прогресс адаптеров, Cancel
System Map    — мандала: 11 доменов, секторы, связи, Alert ring (WS4-TD-03)
Benchmarks    — история предустановленных BenchSpec, RunBench команды
Settings      — конфигурация параметров Engine (UpdateConfigField)
```

### Архитектура

```
App → tokio runtime (отдельный thread)
    → tungstenite WS клиент → axiom-broadcasting (BroadcastHandle)
    → EngineCommand tx / EngineEvent rx
    → eframe::run_native (основной thread)
```

**Show-more pagination** — длинные Conversation-ответы разбиваются на страницы.
**Canvas::Cache** — системная карта кешируется между кадрами.
**Welcome fade-in** — анимация при первом подключении.
**MenuBar + DetachTab** — системные контролы.

---

## Незакрытые задачи (DEFERRED.md v40.0)

| ID | Суть |
|----|------|
| **BRD-TD-05** | `build_system_snapshot()` — многие поля нулевые. Расширяется при axiom-node. |
| **BRD-TD-06** | Pong timeout test — требует raw TCP без WS framing. Очень низкий приоритет. |
| **BRD-TD-07** | Engine tick-loop → BroadcastHandle интеграция через axiom-node (цикл зависимостей). |
| **WS4-TD-03** | System Map: ASHTI sector fill, flow lines, alert ring. При живых данных от Engine. |
| **WS4-TD-04** | SystemSnapshot bottom-panel: hot_path_ns, promotions_today, dream_phase_stats. |
| **Anchor-Fill** | 14 YAML (L1-L4, L6-L8, D2-D8). FNV-1a fallback активен. |
| **WS-V2-*** | V2.0 идеи: история чата, Pause/Resume, custom bench, TLS, sync. |
| **COMP-01** | Vital Signs окно (Companion, ambient display для физического банера). |

---

## Критические инварианты

1. **64-byte alignment** — Token, Connection, Event: `repr(C, align(64))`.
2. **ID > 0** — sutra_id, domain_id, event_id, created_at ненулевые.
3. **COM монотонность** — event_id > parent_event_id, last_event_id >= created_at.
4. **11 доменов фиксированы** — AshtiCore не допускает spawn/collapse в runtime.
5. **Genome frozen** — `Arc<Genome>` не изменяется после boot.
6. **Arbiter all-or-nothing** — все 11 доменов зарегистрированы до routing.
7. **STATE_LOCKED** — якорные токены (в т.ч. промотированные Frame) не мутируются рефлексами.
8. **Causal Horizon** — `min(token.last_event_id)` по всем доменам. Позади — архив.
9. **Единственный writer** — только tick_loop владеет AxiomEngine.
10. **Shell нормализация** — max в [u8; 8] = 255. EMPTY_SHELL если нет связей.
11. **Tension при низкой когерентности** — confidence < 0.6 → TensionTrace.
12. **valence требует mass** — Guardian отклоняет valence≠0 && mass=0.
13. **Frame lineage_hash** — FNV-1a по sorted sutra_id участников (order-independent).
14. **Frame промоция только через DreamCycle** — on_tick не генерирует промоцию-команды.
15. **GENOME on_boot** — FrameWeaver не запускается без 3 явных разрешений в Genome.

---

## Производительность (v9, 2026-04-20, AMD Ryzen 5 3500U)

| Операция | Время | Примечание |
|----------|-------|------------|
| Token::new | 17.2 ns | |
| TickForward (50 tok, 1M тиков) | **96.5 ns/тик** | горячий путь |
| SpatialHashGrid::rebuild (1K) | 9.50 µs | |
| apply_gravity_batch (1K) | 23.4 µs | |
| compute_shell (100 связей) | 197 ns | |
| resonance_search (1K трейсов) | 12.8 µs | O(1) |
| AxiomEngine::new | 992 µs | |

Результаты v9: docs/bench/RESULTS.md.
