# Axiom Development Guide

**Версия:** 3.0  
**Дата:** 2026-04-24

---

## Структура репозитория

```
Axiom/
├── crates/
│   ├── axiom-core/        # Token, Connection, Event — базовые структуры
│   ├── axiom-config/      # DomainConfig, ConfigLoader, AnchorSet, JsonSchema
│   ├── axiom-genome/      # Genome, GenomeInvariants, ModuleId, types
│   ├── axiom-space/       # Семантическое пространство, физика, SpatialHashGrid
│   ├── axiom-shell/       # ShellProfile, SemanticContributionTable, link_types
│   ├── axiom-frontier/    # CausalFrontier V2.0, Storm Control
│   ├── axiom-heartbeat/   # Heartbeat V2.0
│   ├── axiom-ucl/         # UCL V2.0 — бинарный протокол команд (64B)
│   ├── axiom-upo/         # UPO v2.3 — динамические следы опыта
│   ├── axiom-arbiter/     # Arbiter, Experience, Reflector, SkillSet, AshtiCore
│   ├── axiom-domain/      # Domain, DomainState, FractalChain
│   ├── axiom-runtime/     # AxiomEngine, Guardian, Over-Domain Layer, Gateway
│   │   └── src/over_domain/
│   │       ├── traits.rs          # OverDomainComponent, Weaver
│   │       └── weavers/
│   │           └── frame.rs       # FrameWeaver V1.1
│   ├── axiom-persist/     # Персистентность: bincode, AutoSaver, exchange
│   ├── axiom-agent/       # CLI, tick_loop, External Adapters, MLEngine
│   └── axiom-bench/       # Criterion бенчмарки
├── tools/
│   └── axiom-dashboard/   # egui/eframe GUI
├── config/
│   ├── genome.yaml        # Конституция системы (должна совпадать с Genome::default_ashti_core())
│   ├── axiom.yaml         # Основная конфигурация
│   ├── anchors/           # Семантические якоря (axes/, layers/, domains/)
│   └── presets/           # Пресеты DomainConfig
├── docs/
│   ├── spec/              # Спецификации (канон)
│   │   └── Weaver/        # Over-Domain Layer, FrameWeaver
│   ├── guides/            # Руководства для разработчиков
│   ├── arch/              # Архитектурные решения
│   └── bench/             # Результаты бенчмарков
├── Core Invariants.md     # Фундаментальные инварианты системы
├── STATUS.md              # Текущее состояние: тесты, архитектура
├── ROADMAP.md             # Активные задачи (удалять завершённое)
├── DEFERRED.md            # Технический долг и отложенные задачи
└── DEVELOPMENT_GUIDE.md   # Этот файл
```

---

## Архитектурные принципы

### Асимметрия Token и Connection

**Источник:** `docs/spec/Симетрия.md`

| Сущность | Слой | Вопрос | Ответственность |
|----------|------|--------|-----------------|
| **Token** | Бытие | "Что это?" | Состояние, идентичность |
| **Connection** | Действие | "Что это делает?" | Динамика, взаимодействие |

**Правило при добавлении поля:**
> "Кто несёт ответственность, если атрибут нарушен?"
> - Нарушена **сущность** → Token
> - Нарушено **взаимодействие** → Connection

Запрещено: зеркальные поля в обеих структурах, смешивание статики и динамики.

### Модель времени

**Источник:** `docs/spec/time/Time_Model_V1_0.md`

| Слой | Механизм | Где живёт |
|------|----------|-----------|
| Причинный порядок | `event_id` (u64) | Ядро |
| Причинный возраст | `current_event_id - last_event_id` | Ядро |
| Реальное время | wall-clock | Только адаптеры |

Запрещено в ядре: `std::time`, `SystemTime`, `Instant`, `sleep()`, `timestamp_ms`.

### Over-Domain Layer

**Источник:** `docs/spec/Weaver/Over_Domain_Layer_V1_1.md`  
**Гайд:** `docs/guides/FrameWeaver_Guide_V1_1.md`

Компоненты над AshtiCore. Три правила:
1. Читают состояние только через `&AshtiCore` (иммутабельно)
2. Пишут только через UCL-команды, исполняемые Engine
3. Не хранят собственных доменных данных

**При добавлении нового Weaver:**
- Новый `ModuleId` в `axiom-genome/src/types.rs`
- Access rules в `Genome::default_ashti_core()` **и** в `config/genome.yaml` (должны совпадать)
- Тест `test_from_yaml_matches_default` подтвердит синхронность

---

## Workflow

### Стандартный цикл

```
1. Читаем спецификацию (docs/spec/)
2. Реализуем в нужном crate
3. Пишем тесты
4. Обновляем STATUS.md, ROADMAP.md, DEFERRED.md
5. Обновляем Core Invariants.md при изменении инвариантов
6. Коммит + пуш
```

### Правила

- **НИКОГДА не менять спецификации** (`docs/spec/`) без явного запроса — канон
- **README.md — только по явному запросу** пользователя, даже если данные устарели
- **ВСЕГДА обновлять Core Invariants.md** при изменении поведения базовых структур
- **ВСЕГДА проверять синхронность** `Genome::default_ashti_core()` ↔ `config/genome.yaml`
- **Нет wall-clock в ядре** — проверять при любом изменении core/domain/runtime
- **Нет unsafe** — `#![deny(unsafe_code)]` во всех crates ядра

### Критерии готовности задачи

- [ ] Код реализован согласно спецификации
- [ ] Тесты написаны и проходят
- [ ] STATUS.md обновлён (тесты, архитектура)
- [ ] ROADMAP.md обновлён (выполненное удалено)
- [ ] DEFERRED.md пополнен (если остались заглушки)
- [ ] Core Invariants.md обновлён (если изменились инварианты)
- [ ] Коммит создан и запушен

---

## Документация

### STATUS.md
- Одна страница — текущее состояние, не история
- Только факты: число тестов, список crates, архитектурное дерево
- Удалять всё устаревшее немедленно

### ROADMAP.md
- Только активные задачи — удалять завершённое сразу
- Принципы документации в конце файла

### DEFERRED.md
- Технический долг с ID (FW-TD-01, EA-TD-07, ...)
- Для каждого: где в коде, что нужно, когда делать

### Core Invariants.md
- Обновлять при любом изменении базовых структур, новых флагах, новых правилах
- Текущая версия: 4.0

---

## Бенчмарки

**Расположение:** `crates/axiom-bench/`

```
crates/axiom-bench/benches/
├── core_bench.rs    # Token, Connection, Event
├── space_bench.rs   # SpatialHashGrid, distance2
├── domain_bench.rs  # AshtiCore, resonance_search
└── engine_bench.rs  # AxiomEngine полный цикл
```

```bash
cargo test --benches -p axiom-bench          # проверка без измерений
cargo bench -p axiom-bench --bench engine_bench
cargo bench -p axiom-bench
```

Результаты фиксируются в `docs/bench/RESULTS.md`.

---

## Тестирование

```bash
cargo test                                   # все тесты
cargo test -p axiom-runtime                  # один crate
cargo test over_domain::weavers::frame       # один модуль
cargo test --features telegram,opensearch    # все features
```

Требования: unit-тесты для всех pub-функций, integration-тесты для модулей.

---

## Git

```bash
git add <files>
git commit -m "type: subject

- detail

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
git push
```

Типы: `feat:` `fix:` `docs:` `test:` `refactor:` `chore:`
