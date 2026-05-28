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

*(V7 завершён полностью — см. STATUS.md)*

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
