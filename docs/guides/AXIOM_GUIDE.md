# AXIOM Functional Guide

**Версия:** 1.0
**Дата:** 2026-03-28
**Статус системы:** 426 тестов, 0 failures

---

## Содержание

1. [Архитектура](#1-архитектура)
2. [GENOME — конституция системы](#2-genome--конституция-системы)
3. [GUARDIAN — контроль правил](#3-guardian--контроль-правил)
4. [AshtiCore — 11 доменов](#4-ashticore--11-доменов)
5. [Causal Frontier V2.0](#5-causal-frontier-v20)
6. [Arbiter — dual-path routing](#6-arbiter--dual-path-routing)
7. [UCL — команды и AxiomEngine](#7-ucl--команды-и-axiomengine)
8. [Boot sequence](#8-boot-sequence)
9. [Примеры кода](#9-примеры-кода)

---

## 1. Архитектура

### Граф зависимостей

```
axiom-core          — Token (64B), Connection (64B), Event (32B)
axiom-genome        — Genome, GenomeIndex, AccessRule, ProtocolRule
axiom-config        — DomainConfig (128B), HeartbeatConfig, ConfigLoader
axiom-space         — SpatialHashGrid, физика поля
axiom-shell         — Shell V3.0, семантические профили
axiom-frontier      — CausalFrontier V2.0, FrontierConfig, FrontierEntity
axiom-arbiter       — Arbiter, Experience, Maya, COM, AshtiProcessor
axiom-heartbeat     — Heartbeat V2.0
axiom-upo           — UPO, DynamicTrace, Screen
axiom-domain        — Domain, DomainState, AshtiCore (11 доменов)
axiom-ucl           — UclCommand, UclResult, OpCode
axiom-runtime       — AxiomEngine, Guardian, Snapshot, orchestrator
```

Каждый crate зависит только от тех, что левее/выше его. Зависимости строго однонаправленные — нет циклов.

### Роли компонентов

| Компонент | Роль |
|-----------|------|
| **GENOME** | Конституция: неизменяемые правила, инварианты, матрицы доступа |
| **GUARDIAN** | Страж: проверяет GENOME + CODEX, вето на рефлексы, сканирование доменов |
| **AshtiCore** | Ядро: 11 доменов + Arbiter, физический движок, тик симуляции |
| **Arbiter** | Маршрутизатор: dual-path routing через Experience и MAYA |
| **CausalFrontier** | Причинный фронт: приоритизация событий, защита от шторма |
| **Heartbeat** | Пульс: периодический push токенов в Frontier по интервалу |
| **UCL** | Язык команд: бинарный протокол AxiomEngine |
| **UPO** | Наблюдатель: DynamicTrace, Screen, детектирование паттернов |

---

## 2. GENOME — конституция системы

GENOME — первый компонент, загружаемый при старте системы. Определяет что **никогда не меняется**. После валидации замораживается в `Arc<Genome>` — никакой `&mut Genome` в рантайме.

### Структура

```rust
pub struct Genome {
    pub version: u32,
    pub invariants: GenomeInvariants,   // размеры структур, флаги безопасности
    pub access_rules: Vec<AccessRule>,  // матрица прав доступа
    pub protocol_rules: Vec<ProtocolRule>, // разрешённые маршруты данных
    pub config: GenomeConfig,           // параметры Arbiter, Frontier, Heartbeat
}
```

### GenomeInvariants

Физические ограничения, которые compile-time не проверяемы, но обязательны в рантайме:

| Поле | Значение | Смысл |
|------|----------|-------|
| `token_size` | 64 | Token всегда 64 байта |
| `connection_size` | 64 | Connection всегда 64 байта |
| `event_size` | 32 | Event всегда 32 байта |
| `domain_config_size` | 128 | DomainConfig всегда 128 байт |
| `max_domains` | 11 | Один уровень Ashti_Core = 11 доменов |
| `no_wall_clock_in_core` | true | `std::time` запрещён внутри ядра |
| `event_id_monotonic` | true | event_id строго монотонен |

### AccessRule и GenomeIndex

Каждый `AccessRule` задаёт: `module → resource → permission`.

`GenomeIndex` — предвычисленная матрица `[[Permission; 8]; 16]` для O(1) lookup:

```rust
let genome = Genome::default_ashti_core();
let index = GenomeIndex::build(&genome);

// O(1) — один индекс в массиве
index.check_access(ModuleId::Guardian, ResourceId::CodexRules, Permission::ReadWrite); // true
index.check_access(ModuleId::Adapters, ResourceId::CodexRules, Permission::ReadWrite); // false
```

Таблица доступа Ashti_Core V2.0:

| Модуль | SutraTokens | AshtiField | ExperienceMemory | MayaOutput | CodexRules | GenomeConfig |
|--------|-------------|------------|-----------------|------------|------------|--------------|
| Arbiter | Read | Execute | Read | Execute | Read | Read |
| Guardian | Read | Control | Control | Read | **ReadWrite** | Read |
| Heartbeat | — | Read | Read | — | — | Read |
| Shell | — | Read | Read | — | — | — |
| Adapters | — | — | — | Read | — | — |

### ProtocolRule

Разрешённые маршруты данных между модулями:

| source → target | data_type | mandatory |
|----------------|-----------|-----------|
| Sutra → Experience | TokenReference | ✅ |
| Experience → Arbiter | ResonanceResponse | ✅ |
| Arbiter → Logic | PatternHint | ✅ |
| Logic → Maya | ProcessingResult | ✅ |
| Logic → Experience | NewExperience | ✅ |
| Arbiter → Maya | Reflex | — |
| Maya → Arbiter | ComparisonResult | — |
| Arbiter → Experience | Feedback | — |

### Загрузка из YAML

```rust
// Фаза A: захардкоженная конфигурация
let genome = Arc::new(Genome::default_ashti_core());

// Фаза B: из файла
let genome = Arc::new(Genome::from_yaml(Path::new("config/genome.yaml"))?);
// from_yaml автоматически вызывает validate()
```

Файл: `config/genome.yaml` — полное описание Ashti_Core V2.0 конституции.

### validate()

Проверяет инварианты, обязательные правила доступа и протоколы:

```rust
genome.validate()?; // Err(GenomeError::InvariantViolation) / MissingGuardianAccess / ...
```

Невалидный GENOME → система не запускается (`AxiomEngine::try_new` вернёт `Err`).

---

## 3. GUARDIAN — контроль правил

GUARDIAN — над-доменный страж. Два источника правил: GENOME (абсолютные) + CODEX (пластичные, хранятся в DomainState домена Codex).

### Создание

```rust
// С явным Genome (boot sequence)
let guardian = Guardian::new(Arc::clone(&genome));

// С дефолтным Genome (удобный конструктор)
let guardian = Guardian::with_default_genome();
```

### enforce_access / enforce_protocol

Проверки по GENOME — O(1) через GenomeIndex:

```rust
// Имеет ли Arbiter право Execute на AshtiField?
guardian.enforce_access(ModuleId::Arbiter, ResourceId::AshtiField, Permission::Execute);

// Разрешён ли маршрут Sutra → Experience?
guardian.enforce_protocol(ModuleId::Sutra, ModuleId::Experience);
```

При отказе — `violation_count` увеличивается, `stats.access_denied/protocol_denied` обновляются.

### validate_reflex

Возвращает `ReflexDecision` (не `bool`). Проверяет GENOME первым, затем CODEX:

```rust
match guardian.validate_reflex(&token) {
    ReflexDecision::Allow => { /* отправить рефлекс */ }
    ReflexDecision::Veto(VetoReason::TokenLocked) => { /* токен заблокирован */ }
    ReflexDecision::Veto(VetoReason::ValenceWithoutMass) => { /* нарушение физики */ }
    ReflexDecision::Veto(VetoReason::ZeroSutraId) => { /* нулевой sutra_id */ }
    ReflexDecision::Veto(VetoReason::GenomeDenied) => { /* GENOME запрещает */ }
}
```

CODEX правила:
1. `STATE_LOCKED` — рефлекс заблокирован
2. `valence != 0 && mass == 0` — нарушение физики
3. `sutra_id == 0` — недопустимый токен

### scan_domain

Сканирует все токены в DomainState на нарушения CODEX:

```rust
let actions: Vec<InhibitAction> = guardian.scan_domain(&state);
for action in &actions {
    match &action.reason {
        InhibitReason::ValenceWithoutMass { token_index } => {
            // токен с индексом token_index нарушает инвариант
        }
    }
}
```

### update_codex

Модификация CODEX-домена с проверкой прав GENOME:

```rust
guardian.update_codex(&mut codex_domain, CodexAction::AddRule(rule_token))?;
guardian.update_codex(&mut codex_domain, CodexAction::ResetViolations)?;
```

### GUARDIAN_CHECK_REQUIRED

Бит `0x04` в `DomainConfig::arbiter_flags`. Когда установлен — orchestrator вызывает `validate_reflex` перед отправкой рефлекса в MAYA:

```rust
let config = DomainConfig::factory_sutra(100, 0);
// config.arbiter_flags |= GUARDIAN_CHECK_REQUIRED;
```

### Статистика

```rust
let stats = guardian.stats();
println!("allowed: {}, vetoed: {}", stats.reflex_allowed, stats.reflex_vetoed);
println!("access denied: {}", stats.access_denied);
println!("domains scanned: {}", stats.domains_scanned);
```

---

## 4. AshtiCore — 11 доменов

AshtiCore — физический движок одного уровня Ashti_Core. Содержит 11 фиксированных доменов и Arbiter.

### Адресация доменов

```
level_id × 100 + role = domain_id

Уровень 1:
  100 = SUTRA       (роль 0)  — исходный поток
  101 = EXECUTION   (роль 1)  — исполнение
  102 = SHADOW      (роль 2)  — отражение
  103 = CODEX       (роль 3)  — правила
  104 = MAP         (роль 4)  — пространство
  105 = PROBE       (роль 5)  — наблюдение
  106 = LOGIC       (роль 6)  — логика
  107 = DREAM       (роль 7)  — сны
  108 = VOID        (роль 8)  — пустота
  109 = EXPERIENCE  (роль 9)  — опыт
  110 = MAYA        (роль 10) — выход
```

### tick()

Основной шаг симуляции:

```rust
let events: Vec<Event> = ashti.tick();
```

За один тик:
1. Каждый домен проверяет Heartbeat — если накопилось `heartbeat_interval` событий, домен делает пульс
2. При пульсе: `handle_heartbeat` → `process_frontier(tokens, connections, generator)`
3. `EventGenerator` генерирует физические события из токенов на Frontier
4. Все события собираются и возвращаются

### inject_token

```rust
ashti.inject_token(domain_id, token)?; // синхронизирует Domain.active_tokens
```

После `inject_token` домен знает актуальное число токенов — это важно для Heartbeat и Frontier.

### process — dual-path routing

```rust
let result: RoutingResult = ashti.process(token);
// result.reflex     — быстрый рефлекс (если найден в Experience)
// result.routed_events — маршрутизированные события
// result.event_id   — для apply_feedback
```

### drain_events через AxiomEngine

```rust
// Через UCL
engine.process_command(&UclCommand::new(OpCode::TickForward, 0, 1, 0));
let events = engine.drain_events(); // Vec<Event>
```

---

## 5. Causal Frontier V2.0

CausalFrontier — приоритетная очередь событий с защитой от шторма и бюджетом на цикл.

### FrontierConfig

Три предустановки:

```rust
FrontierConfig::tight()   // max_events=512,  storm_threshold=1000, budget=50
FrontierConfig::medium()  // max_events=2048, storm_threshold=5000, budget=200
FrontierConfig::wide()    // max_events=8192, storm_threshold=20000, budget=1000
```

Или кастомно:
```rust
FrontierConfig {
    max_events: 1024,
    memory_limit: 4096,
    storm_threshold: 2000,
    budget_per_cycle: 100,
}
```

### FrontierEntity

Тип сущности на Frontier:

```rust
pub enum FrontierEntity {
    Token(u32),        // token_id
    Connection(u32),   // connection_id
    Region(u32, u32),  // domain_id, region_id
    Batch(u32),        // batch_id
}
```

### Жизненный цикл

```rust
frontier.begin_cycle();          // сбрасывает бюджет цикла
while let Some(entity) = frontier.pop() {  // извлекает по приоритету, бюджет --
    // обработка
}
frontier.end_cycle();            // собирает StormMetrics

let metrics = frontier.storm_metrics();
println!("rate: {:.2}", metrics.frontier_growth_rate); // > 1.0 = шторм
```

### Состояния

| Состояние | Условие |
|-----------|---------|
| `FrontierState::Active` | нормальная работа |
| `FrontierState::Storm` | frontier_growth_rate > 1.0 |
| `FrontierState::Stabilizing` | storm завершён, идёт стабилизация |
| `FrontierState::Saturated` | достигнут memory_limit |

### Интеграция с Domain

Domain автоматически создаёт CausalFrontier с `FrontierConfig::medium()`. При `handle_heartbeat`:

```rust
// Heartbeat пушит все активные токены
domain.frontier.push(FrontierEntity::Token(id), priority);
```

При `process_frontier`:

```rust
domain.process_frontier(&tokens, &connections, &mut generator)
// → Vec<Event>: физические события от взаимодействия токенов
```

---

## 6. Arbiter — dual-path routing

Arbiter реализует два пути обработки токена:

```
Token → Experience(9) → resonance_search
                           ↓
              Reflex path  │  Slow path
              (score ≥ t)  │  (score < t)
                    ↓      │       ↓
                  MAYA     │   ASHTI(1-8) → Logic → Maya
                           ↓
                   finalize_comparison → feedback → Experience
```

### Experience — хранилище опыта

Experience использует UPO (Screen + DynamicTrace) для хранения следов:

```rust
// Поиск резонанса
let resonance = experience.resonance_search(&token);
match resonance.level {
    ResonanceLevel::Reflex      => { /* быстрый путь */ }
    ResonanceLevel::Association => { /* медленный путь */ }
    ResonanceLevel::None        => { /* новый токен */ }
}
```

`DynamicTrace` (32 байта) хранит `pattern: Token` + `weight: f32` + флаги. `Screen` — карта следов с decay и автоочисткой.

### AshtiProcessor

Обрабатывает домены ASHTI(1-8) в slow path. Для каждого домена применяет:
- `arbiter_flags` как маску `type_flags`
- `reflex_threshold` / `association_threshold` из DomainConfig
- физику через axiom-space

### Maya (Compare & Decide)

MAYA финализирует routing: консолидирует результаты медленного пути, сравнивает с рефлексом, принимает финальное решение.

`arbiter_flags & 0x01` (ForceMedianConsolidation) — принудительное медианное усреднение вместо взвешенного.

### COM (Causal Order Manager)

Генерирует монотонные `event_id` и `com_id`. Гарантирует:
- `event_id` строго возрастает (инвариант GENOME)
- Причинный порядок событий

---

## 7. UCL — команды и AxiomEngine

UCL (Unified Command Language) — бинарный протокол команд для AxiomEngine.

### UclCommand (64 байта)

```rust
UclCommand {
    payload: [u8; 48],  // данные команды
    command_id: u64,    // уникальный ID
    target_id: u32,     // целевой domain_id
    opcode: u16,        // OpCode
    priority: u8,
    flags: u8,
}
```

### Коды команд

| OpCode | Значение | Действие |
|--------|----------|---------|
| `SpawnDomain` | 1000 | no-op (домены фиксированы в AshtiCore) |
| `CollapseDomain` | 1001 | no-op |
| `InjectToken` | 2000 | добавить токен в домен |
| `TickForward` | 3000 | один шаг симуляции → physics events |
| `ProcessTokenDualPath` | 4000 | dual-path routing через Arbiter |
| `FinalizeComparison` | 4001 | обратная связь в Experience |
| `CoreReset` | 9001 | сбросить состояние (genome сохраняется) |
| `BackupState` | 9002 | создать snapshot |
| `RestoreState` | 9003 | восстановить из snapshot |
| `CoreShutdown` | 9000 | завершение |

### AxiomEngine::try_new

Основной способ создания Engine — с явной валидацией GENOME:

```rust
use std::sync::Arc;
use axiom_genome::Genome;
use axiom_runtime::{AxiomEngine, AxiomError};

// Из файла
let genome = Arc::new(Genome::from_yaml(Path::new("config/genome.yaml"))?);
let engine = AxiomEngine::try_new(genome)?;

// С захардкоженным Genome
let engine = AxiomEngine::new(); // эквивалент try_new(default_ashti_core()).unwrap()
```

### process_command

```rust
let cmd = UclCommand::new(OpCode::InjectToken, domain_id, command_id, priority);
// + заполнить payload

let result: UclResult = engine.process_command(&cmd);
if result.is_success() {
    println!("events generated: {}", result.events_generated);
}
```

### drain_events

```rust
// После одного или нескольких TickForward
let events: Vec<Event> = engine.drain_events();
// drain_events очищает внутренний буфер
```

---

## 8. Boot sequence

Порядок инициализации системы:

```
1. Genome::from_yaml("config/genome.yaml") или Genome::default_ashti_core()
   → validate() — проверка инвариантов, обязательных правил
   → Arc::new(genome) — заморозка

2. AxiomEngine::try_new(Arc::clone(&genome))
   → Guardian::new(Arc::clone(&genome)) — строит GenomeIndex
   → AshtiCore::new(level_id) — 11 доменов + Arbiter + Experience
   → pending_events = Vec::new()

3. Система готова к командам UCL
```

При невалидном GENOME — `AxiomEngine::try_new` возвращает `Err(AxiomError::InvalidGenome)`, система не запускается.

После `CoreReset` — AshtiCore пересоздаётся, Guardian пересоздаётся с тем же `Arc<Genome>`. Genome не меняется.

---

## 9. Примеры кода

### Создание Engine и инъекция токенов

```rust
use std::sync::Arc;
use axiom_genome::Genome;
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

// Boot
let genome = Arc::new(Genome::default_ashti_core());
let mut engine = AxiomEngine::try_new(genome).unwrap();

// Инъекция токена в LOGIC домен (106)
let mut payload = [0u8; 48];
// target_domain_id = 106 (LOGIC)
payload[0] = 106u16.to_le_bytes()[0];
payload[1] = 106u16.to_le_bytes()[1];
// mass = 100 (float32 at offset 4)
payload[4..8].copy_from_slice(&100.0f32.to_le_bytes());

let cmd = UclCommand { payload, command_id: 1, target_id: 106, opcode: 2000, priority: 0, flags: 0 };
engine.process_command(&cmd);
```

### Симуляция: тик и события

```rust
// Нужно ~1100 тиков для первого heartbeat при medium пресете (интервал 1024)
for i in 0..1100 {
    let tick_cmd = UclCommand { payload: [0;48], command_id: i, target_id: 0, opcode: 3000, priority: 0, flags: 0 };
    engine.process_command(&tick_cmd);
}

let events = engine.drain_events();
println!("physics events: {}", events.len());
```

### GUARDIAN проверки

```rust
use axiom_runtime::{Guardian, ReflexDecision};
use axiom_genome::{ModuleId, ResourceId, Permission};

let mut guardian = Guardian::with_default_genome();

// Проверка GENOME прав
let ok = guardian.enforce_access(ModuleId::Arbiter, ResourceId::AshtiField, Permission::Execute);

// Проверка рефлекса
let token = Token::new(42, 106, [0,0,0], 1);
match guardian.validate_reflex(&token) {
    ReflexDecision::Allow => println!("рефлекс разрешён"),
    ReflexDecision::Veto(reason) => println!("вето: {:?}", reason),
}
```

### Genome из YAML

```rust
use std::path::Path;
use axiom_genome::Genome;

let genome = Genome::from_yaml(Path::new("config/genome.yaml"))
    .expect("genome должен быть валидным при старте");

println!("version: {}", genome.version);
println!("domains: {}", genome.config.ashti_domain_count);
println!("heartbeat: {}", genome.config.default_heartbeat_interval);
```

### Snapshot и restore

```rust
// Сохранить состояние
let backup_cmd = UclCommand { opcode: 9002, command_id: 99, ..Default::default() };
engine.process_command(&backup_cmd);
let snap = engine.snapshot();

// Восстановить
let restored = AxiomEngine::restore_from(&snap);
```

---

## Связанные документы

- [ROADMAP.md](../../ROADMAP.md) — планы разработки
- [STATUS.md](../../STATUS.md) — текущее состояние, тесты
- [DEVELOPMENT_GUIDE.md](../../DEVELOPMENT_GUIDE.md) — правила разработки
- [docs/spec/GENOME_V1_0.md](../spec/GENOME_V1_0.md) — спецификация GENOME
- [docs/spec/GUARDIAN_V1_0.md](../spec/GUARDIAN_V1_0.md) — спецификация GUARDIAN
- [docs/spec/Ashti_Core_V2_1.md](../spec/Ashti_Core_V2_1.md) — архитектура Ashti_Core V2.1
- [docs/guides/UCL_V2.0_Guide.md](UCL_V2.0_Guide.md) — детали UCL протокола
- [docs/guides/DomainConfig_Guide.md](DomainConfig_Guide.md) — конфигурация доменов

---

**Обновлено:** 2026-03-28
