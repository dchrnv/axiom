# FrameWeaver — Реализация V1.3

**Назначение:** Плотный технический документ для AI-ассистента. Описывает полную
реализацию FrameWeaver как она существует сейчас — включая всё, что выходит за рамки
спека V1.2 (см. DEFERRED D1–D4).  
**Обновлено:** 2026-05-07  
**Файл реализации:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`  
**Traits:** `crates/axiom-runtime/src/over_domain/traits.rs`  
**Specs (для справки):**
- `docs/spec/Weaver/Over_Domain_Layer_V1_1.md`
- `docs/spec/Weaver/FrameWeaver_V1_2.md` ("Dream Weaver", текущая официальная спека)

---

## Эволюция версий

**V1.1** — базовая кристаллизация. `check_promotion(anchor: &Token)`. Только
`StabilityReached` как дефолтный механизм.

**V1.2 spec ("Dream Weaver")** — промоция убрана из `on_tick`. Добавлен
`dream_propose()` — вызывается при `FallingAsleep`. `check_promotion` остался
для `propose_to_dream`. `on_tick` шаги 4-5 (промоция) удалены из цикла.

**V1.3 реализация (DEFERRED D1–D4, 2026-05-06):**

| Что | Откуда | Суть |
|-----|--------|------|
| `check_promotion(tick: u64, exp_state, anchors)` | D1 | `tick` вместо signature V1.1 |
| `count_participant_anchors(anchor_id, exp_state, anchor_domain_state)` | D2 | cross-domain: считает участников-анкеров в SUTRA-стейте |
| Все четыре `RuleTrigger` | D3 | StabilityReached, DreamCycle, RepeatedAssembly, HighConfidence — все реализованы |
| GENOME `on_boot` enforcement | D4 | `GenomeIndex::build(genome).check_access(...)` для трёх ресурсов |

---

## Over-Domain Layer: актуальные traits

### OverDomainComponent

```rust
pub trait OverDomainComponent: Send {
    fn name(&self) -> &'static str;
    fn module_id(&self) -> ModuleId;
    fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError>;
    fn on_tick_interval(&self) -> u32 { 1 }  // AxiomEngine вызывает когда tick % interval == 0
    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<(), OverDomainError>;
    fn on_shutdown(&mut self) -> Vec<UclCommand>;
}
```

Инварианты:
- Нет собственного хранилища доменных данных
- Чтение только через `&AshtiCore` (передаётся в `on_tick`)
- Запись только через UCL (`pending_commands`, сливаются через `drain_commands()`)
- Подчинены GUARDIAN

### Weaver

```rust
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

**Не object-safe** из-за `type Pattern`. FrameWeaver хранится по значению в AxiomEngine
(поле `frame_weaver: FrameWeaver`), не как `Box<dyn>`.

### OverDomainError

```rust
pub enum OverDomainError {
    BootFailed(String),
    TickFailed(String),
    GenomeDenied,           // on_boot: GENOME не дал нужных разрешений
}
```

---

## Структуры данных

### FrameWeaver struct

```rust
pub struct FrameWeaver {
    config: FrameWeaverConfig,
    candidates: HashMap<u64, FrameCandidate>,   // key = lineage_hash
    pending_commands: Vec<UclCommand>,
    reactivation_counts: HashMap<u32, u32>,      // anchor_id → count
    dream_cycle_completed: bool,                 // флаг от on_dream_wake()
    pub stats: FrameWeaverStats,
}
```

### FrameWeaverConfig

```rust
pub struct FrameWeaverConfig {
    pub scan_interval_ticks: u32,       // default: 20
    pub stability_threshold: u32,       // default: 3 (дефолтная кристаллизация)
    pub min_participants: usize,         // default: 2
    pub max_storage_depth: u8,          // default: 0 = без ограничений
    pub default_unfold_depth: u8,       // default: 3
    pub max_unfold_depth: u8,           // default: 8
    pub cycle_handling: CycleStrategy,  // default: Allow (EXPERIENCE допускает циклы)
    pub promotion_rules: Vec<PromotionRule>,
    pub crystallization_rules: Vec<CrystallizationRule>,
}
```

### FrameCandidate

```rust
pub struct FrameCandidate {
    pub anchor_position: [i16; 3],   // центроид позиций участников в MAYA
    pub participants: Vec<Participant>,
    pub detected_at_tick: u64,
    pub stability_count: u32,        // сколько сканов подряд паттерн существует без изменений
    pub category: u16,               // FRAME_CATEGORY_SYNTAX (единственная категория V1.3)
    pub lineage_hash: u64,           // FNV-1a по sorted sutra_id участников
    pub confidence: f32,             // среднее Connection.strength по syntactic links (не учитывает head)
}
```

### Participant

```rust
pub struct Participant {
    pub sutra_id: u32,
    pub origin_domain_id: u16,     // обычно MAYA (110)
    pub role_link_type: u16,       // link_type: 0x08XX
    pub layer: u8,                 // (link_type & 0x00F0) >> 4; S1=1 … S8=8
}
```

### CrystallizationRule

```rust
pub struct CrystallizationRule {
    pub id: String,
    pub priority: u8,                     // побеждает правило с наибольшим priority
    pub trigger: RuleTrigger,
    pub conditions: Vec<RuleCondition>,
    pub action: RuleAction,
}

pub enum RuleTrigger {
    StabilityReached(u32),                        // stability_count >= n
    DreamCycle,                                   // dream_cycle_completed == true
    RepeatedAssembly { window_ticks: u32 },       // stability_count * scan_interval >= window
    HighConfidence(f32),                          // confidence >= threshold
}

pub enum RuleCondition {
    DominantLayer(u8),                            // count(layer) * 2 >= total
    MinParticipants(usize),
    RequiresParticipantFromDomain(u16),
    LayerPresent(u8),
    MaxDepth(u8),                                 // max layer index <= d
}

pub enum RuleAction {
    CrystallizeFull,
    CrystallizeAnchorOnly,
    Defer { ticks: u32 },
    Reject,
}
```

### PromotionRule

```rust
pub struct PromotionRule {
    pub id: String,
    pub min_age_ticks: u64,             // current_tick - token.last_event_id >= min_age
    pub min_reactivations: u32,         // reactivation_counts[anchor_id] >= n
    pub min_temperature: u8,
    pub min_mass: u8,
    pub min_participant_anchors: usize, // D2: cross-domain count
    pub requires_codex_approval: bool,  // проверяется GUARDIAN, не здесь
}

// Defaults:
//   min_age_ticks: 100_000
//   min_reactivations: 10
//   min_temperature: 200
//   min_mass: 100
//   min_participant_anchors: 3
//   requires_codex_approval: true
```

### FrameWeaverStats (runtime, feature "adapters")

```rust
pub struct FrameWeaverStats {
    pub scans_performed: u64,
    pub candidates_detected: u64,
    pub candidates_proposed_to_dream: u64,
    pub crystallizations_approved: u64,
    pub crystallizations_vetoed: u64,
    pub frames_in_experience: u64,
    pub frame_reactivations: u64,
    pub promotions_proposed: u64,
    pub promotions_approved: u64,
    pub promotions_vetoed: u64,
    pub frames_in_sutra: u64,
    pub unfold_requests: u64,
    pub cycles_handled: u64,
    pub syntactic_layer_activations: [u8; 8],  // C1: per-layer activation count
}
```

Аналогичный тип `FrameWeaverStats` в `axiom-protocol` (протокольный, публичный):
`{ total_frames, frames_in_sutra, promotions_since_wake, last_crystallization_tick, syntactic_layer_activations }`.

---

## Алгоритм scan_state

```
fn scan_state(&self, maya_state: &DomainState, maya_domain_id: u16) -> Vec<FrameCandidate>

1. Фильтр: syntactic connections (link_type >> 8 == 0x08) && FLAG_ACTIVE
2. Группировать по source_id (Frame-голова)
3. Для каждой группы:
   a. Проверить conns.len() + 1 >= min_participants (участники + голова)
   b. Проверить layers.unique_count >= 2
   c. Собрать participants:
      - голова: { sutra_id=source_id, role=SYNTACTIC_PREDICATE, layer=layer_of(PREDICATE) }
      - targets: { sutra_id=conn.target_id, role=conn.link_type, layer=layer_of(link_type) }
   d. lineage_hash = FNV-1a(sorted(all sutra_ids))
   e. anchor_position = centroid(participants, maya_state.tokens)
   f. confidence = sum(conn.strength) / conns.len()  // головная связь не учитывается
```

**Слой из link_type:** `(link_type & 0x00F0) >> 4`. S1=1…S8=8 (0=неопределён).

**FNV-1a lineage_hash:** сортировка sutra_id перед хешированием — порядок не важен.
Начальное значение: `0xcbf29ce484222325`, множитель: `0x100000001b3`.

### update_candidates

```
1. Убрать из candidates те, которых нет в новом скане (исчезли)
2. Увеличить stability_count у оставшихся
3. Добавить новые (entry.or_insert_with → detected_at_tick = current_tick, stability_count = 1)
```

---

## Алгоритм on_tick

```rust
fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<(), OverDomainError>
```

```
maya_domain_id = ashti.level_id() * 100 + 10   // MAYA
exp_domain_id  = ashti.level_id() * 100 + 9    // EXPERIENCE

1. stats.scans_performed += 1

2. scan_state(maya_state, maya_domain_id) → new_candidates
   (если maya_state None → empty Vec)

3. update_candidates(new_candidates, tick)

4. Для каждого кандидата с stability_count >= stability_threshold:
   
   a. evaluate_crystallization_rules(candidate):
      - Если crystallization_rules пустые:
          stability_count >= stability_threshold → CrystallizeFull
          иначе → Defer
      - Если rules есть: найти правило с наибольшим priority, где
          trigger_matches(rule, candidate) AND conditions_met(rule, candidate)
      
   b. Reject → удалить кандидата, continue
   c. Defer → skip
   d. CrystallizeFull / CrystallizeAnchorOnly:
      - find_existing_anchor(exp_state, lineage_hash)?
          Some(anchor_id) → build_reinforce_command(anchor_id)
                          → reactivation_counts[anchor_id] += 1
                          → stats.frame_reactivations += 1
          None → build_crystallization_commands(candidate, exp_domain_id)
               → stats.crystallizations_approved += 1
               → stats.syntactic_layer_activations[layer].saturating_add(1) per participant
      - pending_commands.extend(cmds)
      - candidates.remove(hash)

5. Обновить stats.frames_in_experience и stats.frames_in_sutra из exp_state

6. dream_cycle_completed = false  // сбросить флаг после первого скана, который его увидел
```

**Промоция в SUTRA не происходит в on_tick** (V1.2 spec, D3). Только через dream_propose().

---

## evaluate_crystallization_rules (детально)

### trigger_matches

```
StabilityReached(n)         → candidate.stability_count >= n
HighConfidence(threshold)   → candidate.confidence >= threshold
DreamCycle                  → self.dream_cycle_completed
RepeatedAssembly{window}    → (stability_count as u64) * max(scan_interval, 1) >= window as u64
```

### conditions_met

```
MinParticipants(n)                  → participants.len() >= n
LayerPresent(l)                     → exists p: p.layer == l
MaxDepth(d)                         → max(p.layer) <= d
DominantLayer(l)                    → count(p.layer==l) * 2 >= total (>50% = доминирующий)
RequiresParticipantFromDomain(did)  → exists p: p.origin_domain_id == did
```

---

## build_crystallization_commands

```
proposed_sutra_id = proposed_id_from_hash(lineage_hash)
    // низшие 32 бита lineage_hash, если 0 → 1

mass = (participants.len() as u8).saturating_mul(16).max(32)

[0] UclCommand(InjectToken, priority=10, flags=FRAME_ANCHOR)
    payload = InjectFrameAnchorPayload {
        lineage_hash, proposed_sutra_id,
        target_domain_id = exp_domain_id,
        type_flags = TOKEN_FLAG_FRAME_ANCHOR | candidate.category,
        position = anchor_position, state = STATE_ACTIVE,
        mass, temperature = 128, valence = 0
    }

[1..N] UclCommand(BondTokens, priority=10) для каждого participant:
    payload = BondTokensPayload {
        source_id = proposed_sutra_id,
        target_id = participant.sutra_id,
        domain_id = exp_domain_id,
        link_type = participant.role_link_type,
        strength = 1.0,
        origin_domain = participant.origin_domain_id  // сохраняется в reserved_gate[0..2]
    }
```

Итого команд = 1 + participants.len().

## build_reinforce_command

```
UclCommand(ReinforceFrame, priority=8)
    payload = ReinforceFramePayload {
        anchor_id,
        delta_mass = 4,
        delta_temperature = 8
    }
```

---

## GENOME on_boot enforcement (D4)

```rust
fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError> {
    let index = GenomeIndex::build(genome);
    let module = ModuleId::FrameWeaver;

    // 1. MAYA Read — сканирование паттернов
    if !index.check_access(module, ResourceId::MayaOutput, Permission::Read) {
        return Err(OverDomainError::GenomeDenied);
    }
    // 2. EXPERIENCE ReadWrite — кристаллизация и реактивация Frame
    if !index.check_access(module, ResourceId::ExperienceMemory, Permission::ReadWrite) {
        return Err(OverDomainError::GenomeDenied);
    }
    // 3. SUTRA Control — промоция через CODEX
    if !index.check_access(module, ResourceId::SutraTokens, Permission::Control) {
        return Err(OverDomainError::GenomeDenied);
    }
    Ok(())
}
```

`Genome::default_ashti_core()` содержит все три разрешения. Тест `on_boot_fails_when_genome_denies_access`
удаляет ExperienceMemory правило и проверяет GenomeDenied.

---

## dream_propose — промоция EXPERIENCE → SUTRA (V1.2/V1.3)

```rust
pub fn dream_propose(&self, ashti: &AshtiCore, tick: u64) -> Vec<DreamProposal>
```

Вызывается из AxiomEngine при переходе `FallingAsleep → Dreaming`.

```
exp_domain_id   = level * 100 + 9
sutra_domain_id = level * 100

Для каждого токена в exp_state:
  - пропустить если не TOKEN_FLAG_FRAME_ANCHOR
  - пропустить если TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE (уже в SUTRA)
  
  participant_anchor_count = count_participant_anchors(
      token.sutra_id, exp_state, sutra_state   // D2: cross-domain
  )
  
  Для каждого PromotionRule:
      qualifies_for_promotion(token, rule, tick, participant_anchor_count)?
          → DreamProposal {
                source: FRAME_WEAVER_ID,
                priority: 100,
                created_at_event: token.last_event_id,
                kind: DreamProposalKind::Promotion {
                    anchor_id: token.sutra_id,
                    source_domain: exp_domain_id,
                    target_domain: sutra_domain_id,
                    rule_id: rule.id.clone()
                }
            }
          break (одна промоция на frame за цикл)
```

Если sutra_state недоступен — `participant_anchor_count = 0`.

---

## qualifies_for_promotion

```rust
fn qualifies_for_promotion(
    &self,
    token: &Token,
    rule: &PromotionRule,
    current_tick: u64,
    participant_anchor_count: u32,
) -> bool

age_ticks = current_tick.saturating_sub(token.last_event_id)
Checks (все обязаны быть выполнены):
  age_ticks >= rule.min_age_ticks
  reactivation_counts[token.sutra_id] >= rule.min_reactivations
  token.temperature >= rule.min_temperature
  token.mass >= rule.min_mass
  participant_anchor_count >= rule.min_participant_anchors as u32
  // requires_codex_approval — проверяется GUARDIAN, не здесь
```

---

## count_participant_anchors (D2)

```rust
fn count_participant_anchors(
    anchor_id: u32,
    exp_state: &DomainState,      // EXPERIENCE: где лежит anchor и его связи
    anchor_domain_state: &DomainState,  // где искать participant-анкеры
                                        // = SUTRA в dream_propose (cross-domain)
                                        // = exp_state в check_promotion (аппроксимация)
) -> u32
```

```
Считает активные syntactic connections из anchor_id в exp_state,
target которых является TOKEN_FLAG_FRAME_ANCHOR в anchor_domain_state.

Фильтр связей:
  c.source_id == anchor_id
  && (c.flags & FLAG_ACTIVE) != 0
  && (c.link_type >> 8) == 0x08

Для каждой такой связи: проверить anchor_domain_state.tokens
  .any(|t| t.sutra_id == c.target_id && (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0)
```

---

## on_dream_wake

```rust
pub fn on_dream_wake(&mut self) {
    self.dream_cycle_completed = true;
}
```

Вызывается AxiomEngine при переходе `Dreaming/Waking → Wake`.
Флаг считывается в следующем `on_tick` через `trigger_matches(DreamCycle)`.
После первого скана, который увидел флаг, сбрасывается в `false`.

---

## restore_frame_from_anchor

Read-only операция — UCL не генерирует.

```rust
pub fn restore_frame_from_anchor(
    anchor_id: u32,
    source_state: &DomainState,
) -> Result<RestoredFrame, RestoreError>

struct RestoredFrame { anchor: Token, anchor_id: u32, category: u16, participants: Vec<Participant> }

enum RestoreError {
    AnchorNotFound,
    NotAFrameAnchor,              // type_flags & TOKEN_FLAG_FRAME_ANCHOR == 0
    DanglingParticipant(u32),     // target_id не найден в source_state.tokens
    InvalidLinkType(u16),         // link_type >> 8 != 0x08
}

Алгоритм:
  1. Найти токен с sutra_id == anchor_id
  2. Проверить TOKEN_FLAG_FRAME_ANCHOR
  3. category = anchor.type_flags & FRAME_CATEGORY_MASK
  4. Обойти connections: source_id == anchor_id && FLAG_ACTIVE
     - link_type >> 8 != 0x08 → Err(InvalidLinkType)
     - target не найден → Err(DanglingParticipant)
     - origin_domain_id = u16::from_be_bytes([reserved_gate[0], reserved_gate[1]])
     - layer = (link_type & 0x00F0) >> 4
```

---

## build_promotion_commands

Генерирует UCL для создания промотированного анкера в SUTRA.
**Используется не в on_tick (D3)**, а AxiomEngine при обработке DreamProposal::Promotion.

```
proposed_sutra_id = proposed_id_from_hash(lineage_hash ^ 0xDEAD_BEEF_0000_0000)
    // отличается от EXPERIENCE-анкера

[0] InjectToken (priority=15, flags=FRAME_ANCHOR):
    type_flags = TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE | category
    state = STATE_LOCKED          // заморожен в SUTRA
    mass = participants.len() * 32
    temperature = 255

[1..N] BondTokens (priority=15) — аналогично кристаллизации
```

Оригинал в EXPERIENCE не трогается. `experience_anchor_id` передаётся но не используется.

---

## find_existing_anchor

```rust
fn find_existing_anchor(experience_state: &DomainState, lineage_hash: u64) -> Option<u32>
```

Ищет в токенах EXPERIENCE тот, у которого:
- `type_flags & TOKEN_FLAG_FRAME_ANCHOR != 0`
- `lineage_hash == lineage_hash` (поле Token, хранит хеш предков)

Возвращает `sutra_id` найденного анкера.

---

## Дедупликация через lineage_hash

1. `scan_state()` вычисляет `lineage_hash = FNV-1a(sorted(sutra_ids))`
2. `candidates: HashMap<u64, FrameCandidate>` — ключ = lineage_hash
3. `find_existing_anchor()` ищет анкер с тем же lineage_hash в EXPERIENCE
4. Если найден → реактивация вместо новой кристаллизации

Гарантии:
- Один и тот же набор участников = один hash вне зависимости от порядка
- Коллизии FNV-1a на 32 бита sutra_id маловероятны, но возможны

---

## Тесты (покрытие)

Все тесты в `frame.rs`, модуль `tests`. Всего ~35 тестов.

**Хеши и детерминизм:**
- `fnv1a_deterministic` — одинаковый вход → одинаковый хэш
- `fnv1a_order_independent` — порядок суtra_id не влияет
- `fnv1a_different_ids_differ` — разные id → разные хэши
- `proposed_id_nonzero_when_low_bits_are_zero` — fallback к 1

**scan_state:**
- `scan_empty_state_returns_no_candidates`
- `scan_single_layer_not_detected` — нужно ≥ 2 разных слоя
- `scan_two_layer_pattern_detected` — базовое обнаружение
- `scan_inactive_connections_ignored` — FLAGS=0
- `scan_non_syntactic_connections_ignored` — link_type >> 8 != 0x08
- `scan_lineage_hash_order_independent`
- `scan_category_is_syntax`
- `scan_state_computes_confidence_from_connection_strength` — (0.8+0.4)/2 = 0.6
- `scan_records_correct_detection_tick`

**on_tick — кристаллизация:**
- `on_tick_crystallizes_at_stability_threshold` — стабилизация → InjectToken
- `on_tick_no_crystallization_below_threshold`
- `on_tick_reactivates_existing_anchor` — повторный паттерн → ReinforceFrame
- `stats_scans_increments_on_tick`
- `stats_candidates_detected_increments` — повторный scan не увеличивает счётчик

**RuleTrigger (D3, все четыре):**
- `trigger_high_confidence_fires_on_threshold` — confidence 0.5→Defer, 0.8→Crystallize
- `trigger_repeated_assembly_fires_after_window` — 4×20=80<100→Defer, 5×20=100→Crystallize
- `trigger_dream_cycle_fires_after_on_dream_wake` — до/после/сброс флага

**check_promotion (D1, D2):**
- `check_promotion_fails_without_reactivations`
- `check_promotion_skips_non_anchors`
- `check_promotion_respects_min_age` — tick=200 (100<500→no), tick=700 (600≥500→yes)
- `check_promotion_respects_min_participant_anchors` — без/с participant-анкерами

**dream_propose (D2 cross-domain):**
- `dream_propose_respects_min_participant_anchors` — без SUTRA-анкеров→нет, с двумя→есть

**GENOME on_boot (D4):**
- `on_boot_passes_with_default_genome`
- `on_boot_fails_when_genome_denies_access` — удаление ExperienceMemory → GenomeDenied

**restore_frame_from_anchor:**
- `restore_simple_frame`
- `restore_returns_error_for_non_anchor`
- `restore_returns_error_for_missing_anchor`
- `restore_detects_dangling_participant`
- `restore_extracts_correct_layers` — 8 участников, по одному на каждый слой

**Промоция через dream_propose:**
- `promotion_creates_sutra_frame_with_participants` — возвращает DreamProposal::Promotion
- `promotion_skipped_for_dangling_anchor` — stats.promotions_proposed остаётся 0
- `promotion_creates_sutra_frame_with_participants` дополнительно проверяет что
  `on_tick` НЕ генерирует BondTokens для промоции (D3: промоция только через DreamCycle)

---

## Константы и флаги (axiom-core)

```rust
pub const TOKEN_FLAG_FRAME_ANCHOR: u16 = 0x0010;
pub const TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE: u16 = 0x0020;
pub const FRAME_CATEGORY_MASK: u16 = 0x00C0;
pub const FRAME_CATEGORY_SYNTAX: u16 = 0x0000;  // пока единственная категория
pub const FRAME_WEAVER_ID: WeaverId = 1;         // для TickSchedule
```

**Синтаксические link_type** (axiom-shell::link_types):
```
SYNTACTIC_PREDICATE = 0x0800 | (layer << 4)  // голова Frame
0x0810 = syntactic, layer 1
0x0820 = syntactic, layer 2
...
0x0880 = syntactic, layer 8
```

---

## Взаимодействие с AxiomEngine

```
boot:
  engine.frame_weaver.on_boot(&engine.genome)?

каждые scan_interval_ticks тиков:
  engine.frame_weaver.on_tick(tick_count, &engine.ashti)?
  let cmds = engine.frame_weaver.drain_commands()
  for cmd in cmds { engine.process_command(&cmd) }

при переходе FallingAsleep → Dreaming:
  let proposals = engine.frame_weaver.dream_propose(&engine.ashti, tick_count)
  // Engine обрабатывает proposals: создаёт SUTRA-анкеры при санкции CODEX

при переходе Dreaming/Waking → Wake:
  engine.frame_weaver.on_dream_wake()

при shutdown:
  let cmds = engine.frame_weaver.on_shutdown()
  // финальная запись незакристаллизованных данных
```

---

## Известные ограничения (не DEFERRED, а архитектурные)

1. **Только FRAME_CATEGORY_SYNTAX** — V1.3 не реализует другие категории Frame.
2. **min_participant_anchors в check_promotion** использует `experience_state` для обоих аргументов
   count_participant_anchors (аппроксимация). `dream_propose` делает полный cross-domain с SUTRA.
3. **build_promotion_commands** помечен `#[allow(dead_code)]` — вызывается AxiomEngine
   при обработке DreamProposal, не напрямую из FrameWeaver.
4. **lineage_hash в Token** (поле `Token.lineage_hash`) переиспользуется для Frame-дедупликации —
   это поле изначально предназначалось для "хэша предков" Token, но Frame-анкеры используют его
   для хранения lineage_hash кандидата.
