# Axiom Roadmap

**Версия:** 73.0  
**Дата:** 2026-06-03

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
axiom-corpus                                        ↑
                                               axiom-broadcasting
```

**1696 тестов, 0 failures.**  
Sensorium V1.0, Waves V1.0, Cross-Modal Binding pipeline замкнуты (2026-06-03).  
DEFERRED аудит: CR-TD-03, EMERGENT-TD-02, FW-TD-01, CMB-TD-03 закрыты.

---

## Активные задачи

---

### PRIM-TD-04 — TemporalPerceptor

**Файл:** `crates/axiom-agent/src/perceptors/temporal.rs` (новый, аналог `vision_l0.rs`)  
**Суть:** перцептор темпоральных маркеров. Определяет время в тексте → инжектирует токены `time_*`-якорей.

**Шаги реализации:**

1. **`temporal_anchor_stable_id(id: &str) → u32`** (в новом файле или рядом с `vision_anchor_stable_id`).  
   Диапазон: `0x1000_0001..0x1FFF_FFFF` (бит 28 установлен, не пересекается с text/vision/anchor диапазонами).  
   Тот же FNV-1a паттерн.

2. **`TemporalPerceptor` struct** по образцу `L0VisionPerceptor`:
   ```
   TemporalPerceptor { anchors: Vec<Anchor>, pending: VecDeque<UclCommand> }
   ```
   Принимает `time_*`-якоря из `AnchorSet.perceptual_anchors()` (или subsystem anchors для time).

3. **`perceive(text: &str)`** — детектор темпоральных маркеров:
   - `time_before`: "до", "раньше", "прежде", "перед", "до того как", "before", "prior"
   - `time_after`: "после", "затем", "позже", "следом", "потом", "after", "then"
   - `time_simultaneous`: "одновременно", "в то время как", "пока", "meanwhile", "while"
   - `time_periodic`: "каждый", "регулярно", "периодически", "снова", "always", "every"
   - `time_duration`: "в течение", "на протяжении", "долго", "during", "for"
   - `time_moment`: "сейчас", "немедленно", "вдруг", "мгновенно", "now", "suddenly"
   - `time_horizon`: "когда-нибудь", "в будущем", "однажды", "eventually", "someday"
   
   Каждый матч → `InjectToken` в SUTRA с `temporal_anchor_stable_id` в `reserved[0..4]`.

4. **Экспорт** из `crates/axiom-agent/src/perceptors/mod.rs`.

5. **`INVARIANTS.md`**: добавить диапазон `0x1000_0001..0x1FFF_FFFF` (temporal_anchor_id, бит 28).

6. **Тесты** (~8-10):  
   - `test_no_tokens_on_neutral_text`  
   - `test_detects_time_before` / `test_detects_time_after` / ...  
   - `test_stable_id_deterministic`  
   - `test_stable_id_range` (бит 28)  
   - `test_multiple_markers_in_one_text`

---

### PRIM-TD-05 — L0 уровень для абстракций

**Файл:** `config/anchors/abstractions/primitives.yaml`  
**Суть:** `abstraction_raw` (C0) семантически является L0 — сырой сенсорный сигнал. Сейчас отмечен только в `tags: ["layer:L0"]`, но поле `layer:` в YAML не выставлено → загружается как L1 по умолчанию.

**Шаги:**

1. Проверить как `AnchorLayer::L0` влияет на поведение при загрузке:
   - L0 якоря **исключены** из `match_text()` (только для VisionPerceptor/TemporalPerceptor)
   - L0 якоря попадают в `perceptual_anchors()`
   
   Вопрос: нужно ли `abstraction_raw` быть доступным для text-матчинга или нет?

2. Если L0 (исключить из text-матчинга — правильно, C0 это сырой сигнал, не языковой):
   ```yaml
   - id: "abstraction_raw"
     layer: L0          # ← добавить эту строку
     word: "сырое"
     ...
   ```

3. Проверить что `load_perceptual()` корректно подхватывает abstraction_raw через subsystem yaml, а не только через `perceptual/` директорию. Если нет — добавить в `perceptual/` или изменить загрузчик.

4. **Тест**: убедиться что `abstraction_raw` не матчится в `match_text()` после изменения.

**Размер:** ~10-20 строк включая тест. Но требует понимания загрузчика перед изменением.

---

### PRIM-TD-03 — ValueGravity и AbstractionGravity

**Файл:** `crates/axiom-space/src/lib.rs` + `crates/axiom-runtime/src/` (возможно новый модуль)  
**Суть:** специфичная гравитация для подсистем Values и Abstractions. Якоря `val_beneficial`/`val_harmful` создают притяжение/отталкивание; высокоабстрактные якоря (A5 theory, A6 constructor) тянут токены из нижних уровней.

**Шаги реализации:**

1. **Определить интерфейс** `SubsystemGravityRule`:
   ```
   SubsystemGravityRule {
       anchor_sutra_id: u32,   // sutra_id якоря (pull/push-центр)
       direction: f32,          // +1.0 = притяжение, -1.0 = отталкивание
       strength_factor: f32,    // множитель к базовой гравитации
       radius: Option<u32>,     // опционально: только в пределах радиуса
   }
   ```

2. **Собрать правила** из AnchorSet при `inject_anchor_tokens`:  
   - Values: `val_beneficial` → pull (direction=+1.0), `val_harmful` → push (direction=-1.0)  
   - Abstractions: A5/A6 якоря → pull токенов из нижних слоёв с умеренной силой

3. **Интегрировать в `apply_gravity_batch`** или новую функцию `apply_subsystem_gravity`:
   - На каждый тик (reconcile_interval или отдельный интервал): применять правила к токенам нужных доменов
   - Не нарушать инвариант: wall-clock запрещён, только event_id

4. **Хранение правил** в `AxiomEngine` (Vec<SubsystemGravityRule> — вычисляется при boot, не меняется).

5. **Тесты**:
   - `test_beneficial_pulls_nearby_token`
   - `test_harmful_repels_nearby_token`
   - `test_no_effect_beyond_radius`
   - `test_rules_loaded_from_anchor_set`

**Примечание:** это касается физики — требует аккуратного измерения влияния на hot path перед мержем. Запускать bench после реализации.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **COMP-01** — Vital Signs окно (Companion). См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **V9** — Active NeuralAdvisor (нейронные модели). После накопленной истории.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
