# Axiom — Технический долг и отложенные задачи

**Обновлено:** 2026-05-23  
**Источник истины:** этот файл + таблица в `docs/BLUEPRINT.md §Незакрытые задачи`

---

## Context Recognizer

### CR-TD-01 — FatigueStore → axiom-experience
`FatigueStore` живёт внутри `ContextRecognizer`. Нужно вынести в `axiom-experience` наравне с `SutraDepthStore` и `InterpretationProfileStore`.  
**Когда:** V7.

### CR-TD-02 — TransitionGraph для directed Cascading
`ActivityAnalyzer` сейчас детектирует переключения, но граф переходов не строится. Нужен `TransitionGraph` для направленного каскадирования (A→B с весом) вместо случайного чередования.  
**Когда:** V7.

### CR-TD-03 — Ethics composite
Composite `ethics` неполный: только Logic. Нет Values, Dilemmas, Morality.  
**Когда:** V7, когда будут соответствующие subsystem-примитивы.

### CR-TD-04 — ActivityTrace сериализация
`ActivityTrace` (кольцевые буферы) не сериализуется. Нужно для V9 NeuralAdvisor (обучение на истории активности).  
**Когда:** V7/V9.

---

## FrameWeaver

### FW-TD-01 — RequestFrameDetails
UCL-команда `RequestFrameDetails` не реализована. Нужна для Workstation V2.0 (детальный просмотр Frame).  
**Когда:** Workstation V2.0.

### FW-TD-02 — Per-pair co-activation
Отслеживание совместной активации пар токенов (для CausalWeaver). Структуру не выбирать заранее — ждёт CausalWeaver.  
**Когда:** После CausalWeaver.

### Shell-TD-01 — ShellProximity + crystallization_rules архитектура
`ShellProximity(threshold)` работает только как opt-in (`crystallization_rules: vec![]` по умолчанию). При непустом списке правил `evaluate_crystallization_rules()` не фолбэчит на `stability_threshold` → все кандидаты получают `Defer` → Frames=0.

Решение: добавлять `ShellProximity` в паре с явным `StabilityReached`-правилом, **или** рефакторить `evaluate_crystallization_rules` так чтобы stability_threshold работал всегда как minimum-baseline.

---

## NeuralAdvisor / Emergent

### EMERGENT-TD-01 — Калибровка порогов под неоднородный корпус
Текущие пороги (MIN_DEPTH=1000, MIN_REACTIVATIONS=5) откалиброваны по OBS-02 однородного корпуса (8 текстов × 40-80 инжекций) → 312/312 frames стали кандидатами.

Для реальной discriminative detection нужен **неоднородный корпус** (часть текстов 2-3 инжекции, часть 100+), после чего повторно откалибровать так чтобы только "часто встречаемые" Frame проходили порог.

### EMERGENT-TD-02 — reactivation_count: DREAM-циклы vs per-injection
Сейчас `reactivation_count` инкрементируется в `apply_evidence` → считает DREAM-циклы с activity (~10-15 за 30k тиков). Семантически правильнее было бы считать per-injection реактивации (каждый `on_tick` где Frame активен в Wake).

Вариант: инкрементировать в `dream_activation_acc` вместо `apply_evidence`, что даст суммарное число Wake-тиков где Frame был активен — более гранулярный и быстрее растущий сигнал.

---

## Shell Metrics

### Shell-TD-02 — resonance_search shell bonus
Shell-бонус для `resonance_search` требует изменений в `axiom-arbiter` (добавить поле или метод в Experience для shell-proximity lookup). Отложено до после Shell Metrics V2.

---

## TextPerceptor / Agent

### AGENT-TD-01 — Embeddings
Path A anchor-matching реализован и даёт 100% per-text accuracy (OBS-02). Следующий шаг: заменить `word_signals` / `char_signals` lookup-таблицы на векторные embeddings (полноценный AGENT-TD-01).

Путь: TextPerceptor получает embedding-модель → `perceive(text)` вычисляет вектор → позиция = проекция в семантическое пространство якорей. Механизм `compute_position_from_anchors` (взвешенный центроид) остаётся.

---

## Observability

### OBS-TD-02 — avg_shell_similarity всегда 0
Кандидаты FrameWeaver кристаллизуются за ~60 тиков (stability=3 × scan_interval=20). При `snapshot_every=500` к моменту снапшота активных кандидатов нет.

Варианты: per-crystallization event capture, rolling avg за последние N кристаллизаций, или уменьшить snapshot_every для shell-наблюдений.

### OBS-TD-03 — delta-energy approach нерабочий
`ContextRecognizer::compute_raw_energies` + `AxiomEngine::snapshot_subsystem_energies` оставлены в коде. delta-energy per-text detection не работает: позиции текстовых токенов (centroid якорей) и subsystem refs разнесены, sq_dist в миллионах → вклад ≈ 0. Пригодится когда позиции будут выровнены (embeddings).

---

## Якоря / Конфиг

### Anchor-id — Domain/Layer якоря без id
Файлы `domains/D*.yaml` и `layers/L*.yaml` загружаются и матчатся через `match_text()`, но поле `id:` пустое (`#[serde(default)]`). `AnchorMatchTable` ищет по id → не видит domain/layer якоря.

Решение: добавить `id:` с префиксом по аналогии (`exec_action`, `L1_body` и т.п.) и расширить `subsystem_from_anchor_id()` или добавить отдельный маппинг для domain/layer контекстов.

---

## Workstation / UI

### WS-V2-* — V2.0 backlog
История чата, Pause/Resume, custom benchmark, TLS, sync между сессиями.

### COMP-01 — Vital Signs
Ambient display окно (Companion). Показывает текущее состояние без взаимодействия.

---

## Закрытые (архив)

| ID | Закрыто | Как |
|----|---------|-----|
| OBS-01 E1 | 2026-05-23 | AnchorMatchTable 2-path TextPerceptor, 100% accuracy |
| OBS-01 E2 | 2026-05 | AE conflict_rate 0.0% (разобрано отдельно) |
| OBS-01 E3 | 2026-05 | apply_dream_update call site добавлен в engine.rs |
| OBS-TD-01 | 2026-05-23 | detect_subsystem Path 2 fallback, убран "каждый" из logic_quantifier |
| Anchor-Fill | 2026-05 | Все 6 subsystem + 8 domain + 8 layer YAML заполнены |
