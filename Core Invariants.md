
# Axiom Core Specification

Version: 5.0 (2026-04-30)

## 1 Назначение

**Axiom Core** определяет фундаментальные инварианты системы.
Любая реализация, модуль или расширение обязаны соблюдать эти инварианты.
Нарушение инвариантов означает несовместимость с Axiom Core.

## 2 Термины

### Базовые сущности
**Token** — голографическая частица смысла (64 байта), существующая в Domain с позицией, импульсом и термодинамикой.
**Connection** — направленная связка между двумя Token (64 байта), реализующая пружинную динамику с шлюзами.
**Domain** — изолированный вычислительный контейнер ("клетка") с уникальной физикой поля и мембраной.
**COM (Causal Order Model)** — модель времени как причинного порядка событий, заменяющая wall-clock время.

### Временная модель
**Event** — изменение состояния системы с монотонным event_id (64 байта).
**event_id** — монотонно возрастающий идентификатор, единственная мера времени в системе.
**CausalHorizon** — минимальный event_id по всем активным токенам; монотонный водораздел для pruning памяти.

### Архитектура высокого уровня
**AshtiCore** — фрактальный уровень из 11 Доменов: SUTRA(100), ASHTI(101-108), EXPERIENCE(109), MAYA(110).
**SUTRA** — Домен роли 0, единственная точка входа новых Token; генератор потока.
**ASHTI** — Домены ролей 1-8, активные процессоры с уникальной физикой поля.
**EXPERIENCE** — Домен роли 9, ассоциативная память (Experience + Reflector + SkillSet).
**MAYA** — Домен роли 10, финальная классификация; точка проекции результата.
**CODEX** — живой закон, эволюционирующий из базовых аксиом GENOME.
**GUARDIAN** — над-доменный слой: CODEX + GENOME валидация, адаптация порогов, DREAM предложения.
**GENOME** — конституция системы, заморожена в `Arc<Genome>` после старта, не изменяется в рантайме.
**Gateway** — единственная точка входа для внешних систем; владеет AxiomEngine.
**Channel** — in-process FIFO очередь команд и событий.
**Anchor** — фиксированный семантический токен (mass=255, temperature=0, state=Locked); ориентир для позиционирования новых токенов.
**Over-Domain Layer** — слой над-доменных компонентов над AshtiCore. Читают состояние через `&AshtiCore`, пишут только через UCL. Не владеют доменными данными.
**FrameWeaver** — Over-Domain компонент; сканирует синтаксические связи (категория 0x08) в MAYA, кристаллизует стабильные паттерны в EXPERIENCE как Frame-анкеры.
**Frame** — синтаксический паттерн, кристаллизованный в EXPERIENCE (TOKEN_FLAG_FRAME_ANCHOR, state=STATE_ACTIVE). Особо устойчивые Frame могут быть промоутированы в SUTRA через CODEX (state=STATE_LOCKED).
**lineage_hash** — FNV-1a hash над отсортированными sutra_ids всех участников Frame. Детерминированный идентификатор паттерна независимо от порядка обнаружения связей.

### DREAM Phase
**DreamPhaseState** — состояние системы относительно когнитивного сна. Четыре значения: `Wake` (нормальная обработка), `FallingAsleep` (переход, `dream_propose()` вызывается однократно), `Dreaming` (DreamCycle активен, SUTRA-запись разрешена), `Waking` (возврат, обработка Critical-команд).
**DreamScheduler** — компонент, принимающий решение о переходе в сон. Три триггера: Idle (накопленные idle-тики), Fatigue (composite score ≥ threshold), ExplicitCommand (`:force-sleep`).
**FatigueTracker** — вычисляет composite fatigue score 0–255 из 4 факторов (uncrystallized_candidates, experience_pressure, pending_heavy_proposals, causal_horizon_growth_rate) с весами.
**DreamCycle** — цикл переработки в DREAMING: Stabilization (накопление proposals) → Processing (рассмотрение batch) → Consolidation (финализация). Прерывается GatewayPriority::Critical.
**DreamProposal** — предложение, формируемое Weaver в `dream_propose()`. Два вида: `Promotion` (кристаллизованный Frame → SUTRA) и `HeavyCrystallization` (V2.0, сейчас Vetoed).
**GatewayPriority** — приоритет входящей команды. `Normal` — игнорируется в `Dreaming`. `Critical` — прерывает DreamCycle, переводит в `Waking`. `Emergency` — в V1.0 = Critical; отдельная семантика deferred.

### Адресация доменов
`domain_id = level_id × 100 + structural_role`

Уровень 1: SUTRA=100, EXECUTION=101, SHADOW=102, CODEX=103, MAP=104, PROBE=105, LOGIC=106, DREAM=107, ETHICS=108, EXPERIENCE=109, MAYA=110.

### Конфигурация и физика
**DomainConfig** — 128 байт конфигурация Домена, определяющая его природу и поведение.
**DreamConfig** — конфигурация DREAM-фазы: `SchedulerConfig` (min_wake_ticks, idle_threshold, fatigue_threshold), `FatigueWeightsConfig` (4 веса факторов), `CycleConfig` (max_dream_duration_ticks, max_proposals_per_cycle, batch_size). Загружается из `config/presets/dream.yaml`.
**CausalFrontier V2.0** — приоритетная очередь событий с State Machine и Storm Control.
**GridHash** — O(1) хэш-индекс для ассоциативной памяти; shift-фактор квантует позиционное пространство.
**AnchorSet** — набор якорных токенов трёх уровней (axes/layers/domains), загружается из `config/anchors/`.

### Протоколы и взаимодействие
**UCL (Unified Command Language)** — бинарный протокол команд (64 байта).
**dual-path routing** — SKILLSET fast path → GridHash O(1) → Physics O(N) → ASHTI slow path.
**run_adaptation** — цикл адаптации: adapt_thresholds + adapt_domain_physics + apply_experience_thresholds.
**snapshot_and_prune** — атомарный snapshot + удаление устаревших следов Experience.

## 3 Структура пространства

3.1. **Фундаментальная единица** — Домен (Domain), изолированный суверенный контейнер.
3.2. **Физика Домена** — уникальная для каждого DomainConfig.
3.3. **Один уровень** — AshtiCore: ровно 11 доменов, адресованных через `level_id × 100 + role`.

### Семантическое пространство
3.4. **Оси** — X (порядок↔хаос), Y (жизнь↔смерть), Z (сила↔тишина). Диапазон координат: i16 (−32768..32767).
3.5. **Якорные токены** — фиксируют семантику осей и слоёв. Загружаются из YAML, инжектируются в движок при старте.
3.6. **Позиционирование** — TextPerceptor использует якорное совпадение (Exact→Alias→Substring); fallback — FNV-1a hash.

## 4 Token (64 байта)

4.1. **Структура** — sutra_id, domain_id, type_flags, position, velocity, momentum, термодинамика (mass, temperature, valence), state, lineage_hash, last_event_id.
4.2. **Существование** — Token всегда принадлежит одному Domain.
4.3. **Время** — синхронизируется через last_event_id (COM event_id).
4.4. **Состояния** — STATE_ACTIVE (1), STATE_SLEEPING (2), STATE_LOCKED (3). Locked-токены не двигаются и не затухают.

## 5 Connection (64 байта)

5.1. **Направленность** — source_id → target_id в рамках одного Domain.
5.2. **Динамика** — пружинная модель: strength, stress, elasticity, ideal_dist.
5.3. **Шлюзы** — фильтрация по mass и temperature.

## 6 Event (64 байта)

6.1. **Монотонность** — event_id строго возрастает (COM).
6.2. **Детерминизм** — одинаковые payload_hash → одинаковые изменения.
6.3. **Причинность** — parent_event_id < event_id.

## 7 COM (Causal Order Model)

7.1. **Запрет wall-clock** — `std::time` запрещён в ядре системы.
7.2. **Монотонный счётчик** — COM::next_event_id() — единственный источник времени.
7.3. **Доменная изоляция** — события разных доменов независимы.

## 8 Персистентность

8.1. **Формат** — bincode (бинарный, без схемы). Не JSON.
8.2. **Атомарность** — запись через временный файл + rename. Частичная запись невозможна.
8.3. **Путь** — `<data_dir>/engine_state.bin`. Манифест: `memory_manifest.yaml`.
8.4. **Import weight** — при импорте чужих traces веса умножаются на 0.7 (GUARDIAN-валидация).

## 9 Фундаментальные инварианты

9.1. **Размеры структур** — Token (64B), Connection (64B), Event (64B), DomainConfig (128B).
9.2. **COM модель времени** — нет wall-clock, только event_id.
9.3. **GENOME неизменен** — после `Arc::new(genome)` мутабельных ссылок нет.
9.4. **Один уровень** — ровно 11 доменов на AshtiCore.
9.5. **Монотонный горизонт** — CausalHorizon только растёт, никогда не убывает.
9.6. **Безопасный код** — `#![deny(unsafe_code)]` во всех crates ядра.
9.7. **Якоря неизменяемы** — state=Locked, temperature=0. Физика поля не двигает якорные токены.
9.8. **Fallback гарантирован** — система работает без якорей (FNV-1a hash). Якоря — улучшение, не зависимость.
9.9. **Frame-анкеры в EXPERIENCE** — state=STATE_ACTIVE, type_flags содержит TOKEN_FLAG_FRAME_ANCHOR. Это не то же самое, что семантические якоря (9.7): Frame-анкеры живые, температура и масса изменяются при реактивации.
9.10. **Промоция Frame в SUTRA** — только через CODEX; промотированный анкер: state=STATE_LOCKED, type_flags содержит TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE. Оригинал в EXPERIENCE сохраняется.
9.11. **lineage_hash детерминированность** — FNV-1a над sorted(sutra_ids участников). Одинаковый набор участников → одинаковый hash при любом порядке обнаружения связей. Служит ключом дедупликации Frame.
9.12. **CycleStrategy в EXPERIENCE** — циклические связи в EXPERIENCE допустимы (CycleStrategy::Allow). DAG-инвариант применяется только при промоции в SUTRA.
9.13. **SUTRA-запись FRAME_ANCHOR только в DREAMING** — запись токена с флагом `TOKEN_FLAG_FRAME_ANCHOR` в SUTRA допустима исключительно в состоянии `DreamPhaseState::Dreaming`. Нарушение блокируется `GUARDIAN::check_frame_anchor_sutra_write()`. Это онтологический инвариант: истина кристаллизуется только в состоянии сна.
9.14. **Четыре состояния DREAM** — допустимые значения `DreamPhaseState`: Wake → FallingAsleep → Dreaming → Waking → Wake. Переходы строго последовательны; обратный переход из Dreaming/Waking возможен только через Waking.
9.15. **GatewayPriority::Critical прерывает сон** — команда с `GatewayPriority::Critical` в состоянии `Dreaming` или `Waking` немедленно прерывает DreamCycle и инициирует переход в Waking → Wake. `Normal`-команды в Dreaming отклоняются без обработки.

## 10 Расширения

10.1. **Нарушение инвариантов** — запрещено.
10.2. **Новые DomainConfig** — допустимы при соблюдении 128-байтового размера.
10.3. **Дополнительные EventType** — допустимы в COM.
10.4. **Внешние транспорты** — реализуют `RuntimeAdapter` или `EventObserver`; ядро не зависит от них.
10.5. **Новые уровни якорей** — добавляются через YAML без изменения кода.
10.6. **Over-Domain компоненты** — не хранят собственных доменных данных; читают только через `&AshtiCore`; изменения вносят исключительно через UCL-команды, обрабатываемые Engine.

## 11 Версионирование

11.1. **Мажорная версия** — при изменении инвариантов или размеров структур.
11.2. **Минорная версия** — при добавлении полей с сохранением размеров.
11.3. **Патч-версия** — при исправлении ошибок без изменения API.
