# Axiom — Отложенные задачи

**Версия:** 85.0
**Обновлён:** 2026-06-13

---

## axiom-runtime

### TEST-TD-01 — test_process_and_observe_slow_path_initially сломан

**Где:** `crates/axiom-runtime/tests/process_and_observe_tests.rs`

Тест падал до любых изменений текущей сессии — pre-existing регрессия.

**Когда:** при ближайшем свободном окне.

---


## Семантическая гравитация к якорям (Anchor Gravity)

### GRAVITY-TD-01 — Токены притягиваются к (0,0,0), а должны к позиции семантического якоря

**Проблема:**  
Сейчас `apply_gravity_batch` тянет все токены к `(0, 0, 0)` — константы `ANCHOR_X/Y/Z = 0`
в `crates/axiom-space/src/lib.rs:28-31`. Это "базовая" гравитация домена.

Но в нескольких спеках прямо написано, что ценности и абстракции должны работать как
**гравитационные аттракторы** — то есть токены тянутся не к центру домена, а к позиции
конкретного семантического якоря:

- **Values_V1_0.md §1:** "Ценности — это гравитационные аттракторы оценки. Токен 'помочь
  раненому' не содержит val_beneficial как компонент — он **гравитационно притягивается** к нему."
- **Values_V1_0.md §6:** "Может потребоваться специальный pass в apply_gravity_batch для
  ценностных якорей с бо́льшим radius_of_influence."
- **Abstractions_V1_0.md §2:** "Каждый Frame в SUTRA может гравитировать к одному из
  абстракционных якорей в зависимости от своей глубины."
- **Abstractions_V1_0.md §7:** "При apply_gravity_batch — дополнительный pass для
  абстракционных якорей: Frame с composition_level=C3 притягивается к abstraction_category."

**Текущее состояние:**  
PRIM-TD-03 ✅ реализован (2026-06-04): `crates/axiom-runtime/src/subsystem_gravity.rs`.
val_beneficial pull(0.20) / val_harmful push(0.20) / abstraction_theory+constructor pull(0.08, radius=8000).
Формула: нормализованный вектор × BASE_FORCE(16) × factor_256/256. Interval=500 тиков.

Но за PRIM-TD-03 стоит более широкий принцип, который пока не реализован:
любой STATE_LOCKED якорь с достаточной массой является потенциальным аттрактором.

**Якоря STATE_LOCKED → позиции фиксированы навечно** — кешируются в SubsystemGravityRule при boot.

**Ключевые позиции (сверено с yaml):**
```
val_beneficial       [8000, 12000, 13000]   — притяжение
val_harmful          [3000,  1000, 11000]   — отталкивание
abstraction_theory   [13000, 10000, 14000]  — C5/A5
abstraction_constructor [14000, 12000, 15000] — C5+/A6
```

**Когда:** после накопления реальных данных о том как якоря взаимодействуют.

---

## FrameWeaver

### FW-TD-02 — Per-pair co-activation не отслеживается

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

Текущая структура: `reactivation_counts: HashMap<u32, u32>` — глобальный счётчик на Frame-анкер. Нет информации о том, какие Frame-ы активировались совместно.

Нужно для: CausalWeaver (причинные связи между Frame), AnalogyWeaver (похожие паттерны).

**Когда:** при проектировании CausalWeaver или AnalogyWeaver. Структуру не выбирать заранее — форма данных зависит от первого потребителя.

---


## Долгосрочный backlog (V8–V9)

### V8 — Axiogenesis through Dilemmas

Аксиогенетические конфликты → новые ценностные якоря в Values. Требует реальной истории дилемм.

**Зависимости:** SubsystemLifecycle ✅, реальная история дилемм (6+ месяцев), Ethics composite ✅.

**Когда:** после накопления живых данных.

---

### V9 — Active NeuralAdvisor (нейронные модели)

Заменить все 5 advisory-слотов обученными моделями (~1M параметров). Тренировка на `AdvisoryHistory` + `DivergenceLog`.

**Зависимости:** G1 ✅, G2 ✅, тысячи resolved advisories.

**Когда:** после накопленной истории.

---

## AxialEvaluator

### AE-TD-06 — NARRATIVE_WINDOW_SIZE: правильный ли размер окна?

**Где:** `crates/axiom-runtime/src/over_domain/axial_evaluator/narrative.rs:13`

`NARRATIVE_WINDOW_SIZE = 8` — не калибровалось. Слишком малое → ложные сдвиги; слишком большое → медленная реакция.

**Когда:** после накопления реального нарратива (тысячи инъекций).

---

## Для проекта Companion

### COMP-01 — Vital Signs окно

Ambient display — постоянное отображение состояния системы на физическом светильнике-банере. Вертикальная композиция, Raspberry Pi + HDMI.

**Когда:** первое окно Companion.

---

## axiom-agent

### AGENT-TD-01 — TextPerceptor: embeddings (Path A реализован)

**Где:** `crates/axiom-agent/src/perceptors/text.rs`

Path A (100% accuracy) ✅. Path B: заменить lookup-таблицы на векторные embeddings (ONNX/candle).

**Когда:** после накопления живых данных на OBS-02+ прогонах.

---

## Shell

### Shell-TD-02 — resonance_search shell bonus

**Где:** `axiom-arbiter` → `resonance_search`

Shell-бонус требует `ShellRegistry` в Experience/Arbiter. Сейчас живёт только в engine.

**Когда:** Shell Metrics V2+.

---

## NeuralAdvisor — Emergent

### EMERGENT-TD-01 — Калибровка порогов под неоднородный корпус

**Где:** `crates/axiom-runtime/src/over_domain/neural_advisor/implementations/emergent.rs`

Пороги откалиброваны по синтетическому корпусу. При реальном тексте потребуется пересмотр.

**Когда:** после production-прогона (10k+ инъекций).

---

### NEURAL-TD-01 — Обучить ReactivationDepthModel (Python/torch → .bin)

**Где:** `crates/axiom-neural/src/reactivation_depth.rs` + тренировочный скрипт (вне репо)

OBS пишет `training_data.jsonl` каждые 200 тиков: FFT-признаки `features[1539]` + teacher reactivation_weights[8].
Тренировка оффлайн (Python/torch): дистилляция rule-based teacher → student, сохранить в `models/reactivation_depth.bin`.
Загрузка: `ReactivationDepthModel::load_from_bin()` — готова.

**Когда:** после накопления ≥10K примеров — то есть после первого production OBS-прогона с реальным текстом (CORPUS-TD-01).

---

### NEURAL-TD-03 — ConfidenceCalibrator не подключён

**Где:** `crates/axiom-runtime/src/over_domain/neural_advisor/neural_depth.rs:128`

`AdvisorOutput.calibrated_confidence = raw_confidence` — `ConfidenceCalibrator` реализован в axiom-neural, но не вызывается.
Без калибровки нельзя переводить советника в AutoApply (confidence не отражает реальную точность).

**Когда:** после первого обученного `.bin` (после NEURAL-TD-01).

---

## Observability

### OBS-TD-03 — delta-energy per-text нерабочий (метод оставлен)

**Где:** `crates/axiom-runtime/src/engine.rs` → `snapshot_subsystem_energies`

Методы оставлены намеренно — пригодятся при embeddings (AGENT-TD-01), когда позиции будут семантически выровнены.

**Когда:** AGENT-TD-01 (embeddings).

---

### OBS-MON-01 — Мониторинг роста traces в production

Traces плато на синтетическом корпусе — нормально. Отслеживать: traces_count после 10k/50k/200k тиков, reflex_hit%.

**Когда:** первый production-прогон с реальным текстом.

---

### OBS-MON-02 — Мониторинг tension и activity dynamics в production

tension=0 на синтетическом корпусе — нормально. На реальном тексте: tension traces, "Converging"/"Oscillating".

**Когда:** первый production-прогон с реальным текстом.

---


## Корпус — Реальные тексты

### CORPUS-TD-01 — Real text corpus для OBS

**Где:** `crates/axiom-corpus/` + `config/obs/`

Синтетический `corpus_showcase.yaml` (18 текстов) даёт 100% accuracy, потому что написан под якоря.
Нужны реальные тексты по подсистемам: диалоги, книги, документация, форумы — 50–200 текстов на подсистему.
Формат: новый YAML-дескриптор + директория с `.txt` файлами. axiom-corpus загружает как `CorpusEntry`.

Польза:
- Реальная точность TextPerceptor (на диком тексте неизвестна)
- Накопление `training_data.jsonl` для NEURAL-TD-01 (нужно ≥10K примеров)
- Первый production OBS-прогон → OBS-MON-01/02 (tension, traces динамика)

**Когда:** до первого production OBS-прогона.

---

## Injection Tool — Семена для SUTRA

### INJECT-TD-01 — Seed Library + Injection Tool

**Идея:** инструмент для создания и инъекции структурированных «семян» в SUTRA в порядке абстракции C0→C5+.

**Seed Library:** YAML-файлы с упорядоченными наборами токенов по типу данных и уровню абстракции:
- По типу: writing, math, temporal, moral, logic, abstractions
- C0 — базовые символы/слова, C1 — простые паттерны, ..., C5+ — сложные концепты
- Каждый токен: подсистема, позиция (якорь), shell-профиль, порядок инъекции

**Injection Tool** — самостоятельный инструмент, не завязан на Workstation:
- CLI: `axiom-inject --seed-file seeds/math_c0.yaml --mode sequential`
- WS API: команды инъекции через WebSocket (доступен из любого клиента, в т.ч. скриптов)
- OBS интеграция: `inject_seeds_before_run: seeds/writing_c1.yaml` в corpus.yaml

**Когда:** после CORPUS-TD-01 (реальные тексты как ориентир для наполнения seed library).

---

### INJECT-TD-02 — Сложные примитивы через формулы

**Идея:** часть seed library (C3–C5+) не задаётся словами — нужна генерация из формул.

Примеры:
- Математические структуры: LaTeX → набор символов/операторов/отношений как токены
- Логические формулы: пропозиции, кванторы, импликации → логические примитивы
- Темпоральные паттерны: формульное описание ритмов (периодичность, фазы, интервалы)

Реализация: Rust-генератор (pure, без LaTeX-рендеринга) → `Vec<UclCommand>` для инъекции.
Может быть частью `axiom-inject` или отдельным `axiom-formula` крейтом.

**Когда:** после INJECT-TD-01.

---

## Cross-Modal Binding

### CMB-TD-01 — Stress-driven revocation

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/cross_modal/`

Когда модальности расходятся — `current_stress` на CROSS_MODAL_BOND должен расти → INHIBITED → предложение к отзыву.

Инфраструктура: `current_stress` на Connection ✅, `cross_modal.allow_revocation` в genome ✅.

**Когда:** после накопления cross-modal bonds в реальной работе (требует Vision pipeline + реальные данные).

---

## axiom-seed

### SEED-TD-01 — Boot-инъекция кристалла: TextPerceptor 2-path

**Где:** `crates/axiom-agent/src/perceptors/`, `crates/axiom-config/src/anchor.rs`

Кристальные якоря (seeds/crystal_c0.yaml) нельзя напрямую класть в config/anchors/:
кратчайшие Exact-матчи на "—", ".", "," дают crystal → "writing" или crystal → "" в
`dominant_subsystem_of()` и создают регрессии в subsystem detection.

Нужна TextPerceptor 2-path архитектура:
- Path 1 (текущая): match_text → позиция + subsystem (семантические якоря, без crystal)
- Path 2 (новая): crystal_match_text → fallback позиция ТОЛЬКО когда path 1 не даёт матча
  (приоритет: словарный > графемный, per spec §5)
- Crystal якоря НЕ участвуют в subsystem detection — только в position fallback

Реализация: отдельный `CrystalAnchorSet` или флаг `skip_subsystem: bool` на Anchor.
Либо: отдельный файл не в subsystems[], а в `AnchorSet::crystal: Vec<Anchor>`.

**Когда:** Foundation Фаза 1 C6 (OBS-прогон кристалла). После SEED-TD-01 — boot-инъекция.

## axiom-experience (Store Optimization)

### STORE-TD-01 — Custom MetaSubsystemId (0x1100+)

**Где:** `crates/axiom-experience/src/meta_store.rs`

После STORE-OPT-01 MetaStore будет `[Option<MetaActivation>; 7]` (индекс = id.0 - 0x1001).
Это покрывает только стандартные мета-режимы 0x1001–0x1007.

Комментарий в коде "Пользовательские мета-режимы: 0x1100+" — архитектурный задел,
не реализованный и не запланированный нигде кроме этого комментария.

Когда/если понадобятся custom ID: добавить `custom: HashMap<MetaSubsystemId, MetaActivation>`
рядом с `standard: [Option<MetaActivation>; 7]`. activate/get/iter объединить оба источника.

**Когда:** только если появится реальный use-case для пользовательских мета-режимов.
