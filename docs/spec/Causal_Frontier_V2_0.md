# AXIOM MODULE SPECIFICATION: Causal Frontier V2.0

**Статус:** Актуальная спецификация (core)  
**Версия:** 2.0.0  
**Дата:** 2026-03-27  
**Назначение:** Структура управления вычислениями — активная причинная граница системы  
**Crate:** `axiom-frontier`  
**Модель времени:** COM `event_id` (причинный порядок, u64)  
**Связанные спеки:** COM V1.0, Token V5.2, Connection V5.0, Event-Driven V1, Heartbeat V2.0, Space V6.0, DomainConfig V2.1, Arbiter V1.0

---

## 1. Назначение

**Causal Frontier** — структура, содержащая только те элементы состояния, которые **могут породить новое событие**.

Frontier — единственный механизм вычислений в AXIOM. Система **никогда не выполняет глобальный проход по состоянию**. Все вычисления выполняются **только внутри frontier**.

Это гарантирует:

- **O(active_entities)** вычислительная сложность (не O(total_entities))
- Локальность вычислений
- Энергоэффективность (causal idle = ноль CPU)
- Возможность работы на слабом оборудовании

Frontier является **механизмом управления вычислениями**, а не частью модели мира. Frontier **не сохраняется в snapshot** (раздел 12).

---

## 2. Основные принципы

### 2.1 Локальность причинности

Любое событие влияет только на ограниченную область состояния:

```
event → affected Token/Connection → frontier
```

Frontier содержит только затронутые сущности и их непосредственных соседей.

### 2.2 Запрет глобальных обновлений

Запрещено в любом модуле AXIOM:

```rust
// ЗАПРЕЩЕНО:
fn scan_all_tokens(domain: &DomainState) { ... }
fn update_all_connections(domain: &DomainState) { ... }
fn recalculate_world(engine: &AxiomEngine) { ... }
```

Все проверки выполняются только для элементов frontier. Единственное исключение — инициализация системы (загрузка из snapshot).

### 2.3 Детерминизм

Порядок обработки frontier строго детерминирован. Разрешены: stable queue, priority queue, ordered set. Запрещены: random iteration, non-deterministic parallel traversal.

При одинаковом начальном состоянии и одинаковой последовательности событий, frontier обрабатывается идентично на любой платформе.

---

## 3. Структура

### 3.1 Типизированные очереди

Frontier типизирован по видам сущностей для избежания runtime-проверок:

```rust
pub struct CausalFrontier {
    // --- Очереди сущностей ---
    token_queue: VecDeque<u32>,        // Индексы токенов для обработки
    connection_queue: VecDeque<u32>,    // Индексы связей для обработки

    // --- Дедупликация ---
    visited_tokens: BitVec,            // Предвыделён до token_capacity
    visited_connections: BitVec,       // Предвыделён до connection_capacity

    // --- Метрики ---
    events_this_cycle: u32,            // Счётчик событий в текущем цикле
    frontier_growth_rate: i32,         // Разница размера frontier между циклами

    // --- Состояние ---
    state: FrontierState,              // Текущее состояние жизненного цикла (раздел 8)

    // --- Лимиты (из конфигурации) ---
    max_frontier_size: u32,            // Жёсткий лимит памяти
    max_events_per_cycle: u32,         // Causal budget
    storm_threshold: u32,              // Порог детекции шторма
}
```

### 3.2 Предвыделение (zero-alloc)

Все внутренние структуры предвыделяются при создании домена:

```rust
impl CausalFrontier {
    pub fn new(config: &FrontierConfig) -> Self {
        Self {
            token_queue: VecDeque::with_capacity(config.max_frontier_size as usize),
            connection_queue: VecDeque::with_capacity(config.max_frontier_size as usize / 2),
            visited_tokens: BitVec::repeat(false, config.token_capacity as usize),
            visited_connections: BitVec::repeat(false, config.connection_capacity as usize),
            events_this_cycle: 0,
            frontier_growth_rate: 0,
            state: FrontierState::Empty,
            max_frontier_size: config.max_frontier_size,
            max_events_per_cycle: config.max_events_per_cycle,
            storm_threshold: config.storm_threshold,
        }
    }
}
```

Во время обработки событий аллокаций не происходит.

---

## 4. Интерфейс

### 4.1 Основные методы

```rust
impl CausalFrontier {
    /// Добавить токен во frontier. Дедупликация через visited_tokens.
    pub fn push_token(&mut self, token_index: u32);

    /// Добавить связь во frontier. Дедупликация через visited_connections.
    pub fn push_connection(&mut self, connection_index: u32);

    /// Извлечь следующую сущность для обработки.
    /// Возвращает None если frontier пуст или budget исчерпан.
    pub fn pop(&mut self) -> Option<FrontierEntity>;

    /// Проверить наличие сущности во frontier.
    pub fn contains_token(&self, token_index: u32) -> bool;
    pub fn contains_connection(&self, connection_index: u32) -> bool;

    /// Очистить frontier и сбросить visited-множества.
    pub fn clear(&mut self);

    /// Текущий размер (сумма обеих очередей).
    pub fn size(&self) -> usize;

    /// Текущее состояние жизненного цикла.
    pub fn state(&self) -> FrontierState;

    /// Начать новый цикл обработки. Сбрасывает events_this_cycle.
    pub fn begin_cycle(&mut self);

    /// Завершить цикл. Обновляет frontier_growth_rate, пересчитывает state.
    pub fn end_cycle(&mut self);
}
```

### 4.2 FrontierEntity

```rust
pub enum FrontierEntity {
    Token(u32),        // Индекс токена в массиве домена
    Connection(u32),   // Индекс связи в массиве домена
}
```

### 4.3 Дедупликация

При вызове `push_token(idx)`:

```rust
if !self.visited_tokens[idx as usize] {
    self.visited_tokens.set(idx as usize, true);
    self.token_queue.push_back(idx);
}
```

Повторное добавление одной сущности не приводит к повторной обработке. BitVec сбрасывается при `clear()` или в начале нового цикла (если нужно — определяется стратегией).

---

## 5. Алгоритм обработки

### 5.1 Основной цикл

```
frontier.begin_cycle()

while let Some(entity) = frontier.pop():

    evaluate_local_rules(entity, domain_state, config)

    if transformation detected:
        event = generate_event(transformation)
        apply_event(event, domain_state)

        affected = collect_neighbors(event, spatial_index)
        for neighbor in affected:
            frontier.push_token(neighbor)  // или push_connection

        frontier.events_this_cycle += 1

frontier.end_cycle()
```

### 5.2 evaluate_local_rules

Это трейт, реализуемый для каждого типа сущности:

```rust
pub trait LocalRules {
    /// Проверяет сущность и генерирует событие, если обнаружена трансформация.
    fn evaluate(
        &self,
        entity: FrontierEntity,
        domain_state: &DomainState,
        config: &DomainConfig,
        current_event_id: u64,
    ) -> Option<Event>;
}
```

Какие проверки выполняются для токена (при Heartbeat или при попадании во frontier):

- **Decay** — причинный возраст `current_event_id - token.last_event_id` превышает порог → событие DecayApplied.
- **Gravity** — пересчёт гравитационного влияния → событие TokenMoved.
- **Thermodynamics** — адаптация температуры к полю → событие TemperatureChanged.
- **Shell reconciliation** — пересчёт Shell из связей → обновление DomainShellCache (не генерирует COM-событие, Shell — кэш).

Какие проверки выполняются для связи:

- **Stress check** — `current_stress` превышает порог разрыва → событие ConnectionWeakened или ConnectionBroken.
- **Distance check** — расстояние между source и target vs `ideal_dist` → обновление stress.

### 5.3 collect_neighbors

Использует trait `SpatialIndex` (определён в axiom-core, реализован в axiom-space):

```rust
fn collect_neighbors(
    event: &Event,
    spatial_index: &impl SpatialIndex,
    tokens: &[Token],
    neighbor_buffer: &mut Vec<u32>,  // Предвыделённый буфер
) {
    neighbor_buffer.clear();

    match event.event_type {
        EventType::TokenMoved | EventType::TokenCreate => {
            let token = &tokens[event.target_id as usize];
            spatial_index.find_neighbors(
                token.position,
                INTERACTION_RADIUS,
                neighbor_buffer,
            );
        }
        EventType::ConnectionStress => {
            // Добавить source и target токены связи
            neighbor_buffer.push(event.source_id);
            neighbor_buffer.push(event.target_id);
        }
        _ => {}
    }
}
```

### 5.4 Обработка всегда локальна

Frontier **никогда не делает глобального прохода**. Он смотрит только область изменений. Это ключевая архитектурная гарантия: стоимость одного тика пропорциональна количеству активных сущностей, а не общему количеству.

Бенчмарк подтверждает: `TickForward` при 100 токенах/домен занимает 159 ns — overhead цикла минимален.

---

## 6. Поставщики работы для Frontier

Frontier получает сущности из двух источников:

### 6.1 Событийный путь (основной)

Каждое COM-событие добавляет во frontier затронутую сущность и её соседей:

```
UCL Command → COM Event → apply_event() → collect_neighbors() → frontier.push()
```

Это основной механизм. Сущности попадают во frontier потому что рядом что-то изменилось.

### 6.2 Heartbeat (фоновый)

Heartbeat V2.0 периодически добавляет батчи токенов/связей для проверки фоновых процессов (decay, gravity, термодинамика):

```
HeartbeatEvent → handle_heartbeat() → frontier.push_token(batch)
```

Формула выбора детерминирована: `token_index = (pulse_number * batch_size + offset) % total_tokens`.

Heartbeat гарантирует полное покрытие: за `ceil(total_tokens / batch_size)` пульсов каждый токен будет проверен хотя бы один раз. Это страховка для сущностей, которые давно не затрагивались событиями.

---

## 7. Storm Control

### 7.1 Определение

**Causal Storm** — ситуация, когда одно событие порождает лавину новых событий. Пример: коллапс группы связей → тысячи TokenMoved → миллионы проверок соседей.

Без контроля это разрушает масштабируемость и может заблокировать систему.

### 7.2 Детекция

Система отслеживает на каждом цикле:

```rust
pub struct StormMetrics {
    pub events_this_cycle: u32,       // Сколько событий сгенерировано
    pub frontier_size: u32,           // Текущий размер frontier
    pub frontier_growth_rate: i32,    // Изменение размера за цикл (может быть отрицательным)
}
```

Шторм детектируется когда:

```rust
fn is_storm(metrics: &StormMetrics, config: &FrontierConfig) -> bool {
    metrics.frontier_size > config.storm_threshold
    || metrics.events_this_cycle > config.max_events_per_cycle
}
```

### 7.3 Mitigation (смягчение)

При обнаружении шторма система применяет три механизма в порядке приоритета:

**Механизм 1: Causal Budget (всегда активен)**

Жёсткий лимит событий на цикл. Когда `events_this_cycle >= max_events_per_cycle`:

```rust
fn pop(&mut self) -> Option<FrontierEntity> {
    if self.events_this_cycle >= self.max_events_per_cycle {
        return None;  // Budget исчерпан, frontier сохраняется до следующего цикла
    }
    // ... обычная логика pop
}
```

Frontier не теряется — обработка продолжится в следующем цикле. Это предотвращает зависание системы.

**Механизм 2: Batch Events (при шторме)**

Схожие события одного типа объединяются:

```
100 × TokenMoved (с малыми дельтами)
→ 1 × BatchTokenMoved (агрегированная дельта)
```

Батчинг уменьшает количество COM-событий и количество проходов collect_neighbors.

**Механизм 3: Frontier Size Limit (предохранитель)**

Жёсткий лимит на размер frontier:

```rust
fn push_token(&mut self, token_index: u32) {
    if self.size() >= self.max_frontier_size as usize {
        // Frontier полон — сущность отбрасывается.
        // Она будет подхвачена Heartbeat при следующем обходе.
        return;
    }
    // ... обычная логика push
}
```

Heartbeat гарантирует, что отброшенные сущности будут проверены позже. Это не потеря данных, а отложенная обработка.

### 7.4 Конфигурация Storm Control

```rust
pub struct FrontierConfig {
    pub max_frontier_size: u32,        // Жёсткий лимит размера (предохранитель)
    pub max_events_per_cycle: u32,     // Causal budget (лимит событий за цикл)
    pub storm_threshold: u32,          // Порог для переключения в состояние Storm
    pub enable_batch_events: bool,     // Включить батчинг при шторме
    pub token_capacity: u32,           // Для предвыделения visited BitVec
    pub connection_capacity: u32,      // Для предвыделения visited BitVec
}
```

### 7.5 Примеры конфигурации

Для слабого оборудования (жёсткие лимиты):

```yaml
frontier:
  max_frontier_size: 1000
  max_events_per_cycle: 100
  storm_threshold: 500
  enable_batch_events: true
```

Для среднего оборудования:

```yaml
frontier:
  max_frontier_size: 10000
  max_events_per_cycle: 1000
  storm_threshold: 5000
  enable_batch_events: true
```

Для мощного сервера:

```yaml
frontier:
  max_frontier_size: 100000
  max_events_per_cycle: 10000
  storm_threshold: 50000
  enable_batch_events: false  # Не нужен — бюджет достаточен
```

### 7.6 Связь с Arbiter

Arbiter V1.0 имеет собственный `storm_threshold` — максимальное количество рефлексов в очереди на отправку в MAYA. Когда Arbiter приостанавливает рефлексы, это снижает нагрузку на MAYA(10), но не влияет напрямую на frontier. Два механизма storm control работают параллельно и независимо.

---

## 8. Жизненный цикл Frontier (State Machine)

### 8.1 Состояния

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrontierState {
    /// Frontier пуст. Нет работы. CPU не используется.
    /// Система ждёт внешнего события (UCL Command) или Heartbeat.
    Empty,

    /// Frontier содержит сущности, обработка идёт нормально.
    /// frontier_size <= storm_threshold, budget не исчерпан.
    Active,

    /// Frontier превысил storm_threshold или budget исчерпан.
    /// Включены механизмы mitigation (batching, budget pause).
    Storm,

    /// Frontier был в Storm, но уменьшился ниже storm_threshold.
    /// Система выходит из шторма, но остаётся бдительной.
    /// Batch events остаются включёнными на один цикл.
    Stabilizing,

    /// Frontier обработан до конца в этом цикле.
    /// Переход в Empty на следующем цикле (если нет новых событий).
    Idle,
}
```

### 8.2 Переходы

```
                  push()
    Empty ──────────────→ Active
      ↑                     │
      │                     │ frontier_size > storm_threshold
      │                     ↓
      │                   Storm
      │                     │
      │                     │ frontier_size < storm_threshold
      │                     ↓
      │                Stabilizing
      │                     │
      │                     │ end_cycle(), frontier_size < storm_threshold
      │                     ↓
    Idle ←──────────── Active (frontier обработан)
      │
      │ begin_cycle(), frontier пуст
      ↓
    Empty
```

### 8.3 Влияние состояния на поведение

| Состояние | Batch events | Budget | Pop возвращает | CPU |
|-----------|-------------|--------|----------------|-----|
| Empty | — | — | None | 0% |
| Active | Off | Active | Сущности | Normal |
| Storm | On (если enable_batch_events) | Жёсткий лимит | Сущности до budget | Limited |
| Stabilizing | On (один доп. цикл) | Active | Сущности | Normal |
| Idle | — | — | None (до begin_cycle) | 0% |

### 8.4 Causal Idle

Когда frontier пуст **и** нет внешних событий **и** Heartbeat не срабатывает (потому что нет событий для счётчика):

- Система входит в состояние `Empty`.
- Никакие вычисления не выполняются.
- CPU потребление — ноль.
- Система ждёт внешнего события (UCL Command).

Это ключевое свойство для работы на слабом оборудовании: если ничего не происходит, система потребляет ноль ресурсов.

---

## 9. Доменная изоляция

Каждый домен имеет собственный `CausalFrontier`:

```rust
pub struct DomainState {
    pub tokens: Vec<Token>,
    pub connections: Vec<Connection>,
    pub frontier: CausalFrontier,      // Собственный frontier
    pub spatial_index: SpatialHashGrid,
    pub shell_cache: DomainShellCache,
    // ...
}
```

Frontier одного домена **не влияет** на другие домены. Междоменное взаимодействие происходит только через COM-события и шины данных (маршруты Ashti_Core: 0→9, 9→1-8, 1-8→10).

---

## 10. Интеграция с другими модулями

### 10.1 С COM V1.0

Каждое событие, сгенерированное внутри frontier, получает `event_id` из CausalClock. Frontier не создаёт событий — он управляет тем, какие сущности проверяются. События генерируются внутри `evaluate_local_rules()`.

### 10.2 С Heartbeat V2.0

Heartbeat — поставщик работы для frontier, не исполнитель. `handle_heartbeat()` только добавляет батч сущностей в frontier через `push_token()` / `push_connection()`. Вся логика (decay, gravity) — внутри стандартного цикла frontier.

### 10.3 С Space V6.0

`collect_neighbors()` использует `SpatialHashGrid` через trait `SpatialIndex` для поиска соседей затронутой сущности. Пространственный хэш перестраивается после батча пространственных событий (не после каждого события).

### 10.4 С Shell V3.0

Когда токен попадает во frontier из-за Connection-события, в рамках `evaluate_local_rules()` выполняется пересчёт Shell (инкрементальный). При Heartbeat — reconciliation (полный пересчёт + сравнение).

### 10.5 С Arbiter V1.0

Arbiter имеет собственный `storm_threshold` для рефлексов. Оба механизма storm control работают параллельно: frontier ограничивает внутридоменные вычисления, Arbiter ограничивает междоменную нагрузку рефлексов.

### 10.6 С DomainConfig V2.1

FrontierConfig является частью конфигурации домена. Разные домены могут иметь разные лимиты. Например, EXPERIENCE(9) может иметь больший `max_frontier_size` (много следов), а SUTRA(0) — минимальный (статическая библиотека).

---

## 11. Инварианты

1. **Локальность.** Frontier содержит только индексы сущностей текущего домена. Глобальные проходы запрещены.
2. **Дедупликация.** Одна сущность может быть в frontier не более одного раза за цикл (BitVec visited).
3. **Детерминизм.** Порядок обработки определяется порядком добавления (stable FIFO). При одинаковых входах — одинаковый результат.
4. **Ограниченная нагрузка.** `events_this_cycle` не может превысить `max_events_per_cycle`. `size()` не может превысить `max_frontier_size`.
5. **Causal budget.** Когда budget исчерпан, `pop()` возвращает `None`. Frontier сохраняется до следующего цикла.
6. **COM совместимость.** Все события, сгенерированные при обработке frontier, проходят через COM и получают `event_id`.
7. **Zero-alloc.** Никаких аллокаций в горячем пути. Все структуры предвыделены при создании домена.
8. **Полное покрытие.** Heartbeat гарантирует, что любая отброшенная или необработанная сущность будет проверена позднее.

---

## 12. Snapshot

Frontier **не сохраняется в snapshot**. Snapshot содержит только:

```
state (tokens, connections, domain configs)
event log (COM events)
```

При восстановлении из snapshot frontier восстанавливается путём переигрывания последних N событий из event log. Каждое переигранное событие добавляет затронутые сущности в frontier, восстанавливая активную причинную границу.

Это архитектурное решение, а не ограничение: frontier — механизм вычислений, не часть модели мира. Данные не теряются — они в event log.

При необходимости в будущем можно добавить **опциональное** включение frontier в snapshot как оптимизацию скорости загрузки (избежать переигрывание). Но это не влияет на корректность.

---

## 13. Causal Horizon (перспектива)

### 13.1 Идея

Со временем в системе появляется естественная граница — **causal horizon** — event_id, после которого старые события уже не могут повлиять на текущую область вычислений.

Если `current_event_id` = 1,000,000 и самый старый активный токен имеет `last_event_id` = 950,000, то все события до 950,000 — за горизонтом. Они уже применены к состоянию и больше не нужны для вычислений.

### 13.2 Практическое значение

Causal horizon позволяет:

- **Архивировать COM event log** — события за горизонтом можно сжать или удалить. На слабом железе event log не может расти бесконечно.
- **Безопасные snapshot** — snapshot + horizon = точка, после которой можно удалить старую историю.
- **Масштабирование** — при огромных мирах (миллионы сущностей, миллиарды событий) horizon позволяет ограничить рабочий набор.

### 13.3 Вычисление

```rust
fn compute_horizon(domain_state: &DomainState) -> u64 {
    let min_token_event = domain_state.tokens.iter()
        .map(|t| t.last_event_id)
        .min()
        .unwrap_or(0);

    let min_connection_event = domain_state.connections.iter()
        .map(|c| c.last_active)
        .min()
        .unwrap_or(0);

    min_token_event.min(min_connection_event)
}
```

Всё, что имеет `event_id < horizon`, уже применено ко всем сущностям. Эти события можно безопасно архивировать.

### 13.4 Статус

Causal horizon — **не реализован**. Это перспективная оптимизация. Реализовать когда COM event log начнёт потреблять заметную память. Спецификация зафиксирована здесь чтобы не забыть при проектировании Snapshot в axiom-runtime.

---

## 14. Конфигурация

### 14.1 FrontierConfig

```rust
pub struct FrontierConfig {
    /// Жёсткий лимит размера frontier (предохранитель).
    /// При превышении — push отбрасывает сущности (подхватит Heartbeat).
    pub max_frontier_size: u32,

    /// Causal budget: максимум событий за один цикл обработки.
    /// При исчерпании — frontier сохраняется до следующего цикла.
    pub max_events_per_cycle: u32,

    /// Порог для перехода в состояние Storm.
    /// Когда frontier_size > storm_threshold, включаются механизмы mitigation.
    pub storm_threshold: u32,

    /// Включить объединение однотипных событий при шторме.
    pub enable_batch_events: bool,

    /// Ёмкость для предвыделения BitVec (должна = DomainConfig.token_capacity).
    pub token_capacity: u32,

    /// Ёмкость для предвыделения BitVec (должна = DomainConfig.connection_capacity).
    pub connection_capacity: u32,
}
```

### 14.2 Примеры для разного оборудования

Слабое оборудование:

```yaml
frontier:
  max_frontier_size: 1000
  max_events_per_cycle: 100
  storm_threshold: 500
  enable_batch_events: true
```

Среднее оборудование:

```yaml
frontier:
  max_frontier_size: 10000
  max_events_per_cycle: 1000
  storm_threshold: 5000
  enable_batch_events: true
```

Мощный сервер:

```yaml
frontier:
  max_frontier_size: 100000
  max_events_per_cycle: 10000
  storm_threshold: 50000
  enable_batch_events: false
```

### 14.3 Связь с DomainConfig

FrontierConfig добавляется в конфигурацию системы (Configuration System V1.0, Runtime Configuration). Значения `token_capacity` и `connection_capacity` должны совпадать с соответствующими полями DomainConfig V2.1.

---

## 15. Валидация

```rust
impl Validate for FrontierConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.max_frontier_size == 0 { return Err("max_frontier_size must be > 0"); }
        if self.max_events_per_cycle == 0 { return Err("max_events_per_cycle must be > 0"); }
        if self.storm_threshold == 0 { return Err("storm_threshold must be > 0"); }
        if self.storm_threshold > self.max_frontier_size {
            return Err("storm_threshold must be <= max_frontier_size");
        }
        if self.token_capacity == 0 { return Err("token_capacity must be > 0"); }
        Ok(())
    }
}

impl Validate for CausalFrontier {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.size() > self.max_frontier_size as usize {
            return Err("frontier exceeds max_frontier_size");
        }
        Ok(())
    }
}
```

---

## 16. История изменений

- **V2.0**: Полная переработка. Терминология приведена к AXIOM (Token, Connection вместо Shell, Cluster). Добавлен FrontierState (state machine с 5 состояниями). Детализирован Storm Control (три механизма: budget, batching, size limit). Добавлен Causal Horizon (перспектива). Конкретные структуры данных и конфигурация. Связь с бенчмарками. Интеграция с Arbiter V1.0 storm control.
- **V1.0**: Первая версия. Концептуальное описание. Терминология NeuroGraph (shells, clusters).
