# Roadmap Ashti_Core — Актуальный план развития

**Версия:** 2.0  
**Дата:** 2026-03-28  
**Философия:** Мы не строим с нуля — мы выращиваем кристалл. Каждый этап — работающая система с увеличивающейся функциональностью. Каждый этап завершается зелёными тестами и бенчмарками.

---

## Текущее состояние (baseline)

Что **уже сделано**:

- Cargo Workspace: 11 crates (axiom-core, axiom-config, axiom-domain, axiom-space, axiom-shell, axiom-arbiter, axiom-heartbeat, axiom-frontier, axiom-upo, axiom-ucl, axiom-runtime).
- Core-структуры: Token V5.2 (64B), Connection V5.0 (64B), COM Event (32B), DomainConfig V2.1 (128B).
- SpatialHashGrid: zero-alloc, O(1) поиск соседей, бенчмарки подтверждены.
- Shell V3.0: семантический кэш L1-L8, compute + incremental_update + reconciliation.
- Arbiter V1.0: классификация, маршрутизация, обратная связь. Бенчмарк ~4 µs.
- AshtiCore pipeline: SUTRA→EXPERIENCE→Arbiter→ASHTI(1-8)→MAYA. Бенчмарк ~40 µs на акт.
- Causal Frontier V2.0: специфицирован (storm control, state machine, causal horizon).
- Тесты: 336+ тестов, покрытие по всем crates.

Что **специфицировано, но не реализовано**:

- GENOME V1.0 — конституционный слой.
- GUARDIAN V1.0 — над-доменный контроллер.
- Ashti_Core V2.1 — обновлённая архитектура с GENOME/GUARDIAN.
- Storm Control в Causal Frontier (три механизма: budget, batching, size limit).
- REFLECTOR и SKILLSET зоны в EXPERIENCE(9).
- GridHash-индекс для ускорения resonance_search.

---

## ЭТАП 1: GENOME + GUARDIAN (Конституция)

**Цель:** Система получает неизменяемый закон и контроллер, который его исполняет.

**Результат:** GENOME загружается первым, GUARDIAN проверяет рефлексы и доступ, CODEX(3) управляется через GUARDIAN.

### Задачи:

**1.1 Crate `axiom-genome`**

Новый crate в workspace. Содержит:
- `Genome` struct: инварианты, права доступа, протоколы, конфигурация.
- `GenomeIndex`: предвычисленная матрица для O(1) lookup (access_matrix, protocol_matrix).
- Загрузка из `config/genome.yaml` через serde_yaml.
- Валидация при загрузке (невалидный GENOME → система не запускается).
- `Arc<Genome>` — shared immutable reference для всех модулей.

Зависимости: axiom-core, serde, serde_yaml.

**1.2 GUARDIAN в `axiom-runtime`**

Расширить axiom-runtime модулем `guardian.rs`:
- `Guardian` struct с `Arc<Genome>` и `GenomeIndex`.
- `validate_reflex()` — проверка рефлекса по GENOME + CODEX.
- `enforce_access()` — O(1) проверка прав через матрицу.
- `enforce_protocol()` — O(1) проверка маршрута через матрицу.
- `scan_domain()` — сканирование узоров на соответствие.
- `update_codex()` — управление правилами CODEX(3).
- COM-события: ReflexApproved, ReflexVetoed, PatternInhibited, CodexRuleUpdated.

**1.3 Boot sequence**

Обновить `AxiomEngine::new()`:
1. Загрузить genome.yaml → Genome → Arc<Genome>.
2. Создать Guardian(Arc<Genome>).
3. Создать домены 0-10 (каждый получает Arc<Genome>).
4. Создать Arbiter(Arc<Genome>).
5. Инициализировать COM.
6. Guardian валидирует DomainConfig'и.

**1.4 Файл `config/genome.yaml`**

Создать конфигурацию с полной таблицей прав и протоколов для 11 доменов Ashti_Core.

**1.5 Интеграция Arbiter ↔ GUARDIAN**

Обновить Arbiter: при GUARDIAN_CHECK_REQUIRED → вызов Guardian.validate_reflex() перед отправкой рефлекса в MAYA.

**Тесты:**
- Загрузка и валидация GENOME.
- O(1) access/protocol checks.
- Reflex approve/veto.
- Неавторизованный доступ → блокировка.
- Boot sequence с GENOME.

**Бенчмарки:**
- `Guardian::validate_reflex` — целевое время < 500 ns.
- `Guardian::enforce_access` — целевое время < 5 ns.
- AshtiCore pipeline с GUARDIAN — измерить overhead.

**Критерий:** `cargo test --workspace` зелёный. GENOME загружается. GUARDIAN проверяет рефлексы. Pipeline работает.

---

## ЭТАП 2: Storm Control (Защита от каскадов)

**Цель:** Система защищена от причинных лавин при интенсивной обработке.

**Результат:** Causal Frontier с budget, batching, state machine. Система не зависает при каскадных событиях.

### Задачи:

**2.1 FrontierState (state machine)**

В axiom-frontier реализовать `FrontierState` enum: Empty, Active, Storm, Stabilizing, Idle. Переходы по правилам из Causal Frontier V2.0, раздел 8.

**2.2 Causal Budget**

`max_events_per_cycle` — жёсткий лимит. `pop()` возвращает None при исчерпании. Frontier сохраняется до следующего цикла.

**2.3 Frontier Size Limit**

`max_frontier_size` — предохранитель. `push_token()` отбрасывает при переполнении (Heartbeat подхватит позже).

**2.4 Batch Events (опционально)**

При состоянии Storm: объединение однотипных событий (100× TokenMoved → 1× BatchTokenMoved).

**2.5 FrontierConfig**

Добавить в Configuration System: max_frontier_size, max_events_per_cycle, storm_threshold, enable_batch_events. Три пресета: weak/medium/strong hardware.

**Тесты:**
- State machine transitions.
- Budget enforcement: pop() → None после лимита.
- Size limit: push отбрасывает при переполнении.
- Storm detection при каскаде.
- Determinism: одинаковые входы → одинаковый результат.

**Бенчмарки:**
- TickForward с 1000+ токенами и Storm Control.
- Overhead state machine transitions.

**Критерий:** Система не зависает при каскаде из 10000+ событий. Budget и size limit работают.

---

## ЭТАП 3: EXPERIENCE(9) — REFLECTOR + SKILLSET

**Цель:** Домен 9 учится на опыте и кристаллизует навыки.

**Результат:** Статистика успешности рефлексов, профилирование, кристаллизация скиллов.

### Задачи:

**3.1 REFLECTOR (структуры данных в DomainState домена 9)**

- `ReflexStats`: счётчик per-reflex — сколько раз совпал/не совпал с результатом 1-8.
- `DomainProfile`: какие Shell-профили (L1-L8) чаще приводят к успеху в каких доменах.
- Обновление при обратной связи из MAYA → Arbiter → EXPERIENCE.
- Интерфейс для GUARDIAN: данные для адаптации порогов.

**3.2 SKILLSET (зона кристаллизации)**

- Критерии кристаллизации: минимальный weight, минимальное количество подтверждений (N), устойчивость кластера (все связи в кластере > threshold).
- `Skill` struct: группа token_indices + connection_indices, помеченных как скилл.
- Активация скилла как единого целого при resonance_search.
- Экспорт/импорт скиллов (сериализация кластера).

**3.3 Обновление resonance_search**

Учесть скиллы при поиске: если входящий паттерн резонирует со скиллом, весь скилл возвращается как единый ответ (не поэлементно).

**Тесты:**
- REFLECTOR: подсчёт статистики, обновление при обратной связи.
- SKILLSET: кристаллизация кластера при достижении критериев.
- Активация скилла при resonance_search.
- Экспорт/импорт скилла.

**Бенчмарки:**
- resonance_search с SKILLSET vs без.
- Overhead REFLECTOR при обратной связи.

**Критерий:** Рефлексы имеют статистику. Кластеры кристаллизуются. Скиллы активируются.

---

## ЭТАП 4: GridHash-индекс (Ускорение Fast Path)

**Цель:** resonance_search ускоряется от O(N) до O(1) для знакомых ситуаций.

**Результат:** GridHash-индекс поверх физики поля. 30-50 ns на lookup знакомого паттерна.

### Задачи:

**4.1 GridHash функция**

Адаптация из IntuitionEngine NeuroGraph: хэширование Shell-профиля [u8; 8] + position в один u64 ключ. Побитовый сдвиг (shift-фактор), XOR, rotate_left. Только целочисленная арифметика.

**4.2 AssociativeIndex**

Предвыделённая хэш-таблица внутри DomainState домена 9:
- Ключ: u64 (GridHash).
- Значение: SmallVec<[u32; 4]> — индексы следов в домене 9.
- Zero-alloc: предвыделена до capacity.

**4.3 Интеграция с resonance_search**

Двухфазный поиск:
1. **Phase 1 (GridHash):** O(1) lookup по хэшу. Если Hit и weight >= reflex_threshold → ранний выход, рефлекс.
2. **Phase 2 (физика поля):** Если Miss или weight недостаточен → полный резонансный поиск (текущий алгоритм).

GridHash **не заменяет** физику поля — дополняет. Физика сохраняется для ассоциаций, тишины, и уточнения результатов.

**4.4 Обучение индекса**

При записи нового опыта в EXPERIENCE(9): вычислить GridHash, добавить в AssociativeIndex. При ослаблении следа ниже порога: удалить из индекса. TTL не нужен — затухание через причинный возраст.

**4.5 Shift-фактор (настройка)**

Конфигурируемый параметр. Слишком мал → индекс слишком мелкий, постоянные Miss. Слишком велик → путает разные ситуации. Подбирается на тестах.

**Тесты:**
- GridHash: детерминизм, распределение коллизий.
- AssociativeIndex: Hit/Miss при разных shift-факторах.
- Двухфазный resonance_search: корректность результатов при Hit и Miss.
- Обучение: добавление и удаление из индекса.

**Бенчмарки:**
- resonance_search с GridHash vs без (целевое ускорение: 10x+ при Hit).
- GridHash computation: целевое < 30 ns.
- Full pipeline с GridHash: целевое < 35 µs на акт (улучшение от текущих 40 µs).

**Критерий:** Знакомые ситуации обрабатываются за 30-50 ns. Незнакомые — как раньше. Корректность не нарушена.

---

## ЭТАП 5: Адаптивные пороги и динамическая физика

**Цель:** Система адаптируется к опыту: пороги Arbiter корректируются, физика доменов настраивается.

**Результат:** REFLECTOR данные влияют на DomainConfig через GUARDIAN.

### Задачи:

**5.1 Адаптивные пороги Arbiter**

REFLECTOR накапливает статистику: "рефлексы EXECUTION(1) совпадают в 95% случаев". GUARDIAN видит эту статистику и может снизить reflex_threshold для EXECUTION(1) в DomainConfig. Или повысить для LOGIC(6) если совпадение только 40%.

Механизм: GUARDIAN → обновление DomainConfig → COM-событие DomainConfigUpdated → Arbiter перечитывает пороги.

**5.2 Динамическая реконфигурация доменов**

DomainConfig изменяемый в рантайме (через GUARDIAN). Температура, гравитация, resonance_freq могут адаптироваться к нагрузке и качеству обработки.

**5.3 DREAM(7) как оптимизатор**

DREAM(7) в фоновом режиме (через Heartbeat) анализирует следы в EXPERIENCE(9), ищет неочевидные связи, предлагает изменения правил CODEX через GUARDIAN.

**Тесты:**
- Адаптация порогов при накоплении статистики.
- DomainConfig update через GUARDIAN.
- DREAM фоновая оптимизация.

**Критерий:** Пороги адаптируются. DomainConfig меняется в рантайме. DREAM генерирует предложения.

---

## ЭТАП 6: Causal Horizon + Масштабирование

**Цель:** Система работает устойчиво при длительных запусках. COM event log не растёт бесконечно.

**Результат:** Архивация истории за горизонтом причинности. Snapshot + Horizon.

### Задачи:

**6.1 Causal Horizon (из Causal Frontier V2.0, раздел 13)**

Вычисление horizon = min(last_event_id) по всем активным сущностям. События до horizon безопасно архивируются.

**6.2 Event Log pruning**

Snapshot фиксирует состояние. События до snapshot_event_id удаляются из рабочего набора. Опционально: сжатие и запись на диск.

**6.3 Фрактальные уровни**

Протокол 10→0: выход MAYA одного уровня → вход SUTRA следующего. Запуск нескольких уровней Ashti_Core.

**6.4 Обмен скиллами**

Экспорт/импорт SKILLSET между экземплярами AXIOM. Импортированные скиллы начинают с низким weight (проверяются собственной обработкой перед усилением).

**Тесты:**
- Horizon computation.
- Event log pruning + snapshot restore.
- Двухуровневая цепочка 10→0.
- Экспорт/импорт скиллов.

**Критерий:** Система работает часами без роста памяти. Фрактальная цепочка функционирует.

---

## Сводка этапов

| Этап | Название | Ключевой результат | Зависимости |
|------|---------|-------------------|-------------|
| 1 | GENOME + GUARDIAN | Конституция, контроль доступа, проверка рефлексов | Текущий baseline |
| 2 | Storm Control | Защита от каскадов, state machine frontier | Этап 1 |
| 3 | REFLECTOR + SKILLSET | Статистика, кристаллизация скиллов | Этапы 1-2 |
| 4 | GridHash | O(1) fast path для знакомых ситуаций | Этап 3 |
| 5 | Адаптивные пороги | Самонастройка системы | Этапы 3-4 |
| 6 | Horizon + Масштаб | Долгий запуск, фракталы, обмен скиллами | Этапы 1-5 |

**Принцип:** Каждый этап — работающая система. Никогда не работаем со всем сразу. Один новый модуль интегрируется в уже работающее ядро.

---

## История изменений

- **V2.0**: Полная переработка. Отсчёт от текущего baseline (мигрированный workspace, бенчмарки). Добавлены GENOME/GUARDIAN, Storm Control, REFLECTOR/SKILLSET, GridHash, адаптивные пороги, Causal Horizon. Убраны пройденные этапы (заглушки, базовые структуры).
- **V1.0**: Оригинальный роадмап NeuroGraph. Этапы 0-5 от нуля.
