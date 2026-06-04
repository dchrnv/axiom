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

**1696 тестов, 0 failures.**  
Phases E–H завершены. V7 (A–E) завершён. Performance & Tooling Sprint завершён (2026-05-29).  
DilemmaDetector V2.0 завершён (2026-05-31). Cross-Modal Binding V1.0 завершён (2026-05-31).  
**Sensorium V1.0 завершён (2026-06-03).** **Waves V1.0 завершён (2026-06-03).**
**Cross-Modal Binding pipeline замкнут (2026-06-03):** vision_anchor_stable_id (bit 29, FNV-1a),
Vision Frames теперь кристаллизуются корректно; CMB-TD-03 закрыт (CrossModalBondProposed event);
3 integration теста; BroadcastSnapshot += cross_modal_candidates/bonds. CMB-TD-01 (revocation) → DEFERRED.

---

## Активные задачи

**Следующий этап — по roadmap:** Кросс V1.0 (Cross-Modal Binding полный пайплайн) или
доработка Sensorium V1.1 (адаптер Workstation — первый реальный consumer).
Также можно: DilemmaDetector V2.1 углубление, embeddings (AGENT-TD-01).

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
