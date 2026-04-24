# FrameWeaver V1.1 — Руководство

**Версия:** 1.1  
**Дата:** 2026-04-24  
**Спека:** [FrameWeaver_V1_1.md](../spec/Weaver/FrameWeaver_V1_1.md)  
**Архитектура:** [Over_Domain_Layer_V1_1.md](../spec/Weaver/Over_Domain_Layer_V1_1.md)

---

## Что такое FrameWeaver

FrameWeaver — первый компонент Over-Domain Layer. Он сканирует синтаксические связи в домене MAYA, обнаруживает устойчивые паттерны (Frame-кандидатов) и кристаллизует их в EXPERIENCE в виде Frame-анкеров. Особо устойчивые Frame могут быть промоутированы в SUTRA через CODEX.

### Онтология трёх доменов

```
SUTRA    (domain_id = level*100 + 0)   — вечные истины, STATE_LOCKED
EXPERIENCE (domain_id = level*100 + 9) — накопленный опыт, Frame-анкеры
MAYA     (domain_id = level*100 + 10)  — живое текущее состояние
```

Жизненный цикл Frame:

```
MAYA (0x08 синтаксические связи)
  └─→ scan_state() → FrameCandidate
        └─→ update_candidates() → stability_count++
              └─→ stability >= threshold → CrystallizeFull
                    ├─ нет анкера в EXPERIENCE → InjectFrameAnchor + BondTokens×N
                    └─ анкер есть           → ReinforceFrame (delta_mass+delta_temp)
EXPERIENCE (Frame-анкеры, TOKEN_FLAG_FRAME_ANCHOR)
  └─→ qualifies_for_promotion() → build_promotion_commands()
        └─→ SUTRA: STATE_LOCKED + TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE
```

---

## Архитектура

### Ключевые типы

| Тип | Где | Назначение |
|-----|-----|-----------|
| `FrameWeaver` | `over_domain/weavers/frame.rs` | Основной компонент |
| `FrameCandidate` | `frame.rs` | Незакристаллизованный паттерн |
| `Participant` | `frame.rs` | Токен с синтаксической ролью |
| `FrameWeaverConfig` | `frame.rs` | Конфигурация (scan_interval, threshold…) |
| `FrameWeaverStats` | `frame.rs` | Счётчики для BroadcastSnapshot |
| `OverDomainComponent` | `over_domain/traits.rs` | Object-safe базовый trait |
| `Weaver` | `over_domain/traits.rs` | Расширение с `type Pattern` |

### Место в AxiomEngine

```rust
pub struct AxiomEngine {
    pub ashti: AshtiCore,
    pub guardian: Guardian,
    pub frame_weaver: FrameWeaver,          // по значению, не Box<dyn>
    pub over_domain_components: Vec<Box<dyn OverDomainComponent>>,
    ...
}
```

`FrameWeaver` хранится по значению — это нужно чтобы можно было вызвать `drain_commands()` через типизированный API. Остальные компоненты Over-Domain Layer — через `Box<dyn OverDomainComponent>`.

---

## Как работает сканирование

### Фильтр синтаксических связей

`scan_state` ищет в MAYA активные связи с категорией `0x08` (синтаксические):

```rust
(connection.link_type >> 8) == 0x08   // категория SYNTACTIC
(connection.flags & FLAG_ACTIVE) != 0  // активная
```

Связи группируются по `source_id` (Frame-голова = PREDICATE). Группа проходит проверку:
- `participants >= min_participants` (default: 2, включая голову)
- `distinct layers >= 2` — слой вычисляется как `(link_type & 0x00F0) >> 4`

### lineage_hash — дедупликация по паттерну

```rust
fn fnv1a_lineage_hash(ids: &[u32]) -> u64 {
    // сортировка → FNV-1a
}
```

Одинаковый набор `sutra_id` участников (в любом порядке) → одинаковый `lineage_hash`. Это ключ для:
- Отслеживания стабильности кандидата
- Поиска существующего анкера в EXPERIENCE (`find_existing_anchor`)
- Предотвращения дублей при кристаллизации

### Стабильность кандидата

```
tick 1: scan → кандидат добавлен, stability_count = 1
tick 2: scan → тот же hash → stability_count = 2
tick 3: scan → тот же hash → stability_count = 3 (= threshold)
              → evaluate_crystallization_rules → CrystallizeFull
              → кандидат удалён из candidates map
```

Если паттерн исчезает — кандидат удаляется из map без кристаллизации.

---

## UCL-команды

### InjectFrameAnchorPayload (OpCode::InjectToken + flags::FRAME_ANCHOR)

```rust
pub struct InjectFrameAnchorPayload {  // repr(C), 48 байт
    pub lineage_hash:      u64,        // FNV-1a hash участников
    pub proposed_sutra_id: u32,        // детерминированный из lineage_hash
    pub target_domain_id:  u16,        // 109 = EXPERIENCE
    pub type_flags:        u16,        // TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX
    pub position:          [i16; 3],   // центроид позиций участников
    pub state:             u8,         // STATE_ACTIVE (EXPERIENCE) / STATE_LOCKED (SUTRA)
    pub mass:              u8,         // participants.len() * 16, min 32
    pub temperature:       u8,         // 128
    pub valence:           i8,
    pub reserved:          [u8; 22],
}
```

Важно: поля упорядочены от бо́льшего alignment к меньшему. Без этого `repr(C)` дал бы 52 байта из-за padding'а вокруг `u64`.

### BondTokensPayload (OpCode::BondTokens)

Одна команда на каждого участника Frame. Связывает Frame-анкер с участником:

```
source_id = proposed_sutra_id (анкер)
target_id = participant.sutra_id
link_type = participant.role_link_type (0x08XX)
domain_id = experience_domain_id
```

### ReinforceFramePayload (OpCode::ReinforceFrame)

Усиливает существующий Frame-анкер при повторном обнаружении паттерна:

```rust
pub struct ReinforceFramePayload {
    pub anchor_id:         u32,  // sutra_id анкера в EXPERIENCE
    pub delta_mass:        u8,   // +4
    pub delta_temperature: u8,   // +8
    pub reserved:          [u8; 42],
}
```

---

## Конфигурация

```rust
FrameWeaverConfig {
    scan_interval_ticks:   20,   // сканировать каждые 20 тиков
    stability_threshold:   3,    // N сканов без изменений → кристаллизация
    min_participants:      2,    // минимум участников (включая голову)
    cycle_handling:        CycleStrategy::Allow, // циклы в EXPERIENCE допустимы
    promotion_rules:       vec![PromotionRule::default()],
    crystallization_rules: vec![], // пусто → использовать дефолтный порог
    ...
}
```

### CycleStrategy::Allow

В EXPERIENCE циклические связи разрешены — опыт может быть противоречивым. DAG-инвариант применяется только при промоции в SUTRA (где связи должны образовывать ориентированный граф без циклов).

### PromotionRule (default)

```rust
PromotionRule {
    min_age_ticks:          100_000,
    min_reactivations:      10,
    min_temperature:        200,
    min_mass:               100,
    min_participant_anchors: 3,       // FW-TD-02: не проверяется
    requires_codex_approval: true,
}
```

---

## Интеграция в AxiomEngine

В `handle_tick_forward` (`engine.rs`):

```rust
// Over-Domain: все компоненты через Box<dyn>
let mut components = std::mem::take(&mut self.over_domain_components);
for component in &mut components {
    let interval = component.on_tick_interval();
    if interval > 0 && t % interval as u64 == 0 {
        let _ = component.on_tick(t, &self.ashti);
    }
}
self.over_domain_components = components;

// FrameWeaver: по значению — borrow-safe (frame_weaver и ashti разные поля)
let fw_interval = self.frame_weaver.on_tick_interval();
if fw_interval > 0 && t % fw_interval as u64 == 0 {
    let _ = self.frame_weaver.on_tick(t, &self.ashti);
}
let fw_commands: Vec<UclCommand> = self.frame_weaver.drain_commands();
for fw_cmd in fw_commands {
    let _ = self.process_command(&fw_cmd);
}
```

`drain_commands` использует `mem::take` — O(1), без копирования.

### GENOME permissions

В `Genome::default_ashti_core()` и `config/genome.yaml`:

| Модуль | Ресурс | Право |
|--------|--------|-------|
| FrameWeaver | MayaOutput | Read |
| FrameWeaver | ExperienceMemory | ReadWrite |
| FrameWeaver | SutraTokens | Control (только через CODEX) |
| FrameWeaver | AshtiField | Read |
| FrameWeaver | GenomeConfig | Read |

### BroadcastSnapshot

При включённом feature `adapters` в снэпшот добавляется:

```rust
pub frame_weaver_stats: Option<FrameWeaverStats>
```

Содержит счётчики: `scans_performed`, `candidates_detected`, `crystallizations_approved`, `frame_reactivations`, `promotions_proposed`, `frames_in_experience`, `frames_in_sutra`.

---

## Константы в axiom-core

```rust
TOKEN_FLAG_FRAME_ANCHOR             = 0x0010  // анкер в EXPERIENCE
TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE = 0x0020  // промоутирован в SUTRA
FRAME_CATEGORY_SYNTAX               = 0x0100  // синтаксический паттерн
FRAME_CATEGORY_MASK                 = 0xFF00  // маска для извлечения категории
```

В `axiom-genome/src/types.rs`:
```rust
ModuleId::FrameWeaver = 16
MAX_MODULES           = 17
```

---

## Как добавить новый Weaver

1. Создать файл `crates/axiom-runtime/src/over_domain/weavers/my_weaver.rs`
2. Определить тип паттерна: `pub struct MyPattern { ... }`
3. Реализовать `OverDomainComponent` (name, module_id, on_boot, on_tick, on_shutdown)
4. Реализовать `Weaver` (type Pattern = MyPattern, scan, propose_to_dream, check_promotion)
5. Добавить `pub const MY_WEAVER_ID: WeaverId = 2;`
6. Добавить `ModuleId::MyWeaver` в `axiom-genome/src/types.rs`
7. Добавить access rules в `Genome::default_ashti_core()` и `config/genome.yaml`
8. Добавить `pub mod my_weaver;` в `weavers/mod.rs` и реэкспортировать
9. Решить: хранить по значению в `AxiomEngine` (если нужен typed API) или через `Box<dyn OverDomainComponent>` в `over_domain_components`

---

## Известные ограничения (DEFERRED)

| ID | Проблема |
|----|----------|
| FW-TD-02 | `min_participant_anchors` не проверяется в `qualifies_for_promotion` |
| FW-TD-03 | `Weaver::check_promotion` использует `tick_proxy = 0` вместо реального tick |
| FW-TD-04 | `on_boot` не проверяет GENOME-права для FrameWeaver |
| FW-TD-05 | `propose_to_dream` — DREAM-фаза не реализована, команды пустые |
| FW-TD-06 | Промоция из `on_tick` создаёт SUTRA-анкер без BondTokens к участникам |
| FW-TD-07 | `RuleTrigger::DreamCycle`, `HighConfidence`, `RepeatedAssembly` всегда false |

---

## Тесты

Юнит-тесты в `frame.rs` (26 тестов, `#[cfg(test)] mod tests`):

```
cargo test -p axiom-runtime over_domain::weavers::frame
```

Покрытие: fnv1a_lineage_hash, proposed_id_from_hash, scan_state (6 сценариев), build_crystallization_commands, update_candidates, on_tick (кристаллизация + реактивация), drain_commands, check_promotion, stats.
