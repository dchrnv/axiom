# Axiom Roadmap

**Версия:** 63.0  
**Дата:** 2026-05-28

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

**1631 тестов, 0 failures.**  
Phases E–H завершены. NeuralAdvisor V3, OverDomainArbiter V3, DREAM Phase V1.1, CR V6 — в продакшне.  
Workstation V2, axiom-node, axiom-corpus — в продакшне.  
V7 (A–E) завершён: TransitionMatrix, FatigueStore→experience, directed cascade, CompositeSubsystem, SubsystemVersionStore, SplitMergeDetector, SubsystemDependencies, EmergentSubsystemRules (GUARDIAN), L0VisionPerceptor.

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

**V7-C2 — CompositeSubsystem full detection ✅**

`CompositeSubsystemProfile` + `BidirectionalCoupling` — полный профиль с directed coupling.  
`detect_composite_profiles(recent, sigs, matrix, threshold=0.15)` — coverage + bidirectional pairs.  
`composite_profiles` в CR, `composite_profiles()` accessor. 6 новых тестов.  
V6 `composite_suspects` сохранён (backward compat).

---

#### V7-D: SubsystemLifecycle (после C)

**V7-D1 — SubsystemVersioning ✅**

`version` поле в `FlatAnchorFile` (default "1.0"). `AnchorSet.subsystem_versions: HashMap<String, String>`.  
`SubsystemVersionStore` в axiom-runtime: `init()` / `check_migration()` → stale subsystems / `drain_stale()`.  
`ContextRecognizer.version_store`: инициализируется в `from_anchor_set`, `update_subsystem_versions()` при hot-reload.  
8 unit-тестов SubsystemVersionStore.

**V7-D2 — Splitting + Merging ✅**

`SplitMergeDetector::detect(fatigue, matrix)` → `SplitMergeCandidateStore`.  
Split-сигнал: `activation_load ≥ 0.6·MAX` + Shannon-энтропия исходящих переходов ≥ 1.5.  
Merge-сигнал: prob(A→B) ≥ 0.25 AND prob(B→A) ≥ 0.25 (bidirectional coupling).  
`split_merge_candidates()` accessor в CR. Предлагается chrnv, не применяется авто.  
9 unit-тестов. Вызывается на каждом тике CR после fatigue.update().

**V7-D3 — SubsystemDependencyGraph (Вариант C) ✅**

`config/subsystem_dependencies.yaml` — 7 подсистем, `builds_on` + `natural_tensions`.  
`SubsystemDependencies` loader в axiom-config: load_or_empty, is_natural_tension (симметрично), load_order() (топо-сорт, детект цикла).

**V7-D4 — Genome для emergent subsystems ✅**

`EmergentSubsystemRules` в axiom-genome: `min_primitives`, `min_evidence_strength`, `require_review`, `max_active_candidates`.
`emergent_subsystems` секция в genome.yaml. `approve_with_rules()` в SubsystemCandidateStore — GUARDIAN проверяет evidence + лимит активных.
`discover_subsystem_candidates` использует `min_primitives` из genome. 6 новых тестов.

Детали: §2.9

---

#### V7-E: Первый L0-слой (параллельно D)

**V7-E1 — L0 perceptual primitives ✅** *(субсумирован V7-A2)*

22 L0 якоря созданы в V7-A2: `visual_primitives.yaml` (8), `spatial_primitives.yaml` (8), `causal_primitives.yaml` (6). AnchorSet расширен. Архитектура L0/L1 завершена.

**V7-E2 — VisionPerceptor базовый ✅**

`L0VisionPerceptor` в `axiom-agent/src/perceptors/vision_l0.rs`.
Pipeline: RGBA8 → grayscale → Sobel edge detection → stroke classification (horizontal/vertical/diagonal) → InjectToken в SUTRA(100).
`EdgeAnalysis` struct: edge_density + fraction per direction. Порог: 0.02.
10 unit-тестов: flat image no-ops, vertical/horizontal edge dominance, anchor position propagation, full integration.

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
