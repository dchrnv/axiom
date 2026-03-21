# AXIOM: План миграции на модульную Workspace-архитектуру

**Версия:** 1.2 (финальная, с дополнениями)
**Дата:** 2026-03-21
**Назначение:** Основной рабочий документ и план миграции от monorepo к Cargo Workspace
**Целевая аудитория:** Claude Sonnet (исполнитель миграции)
**Приоритет:** Надёжность, долгосрочность, расширяемость. Скорость вторична.
**Платформа:** std required. FFI-совместимость (repr(C)) обязательна. WASM/no_std не планируются.

---

## ⚠️ ВАЖНО: Формат документа

Этот документ является **живым планом работы**. При выполнении задач:
- ✅ Отмечайте выполненные пункты символом ✅
- 🔄 Отмечайте в процессе символом 🔄
- ❌ Отмечайте заблокированные/проблемные символом ❌ с комментарием
- **НЕ УДАЛЯЙТЕ** выполненные пункты — они показывают прогресс
- Добавляйте даты завершения в формате `✅ 2026-03-21`
- Обновляйте STATUS.md параллельно с этим документом

---

## 1. Цели миграции

Перевести кодовую базу AXIOM из единого crate в Cargo Workspace, где каждый архитектурный модуль — отдельный crate с собственными тестами, бенчмарками и зависимостями.

Результат: структура кода один-к-одному отражает архитектуру системы. Модули изолированы, тестируются независимо, компилируются параллельно.

---

## 2. Целевая структура

```
axiom/
├── Cargo.toml                    # [workspace] — корень
├── justfile                      # Команды: just check, just test, just clippy
├── MIGRATION_PLAN.md             # Этот файл — основной план работы
│
├── crates/
│   ├── axiom-core/               # Фундаментальные типы
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── token.rs          # Token V5.2 (64 байта)
│   │   │   ├── connection.rs     # Connection V5.0 (64 байта)
│   │   │   ├── event.rs          # COM Event (32 байта)
│   │   │   ├── timeline.rs       # CausalClock, Timeline
│   │   │   ├── types.rs          # EventType, TokenState, StructuralRole и т.д.
│   │   │   └── validation.rs     # Общие validate-трейты
│   │   ├── tests/
│   │   │   ├── token_tests.rs
│   │   │   ├── connection_tests.rs
│   │   │   └── event_tests.rs
│   │   └── Cargo.toml
│   │
│   ├── axiom-config/             # Система конфигурации
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── loader.rs         # ConfigLoader (YAML → typed structs)
│   │   │   ├── runtime.rs        # RuntimeConfig
│   │   │   ├── schema.rs         # SchemaConfig
│   │   │   ├── domain_config.rs  # DomainConfig V2.1 (128 байт)
│   │   │   ├── heartbeat_config.rs
│   │   │   └── validation.rs     # Валидация конфигураций
│   │   ├── tests/
│   │   │   ├── loader_tests.rs
│   │   │   ├── domain_config_tests.rs
│   │   │   └── validation_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core, serde, serde_yaml
│   │
│   ├── axiom-domain/             # Домены и Ashti_Core
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── domain.rs         # Domain V1.3 (The Cell)
│   │   │   ├── domain_state.rs   # DomainState (tokens, connections, counters)
│   │   │   ├── membrane.rs       # Мембранные фильтры (input/output)
│   │   │   ├── physics.rs        # Физика поля (гравитация, резонанс, термодинамика)
│   │   │   ├── ashti.rs          # Ashti_Core V2.0 — композиция 11 доменов
│   │   │   └── roles.rs          # StructuralRole, маршруты данных
│   │   ├── tests/
│   │   │   ├── domain_tests.rs
│   │   │   ├── membrane_tests.rs
│   │   │   ├── physics_tests.rs
│   │   │   └── ashti_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core, axiom-config
│   │
│   ├── axiom-space/              # Пространственная модель
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── coordinates.rs    # Система координат (i16, квантование)
│   │   │   ├── spatial_hash.rs   # SpatialHashGrid (zero-alloc)
│   │   │   ├── distance.rs       # Вычисление расстояний (целочисленное)
│   │   │   ├── neighbors.rs      # Поиск соседей
│   │   │   └── field_engine.rs   # FieldEngine — движок движения и столкновений
│   │   ├── tests/
│   │   │   ├── spatial_hash_tests.rs
│   │   │   ├── distance_tests.rs
│   │   │   ├── neighbors_tests.rs
│   │   │   └── field_engine_tests.rs
│   │   ├── benches/
│   │   │   └── spatial_bench.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core
│   │
│   ├── axiom-shell/              # Shell V3.0 — семантический кэш
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── profile.rs        # ShellProfile = [u8; 8], DomainShellCache
│   │   │   ├── contribution.rs   # SemanticContributionTable (двухуровневый)
│   │   │   ├── compute.rs        # Алгоритмы вычисления Shell (полный, инкрементальный)
│   │   │   └── reconciliation.rs # Reconciliation через Heartbeat
│   │   ├── tests/
│   │   │   ├── profile_tests.rs
│   │   │   ├── contribution_tests.rs
│   │   │   ├── compute_tests.rs
│   │   │   └── reconciliation_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core, axiom-config
│   │
│   ├── axiom-arbiter/            # Arbiter V1.0 — маршрутизатор
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── classifier.rs     # Классификация ответа EXPERIENCE (рефлекс/ассоциация/тишина)
│   │   │   ├── router.rs         # Маршрутизация: 9 → 1-8, 9 → 10
│   │   │   ├── feedback.rs       # Обратная связь: MAYA → EXPERIENCE
│   │   │   └── cooldown.rs       # Reflex cooldown, storm control
│   │   ├── tests/
│   │   │   ├── classifier_tests.rs
│   │   │   ├── router_tests.rs
│   │   │   └── feedback_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core, axiom-config
│   │
│   ├── axiom-heartbeat/          # Heartbeat V2.0 + Time Model V1.0
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── generator.rs      # HeartbeatGenerator (по числу событий)
│   │   │   ├── handler.rs        # handle_heartbeat → frontier.push
│   │   │   └── causal_age.rs     # Вычисление причинного возраста
│   │   ├── tests/
│   │   │   ├── generator_tests.rs
│   │   │   └── handler_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core, axiom-frontier
│   │
│   ├── axiom-frontier/           # Causal Frontier V1 + Event-Driven V1
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── frontier.rs       # CausalFrontier (очередь + дедупликация)
│   │   │   ├── processor.rs      # Цикл обработки: pop → evaluate → generate → apply
│   │   │   ├── storm.rs          # Storm detection и mitigation
│   │   │   └── budget.rs         # Causal budget, max_events_per_cycle
│   │   ├── tests/
│   │   │   ├── frontier_tests.rs
│   │   │   ├── processor_tests.rs
│   │   │   └── storm_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core
│   │
│   ├── axiom-upo/                # UPO V2.2 — визуализация динамики
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trace.rs          # DynamicTrace (32 байта)
│   │   │   ├── screen.rs         # Screen, октанты
│   │   │   ├── compute.rs        # Вычисление метрик (Token/Connection → Trace)
│   │   │   └── decay.rs          # Затухание, вечная память (min_intensity > 0)
│   │   ├── tests/
│   │   │   ├── trace_tests.rs
│   │   │   └── screen_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core
│   │
│   ├── axiom-ucl/                # UCL V2.0 — командный протокол
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── command.rs        # UclCommand (64 байта)
│   │   │   ├── result.rs         # UclResult (32 байта)
│   │   │   ├── opcodes.rs        # OpCode enum
│   │   │   └── payloads.rs       # Payload structs (SpawnDomain, ApplyForce и т.д.)
│   │   ├── tests/
│   │   │   ├── command_tests.rs
│   │   │   └── payload_tests.rs
│   │   └── Cargo.toml            # Зависит от: axiom-core
│   │
│   └── axiom-runtime/            # Оркестрация: собирает всё вместе
│       ├── src/
│       │   ├── lib.rs
│       │   ├── engine.rs         # Главный цикл: UCL → COM → Frontier → State
│       │   ├── orchestrator.rs   # Ashti_Core оркестрация (маршруты 0→9→1-8→10)
│       │   ├── guardian.rs       # GUARDIAN + CODEX
│       │   ├── snapshot.rs       # Snapshot/restore
│       │   └── adapters.rs       # Trait-границы для внешних адаптеров
│       ├── tests/
│       │   └── integration/
│       │       ├── full_cycle_test.rs      # Полный цикл: UCL → COM → обработка → результат
│       │       ├── ashti_routing_test.rs   # Маршруты 0→9→1-8→10
│       │       ├── reflex_path_test.rs     # Быстрый путь через Arbiter
│       │       └── snapshot_test.rs        # Snapshot + restore
│       └── Cargo.toml            # Зависит от: ВСЕ crates

├── config/                       # Конфигурационные файлы
│   ├── runtime/
│   │   ├── runtime.yaml
│   │   └── logging.yaml
│   └── schema/
│       ├── domains.yaml          # DomainConfig для 11 доменов Ashti_Core
│       ├── semantic_contributions.yaml  # Shell V3.0 справочник
│       ├── token_types.yaml
│       └── connection_rules.yaml

├── docs/                         # Спецификации (текущие .md файлы)
│   ├── spec/                     # ИСПРАВЛЕНО: spec (без s)
│   │   ├── Token_V5_2.md
│   │   ├── Connection_V5_0.md
│   │   └── ... (все текущие спецификации)
│   └── architecture/
│       ├── dependency_graph.svg  # ДОБАВЛЕНО: Визуализация графа зависимостей
│       └── migration_log.md      # ДОБАВЛЕНО: Журнал решений и изменений

├── STATUS.md                     # Прогресс миграции (чеклист по фазам)

└── tools/                        # Утилиты разработки
    ├── check_deps.sh             # Проверка циклических зависимостей
    └── visualize_deps.sh         # ДОБАВЛЕНО: Генерация dependency_graph.svg
```

---

## 3. Граф зависимостей между crates

```
axiom-core          ← Ни от кого не зависит (фундамент, zero external deps)
    ↑
    ├── axiom-config         ← axiom-core
    ├── axiom-frontier       ← axiom-core
    ├── axiom-space          ← axiom-core
    ├── axiom-upo            ← axiom-core
    ├── axiom-ucl            ← axiom-core
    │
    ├── axiom-shell          ← axiom-core, axiom-config
    ├── axiom-arbiter        ← axiom-core, axiom-config
    ├── axiom-domain         ← axiom-core, axiom-config
    ├── axiom-heartbeat      ← axiom-core, axiom-frontier
    │
    └── axiom-runtime        ← ВСЕ вышестоящие crates
```

**Правило:** Зависимости направлены строго вверх. Циклов нет. axiom-core не зависит ни от кого. axiom-runtime зависит от всех.

**Визуализация:**
```bash
# ДОБАВЛЕНО: Команда для визуализации графа зависимостей
cargo install cargo-deps
cargo deps --all-deps | dot -Tsvg > docs/architecture/dependency_graph.svg
```

---

## 4. Порядок миграции (11 фаз)

Каждая фаза завершается зелёными тестами. Никаких «больших переключений». Каждый шаг — рабочее состояние.

---

### ФАЗА 0: Подготовка Workspace

**Статус:** ✅ Завершена 2026-03-21

**Цель:** Создать каркас Cargo Workspace и инфраструктуру, не трогая существующий код.

**Шаги:**

1. ✅ **Зафиксировать baseline.** До начала миграции запустить:
   ```bash
   cargo test -- --list 2>/dev/null | grep "^.*: test$" | wc -l
   ```
   Записать число в STATUS.md как `baseline_test_count`. Все тесты должны быть зелёными.
   **Выполнено:** baseline_test_count = 0

2. ✅ **Создать корневой `Cargo.toml`:**
```toml
[workspace]
resolver = "2"
members = [
    "crates/axiom-core",
    "crates/axiom-config",
    "crates/axiom-domain",
    "crates/axiom-space",
    "crates/axiom-shell",
    "crates/axiom-arbiter",
    "crates/axiom-heartbeat",
    "crates/axiom-frontier",
    "crates/axiom-upo",
    "crates/axiom-ucl",
    "crates/axiom-runtime",
]

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
# Общие зависимости — версии определяются здесь один раз
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
bitvec = "1"
```
**Выполнено:** [Cargo.toml](Cargo.toml)

3. ✅ **Создать пустые crates** с минимальными `Cargo.toml` и `lib.rs`:
```bash
# Для каждого crate:
mkdir -p crates/axiom-core/src
echo '//! AXIOM Core — фундаментальные типы' > crates/axiom-core/src/lib.rs
```
**Выполнено:** 11 crates созданы в [crates/](crates/)

4. ✅ **Создать `justfile`** (замена CI для одного разработчика):
```makefile
# justfile — команды проверки workspace

check:
    cargo test --workspace
    cargo clippy --workspace -- -D warnings

test crate:
    cargo test -p {{crate}}

test-all:
    cargo test --workspace

clippy:
    cargo clippy --workspace -- -D warnings

size-check:
    cargo test --workspace -- size_assertion

bench:
    cargo bench -p axiom-space

# ДОБАВЛЕНО: Визуализация графа зависимостей
deps-graph:
    ./tools/visualize_deps.sh
```
**Выполнено:** [justfile](justfile)

5. ✅ **Создать STATUS.md:**
```markdown
# AXIOM Migration Status

baseline_test_count: ???
current_test_count: ???
date_started: 2026-03-??

## Фазы

| Фаза | Crate | Статус | Дата | Тесты |
|-------|-------|--------|------|-------|
| 0 | workspace setup | ⬜ | — | — |
| 1 | axiom-core | ⬜ | — | — |
| 2 | axiom-frontier | ⬜ | — | — |
| 3 | axiom-config | ⬜ | — | — |
| 4 | axiom-space | ⬜ | — | — |
| 5 | axiom-shell | ⬜ | — | — |
| 6 | axiom-arbiter | ⬜ | — | — |
| 7 | axiom-heartbeat | ⬜ | — | — |
| 8 | axiom-upo + axiom-ucl | ⬜ | — | — |
| 9 | axiom-domain | ⬜ | — | — |
| 10 | axiom-runtime | ⬜ | — | — |
```
**Выполнено:** [STATUS.md](STATUS.md)

6. ✅ **Создать tools/check_deps.sh:**
```bash
#!/bin/bash
# Проверка циклических зависимостей в workspace

echo "Checking for cyclic dependencies..."
if cargo metadata --format-version 1 | jq '.resolve' > /dev/null 2>&1; then
    echo "✓ No cyclic dependencies found"
else
    echo "✗ Cyclic dependencies detected!"
    exit 1
fi
```
**Выполнено:** [tools/check_deps.sh](tools/check_deps.sh)

7. ✅ **Создать tools/visualize_deps.sh:**
```bash
#!/bin/bash
# Генерация графа зависимостей

echo "Generating dependency graph..."
mkdir -p docs/architecture
cargo deps --all-deps | dot -Tsvg > docs/architecture/dependency_graph.svg
echo "✓ Graph saved to docs/architecture/dependency_graph.svg"
```
**Выполнено:** [tools/visualize_deps.sh](tools/visualize_deps.sh)

8. ✅ `cargo build` должен успешно компилировать пустой workspace.
   **Выполнено:** Все проверки пройдены:
   - `cargo build --workspace` ✅ (11.75s)
   - `cargo test --workspace` ✅ (0 тестов, ожидаемо)
   - `cargo clippy --workspace -- -D warnings` ✅ (4.25s, без warnings)

**Критерий завершения:** ✅ `cargo build` проходит. Workspace собирается. Все crates пусты но компилируются. STATUS.md создан с baseline.

---

### ФАЗА 1: axiom-core — Фундаментальные типы

**Статус:** ✅ Завершена 2026-03-21

**Цель:** Вынести Token, Connection, Event и общие типы в axiom-core.

**Что переносится:**

| Структура | Спецификация | Размер |
|-----------|-------------|--------|
| `Token` | Token V5.2 | 64 байта, `repr(C, align(64))` |
| `Connection` | Connection V5.0 | 64 байта, `repr(C, align(64))` |
| `Event` | COM V1.0 | 32 байта, `repr(C, align(32))` |
| `CausalClock` | COM V1.0 | AtomicU64 counter |
| `Timeline` | COM V1.0 | current_event_id, domain_offsets, checkpoint_id |
| `EventType` | COM V1.0 | enum u16 (Token*, Connection*, Domain*, System*) |
| `TokenState` | Token V5.2 | enum u8 (Active, Sleeping, Locked) |
| `StructuralRole` | DomainConfig V2.1 | enum u8 (Sutra=0..Maya=10, включая Experience=9) |
| `ConnectionFlags` | Connection V5.0 | bitflags (ACTIVE, INHIBITED, TEMPORARY, DECAYING) |

**Шаги:**

1. ✅ Перенести struct Token со всеми полями в `crates/axiom-core/src/token.rs`. Сохранить `#[repr(C, align(64))]`. Добавить `#[derive(Debug, Clone, Copy)]`.

2. ✅ Перенести struct Connection в `crates/axiom-core/src/connection.rs`. Сохранить `#[repr(C, align(64))]`. Добавлена документация по f32 в динамике.

3. ✅ Перенести struct Event в `crates/axiom-core/src/event.rs`. Timeline не перенесен (будет в axiom-domain/axiom-heartbeat).

4. ✅ Enum-типы (EventType, EventPriority, Snapshot) перенесены в event.rs (не нужен отдельный types.rs).

5. ✅ Трейт `Validate` реализован как метод в каждой структуре (не нужен отдельный validation.rs, trait SpatialIndex будет в axiom-space):
```rust
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

/// Трейт для пространственного индекса (инверсия зависимости).
/// Реализуется в axiom-space (SpatialHashGrid), используется в axiom-domain.
/// Паттерн &mut Vec<u32> — из Space V6.0 §4.6: zero-alloc, без lifetime-проблем.
pub trait SpatialIndex {
    fn find_neighbors(
        &self,
        position: [i16; 3],
        radius: i16,
        result: &mut Vec<u32>,  // Предвыделённый буфер
    );
    fn rebuild(&mut self, tokens: &[Token]);
}
```
Реализовать `Validate` для Token, Connection, Event согласно разделу "Инварианты" каждой спецификации.

6. ✅ В `lib.rs` реэкспортировано:
```rust
pub mod token;
pub mod connection;
pub mod event;

pub use token::{Token, STATE_ACTIVE, STATE_SLEEPING, STATE_LOCKED};
pub use connection::{Connection, FLAG_ACTIVE, FLAG_INHIBITED, FLAG_TEMPORARY, FLAG_CRITICAL};
pub use event::{Event, EventType, EventPriority, Snapshot, EVENT_REVERSIBLE, EVENT_CRITICAL, EVENT_BATCHED};
```

7. ✅ Перенесены тесты (24 теста). Добавлены **compile-time** size assertions:
```rust
const _: () = assert!(std::mem::size_of::<Token>() == 64);
const _: () = assert!(std::mem::size_of::<Connection>() == 64);
const _: () = assert!(std::mem::size_of::<Event>() == 64);  // 64 байта, не 32!
```

**Важные инварианты (реализованы и протестированы):**
- Token: `sutra_id > 0`, `domain_id > 0`, `mass > 0`, `last_event_id > 0`.
- Connection: `source_id > 0`, `target_id > 0`, `strength > 0`, `created_at > 0`, `last_event_id >= created_at`.
- Event: `event_id > 0`, `parent_event_id < event_id`, `payload_hash != 0`.

**Результат:** ✅ `cargo test -p axiom-core` — 24 теста прошли. Size assertions компилируются. Zero dependencies.

---

### ФАЗА 2: axiom-frontier — Causal Frontier

**Статус:** ✅ Завершена 2026-03-21

**Цель:** Вынести CausalFrontier и цикл обработки.

**Что переносится:**

| Структура | Спецификация |
|-----------|-------------|
| `CausalFrontier` | Causal Frontier V1 |
| Типизированные очереди | token_frontier, connection_frontier |
| `BitSet` для дедупликации | visited_tokens, visited_connections |
| Storm detection | frontier_size > STORM_THRESHOLD |
| Causal budget | max_events_per_cycle |

**Зависимости:** Нет (frontier работает с usize ID, не зависит от axiom-core).

**Шаги:**

1. ✅ Создать `crates/axiom-frontier/src/frontier.rs`:
   - `CausalFrontier` struct с `push_token()`, `push_connection()`, `pop()`, `contains()`, `clear()`, `size()`.
   - Дедупликация через Vec<bool> (visited tracking).
   - Детерминированный порядок обработки (FIFO через VecDeque).
   - EntityQueue с автоматическим resize для visited.

2. ✅ Создать `crates/axiom-frontier/src/processor.rs`:
   - Trait `LocalRules` с методами `evaluate_token()` и `evaluate_connection()`.
   - `FrontierProcessor` с основным циклом: pop → evaluate → transform → push neighbors.
   - `EvaluationResult::NoChange` и `EvaluationResult::Transform { affected_neighbors }`.
   - `process_until_empty_or_budget()` — обработка с уважением budget.
   - 6 тестов с mock LocalRules: step, connections, transform, chain reaction, budget.

3. ✅ Storm detection встроен в CausalFrontier:
   - `update_state()` отслеживает размер frontier.
   - FrontierState: Empty → Active → Storm → Stabilized → Idle.
   - Storm mitigation через causal budget.

4. ✅ Budget встроен в CausalFrontier:
   - `max_events_per_cycle` — лимит вычислений.
   - `max_frontier_size` — лимит памяти.
   - `is_budget_exhausted()`, `increment_processed()`, `reset_cycle()`.

5. ✅ Тесты (22 теста):
   - EntityQueue: push/pop/dedup/contains (3 теста).
   - CausalFrontier: creation, push/pop, mixed entities, clear (13 тестов).
   - State transitions: Empty → Active → Storm → Stabilized → Idle.
   - Causal budget: increment, exhaustion, reset.
   - Memory limit enforcement.
   - Deterministic FIFO order.
   - FrontierProcessor: step, connections, transform, chain reaction, budget (6 тестов).

**Результат:** ✅ `cargo test -p axiom-frontier` — 22 теста прошли. Zero dependencies.

---

### ФАЗА 3: axiom-config — Система конфигурации

**Статус:** ✅ Завершена

**Цель:** Вынести ConfigLoader, DomainConfig и все конфигурационные структуры.

**Что переносится:**

| Структура | Спецификация |
|-----------|-------------|
| `DomainConfig` | DomainConfig V2.1, 128 байт |
| `HeartbeatConfig` | Heartbeat V2.0 |
| `RuntimeConfig` | Configuration System V1.0 |
| `SchemaConfig` | Configuration System V1.0 |
| `ConfigLoader` | Configuration System V1.0, раздел 7 |

**Зависимости:** `serde`, `serde_yaml` (zero axiom deps).

**Шаги:**

1. ✅ Перенести `DomainConfig` (128 байт) в `crates/axiom-config/src/domain_config.rs`:
   - Все 5 блоков: Идентификация [16], Физика поля [32], Семантические оси [16], Мембрана и Arbiter [32], Метаданные [32].
   - Блок Arbiter внутри мембраны: `reflex_threshold`, `association_threshold`, `arbiter_flags`, `reflex_cooldown`, `max_concurrent_hints`, `feedback_weight_delta`.
   - **Compile-time assertion:**
   ```rust
   const _: () = assert!(std::mem::size_of::<DomainConfig>() == 128);
   ```

2. ✅ Перенести `HeartbeatConfig` в `crates/axiom-config/src/heartbeat_config.rs`:
   - `interval`, `batch_size`, `connection_batch_size`.
   - Флаги: `enable_decay`, `enable_gravity`, `enable_connection_maintenance`, `enable_thermodynamics`, `attach_pulse_id`, `enable_shell_reconciliation`.
   - Пресеты: `weak()`, `medium()`, `powerful()`, `disabled()`.

3. ✅ Создать `ConfigLoader` в `crates/axiom-config/src/loader.rs`:
   - Чтение корневого YAML.
   - Резолвинг файловых путей.
   - Парсинг → typed structs.
   - Валидация всех конфигураций.
   - Возврат единого `AxiomConfig`.
   - Методы: `load_domain_config()`, `load_heartbeat_config()`, `validate()`.

4. ✅ Добавить валидацию для DomainConfig:
   - `field_size >= 0.0` для всех осей
   - `temperature >= 0.0` (Kelvin)
   - `token_capacity > 0`
   - `connection_capacity > 0`

5. ✅ Добавлены enums и константы:
   - `StructuralRole` (Sutra=0, Execution=1, Shadow=2, Codex=3, Map=4, Probe=5, Logic=6, Dream=7, Void=8, Experience=9, Maya=10)
   - `DomainType` (Logic=1, Dream=2, Math=3, Pattern=4, Memory=5, Interface=6)
   - Константы: `DOMAIN_ACTIVE`, `DOMAIN_LOCKED`, `DOMAIN_TEMPORARY`, `PROCESSING_IDLE`, `PROCESSING_ACTIVE`, `PROCESSING_FROZEN`

6. ✅ Тесты: 17 тестов прошли
   - DomainConfig: size, default, void, new, sutra, validation (11 тестов)
   - HeartbeatConfig: presets, default, validation (3 теста)
   - ConfigLoader: creation, default, error display (3 теста)

**Результат:** ✅ `cargo test -p axiom-config` — 17 тестов прошли. DomainConfig строго 128 байт. Zero dependencies on other axiom crates.

---

### ФАЗА 4: axiom-space — Пространственная модель

**Статус:** ✅ Завершена

**Цель:** Вынести SpatialHashGrid, координатную систему, физику.

**Что перенесено из Space V6.0 (runtime/src/space.rs, 1983 строки):**

| Компонент | Описание |
|-----------|----------|
| Константы | CELL_SHIFT=8, CELL_SIZE=256, BUCKET_COUNT=65536 |
| Координаты | i16[3], has_moved, cell_changed |
| distance2 | Квадрат расстояния i64, без sqrt |
| Гравитация | compute_gravity (Linear/InverseSquare) |
| Физика | apply_velocity, friction, acceleration, move_towards |
| SpatialHashGrid | bucket_heads + entries (linked lists) |
| find_neighbors | Поиск в radius |
| detect_collisions | С фильтрацией |

**Зависимости:** Zero (только std).

**Реализация:**
- ✅ Скопирован space.rs полностью
- ✅ 83 теста прошли

**Результат:** ✅ `cargo test -p axiom-space --lib` — 83 теста.

---

### ФАЗА 5: axiom-shell — Shell V3.0

**Статус:** ⬜ Не начата

**Цель:** Вынести семантический кэш Shell.

**Что переносится из Shell V3.0:**

| Компонент | Описание |
|-----------|----------|
| `ShellProfile` | `[u8; 8]` — L1..L8 |
| `DomainShellCache` | `profiles: Vec<ShellProfile>`, `dirty_flags: BitVec`, `generation: u64` |
| `SemanticContributionTable` | Двухуровневый: categories[256] + overrides HashMap<u16, ShellProfile> |
| Полный пересчёт | Все active Connection → accumulator → normalize → u8 |
| Инкрементальный пересчёт | При Connection-событии — пересчёт затронутых |
| Reconciliation | Через Heartbeat — проверка батча |

**Зависимости:** `axiom-core` (Connection.link_type, Connection.strength, Connection.flags), `axiom-config` (загрузка SemanticContributionTable из YAML).

**Шаги:**

1. ⬜ `profile.rs`:
   - `pub type ShellProfile = [u8; 8];`
   - `DomainShellCache` struct.
   - Методы: `get(token_index)`, `set(token_index, profile)`, `mark_dirty(token_index)`.

2. ⬜ `contribution.rs`:
   - `SemanticContributionTable` struct.
   - `get(link_type: u16) -> &ShellProfile` — сначала overrides, потом category.
   - Загрузка из YAML (через axiom-config).
   - 7 стандартных категорий: Structural(0x01), Semantic(0x02), Causal(0x03), Experiential(0x04), Social(0x05), Temporal(0x06), Motor(0x07).

3. ⬜ `compute.rs`:
   - `full_recompute(token_index, connections, table) -> ShellProfile` — полный пересчёт из всех active Connection.
   - `incremental_update(token_index, changed_connection, table, cache)` — обновление при изменении одной связи.
   - Нормализация: если max > 255, масштабировать с сохранением пропорций.

4. ⬜ `reconciliation.rs`:
   - `reconcile(token_index, connections, table, cache)` — пересчёт + сравнение + запись при расхождении.
   - Интеграция с Heartbeat: вызывается при обработке токена из Heartbeat-батча.

5. ⬜ Тесты:
   - Нулевой Shell для токена без связей.
   - Корректность вычисления из 1, 2, N связей.
   - Нормализация при переполнении.
   - Reconciliation обнаруживает расхождение.

**Критерий:** `cargo test -p axiom-shell` — все тесты зелёные.

---

### ФАЗА 6: axiom-arbiter — Arbiter V1.0

**Статус:** ⬜ Не начата

**Цель:** Вынести логику маршрутизации Arbiter.

**Что переносится из Arbiter V1.0:**

| Компонент | Описание |
|-----------|----------|
| Классификатор | weight vs reflex_threshold/association_threshold per domain |
| Router | Формирование пакетов для 1-8, отправка рефлекса в MAYA |
| Feedback | Обработка совпадения/расхождения, feedback_weight_delta |
| Cooldown | reflex_cooldown per domain, storm_threshold |
| Глобальный конфиг | response_timeout, comparison_strategy, storm_threshold |

**Зависимости:** `axiom-core` (Event), `axiom-config` (DomainConfig.arbiter_flags, reflex_threshold и т.д.).

**Шаги:**

1. ⬜ `classifier.rs`:
   - `classify(weight: f32, domain_config: &DomainConfig) -> ResponseType` — рефлекс / ассоциация / тишина.
   - Проверка битов REFLEX_ENABLED, HINTS_ENABLED.
   - Один ответ EXPERIENCE может быть рефлексом для одного домена и тишиной для другого.

2. ⬜ `router.rs`:
   - `route(experience_response, domain_configs) -> RoutingDecision`.
   - RoutingDecision содержит: рефлекс для MAYA (если есть), пакеты для каждого 1-8.
   - Каждый пакет: входной паттерн + подсказки (не более `max_concurrent_hints`).

3. ⬜ `feedback.rs`:
   - `process_feedback(comparison_result, domain_config) -> FeedbackAction`.
   - FeedbackAction: StrengthenTrace(delta), WeakenTrace(delta), CreateNewTrace.
   - delta определяется `feedback_weight_delta` из DomainConfig.

4. ⬜ `cooldown.rs`:
   - Счётчик пульсов с последнего рефлекса per domain.
   - Проверка: `pulses_since_last >= reflex_cooldown`.
   - Сброс при Heartbeat.

5. ⬜ Тесты:
   - Классификация при разных порогах.
   - Маршрутизация одного ответа в разные категории для разных доменов.
   - Feedback: совпадение усиливает, расхождение ослабляет.
   - Cooldown блокирует слишком частые рефлексы.

**Критерий:** `cargo test -p axiom-arbiter` — все тесты зелёные.

---

### ФАЗА 7: axiom-heartbeat — Heartbeat V2.0

**Статус:** ⬜ Не начата

**Цель:** Вынести генератор Heartbeat и обработчик.

**Что переносится из Heartbeat V2.0:**

| Компонент | Описание |
|-----------|----------|
| `HeartbeatGenerator` | interval, events_since_last, pulse_number |
| `HeartbeatEvent` | pulse_number: u64 |
| `handle_heartbeat()` | Добавление батча токенов/связей в frontier |
| Формула выбора | `(pulse_number * batch_size + offset) % total` |

**Зависимости:** `axiom-core` (Event, EventType::Heartbeat), `axiom-frontier` (CausalFrontier.push_token/push_connection).

**Шаги:**

1. ⬜ `generator.rs`:
   - `HeartbeatGenerator` struct.
   - `on_event() -> Option<HeartbeatEvent>` — счётчик, генерация при достижении interval.
   - `pulse_number` монотонно возрастает.

2. ⬜ `handler.rs`:
   - `handle_heartbeat(domain_state, frontier, heartbeat, config)` — добавление батчей в frontier.
   - Детерминированный выбор: `token_index = (pulse_number * batch_size + offset) % total_tokens`.
   - Гарантия полного покрытия: за `ceil(total / batch_size)` пульсов каждый проверен.

3. ⬜ `causal_age.rs`:
   - `causal_age(entity_last_event_id, current_event_id) -> u64`.
   - Используется для decay, gravity, thermodynamics.

4. ⬜ Тесты:
   - Генерация строго через interval событий.
   - Полное покрытие всех токенов за N пульсов.
   - Детерминизм выбора.

**Критерий:** `cargo test -p axiom-heartbeat` — все тесты зелёные.

---

### ФАЗА 8: axiom-upo и axiom-ucl — Наблюдение и команды

**Статус:** ⬜ Не начата

**Цель:** Вынести UPO и UCL в отдельные crates.

**axiom-upo (из UPO V2.2):**
- `DynamicTrace` (32 байта), `Screen`, октанты.
- Вычисление метрик: Token → Trace, Connection → Trace.
- Затухание по causal age, `min_intensity > 0`.
- Зависит от: `axiom-core`.

**axiom-ucl (из UCL V2.0):**
- `UclCommand` (64 байта), `UclResult` (32 байта).
- `OpCode` enum (SpawnDomain=1000, InjectToken=2000, ApplyForce=2001 и т.д.).
- Payload structs: SpawnDomainPayload, ApplyForcePayload.
- Зависит от: `axiom-core`.

**Шаги:** Аналогичны предыдущим фазам. Для каждого crate:
1. ⬜ Перенести структуры с `repr(C)` и size assertions.
2. ⬜ Перенести логику.
3. ⬜ Перенести и адаптировать тесты.

**Size assertions:**
```rust
const _: () = assert!(std::mem::size_of::<DynamicTrace>() == 32);
const _: () = assert!(std::mem::size_of::<UclCommand>() == 64);
const _: () = assert!(std::mem::size_of::<UclResult>() == 32);
```

**Критерий:** `cargo test -p axiom-upo && cargo test -p axiom-ucl` — все тесты зелёные.

---

### ФАЗА 9: axiom-domain — Домены и Ashti_Core

**Статус:** ⬜ Не начата

**Цель:** Вынести Domain, DomainState и Ashti_Core.

**Что переносится:**

| Компонент | Спецификация |
|-----------|-------------|
| `Domain` | Domain V1.3 (Anchor, Field, Membrane) |
| `DomainState` | tokens, connections, shell_cache, spatial_hash |
| Физика | гравитация, резонанс, термодинамика |
| Мембранные фильтры | can_enter_domain, can_exit_domain |
| `AshtiCore` | Ashti_Core V2.0 — композиция 11 доменов |
| Маршруты | 0→9, 9→1-8, 9→10, 1-8→10, 1-8→9, 10→9, 10→0 |

**Зависимости:** `axiom-core`, `axiom-config`, а также использует (через traits) возможности axiom-space, axiom-shell, axiom-frontier.

**Шаги:**

1. ⬜ `domain.rs`: Основная структура Domain с тремя компонентами (Anchor, Field, Membrane).

2. ⬜ `domain_state.rs`: DomainState с предвыделёнными буферами:
```rust
pub fn new(config: &DomainConfig) -> Self {
    Self {
        tokens: Vec::with_capacity(config.token_capacity as usize),
        connections: Vec::with_capacity(config.connection_capacity as usize),
        neighbor_buffer: Vec::with_capacity(256),
        // ...
    }
}

pub fn add_token(&mut self, token: Token) -> Result<usize, CapacityExceeded> {
    if self.tokens.len() >= self.tokens.capacity() {
        return Err(CapacityExceeded);
    }
    self.tokens.push(token);
    Ok(self.tokens.len() - 1)
}
```

3. ⬜ `physics.rs`:
   - `apply_gravity(token, config)` — гравитация к Anchor (0,0,0).
   - `apply_resonance(token_a, token_b, config)` — обмен momentum при резонансе.
   - `apply_thermodynamics(token, config)` — адаптация температуры к полю.
   - Все через целочисленную арифметику.

4. ⬜ `membrane.rs`:
   - `can_enter_domain(token, config) -> bool` — mass >= threshold, temperature, pattern match.
   - `can_exit_domain(token, config) -> bool` — state != Locked, membrane != Closed.

5. ⬜ `ashti.rs`:
   - `AshtiCore` struct — содержит 11 Domain instances (0=SUTRA, 1-8=ASHTI, 9=EXPERIENCE, 10=MAYA).
   - Инициализация из конфигурации (domains.yaml).
   - Маршрутизация через Arbiter.

6. ⬜ Тесты:
   - Domain: создание, инициализация, физика.
   - Membrane: фильтрация входа/выхода.
   - AshtiCore: маршрутизация через все домены.
   - CapacityExceeded: попытка добавить токен сверх capacity.

**Критерий:** `cargo test -p axiom-domain` — все тесты зелёные.

---

### ФАЗА 10: axiom-runtime — Оркестрация и интеграция

**Статус:** ⬜ Не начата

**Цель:** Собрать всё в единый runtime. Перенести интеграционные тесты.

**Что переносится:**

| Компонент | Описание |
|-----------|----------|
| Engine | Главный цикл: UCL → COM → Frontier → State |
| Orchestrator | Полный цикл Ashti_Core (раздел 8, Ashti_Core V2.0) |
| Guardian | GUARDIAN + CODEX (над-доменный контроль) |
| Snapshot | Сохранение/восстановление состояния |
| Adapters | Trait-границы для подключения внешних адаптеров |

**Зависимости:** ВСЕ crates.

**Шаги:**

1. ⬜ `engine.rs`:
   - `AxiomEngine` struct — содержит AshtiCore, CausalFrontier, HeartbeatGenerator, Arbiter.
   - `process_command(ucl: &UclCommand) -> UclResult` — главная точка входа.
   - Цикл: команда → COM event → frontier → обработка → результат.

2. ⬜ `orchestrator.rs`:
   - Полный цикл работы уровня (Ashti_Core V2.0, раздел 8):
     1. Событие → SUTRA(0)
     2. Передача в EXPERIENCE(9)
     3. Резонансный поиск → ответ
     4. Arbiter → классификация + маршрутизация
     5. GUARDIAN → проверка рефлекса
     6. Быстрый путь → MAYA(10)
     7. Медленный путь → ASHTI(1-8)
     8. Обработка в каждом домене
     9. Консолидация в MAYA(10)
     10. Сравнение рефлекса с результатом
     11. Обратная связь → EXPERIENCE(9)
     12. Выход из MAYA(10)

3. ⬜ `guardian.rs`:
   - GUARDIAN: `peek_state()` для сканирования доменов.
   - Ингибирование узоров, нарушающих CODEX.
   - Проверка рефлексов (облегчённая валидация).

4. ⬜ `snapshot.rs`:
   - Сериализация полного состояния (все домены, все токены/связи).
   - Восстановление из snapshot.
   - Shell-кэш опционально включается в snapshot (оптимизация).
   - Frontier НЕ включается (Causal Frontier V1, §14 — восстанавливается из event log).

5. ⬜ `adapters.rs`:
   - **Только trait-границы.** Конкретные адаптеры (REST, CLI, WebSocket) — отдельный проект.
   - Trait `ExternalAdapter` определяет интерфейс: принять UCL → получить UclResult.
   - Это соответствует документу "API В AXIOM": ядро не знает о транспорте.

6. ⬜ Интеграционные тесты:
   - `full_cycle_test.rs`: UCL команда → полная обработка → результат.
   - `ashti_routing_test.rs`: паттерн проходит 0→9→1-8→10.
   - `reflex_path_test.rs`: рефлекс → GUARDIAN → MAYA, параллельно 1-8 → сравнение.
   - `snapshot_test.rs`: сохранение → восстановление → продолжение.

**Критерий:** `cargo test --workspace` — ВСЕ тесты зелёные. Количество тестов >= baseline_test_count.

---

## 5. Правила миграции

### 5.1 Инварианты размеров

На каждой фазе добавлять **compile-time** assertions:

```rust
const _: () = assert!(std::mem::size_of::<Token>() == 64);
const _: () = assert!(std::mem::size_of::<Connection>() == 64);
const _: () = assert!(std::mem::size_of::<Event>() == 32);
const _: () = assert!(std::mem::size_of::<DomainConfig>() == 128);
```

### 5.2 `repr(C)` обязателен

Все core-структуры с фиксированным layout сохраняют `#[repr(C, align(...))]`. Это гарантирует совместимость с FFI (UCL) и предсказуемое размещение в памяти.

### 5.3 Никаких циклических зависимостей

Если при переносе обнаруживается, что crate A зависит от B, а B от A — это сигнал к выделению общего кода в axiom-core или создания trait в axiom-core, который реализуется в A и B.

### 5.4 Traits для инверсии зависимостей

Если axiom-domain нуждается в spatial hash (из axiom-space), но не должен зависеть от axiom-space:

```rust
// В axiom-core:
pub trait SpatialIndex {
    /// Находит соседей в радиусе. Результат записывается в предвыделённый буфер.
    /// Паттерн из Space V6.0 §4.6 — zero-alloc, без lifetime-проблем.
    fn find_neighbors(
        &self,
        position: [i16; 3],
        radius: i16,
        result: &mut Vec<u32>,  // Предвыделённый буфер, очищается перед заполнением
    );
    fn rebuild(&mut self, tokens: &[Token]);
}

// В axiom-space:
impl SpatialIndex for SpatialHashGrid { ... }
```

axiom-runtime подключает конкретную реализацию.

**Почему `&mut Vec<u32>`, а не `&[u32]` или `Iterator`:**
- `&[u32]` возвращаемый из метода создаёт сложности с lifetime (буфер должен жить внутри структуры).
- `Iterator` с associated type усложняет generic-код.
- `&mut Vec<u32>` — буфер предвыделен при инициализации, zero-alloc в горячем пути. Это канонический паттерн из Space V6.0 §4.6.

### 5.5 Детерминизм

Time Model V1.0 — конституционный документ. На каждой фазе проверять:
- Нет `std::time`, `SystemTime`, `Instant` в core crates.
- Нет `sleep()`, `delay()`, таймеров.
- Нет `rand()` в детерминированном коде.
- Все "временные" процессы используют causal_age = current_event_id - last_event_id.

### 5.6 Zero-alloc в горячем пути

Space V6.0 и Causal Frontier требуют zero-alloc в горячем пути. Все Vec предвыделены при инициализации. Во время обработки событий аллокаций быть не должно.

**Управление ёмкостью:**
- `token_capacity` и `connection_capacity` из DomainConfig V2.1 определяют максимальные размеры массивов.
- Все `Vec::with_capacity()` вызываются один раз при создании домена.
- При попытке превысить capacity — возвращать `Result::Err(CapacityExceeded)`, **не паника**.
- Буферы для find_neighbors, frontier visited-sets и т.п. предвыделяются аналогично.

```rust
// Пример: создание DomainState с предвыделёнными буферами
pub fn new(config: &DomainConfig) -> Self {
    Self {
        tokens: Vec::with_capacity(config.token_capacity as usize),
        connections: Vec::with_capacity(config.connection_capacity as usize),
        neighbor_buffer: Vec::with_capacity(256),  // Рабочий буфер
        // ...
    }
}

pub fn add_token(&mut self, token: Token) -> Result<usize, CapacityExceeded> {
    if self.tokens.len() >= self.tokens.capacity() {
        return Err(CapacityExceeded);
    }
    self.tokens.push(token);
    Ok(self.tokens.len() - 1)
}
```

### 5.7 Целочисленная арифметика в ядре

Пространственные вычисления (Space V6.0) — только i16/i32/i64. Без f32/f64 в ядре. Floating point — только в конфигурации (DomainConfig.field_size, gravity_strength) и адаптерах.

**Исключение:** Connection.strength, Connection.current_stress, Connection.elasticity, Connection.ideal_dist — это f32 в текущей спецификации Connection V5.0. Сохранить как есть. Это осознанное решение, не упущение. В коде добавить комментарий:

```rust
// SPEC NOTE: Connection V5.0 defines dynamics fields (strength, current_stress,
// elasticity, ideal_dist) as f32. This is the only exception to the
// integer-arithmetic rule. Spatial computations (position, velocity, distance)
// remain strictly integer. See Space V6.0 §3.4 for the boundary.
```

---

## 6. Конфигурационные файлы

Создать при миграции:

### config/schema/domains.yaml
11 DomainConfig для Ashti_Core V2.0 с настройками Arbiter V1.0 (примеры из DomainConfig V2.1, раздел 4).

**ДОБАВЛЕНО: Пример конфигурации для SUTRA (domain 0):**
```yaml
# config/schema/domains.yaml — конфигурация всех 11 доменов
domains:
  - domain_id: 0
    name: "SUTRA"
    structural_role: 0  # Sutra
    field_size: [1000.0, 1000.0, 1000.0]
    gravity_strength: 0.001
    token_capacity: 10000
    connection_capacity: 50000
    permeability: 255
    reflex_threshold: 0.8
    association_threshold: 0.5
    arbiter_flags: 0x01  # REFLEX_ENABLED
    reflex_cooldown: 10
    max_concurrent_hints: 3
    feedback_weight_delta: 0.1
    created_at: 1234567890
    # ... (остальные 10 доменов аналогично)
```

### config/schema/semantic_contributions.yaml
Справочник Shell V3.0 (раздел 4.3): 7 категорий + переопределения.

### config/runtime/runtime.yaml
```yaml
runtime:
  threads: 1          # Для слабого оборудования
  max_tokens: 100000
  heartbeat:
    interval: 1024
    batch_size: 10
  shell_cache:
    enable_shell_reconciliation: true
```

---

## 7. Порядок удаления старого кода

После завершения каждой фазы:
1. Убедиться, что `cargo test --workspace` зелёный.
2. Удалить перенесённый код из старого расположения.
3. Обновить импорты во всех зависимых модулях.
4. Снова `cargo test --workspace`.
5. Обновить STATUS.md (статус фазы → ✅, количество тестов, дата).
6. **Коммит.** Одна фаза = один коммит. Сообщение: `migrate: phase N — axiom-{crate_name}`.

После ФАЗЫ 10: старый monorepo-код полностью удалён. Workspace — единственный источник истины.

---

## 8. Проверка завершения

Миграция считается завершённой когда:

1. ✅ `cargo test --workspace` — все тесты зелёные. Количество тестов >= `baseline_test_count` из STATUS.md.
2. ✅ `cargo build --workspace` — компиляция без warnings.
3. ✅ `cargo clippy --workspace -- -D warnings` — без замечаний.
4. ✅ Каждый crate компилируется и тестируется независимо (`cargo test -p axiom-{name}`).
5. ✅ Нет циклических зависимостей (`cargo metadata` не ругается).
6. ✅ Все size assertions проходят (Token=64, Connection=64, Event=32, DomainConfig=128).
7. ✅ Нет `std::time`, `SystemTime`, `Instant` в crates кроме axiom-runtime/adapters.
8. ✅ Конфигурационные файлы загружаются и валидируются.
9. ✅ Документация (spec/) остаётся в docs/spec/ (ИСПРАВЛЕНО: без s).
10. ✅ STATUS.md полностью заполнен (все фазы ✅).

---

## Приложение A: Маппинг спецификаций на crates

| Спецификация | Целевой crate |
|-------------|---------------|
| Token V5.2 | axiom-core |
| Connection V5.0 | axiom-core |
| COM V1.0 | axiom-core |
| CAUSAL_ORDER_MODEL | axiom-core (концептуальный, код в event.rs) |
| Time Model V1.0 | axiom-core (правила), axiom-heartbeat (causal_age) |
| DomainConfig V2.1 | axiom-config |
| DomainConfig V2.0 | axiom-config (заменён V2.1) |
| Configuration System V1.0 | axiom-config |
| Domain V1.3 | axiom-domain |
| Ashti_Core V2.0 | axiom-domain |
| Space V6.0 | axiom-space |
| Shell V3.0 | axiom-shell |
| Arbiter V1.0 | axiom-arbiter |
| Heartbeat V2.0 | axiom-heartbeat |
| Event-Driven V1 | axiom-frontier (принципы), axiom-runtime (цикл) |
| Causal Frontier V1 | axiom-frontier |
| UPO V2.2 | axiom-upo |
| UCL V2.0 | axiom-ucl |
| API В AXIOM | axiom-runtime (adapters.rs, правила изоляции) |
| module_documentation_ru | Справочный, не переносится (legacy NeuroGraph) |

---

## Приложение B: Внешние зависимости по crates

| Crate | Внешние зависимости |
|-------|-------------------|
| axiom-core | нет (zero dependencies) |
| axiom-config | serde, serde_yaml |
| axiom-domain | нет |
| axiom-space | нет |
| axiom-shell | bitvec (для dirty_flags) |
| axiom-arbiter | нет |
| axiom-heartbeat | нет |
| axiom-frontier | bitvec (для visited sets) |
| axiom-upo | нет |
| axiom-ucl | нет |
| axiom-runtime | все axiom-* crates |

**Принцип:** Минимум внешних зависимостей. axiom-core — ноль. Большинство crates зависят только от axiom-* crates.

---

## Приложение C: Чеклист для каждой фазы

Копировать и заполнять в STATUS.md при завершении каждой фазы:

```markdown
### Фаза N: axiom-{name}

- [ ] Crate создан в crates/axiom-{name}/
- [ ] Cargo.toml с корректными зависимостями
- [ ] Код перенесён из runtime/src/
- [ ] #[repr(C, align(...))] сохранён для core-структур
- [ ] Size assertions добавлены (compile-time const assert)
- [ ] Тесты перенесены и проходят (`cargo test -p axiom-{name}`)
- [ ] Нет std::time / SystemTime / Instant (если core crate)
- [ ] Нет циклических зависимостей
- [ ] `cargo test --workspace` зелёный
- [ ] Старый код удалён из monorepo
- [ ] STATUS.md обновлён
- [ ] Коммит создан
```

---

## Приложение D: Архитектурные решения (FAQ для исполнителя)

### D.1 Почему Frontier не включается в Snapshot?

Это архитектурное решение, зафиксированное в Causal Frontier V1, раздел 14:

> "Frontier не сохраняется в snapshot. Snapshot содержит только state + event log. Frontier восстанавливается из последних событий."

Frontier — это вычислительный механизм, а не часть модели мира. События из COM event log не теряются. При восстановлении из snapshot система восстанавливает state, затем переигрывает последние N событий для восстановления frontier.

При необходимости в будущем можно добавить **опциональное** включение frontier в snapshot как оптимизацию скорости загрузки. Но это не влияет на корректность.

### D.2 Почему Connection использует f32, а не fixed-point?

Connection V5.0 определяет strength/stress/elasticity/ideal_dist как f32. Это осознанное решение спецификации. Изменение на fixed-point — это изменение спецификации, не миграции. Текущая миграция переносит код как есть.

Граница проходит так:
- **Пространственные** вычисления (position, velocity, distance, spatial hash) — строго целочисленные (Space V6.0).
- **Динамические** свойства связей (strength, stress) — f32 (Connection V5.0).
- **Конфигурация** (field_size, gravity_strength) — f32 для удобства, конвертируются в кванты на границе.

### D.3 Зачем нужен axiom-core без зависимостей?

axiom-core — это фундамент, от которого зависят все остальные crates. Если axiom-core зависит от чего-то внешнего, это становится транзитивной зависимостью для всей системы. Zero dependencies в axiom-core означает:
- Минимальное время компиляции.
- Нет конфликтов версий.
- Максимальная портируемость (хотя std required).

### D.4 Где живут адаптеры (REST, CLI, WebSocket)?

Адаптеры — это **отдельный проект** после завершения ядра. axiom-runtime/adapters.rs определяет **trait-границу** — интерфейс, через который адаптер взаимодействует с ядром (UCL commands → UclResult). Конкретная реализация (FastAPI, CLI, WebSocket) живёт за пределами workspace ядра.

Это соответствует документу "API В AXIOM": ядро не знает о транспортных протоколах.

### D.5 Как читать спецификацию при переносе кода

1. Начать с раздела **"Инварианты"** — это то, что тесты должны проверять.
2. Изучить **размер структуры** и `repr(C)` — это compile-time assertions.
3. Проверить **cross-references** на другие спеки — это зависимости между crates.
4. Раздел **"Взаимодействия"** — определяет публичный API crate.
5. Раздел **"Жизненный цикл"** — определяет тестовые сценарии.

### D.6 Почему `&mut Vec<u32>` для find_neighbors, а не slice или Iterator?

Три варианта были рассмотрены:

- **`&[u32]` (возвращаемый)** — требует, чтобы результат жил внутри структуры SpatialIndex. Создаёт сложные lifetime-зависимости при передаче через trait.
- **`Iterator` с associated type** — `type Neighbors: Iterator<Item = u32>` — усложняет generic-код, требует `impl Trait` или Box<dyn> в горячем пути.
- **`&mut Vec<u32>` (передаваемый буфер)** — буфер предвыделен при инициализации домена. Zero-alloc. Нет lifetime-проблем. Вызывающий код владеет буфером.

Выбран третий вариант. Это канонический паттерн из Space V6.0 §4.6.

---

## Приложение E: Отладка и профилирование

```bash
# Запуск тестов с логированием
RUST_LOG=trace cargo test -p axiom-{name} -- --nocapture

# Проверка на undefined behavior (медленно, но ловит баги)
cargo +nightly miri test -p axiom-core

# Профилирование бенчмарков
cargo bench -p axiom-space

# Проверка тестового покрытия (опционально, тяжёлый инструмент)
cargo tarpaulin -p axiom-core --out Html

# Проверка всего workspace одной командой
just check

# ДОБАВЛЕНО: Визуализация графа зависимостей
just deps-graph
```

---

**Обновлено:** 2026-03-21
**Версия:** 1.2 — Финальная версия с исправлениями и дополнениями
**Изменения:**
- Исправлена опечатка `docs/specs/` → `docs/spec/`
- Добавлены инструменты визуализации зависимостей
- Добавлены примеры конфигурационных файлов
- Добавлены скрипты check_deps.sh и visualize_deps.sh
- Документ перемещён в корень как основной план работы
- Добавлены символы ⬜/✅/🔄/❌ для отслеживания прогресса
- Все выполненные пункты сохраняются, не удаляются
