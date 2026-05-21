# Axiom Roadmap

**Версия:** 57.0  
**Дата:** 2026-05-17

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
                                                    ↑
                                               axiom-broadcasting
                                                    ↑
                                               axiom-workstation
```

**1344 тестов, 0 failures.**  
Phase C (C1–C5), Phase I (I1–I4, I6), Phase E (E1) завершены.  
Workstation V1.0, axiom-node, Axiom Sentinel V1.1 в продакшне.

---

## Активные задачи

### CR-V6 — ContextRecognizer V6: Meta-level Recognition

Спек: `docs/architecture/ContextRecognizer_Roadmap_V6_V9.md §1`

**Фаза 0 — SyntacticBridge** *(блокирует всё остальное)*
- Проблема (найдена в OBS-01): FrameWeaver кристаллизует Frame-анкеры из `0x08`-связей в домене MAYA,
  но роутинговый пайплайн (`route_token`) туда ничего **не пишет** — consolidated токен вычисляется,
  но в состояние домена не попадает. Итог: Frames = 0, CR profiles = 0, AE evaluations = 0.
- Решение: после `orchestrator::route_token` инжектировать в MAYA domain state синтаксические связи:
  `source_id` = hash стабильного ID консолидированного результата,
  `target_id` = sutra_id каждого из 8 ASHTI-результатов,
  `link_type = 0x0800 | (role << 4)` где role = 1..8.
- Место: `axiom-runtime/src/orchestrator.rs` или новый модуль `perceptual_bridge`.
- После этой фазы: FrameWeaver начнёт видеть паттерны уже после 3-го повторения текста.

**Фаза A — ActivityTrace + Dynamics Layer** *(фундамент)*
- `ActivityTrace` — три кольцевых буфера `(SubsystemId, event_id)`:
  short=16 (oscillation), mid=64 (convergence), long=256 (fatigue)
- `ActivityDynamics` — непрерывные метрики по всем трём окнам:
  `entropy_gradient` (smoothed, по третям), `oscillation_score`, `cascade_score`, `dominant_persistence`
- `classify(dynamics) -> Vec<ActivitySignature>` — лейблы выводятся поверх метрик, не напрямую
- Холодный старт: `Uncertain` до `MIN_WINDOW_FILL=16` (short window)
- Cascading: строго новая подсистема в каждом шаге цепочки (≥3 runs)
  *(known limitation: не отличает directed propagation — TransitionGraph в V7)*
- Приоритет классификации: Steady → Oscillating → Cascading → Converging → Diverging
- `TransitionDetector` остаётся (lightweight), переименовывается в `ActivityAnalyzer`

**Фаза B — SubsystemFatigue**
- `SubsystemFatigue { activation_load: f32, recovery_debt: f32 }` — два компонента
- Накопление по `event_id` дельте; `recovery_debt` lingers при смене primary (не обнуляется)
- `effective_weight = base_weight * (1.0 - 0.5 * min(1.0, activation_load / max))`
- DREAM wake: `fatigue *= 0.35` (partial recovery, не полный сброс)
- В V6 хранится в CR; перенос в `axiom-experience` — V7 (tech debt)

**Фаза C — MetaSubsystemId + MetaStore**
- `MetaSubsystemId(u16)` (0x1001–0x1007) в `axiom-experience`
- `MetaDetector` матчит `ActivityDynamics` + subsystem combo на `meta_primitives.yaml`
- `MetaStore: HashMap<MetaSubsystemId, MetaActivation>` в `axiom-experience`

**Фаза D — CompositeSubsystemDef + сигнал co-activation**
- 5 статических def: Calculus (Math+Time), Rhythm (Music+Time), Geometry (Math+Writing),
  Narrative (Writing+Time), Ethics (Values+Logic)
- При `Converging` с парой подсистем из def → `CompositeActivationSuspected { def, confidence }`
- Полная детекция composite (TransitionGraph, stable topology) — V7

**Тесты:** unit на каждую сигнатуру + cold start + вытеснение из буфера + смена сигнатуры на лету +
интеграционный (fatigue → DREAM → partial recovery → новый паттерн)

---

### I5 — OBS-01: живое наблюдение

**Проблема:** система ещё не запускалась с Phase C + полным якорным словарём на живых данных.

**Что сделать:** запустить `./run.sh`, подавать тексты через Conversation. Зафиксировать:

1. Какие Frame кристаллизуются? На каких текстах?
2. Какие SubsystemId определяет ContextRecognizer? Правильно ли?
3. Есть ли конфликты octant analytic vs synthetic в AxialEvaluator?
4. Появляются ли emergent-кандидаты в NeuralAdvisor?
5. Первый `NotifyEmergentCandidate` — при каких условиях?
6. Корректно ли пороги DepthThresholdEmergentDetector (8000/30/100)?

Результат: список наблюдений → tuning порогов → возможные errata.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **WS-V2-***, **COMP-01** — V2.0 идеи и Companion. См. DEFERRED.md.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
