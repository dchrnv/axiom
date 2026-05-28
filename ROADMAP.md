# Axiom Roadmap

**Версия:** 62.0  
**Дата:** 2026-05-27

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
                                                    ↑
                                               axiom-workstation
```

**1569 тестов, 0 failures.**  
Phases E–H завершены. NeuralAdvisor V3, OverDomainArbiter V3, DREAM Phase V1.1, CR V6 — в продакшне.  
Workstation V2, axiom-node, axiom-corpus — в продакшне.  
Primitive YAMLs (morality/abstractions/time/values), DilemmaStore V1.1, SubsystemDependencies loader — завершены.

---

## Активные задачи

### V7 — Generative Subsystems

Спецификация: `docs/architecture/ContextRecognizer_Roadmap_V6_V9.md` §2  
UGS-фундамент: `docs/architecture/universal_grounding/Universal_Grounding_Roadmap.md`

---

#### V7-A: Фундамент (блокирует всё остальное)

**V7-A1 — Composition bonds в FrameWeaver ✅**

`FrameCandidate.composed_of: Vec<u32>` — участники совпадающие с Frame-анкерами EXPERIENCE = родители.  
`FrameCompositionStore` — иерархия post-crystallization.  
`COMPOSITION_BOND` (0x0901) в axiom-shell — UCL-запись родителей.  
`detect_composed_of()`, `composition_level()` → FrameComposition.

**V7-A2 — L0/L1 структура в anchors ✅**

`AnchorLayer` enum (L0/L1) в axiom-config, `layer` поле в `Anchor` (default L1).  
`AnchorSet.perceptual: Vec<Anchor>` + `load_perceptual()` из `config/anchors/perceptual/`.  
22 L0 якоря: `visual_primitives.yaml` (8), `spatial_primitives.yaml` (8), `causal_primitives.yaml` (6).  
L0 исключены из `match_text()` — используются только VisionPerceptor через `perceptual_anchors()`.  
Тесты: 1576. (V7-E1 субсумирован.)

Детали: `Universal_Grounding_Roadmap.md` §0.2

---

#### V7-B: TransitionMatrix + FatigueStore

**V7-B1 — TransitionMatrix в ContextRecognizer ✅**

`[[f32; 16]; 16]` рядом с FatigueStore. `record(from, to)` при смене доминанты в on_tick.  
`decay(0.995)` на каждом тике. `probability_of()`, `most_likely_next()`. Unknown игнорируется.  
Тесты: 1587. (7 unit + 3 CR-интеграционных.)

Размер: ~1 KB. Выгода для будущих LM: фиксированный тензор.

**V7-B2 — FatigueStore → axiom-experience ✅**

`FatigueStore` + `SubsystemFatigue` перенесены в `axiom-experience/src/fatigue_store.rs`.  
axiom-runtime/subsystem_fatigue.rs → тонкий ре-экспорт (backward compat).  
Константы: `FATIGUE_DECAY_FACTOR`, `FATIGUE_DEBT_DECAY`, `FATIGUE_DEBT_RATE`, `FATIGUE_DREAM_RECOVERY`.

---

#### V7-C: Cascading и CompositeSubsystem (после B1)

**V7-C1 — Cascading upgrade ✅**

`ActivityDynamics.directed_cascade_score: f32` — directed cascade через TransitionMatrix.  
`ActivityTrace::directed_cascade_score(matrix, threshold=0.20)` — цепочка A→B→C где prob(A→B) ≥ threshold.  
`classify()`: предпочитает directed_cascade_score если > 0, иначе fallback на cascade_score (backward compat).  
Вычисляется в CR::on_tick после transition_matrix.record(). 5 новых тестов.

**V7-C2 — CompositeSubsystem full detection**

V6 даёт только `CompositeActivationSuspected`. V7 строит полный профиль: `composes_with` = bidirectional coupling в TransitionMatrix (A→B И B→A оба сильные). Предлагает chrnv.

---

#### V7-D: SubsystemLifecycle (после C)

**V7-D1 — SubsystemVersioning**

Версионирование yaml подсистем: `config/anchors/subsystems/mathematics/v1.0.yaml` + `current → v1.0.yaml`. Migration trace для Frame-профилей при обновлении.

Детали: `ContextRecognizer_Roadmap_V6_V9.md` §2.4

**V7-D2 — Splitting + Merging**

В DREAM Phase: обнаружение двух кластеров внутри подсистемы (→ Split) или перекрытия двух подсистем (→ Merge). Предлагается chrnv, не автоматически.

Детали: §2.5, §2.6

**V7-D3 — SubsystemDependencyGraph (Вариант C) ✅**

`config/subsystem_dependencies.yaml` — 7 подсистем, `builds_on` + `natural_tensions`.  
`SubsystemDependencies` loader в axiom-config: load_or_empty, is_natural_tension (симметрично), load_order() (топо-сорт, детект цикла).

**V7-D4 — Genome для emergent subsystems**

Секция `emergent_subsystems` в genome.yaml. GUARDIAN валидирует предложения по правилам.

Детали: §2.9

---

#### V7-E: Первый L0-слой (параллельно D)

**V7-E1 — L0 perceptual primitives ✅** *(субсумирован V7-A2)*

22 L0 якоря созданы в V7-A2: `visual_primitives.yaml` (8), `spatial_primitives.yaml` (8), `causal_primitives.yaml` (6). AnchorSet расширен. Архитектура L0/L1 завершена.

**V7-E2 — VisionPerceptor базовый**

Перевести VisionPerceptor из заглушки в рабочую реализацию: изображение → L0 visual примитивы → inject в SUTRA. Минимальный pipeline: edge detection → stroke primitives.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **AE-TD-08** — Full semantic connections at injection time. Приоритет повышается в V7 (связан с L0 bonds). См. DEFERRED.md.
- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **WS-V2-***, **COMP-01** — V2.0 идеи и Companion. См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами. После стабилизации SubsystemVersioning. См. §2.8.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **V9** — Active NeuralAdvisor (нейронные модели). После накопленной истории.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
