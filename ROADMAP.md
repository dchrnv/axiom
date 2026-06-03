# Axiom Roadmap

**Версия:** 72.0  
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

**1683 тестов, 0 failures.**  
Phases E–H завершены. V7 (A–E) завершён. Performance & Tooling Sprint завершён (2026-05-29).  
DilemmaDetector V2.0 завершён (2026-05-31). Cross-Modal Binding V1.0 завершён (2026-05-31).  
**Sensorium V1.0 завершён (2026-06-03):** ModuleId=21, MAX_MODULES=22, SensoriumState (4 группы),
SensoriumView/Schedule/Registry/Expression, collect() в конце wake-тика, on_dream_wake().
Параллельно TickSnapshot (SEN-TD-01 → V2.0 полное поглощение в DEFERRED).

---

## Активные задачи

**Следующий этап: Волны V1.0 (Internal Drive)** — спека готова (`docs/spec/Waves_Internal_Drive_V1_0.md`).
Три источника импульса (дилеммы / SutraDepth / FrameWeaver candidates), internal_dominance_factor,
защиты от штормов. Зависит от готового материала (всё на месте).

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **COMP-01** — Vital Signs окно (Companion). См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами. После стабилизации SubsystemVersioning. См. §2.8.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **V9** — Active NeuralAdvisor (нейронные модели). После накопленной истории.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
