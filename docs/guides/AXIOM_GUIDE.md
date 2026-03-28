# AXIOM Functional Guide

**Версия:** 2.0
**Дата:** 2026-03-28
**Статус системы:** 568 тестов, 0 failures

---

## Содержание

1. [Архитектура](#1-архитектура)
2. [GENOME — конституция системы](#2-genome--конституция-системы)
3. [GUARDIAN — контроль и адаптация](#3-guardian--контроль-и-адаптация)
4. [AshtiCore — 11 доменов](#4-ashticore--11-доменов)
5. [Causal Frontier V2.0](#5-causal-frontier-v20)
6. [Arbiter — dual-path routing](#6-arbiter--dual-path-routing)
7. [Configuration System](#7-configuration-system)
8. [Causal Horizon и Memory Management](#8-causal-horizon-и-memory-management)
9. [UCL — команды и AxiomEngine](#9-ucl--команды-и-axiomengine)
10. [Boot sequence](#10-boot-sequence)
11. [Примеры кода](#11-примеры-кода)

---

## 1. Архитектура

### Граф зависимостей crates

```
axiom-core          — Token (64B), Connection (64B), Event (32B)
axiom-genome        — Genome, GenomeIndex, AccessRule, ProtocolRule
axiom-config        — DomainConfig (128B), HeartbeatConfig, ConfigLoader
axiom-space         — SpatialHashGrid, SpatialConfig, физика поля
axiom-shell         — Shell V3.0, SemanticContributionTable, семантические профили
axiom-frontier      — CausalFrontier V2.0, FrontierConfig, FrontierEntity, Storm Control
axiom-arbiter       — Arbiter, Experience, Reflector, SkillSet, GridHash, COM
axiom-heartbeat     — Heartbeat V2.0
axiom-upo           — UPO v2.2, DynamicTrace, Screen
axiom-domain        — Domain, DomainState, AshtiCore, CausalHorizon
axiom-ucl           — UclCommand, UclResult, OpCode
axiom-runtime       — AxiomEngine, Guardian, Snapshot, orchestrator
```

Зависимости строго однонаправленные — нет циклов.

### Ролевая карта системы

| Компонент | Роль |
|-----------|------|
| **GENOME** | Конституция: неизменяемые правила, матрицы доступа, инварианты |
| **GUARDIAN** | Страж: CODEX + GENOME валидация, адаптация порогов, DREAM предложения |
| **AshtiCore** | Ядро: 11 доменов + Arbiter, физический движок, тик симуляции |
| **Arbiter** | Маршрутизатор: dual-path routing, REFLECTOR, SKILLSET, GridHash |
| **Experience** | Ассоциативная память: следы паттернов, двухфазный резонансный поиск |
| **REFLECTOR** | Статистика рефлексов: per-pattern и per-domain профили |
| **SKILLSET** | Кристаллизованные навыки: мгновенный ответ без физики поля |
| **CausalFrontier** | Причинный фронт: приоритизация событий, защита от шторма |
| **CausalHorizon** | Горизонт причинности: контроль роста памяти, pruning |
| **ConfigLoader** | YAML-инфраструктура: загрузка всех компонентов из файлов |
| **Heartbeat** | Пульс: периодический push токенов в Frontier |
| **UCL** | Язык команд: бинарный протокол AxiomEngine |

---

## 2. GENOME — конституция системы

GENOME — первый компонент при старте. Определяет что **никогда не меняется**. После валидации замораживается в `Arc<Genome>` — никакого `&mut Genome` в рантайме.

### Структура

```rust
pub struct Genome {
    pub version: u32,
    pub invariants: GenomeInvariants,
    pub access_rules: Vec<AccessRule>,
    pub protocol_rules: Vec<ProtocolRule>,
    pub config: GenomeConfig,
}
```

### GenomeInvariants

| Поле | Значение | Смысл |
|------|----------|-------|
| `token_size` | 64 | Token всегда 64 байта |
| `connection_size` | 64 | Connection всегда 64 байта |
| `event_size` | 32 | Event всегда 32 байта |
| `domain_config_size` | 128 | DomainConfig всегда 128 байт |
| `max_domains` | 11 | Один уровень = 11 доменов |
| `no_wall_clock_in_core` | true | `std::time` запрещён в ядре |
| `event_id_monotonic` | true | event_id строго монотонен |

### Матрица доступа (Ashti_Core V2.0)

| Модуль | SutraTokens | AshtiField | ExperienceMemory | MayaOutput | CodexRules | GenomeConfig |
|--------|-------------|------------|-----------------|------------|------------|--------------|
| Arbiter | Read | Execute | Read | Execute | Read | Read |
| Guardian | Read | Control | Control | Read | **ReadWrite** | Read |
| Heartbeat | — | Read | Read | — | — | Read |
| Shell | — | Read | Read | — | — | — |
| Adapters | — | — | — | Read | — | — |

`GenomeIndex` — предвычисленная матрица `[[Permission; 8]; 16]` для O(1) lookup:

```rust
let genome = Arc::new(Genome::default_ashti_core());
// или из файла:
let genome = Arc::new(Genome::from_yaml(Path::new("config/genome.yaml"))?);
```

Невалидный GENOME → `AxiomEngine::try_new` вернёт `Err(AxiomError::InvalidGenome)`.

---

## 3. GUARDIAN — контроль и адаптация

Guardian имеет два источника правил: GENOME (абсолютные) + CODEX (пластичные в DomainState). Начиная с Этапа 6, Guardian также отвечает за адаптацию параметров системы.

### Создание

```rust
let guardian = Guardian::new(Arc::clone(&genome));
let guardian = Guardian::with_default_genome(); // для тестов и отладки
```

### Валидация рефлексов

```rust
match guardian.validate_reflex(&token) {
    ReflexDecision::Allow => { /* отправить рефлекс */ }
    ReflexDecision::Veto(VetoReason::TokenLocked) => {}
    ReflexDecision::Veto(VetoReason::ValenceWithoutMass) => {}
    ReflexDecision::Veto(VetoReason::ZeroSutraId) => {}
    ReflexDecision::Veto(VetoReason::GenomeDenied) => {}
}
```

CODEX правила: `STATE_LOCKED`, `valence != 0 && mass == 0`, `sutra_id == 0`.

Бит `GUARDIAN_CHECK_REQUIRED` (0x04) в `DomainConfig::arbiter_flags` — orchestrator вызывает validate_reflex перед отправкой рефлекса в MAYA.

### Сканирование доменов

```rust
let actions: Vec<InhibitAction> = guardian.scan_domain(&domain_state);
// InhibitReason::ValenceWithoutMass { token_index }
```

### Адаптивные пороги (Этап 6)

GUARDIAN читает статистику REFLECTOR и корректирует DomainConfig. Вызов через `AxiomEngine::run_adaptation()`:

```rust
// Автоматически: собирает stats → adapt_thresholds → adapt_domain_physics → apply_experience_thresholds
let updated_domain_ids = engine.run_adaptation();
```

Алгоритм `adapt_thresholds`:
- `success_rate > 0.8` AND `calls ≥ 10` → снизить `reflex_threshold` на 5 (система учится доверять рефлексам)
- `success_rate < 0.3` AND `calls ≥ 10` → повысить `reflex_threshold` на 5 (система становится консервативнее)

Алгоритм `adapt_domain_physics`:
- `success_rate > 0.7` → охладить домен (temperature −5) + ускорить резонанс (resonance_freq +10)
- `success_rate < 0.3` → нагреть (temperature +5) + замедлить (resonance_freq −10)

### DREAM(7) — предложения CODEX

```rust
// Анализирует Experience: паттерны с weight ≥ 0.9 и success_count ≥ 5
let proposals: Vec<CodexAction> = engine.dream_propose();
// CodexAction::AddRule(token) — предложение добавить токен в CODEX
```

Или через Guardian напрямую с кастомными кандидатами:

```rust
let proposals = guardian.dream_propose(&candidate_tokens); // до 5 за вызов
```

### Статистика

```rust
let stats = guardian.stats();
// stats.reflex_allowed, stats.reflex_vetoed
// stats.thresholds_adapted — сколько раз адаптировались пороги
// stats.dream_proposals    — сколько CodexAction предложений сгенерировано
```

---

## 4. AshtiCore — 11 доменов

### Адресация

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
  107 = DREAM       (роль 7)  — фоновая оптимизация
  108 = VOID        (роль 8)  — пустота
  109 = EXPERIENCE  (роль 9)  — ассоциативная память
  110 = MAYA        (роль 10) — выход
```

### Основные методы

```rust
let mut core = AshtiCore::new(1);

// Тик физики
let events: Vec<Event> = core.tick();

// Dual-path обработка токена
let result: RoutingResult = core.process(token);

// Инъекция токена
core.inject_token(domain_id, token)?;

// Обратная связь в Experience
core.apply_feedback(event_id)?;
```

### Доступ к компонентам

```rust
core.reflector()                // &Reflector — для чтения статистики
core.experience_mut()           // &mut ExperienceModule
core.arbiter_domain_configs_mut() // &mut HashMap<u32, DomainConfig>
core.apply_experience_thresholds() // синхронизировать пороги Experience
core.export_skills()            // Vec<Skill> — экспорт навыков
core.import_skills(&skills)     // usize — импортировать навыки, возвращает кол-во импортированных
```

---

## 5. Causal Frontier V2.0

CausalFrontier — приоритетная очередь событий с защитой от каскадного шторма.

### Пресеты FrontierConfig

```rust
FrontierConfig::tight()   // max_events=512,  storm_threshold=1000,  budget=50
FrontierConfig::medium()  // max_events=2048, storm_threshold=5000,  budget=200
FrontierConfig::wide()    // max_events=8192, storm_threshold=20000, budget=1000
```

### FrontierEntity

```rust
pub enum FrontierEntity {
    Token(u32),
    Connection(u32),
    Region(u32, u32),
    Batch(u32),
}
```

### Жизненный цикл

```rust
frontier.begin_cycle();
while let Some(entity) = frontier.pop() { /* обработка */ }
frontier.end_cycle();

let metrics = frontier.storm_metrics();
// metrics.frontier_growth_rate > 1.0 → шторм
```

### Состояния

`Active` → `Storm` → `Stabilizing` → `Idle`. При достижении `max_events` — `push()` отбрасывает события (Heartbeat подберёт при следующем цикле).

---

## 6. Arbiter — dual-path routing

```
Token → SKILLSET?  ──yes──→ мгновенный рефлекс (кристаллизованный навык)
          │no
          ↓
      resonance_search [GridHash O(1) Phase 1 → Physics O(N) Phase 2]
          │
    score ≥ reflex_t?  ──yes──→  рефлекс (fast path) ─────────────┐
          │no                                                        │
          ↓                                                          ↓
     ASHTI(1-8) slow path → MAYA → consolidated       finalize_comparison
          │                                                     │
          └──────────────────────────────────────────→ REFLECTOR + Experience
```

### Experience — ассоциативная память

Хранит до 1000 следов (`ExperienceTrace`). Каждый след: `pattern: Token`, `weight: f32`, `created_at: u64`, `last_used: u64`, `success_count: u32`.

**Двухфазный поиск:**
- **Phase 1 (GridHash O(1)):** `AssociativeIndex` ищет следы в той же ячейке grid. Ранний выход при `score ≥ reflex_threshold`.
- **Phase 2 (Physics O(N)):** FNV-1a prefilter + полный линейный поиск. Активируется при промахе Phase 1.

```rust
let result = experience.resonance_search(&token);
match result.level {
    ResonanceLevel::Reflex      => { /* быстрый ответ */ }
    ResonanceLevel::Association => { /* подсказка для slow path */ }
    ResonanceLevel::None        => { /* новый паттерн */ }
}
```

### GridHash

```rust
use axiom_arbiter::{grid_hash, grid_hash_with_shell, AssociativeIndex};

let key = grid_hash(&token, 4);          // shift=4 → ячейки 16 квантов
let key = grid_hash_with_shell(&token, &shell_profile, 4); // + Shell профиль

// AssociativeIndex: HashMap<grid_key, Vec<trace_id>>
let mut index = AssociativeIndex::new(4);
index.insert(key, trace_id);
let traces: Option<&[u64]> = index.lookup(key);
index.remove_by_trace_id(trace_id);
```

Смысл `shift`: при `shift=4` позиции [0..15, 0..15, 0..15] попадают в одну ячейку. Чем больше shift, тем грубее квантование.

### REFLECTOR — статистика рефлексов

```rust
// per-pattern
reflector.record_reflex(pattern_hash, success);
let stats: Option<&ReflexStats> = reflector.get_stats(pattern_hash);
// stats.success_count, stats.fail_count, stats.success_rate()

// per-domain (role 1..8)
reflector.record_domain(role, &shell_profile, success);
let profile: Option<&DomainProfile> = reflector.domain_profile(role);
// profile.total_calls()
// profile.overall_success_rate()
// profile.layer_success_rate(layer)

// глобально
let rate: f32 = reflector.global_success_rate();
```

REFLECTOR автоматически обновляется в `finalize_comparison`. Данные читаются Guardian для `run_adaptation()`.

### SKILLSET — кристаллизованные навыки

Навык кристаллизуется из ExperienceTrace при: `weight ≥ 0.8` AND `success_count ≥ 50`.

```rust
// Кристаллизация (вызывается автоматически в finalize_comparison)
skillset.try_crystallize(&trace); // → bool

// Поиск (similarity ≥ 0.9)
if let Some(skill) = skillset.find_skill(&token) {
    // мгновенный ответ
}
```

**Обмен скиллами между экземплярами:**

```rust
// Источник
let snapshot: Vec<Skill> = engine_a.export_skills();

// Получатель (дедупликация + вес × 0.3)
let imported: usize = engine_b.import_skills(&snapshot);
```

Импортированные навыки начинают с `activation_weight × 0.3` и `success_count = 0` — система "не доверяет" чужому опыту сразу.

---

## 7. Configuration System

Все параметры системы загружаются из YAML. Никакого hardcode в рантайме.

### ConfigLoader

```rust
use axiom_config::{ConfigLoader, LoadedAxiomConfig};

let loaded: LoadedAxiomConfig = ConfigLoader::load_all("config/axiom.yaml")?;

// Доступ к конфигам
let domains: &HashMap<String, DomainConfig> = &loaded.domains;
let sutra_cfg = &domains["sutra"];

// Пути к другим конфигам (загружаются своими crate'ами)
let spatial_path = spatial_config_path(&loaded, base);
let semantic_path = semantic_contributions_path(&loaded, base);
```

### Структура config/

```
config/
  axiom.yaml                    — корневой конфиг (ссылается на остальные)
  genome.yaml                   — конституция системы
  presets/
    domains/                    — 11 пресетов DomainConfig
      sutra.yaml, execution.yaml, shadow.yaml, codex.yaml,
      map.yaml, probe.yaml, logic.yaml, dream.yaml,
      void.yaml, experience.yaml, maya.yaml
    spatial/
      tight.yaml, medium.yaml, loose.yaml
  schema/
    semantic_contributions.yaml — вклады Shell-слоёв по категориям
```

### DomainConfig из YAML

```rust
let cfg = DomainConfig::from_yaml(Path::new("config/presets/domains/logic.yaml"))?;
cfg.validate()?; // проверка инвариантов (ёмкость, пороги, размеры)
```

11 factory-методов для создания без YAML: `factory_sutra`, `factory_execution`, ..., `factory_maya`.

### SpatialConfig

```rust
let spatial = SpatialConfig::from_yaml(Path::new("config/presets/spatial/medium.yaml"))?;
let grid = SpatialHashGrid::with_config(&spatial);

// Пресеты
SpatialConfig::tight()   // cell_shift=6,  bucket_count_log2=17 — плотная сетка
SpatialConfig::medium()  // cell_shift=8,  bucket_count_log2=16
SpatialConfig::loose()   // cell_shift=10, bucket_count_log2=14 — разреженная
```

### SemanticContributionTable

```rust
let table = SemanticContributionTable::from_yaml(
    Path::new("config/schema/semantic_contributions.yaml")
)?;
// Используется Shell V3.0 для вычисления профилей токенов
```

---

## 8. Causal Horizon и Memory Management

Система предотвращает рост памяти при долгих запусках через механизм причинного горизонта.

### CausalHorizon

Горизонт = `min(token.last_event_id)` по всем активным токенам всех доменов. Монотонный — только растёт.

```rust
use axiom_domain::CausalHorizon;

let mut horizon = CausalHorizon::new();
let state_refs: Vec<&DomainState> = states.iter().collect();

// Вычислить текущий горизонт
let h: u64 = CausalHorizon::compute(&state_refs);

// Обновить (монотонно)
horizon.advance(&state_refs);

// Проверить устаревание
horizon.is_behind(last_event_id) // → true если устарел
```

Через `AxiomEngine`:

```rust
let h: u64 = engine.causal_horizon();
let removed: usize = engine.run_horizon_gc(); // удалить устаревшие следы Experience
```

### Event Log Pruning

Snapshot фиксирует состояние системы. Все следы Experience с `last_used < snapshot.created_at` можно безопасно удалить.

```rust
// Вариант 1: раздельно
let snap = engine.snapshot();
// snap.snapshot_event_id() == engine.causal_horizon() на момент вызова
let pruned: usize = engine.prune_after_snapshot(&snap);

// Вариант 2: атомарно
let (snap, pruned) = engine.snapshot_and_prune();
println!("зафиксировано доменов: {}, удалено следов: {}", snap.domain_count(), pruned);
```

Инспекция до удаления:

```rust
let count = engine.ashti.experience_mut().prunable_count(engine.causal_horizon());
println!("можно удалить {} следов", count);
```

### Обмен скиллами

Скиллы можно переносить между экземплярами без передачи всей базы Experience:

```rust
// export / import — см. раздел 6 (SKILLSET)
// Типичный сценарий: сохранить обученные навыки перед прунингом памяти
let skills = engine.export_skills();
engine.snapshot_and_prune();        // очистить старую память
engine.import_skills(&skills);      // восстановить навыки
```

---

## 9. UCL — команды и AxiomEngine

### UclCommand (64 байта)

```rust
UclCommand {
    payload: [u8; 48],
    command_id: u64,
    target_id: u32,
    opcode: u16,
    priority: u8,
    flags: u8,
}
```

### Коды команд

| OpCode | Значение | Действие |
|--------|----------|---------|
| `SpawnDomain` | 1000 | no-op (домены фиксированы) |
| `InjectToken` | 2000 | добавить токен в домен |
| `TickForward` | 3000 | один шаг физики → events |
| `ProcessTokenDualPath` | 4000 | dual-path routing |
| `FinalizeComparison` | 4001 | обратная связь → Experience |
| `BackupState` | 9002 | snapshot |
| `CoreReset` | 9001 | сбросить состояние |
| `CoreShutdown` | 9000 | завершение |

### Прямые методы AxiomEngine

Помимо UCL, AxiomEngine предоставляет прямой API:

```rust
// Физика и маршрутизация
engine.ashti.tick()                    // шаг физики
engine.ashti.process(token)            // dual-path
engine.drain_events()                  // Vec<Event>

// Адаптация (Этап 6)
engine.run_adaptation()                // Vec<u32> обновлённых domain_id
engine.dream_propose()                 // Vec<CodexAction>

// Memory management (Этап 7)
engine.causal_horizon()                // u64
engine.run_horizon_gc()                // usize удалено
engine.snapshot()                      // EngineSnapshot
engine.snapshot_and_prune()            // (EngineSnapshot, usize)
engine.prune_after_snapshot(&snap)     // usize удалено

// Навыки (Этап 7)
engine.export_skills()                 // Vec<Skill>
engine.import_skills(&skills)          // usize импортировано
```

---

## 10. Boot sequence

```
1. Genome::from_yaml("config/genome.yaml")  или  Genome::default_ashti_core()
   → validate()
   → Arc::new(genome) — заморозка

2. AxiomEngine::try_new(Arc::clone(&genome))
   → Guardian::new(genome) — строит GenomeIndex
   → AshtiCore::new(level_id=1) — 11 доменов + Arbiter
     → Experience (max_traces=1000, shift=4)
     → Reflector, SkillSet
     → Arbiter регистрирует все 11 доменов

3. Опционально: загрузка конфигов
   let loaded = ConfigLoader::load_all("config/axiom.yaml")?;
   // применить загруженные DomainConfig к Arbiter

4. Система готова к UCL командам и прямым вызовам
```

После `CoreReset` — AshtiCore и Guardian пересоздаются, `Arc<Genome>` не меняется.

---

## 11. Примеры кода

### Создание Engine

```rust
use std::sync::Arc;
use axiom_genome::Genome;
use axiom_runtime::AxiomEngine;

let mut engine = AxiomEngine::new(); // дефолтный Genome
// или:
let genome = Arc::new(Genome::from_yaml(Path::new("config/genome.yaml"))?);
let mut engine = AxiomEngine::try_new(genome)?;
```

### Инъекция токена и тик

```rust
use axiom_core::Token;

let domain_id = engine.ashti.domain_id_at(6).unwrap(); // LOGIC
let mut token = Token::new(1, domain_id as u16, [0, 0, 0], 1);
token.temperature = 200;
token.mass = 100;
engine.ashti.inject_token(domain_id, token).unwrap();

// ~1024 тиков до первого heartbeat
for i in 0..1100u64 {
    engine.ashti.tick();
}
let events = engine.drain_events();
```

### Dual-path routing с обратной связью

```rust
let token = Token::new(42, 100, [100, 200, 50], 1);

let result = engine.ashti.process(token);
println!("reflex: {:?}", result.reflex);
println!("consolidated: {:?}", result.consolidated);

// Обратная связь: обучаем Experience
engine.ashti.apply_feedback(result.event_id).ok();
```

### Цикл адаптации

```rust
// После накопления статистики (обычно 10+ рефлексов)
let updated = engine.run_adaptation();
if !updated.is_empty() {
    println!("обновлены конфиги доменов: {:?}", updated);
}

// DREAM предложения
let proposals = engine.dream_propose();
for p in &proposals {
    if let axiom_runtime::CodexAction::AddRule(token) = p {
        // применить предложение в CODEX
    }
}
```

### Memory management при долгом запуске

```rust
// Периодически (например, каждые 1000 тиков)
let horizon = engine.causal_horizon();
let prunable = engine.ashti.experience_mut().prunable_count(horizon);

if prunable > 100 {
    // Сохраняем навыки перед очисткой
    let skills = engine.export_skills();

    // Snapshot + prune
    let (snap, removed) = engine.snapshot_and_prune();
    println!("snapshot event_id: {}, удалено следов: {}", snap.snapshot_event_id(), removed);

    // Навыки не теряются — они в SkillSet, не в Experience
    println!("навыков сохранено: {}", engine.export_skills().len());
}
```

### Обмен навыками между экземплярами

```rust
// Экспорт из обученного экземпляра
let trained_skills = engine_a.export_skills();
println!("экспортировано: {} навыков", trained_skills.len());

// Импорт в новый экземпляр (дедупликация + вес × 0.3)
let imported = engine_b.import_skills(&trained_skills);
println!("импортировано: {} (без дублей)", imported);
```

### YAML конфигурация

```rust
use axiom_config::{ConfigLoader, DomainConfig};
use std::path::Path;

let loaded = ConfigLoader::load_all(Path::new("config/axiom.yaml"))?;
println!("загружено доменов: {}", loaded.domains.len());

// Отдельная загрузка
let logic_cfg = DomainConfig::from_yaml(Path::new("config/presets/domains/logic.yaml"))?;
logic_cfg.validate()?;
```

---

## Связанные документы

- [ROADMAP.md](../../ROADMAP.md) — история и планы разработки
- [STATUS.md](../../STATUS.md) — текущее состояние, тесты по crates
- [DEVELOPMENT_GUIDE.md](../../DEVELOPMENT_GUIDE.md) — правила разработки
- [DEFERRED.md](../../DEFERRED.md) — технический долг и отложенные задачи
- [docs/spec/GENOME_V1_0.md](../spec/GENOME_V1_0.md) — спецификация GENOME
- [docs/spec/GUARDIAN_V1_0.md](../spec/GUARDIAN_V1_0.md) — спецификация GUARDIAN
- [docs/spec/Ashti_Core_V2_1.md](../spec/Ashti_Core_V2_1.md) — архитектура Ashti_Core V2.1
- [docs/guides/UCL_V2.0_Guide.md](UCL_V2.0_Guide.md) — детали UCL протокола
- [docs/guides/DomainConfig_Guide.md](DomainConfig_Guide.md) — конфигурация доменов

---

**Обновлено:** 2026-03-28 (Этапы 1–7 завершены, 568 тестов)
