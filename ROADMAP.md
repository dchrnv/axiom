# Axiom Roadmap

**Версия:** 9.3
**Дата:** 2026-03-27

---

## 🔄 В работе: Causal Frontier V2.0

**Спека:** [docs/spec/Causal_Frontier_V2_0.md](docs/spec/Causal_Frontier_V2_0.md)
**Crate:** `axiom-frontier`
**Внешних потребителей:** 1 (`axiom-domain/src/domain.rs:74`)

### Что меняется относительно V1

| Аспект | V1 | V2.0 |
|--------|-----|------|
| Конфиг | `with_config(3 raw params)` | `FrontierConfig` struct + presets |
| Индексы | `usize` | `u32` |
| Visited | `Vec<bool>` | `BitVec` (dep уже есть) |
| Pop | `pop_token()` / `pop_connection()` | `pop() -> Option<FrontierEntity>` с budget внутри |
| Цикл | `reset_cycle()` | `begin_cycle()` / `end_cycle()` |
| State | `Stabilized` | `Stabilizing` |
| Новые поля | — | `frontier_growth_rate: i32`, `StormMetrics` |
| Соседи | только токены | токены + связи (`Vec<FrontierEntity>`) |

### Шаг 1 — `FrontierConfig` + `FrontierEntity`

`crates/axiom-frontier/src/frontier.rs`

```rust
pub struct FrontierConfig {
    pub max_frontier_size: u32,
    pub max_events_per_cycle: u32,
    pub storm_threshold: u32,
    pub enable_batch_events: bool,
    pub token_capacity: u32,
    pub connection_capacity: u32,
}
impl FrontierConfig {
    pub fn weak() -> Self    // 1000 / 100 / 500
    pub fn medium() -> Self  // 10000 / 1000 / 5000
    pub fn powerful() -> Self// 100000 / 10000 / 50000
}

pub enum FrontierEntity { Token(u32), Connection(u32) }
```

### Шаг 2 — `EntityQueue`: `Vec<bool>` → `BitVec`, `usize` → `u32`

Механический рефактор. `bitvec` уже в `Cargo.toml` axiom-frontier.

### Шаг 3 — `CausalFrontier`: новые поля + новый API

- Конструктор: `new(config: FrontierConfig)` вместо `with_config(...)`
- `FrontierState::Stabilized` → `Stabilizing`
- Добавить `frontier_growth_rate: i32`, `prev_size: usize`
- `begin_cycle()` — сбрасывает `events_this_cycle`
- `end_cycle()` — обновляет `frontier_growth_rate`, пересчитывает state
- `pop() -> Option<FrontierEntity>` — budget-check внутри, токены приоритетнее связей
- `StormMetrics` struct + `metrics()` метод

### Шаг 4 — `FrontierProcessor`

- `EvaluationResult::Transform`: `affected_neighbors: Vec<FrontierEntity>` (вместо `Vec<usize>`)
- `step()` использует `pop()` → матч на `FrontierEntity`

### Шаг 5 — `axiom-domain`: обновить точку интеграции

`crates/axiom-domain/src/domain.rs:74`

```rust
// было:
frontier: CausalFrontier::with_config(storm_threshold, max_size, budget)
// стало:
frontier: CausalFrontier::new(FrontierConfig::medium())
```

### Шаг 6 — Тесты

Все 16 существующих тестов: адаптировать под новый API.
Добавить: `begin_cycle/end_cycle`, `frontier_growth_rate`, `StormMetrics`, `FrontierConfig presets`.
Цель: ~25 тестов.

---

## 🔮 Долгосрочные цели

### axiom-upo тесты
UPO v2.2 мигрирован без тестов. Покрыть: `DynamicTrace`, `UPOEngine::record_*`, `generate_patch`. Низкий приоритет.

### Configuration System
YAML-загрузка пространственных параметров и semantic_contributions. Требует согласования с DomainConfig 128-byte constraint.

### Адаптеры
Python bindings, REST API, gRPC — нужны для внешней интеграции.

### Производительность
SIMD (AVX-512), incremental spatial hash rebuild — после стабилизации архитектуры.

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-27
