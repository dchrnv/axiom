# AXIOM — Neural Integration V1.0

**Статус:** Спецификация
**Версия:** 1.0
**Дата:** 2026-06-05
**Категория:** Активная фаза NeuralAdvisor — обучаемые модели внутри AXIOM
**Crate:** `axiom-runtime` (over_domain/neural_advisor) + `axiom-experience` (training data) + новый `axiom-neural` (модели)
**Опирается на:** `NeuralAdvisor V3` (код), `Sensorium V1.0`, `DreamPhase`, `OverDomainArbiter V3`, `GENOME`, `GUARDIAN`, `ContextRecognizer Roadmap V6-V9` (V9 = эта фаза)
**Источник идеи:** Axiom_Bridge_V8 (адаптирован к реальной модели — см. §1)

> **Стиль:** комментарии вместо примеров кода, чтобы chrnv восстанавливал картину.

---

## 1. Что взято из Bridge V8 и что отброшено

Идея Bridge V8 — «внедрить нейронку, ядро её не ждёт». Ядро идеи верное. Но Bridge V8 написан под **другую физическую модель** (float-координаты [0,1]³, charge, LLM перераспределяет массы, внешний мост SystemJournal). Эти решения **конфликтуют с реальным AXIOM** и заменены на уже существующие.

### Отброшено (конфликт с моделью)

| Bridge V8 | Почему отброшено | Чем заменено в AXIOM |
|-----------|------------------|----------------------|
| `TokenState{mass,temp,charge}` f32 | Token 64B HARD, u8, нет charge | существующий Token |
| координаты `[0,1]³` float | `[i16;3]` integer [0..32767] | существующая геометрия SPACE |
| LLM «перераспределяет массы» (прямая правка) | нарушает Advisory-Only | NeuralAdvisor advisory + GUARDIAN/UCL |
| внешний LLM в цикле коррекции | AXIOM самодостаточен | локальные малые модели |
| `charge Q` притяжение/отталкивание | нет в модели | valence + SubsystemGravity (PRIM-TD-03 ✓) |
| `SystemJournal` (новый мост) | дублирует существующее | BroadcastHandle + EventBus + CausalFrontier + Sensorium |
| ParserAgent назначает коорд. через LLM | есть anchor-matching | TextPerceptor 2-path ✓ |
| октанты `energy_density` = «единств. сенсор» | примитивнее имеющегося | AxialEvaluator + EmergentDetector + SubsystemCandidate ✓ |
| закон сохранения массы, глоб. охлаждение | другая термодинамика | min_intensity>0, fade ✓ |

### Сохранено (ценное, переложено на наши механизмы)

| Принцип Bridge V8 | Как реализуется в AXIOM |
|-------------------|--------------------------|
| Ядро не ждёт нейронку | NeuralAdvisor advisory, не на горячем пути (уже) |
| Защита от зависания («LLM тупит — физика тикает») | детерминистический fallback всегда доступен |
| Quota (ограничение частоты вмешательства) | tick-интервалы + confidence + TrustConfig |
| Telescope (дамп для контроля деградации) | Sensorium срез + DivergenceLog |
| «Ты строишь песочницу, нейронка — инструмент» | НО у нас нейронка = **советник**, не правщик масс |

**Вывод:** «внедрить нейронку» в AXIOM = **активировать NeuralAdvisor реальными обученными моделями**, вход — Sensorium, тренировка — DREAM, контроль — GUARDIAN/Arbiter. Не новая архитектура — наполнение существующего каркаса.

---

## 2. Что уже готово (каркас)

```
NeuralAdvisor V3 (tick=11, ModuleId=19) — РАБОТАЕТ, но rule-based:
  5 советников: ReactivationDepth, DepthHistoryBias, PatternLearningResolver,
                AnchorVoting, DepthThresholdEmergent
  DivergenceLog (ring 256) — расхождение совета и факта
  NeuralAdvisorConfig из genome.yaml
  Advisory-Only — никогда не управляет напрямую

OverDomainArbiter V3 (tick=13) — координатор:
  TrustConfig: Ignore / AutoApply / RequireConfirmation × min_confidence
  online learning rate=0.05, CognitiveProfile octant_weights[8]
  PendingQueue → Workstation confirm/reject

Sensorium V1.0 — полный срез (ВХОД для нейронки, §4)
DreamPhase — место тренировки (offline)
GUARDIAN + UCL — единственный путь мутации
```

Каркас полный. Нужно: заменить rule-based на обучаемые модели + конвейер тренировки + калибровку. Это и есть Neural Integration.

---

## 3. Природа моделей

```
НЕ LLM. НЕ облако. НЕ GPT.
Малые специализированные локальные модели, тренируемые на истории САМОГО AXIOM.

Размеры (ориентир):
  ReactivationDepth advisor   ~50K параметров  (MLP над признаками)
  OctantCorrection            ~200K            (может включать embeddings)
  CorpusCallosumResolver      ~30K             (classifier)
  SubsystemAttribution        ~500K            (контекстный classifier)
  EmergentPattern             ~100K
  Σ ~1M параметров на всё. Inference < 100 µs на советника.

Crate: новый axiom-neural (модели + inference). Чистый Rust (candle/burn/ndarray —
  выбор за реализацией; без внешних сервисов). Веса в файлах models/*.bin.
Weak hardware: inference только в момент тика советника (t%11), не каждый тик.
```

Почему локально и малые: AXIOM самодостаточен (принцип). Большие модели не нужны — задача узкая (предсказать глубину, поправить октант, разрешить конфликт), специализированная модель на своих данных бьёт универсальную.

---

## 4. Вход — Sensorium как зеркало

```
Нейронка читает состояние системы ЧЕРЕЗ Sensorium (не лезет в store напрямую).
Это ровно роль, под которую Sensorium проектировался (V3.0 «зеркало для нейронки»).

  Sensorium → SensoriumState (полный срез) → вход модели

Каждый советник берёт нужный СРЕЗ Sensorium-состояния:
  ReactivationDepth   ← depth_per_octant, fatigue, recent_crystals
  OctantCorrection    ← octant_profile, axial_scores, corpus_callosum
  CorpusCallosumResolver ← corpus_callosum, active_subsystems
  SubsystemAttribution   ← active_subsystems, fractal_levels, depth_hotspots
  EmergentPattern     ← emergent_candidates, composite_suspects, cross_modal

Уровень доступа: нейронка = доверенный потребитель, CODEX даёт уровень 2
  (полный срез). Реестр потребителей: NeuralAdvisor читает уровень 2 на t%11.
Sensorium отдаёт РОДНОЙ формат — нейронка ест почти без перевода (особый адаптер).
```

Это закрывает «нет геометрии/входа» из критики Bridge V8: вход — структурированный полный срез Sensorium, не сырые токены.

---

## 5. Выход — Advisory, никогда не управление

```
Каждый советник выдаёт AdvisorOutput { value, confidence, computation_time_ns }.
Confidence ПОСЛЕ калибровки (§7).

Выход НЕ применяется напрямую. Путь:
  модель → AdvisorOutput → OverDomainArbiter (TrustConfig) → решение

TrustConfig (уже есть) определяет per-advisor:
  Ignore              — совет логируется, не применяется
  RequireConfirmation — в PendingQueue → chrnv подтверждает в Workstation
  AutoApply (× min_confidence) — применяется если confidence > порог

Применение — только через UCL → GUARDIAN. Нейронка физически не может
  обойти инварианты (Token 64B, min_intensity>0, SUTRA-only-in-DREAM).

ГРАДИЕНТ ДОВЕРИЯ (gradual promotion, из roadmap V9):
  старт: все советники Ignore или RequireConfirmation
  по мере накопления точности → AutoApply с растущим min_confidence
  промоция — только через genome update (chrnv решает), не авто
  макс влияние — как у advisory сейчас (не 100%, fallback всегда жив)
```

Это разрешает главный конфликт с Bridge V8: там LLM прямо правит массы. У нас нейронка **советует**, Arbiter+GUARDIAN решают, chrnv контролирует промоцию. Никакой прямой власти над состоянием.

---

## 6. Тренировка — в DREAM, на своей истории

```
ТОЛЬКО offline или в DREAM Phase. НИКОГДА на горячем пути. НИКОГДА online в WAKE.

Источник данных тренировки — DivergenceLog + история:
  DivergenceLog (ring 256, уже есть) пишет пары (совет, факт):
    что советник предсказал ← → что реально случилось
  Это готовый размеченный датасет — система сама себя размечает.

Конвейер (этап AdvisorTraining в DREAM, опциональный):
  if training_enabled:
    for advisor in advisors:
      batch = collect_from_divergence_log(advisor, since_last_training)
      if batch.len() > MIN_BATCH:
        advisor.model.train_step(batch, max_steps=100)   # микро-обучение
        acc = advisor.validate(holdout)
        if acc > prev_acc: advisor.save_checkpoint()
        else:             advisor.rollback_checkpoint()   # не ухудшать

Защиты:
  - rehearsal buffer против catastrophic forgetting
  - rollback если точность упала
  - тренировка не блокирует DREAM-консолидацию (отдельный под-этап, бюджет времени)
  - distribution shift: если AXIOM долго в одной области (Math) — модель
    специализируется; при смене области точность временно падает → DivergenceLog
    это покажет → дообучение выправит
```

Это закрывает «как нейронка учится» — на собственной истории расхождений, в своё время (сон), не мешая работе.

---

## 7. Калибровка confidence (критично)

```
Модель может быть уверена в неверном. Без калибровки confidence бесполезен
для TrustConfig (AutoApply по порогу даст мусор).

ConfidenceCalibrator на каждого советника:
  учится отдельно на парах (raw_confidence, was_correct) из DivergenceLog
  отображает raw → истинную вероятность правоты
  пример: raw 0.9 но реально прав в 60% → calibrated 0.6

TrustConfig использует ТОЛЬКО калиброванный confidence.
Без калибровки advisor остаётся в Ignore/RequireConfirmation (не AutoApply).
```

---

## 8. Дистилляция rule-based → neural (плавный переход)

```
Сейчас советники rule-based (PatternLearningResolver, DepthThresholdEmergent...).
Не выбрасываем — используем как УЧИТЕЛЯ.

Этап 1: teacher = rule-based, student = neural.
  Student учится воспроизводить выход teacher (готовый источник разметки).
Этап 2: student воспроизводит teacher на 95%+.
  Включается дообучение на DivergenceLog (где teacher ошибался — student правит).
Этап 3: student > teacher на сложных случаях.
  Промоция Ignore→RequireConfirmation→AutoApply через genome (chrnv).

Плавно: поведение не прыгает. Rule-based fallback жив всегда — если модель
  деградирует или не уверена, Arbiter берёт детерминистический выход.
```

Это снимает риск «включили нейронку — система поехала». Переход постепенный, откатываемый, с живым fallback.

---

## 9. Защита от зависания (из Bridge V8, переложено)

```
Принцип Bridge V8 «LLM тупит — физика тикает» — сохранён by design:

  - Inference советника на t%11, с таймаутом. Превысил — пропуск, fallback.
  - Ядро (AshtiCore::tick) НЕ зависит от нейронки. Tick идёт всегда.
  - Нейронка advisory — её отсутствие = система работает на rule-based/детерминизме.
  - DREAM-тренировка в бюджете времени — не затягивает сон.
  - Деградация Sensorium под нагрузкой (§11 Sensorium) роняет уровень →
    нейронка временно получает меньше данных, но система жива.

Нейронка — улучшение, не зависимость. Выдерни её — AXIOM работает.
```

---

## 10. Контроль деградации (Telescope из Bridge V8 → Sensorium + DivergenceLog)

```
Bridge V8 предлагал Telescope — JSON-дамп раз в 1000 тиков для анализа
«смысловой деградации». У нас это уже есть лучше:

  - Sensorium срез (уровень 3 + память) = полный дамп состояния по запросу
  - DivergenceLog = метрика качества советов во времени
  - Workstation Advisor lab (Sensorium V3 §): accuracy, calibration, divergence
  - chrnv видит: растёт ли точность, не поехала ли модель

Сигнал деградации: DivergenceLog показывает рост расхождений → модель поехала →
  rollback к checkpoint или понижение TrustConfig (auto→confirm).
```

---

## 11. Инварианты

| Правило | Значение |
|---------|----------|
| Природа моделей | малые, локальные, специализированные; НЕ LLM, НЕ облако |
| Самодостаточность | нет внешних сервисов/сети для inference |
| Вход | через Sensorium (уровень 2), не прямой доступ к store |
| Выход | Advisory; применение через Arbiter → UCL → GUARDIAN |
| Прямое управление | ЗАПРЕЩЕНО (как весь NeuralAdvisor) |
| Тренировка | только DREAM/offline; никогда hot path; никогда online WAKE |
| Источник данных | DivergenceLog (самразметка) + история |
| Confidence | только калиброванный идёт в TrustConfig |
| Промоция доверия | Ignore→Confirm→AutoApply только через genome (chrnv) |
| Fallback | rule-based/детерминизм всегда жив; нейронка — улучшение, не зависимость |
| Backprop через токены | ЗАПРЕЩЁН (модели на агрегатах/срезах) |
| Token/Connection 64B | не трогаются; модели не меняют структуру ядра |

---

## 12. Что в коде

```
crates/axiom-neural/            — НОВЫЙ
  models/                       — 5 моделей (по советнику), inference
  calibration.rs                — ConfidenceCalibrator
  training.rs                   — train_step, checkpoint, rollback (вызов из DREAM)
  distill.rs                    — teacher(rule)→student(neural)

crates/axiom-runtime/over_domain/neural_advisor/
  + интеграция: rule-based ИЛИ neural per advisor (флаг в NeuralAdvisorConfig)
  + чтение Sensorium-среза как входа
  + калиброванный confidence в AdvisorOutput

crates/axiom-experience/
  + training data из DivergenceLog (расширить ring или persist)

DREAM Phase:
  + под-этап AdvisorTraining (опциональный, в бюджете времени)

genome.yaml:
  + neural_advisor: { per-advisor mode: rule|neural|distill,
                      trust: ignore|confirm|autoapply, min_confidence }
  + neural_models: { пути к весам, provenance, rollback target }

models/*.bin                    — веса (вне репо или git-lfs)
```

---

## 13. Порядок реализации

```
ФАЗА 0 — каркас inference (без обучения)
  axiom-neural, загрузка модели-заглушки, inference-путь, Sensorium-вход.
  Один советник (напр. ReactivationDepth) как пилот.
  Критерий: модель читает срез, выдаёт AdvisorOutput, идёт в Arbiter (Ignore).

ФАЗА 1 — дистилляция (teacher→student)
  Student учится воспроизводить rule-based teacher. Калибровка confidence.
  Критерий: student воспроизводит teacher 95%+, confidence калиброван.

ФАЗА 2 — обучение на DivergenceLog (в DREAM)
  AdvisorTraining под-этап, rehearsal, rollback. На пилотном советнике.
  Критерий: точность растёт на истории, rollback работает, DREAM не затянут.

ФАЗА 3 — промоция доверия (chrnv)
  Ignore → RequireConfirmation → AutoApply через genome, по факту точности.
  Критерий: пилот в AutoApply, fallback жив, деградация ловится DivergenceLog.

ФАЗА 4 — остальные 4 советника
  По отработанному на пилоте паттерну.

Параллельно/после: Sensorium V3.0 (Advisor lab в Workstation — видеть accuracy).
```

---

## 14. Связь с планом

```
Это roadmap-пункт «внутренняя нейронка» / NeuralAdvisor V9 / Sensorium V3.0 —
самый дальний горизонт, конкретизированный.

Зависит от:
  - Sensorium V1.0 (вход) — спека есть, реализуется в фазе 1 общего плана
  - NeuralAdvisor V3 (каркас) — в коде ✓
  - DivergenceLog (данные) — в коде ✓
  - DREAM (тренировка) — в коде ✓

Логично делать ПОСЛЕ Сенсориума и накопления истории (DivergenceLog должен
  набрать данные на реальной работе — месяцы). Раньше — нечем обучать.

НЕ срочно. Это финал, не следующий шаг. Тройка (Кросс/Волны/Сенсориум) и
  Vision pipeline — раньше. Нейронка приходит когда есть на чём учиться
  и что выражать.
```

---

## 15. Резюме

```
«Внедрить нейронку» в AXIOM — это НЕ строить новую песочницу с charge и
LLM-правкой масс (как в Bridge V8). Это НАПОЛНИТЬ существующий каркас:

  NeuralAdvisor (5 советников) — заменить rule-based на обучаемые модели
  Sensorium — вход (полный срез = зеркало для нейронки)
  DivergenceLog — данные (система сама себя размечает)
  DREAM — тренировка (offline, своё время)
  GUARDIAN/UCL/Arbiter — выход остаётся advisory, под контролем
  genome — промоция доверия только через chrnv

Малые локальные модели, не LLM, не облако. Advisory, не власть. Fallback всегда
жив. Нейронка — улучшение поверх работающей системы, не её сердце.
Выдерни — AXIOM продолжит думать.
```

---

## История

- **V1.0** (2026-06-05): адаптация идеи Axiom_Bridge_V8 к реальной модели AXIOM. Отброшены конфликтующие решения (float-координаты, charge, LLM-правка масс, SystemJournal, внешний LLM) в пользу существующих (Token 64B, NeuralAdvisor advisory, Sensorium, DREAM, GUARDIAN/UCL). Сохранены ценные принципы Bridge V8 (асинхронность, защита от зависания, quota, контроль деградации) на наших механизмах. Суть: внедрение нейронки = активная фаза NeuralAdvisor (обучаемые модели вместо rule-based), вход через Sensorium, тренировка на DivergenceLog в DREAM, выход advisory под GUARDIAN, промоция доверия через genome. Малые локальные модели, самодостаточность, fallback. Roadmap-пункт «внутренняя нейронка» конкретизирован.
```
