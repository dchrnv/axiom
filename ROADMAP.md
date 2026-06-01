# Axiom Roadmap

**Версия:** 71.0  
**Дата:** 2026-06-01

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

**1663 тестов, 0 failures.**  
Phases E–H завершены. V7 (A–E) завершён. Performance & Tooling Sprint завершён (2026-05-29):  
token lifecycle (STATE_SLEEPING), parallel domain ticks, parallel OBS shards, streaming JSONL, Lab UI panel.  
DilemmaDetector V2.0 завершён (2026-05-31): Сигнал A, кристаллизация Frame в EXPERIENCE.  
OBS §9 верификация: 8/8 дилемм, DilemmaDetector рабочий.  
Фикс: SubsystemDependencies YAML wrapper, TextPerceptor subsystem-позиция, engine.rs record_injection_signal.  
Cross-Modal Binding V1.0 завершён (2026-05-31): ModalityStore, CrossModalDetector, CROSS_MODAL_BOND=0x0A01; 1623 тестов.

---

## Активные задачи

### P1 — DilemmaDetector Signal C (Corpus Callosum)
**Файл:** `dilemma/detector.rs`, `axial_bridge.rs`  
**Что:** Signal C из спеки V2.1 — читать `AxialConflict` из AxialEvaluator: analytic octant ≠ synthetic octant → дилемма уровня 3 (ModelConflict). Закрывает DilemmaDetector V2.1 полностью.  
**Почему первый:** закрывает целую версию, вся инфраструктура готова (AxialConflict существует в AE, CR читает AE через axial_bridge), высокий impact.

---

### P2 — EMERGENT-TD-02: reactivation_count гранулярность
**Файл:** `crates/axiom-experience/src/sutra_depth_store.rs` → `apply_evidence`  
**Что:** `reactivation_count` считает DREAM-циклы (~10-15 за 30k тиков) — слишком грубо. Инкрементировать в `dream_activation_acc` (каждый Wake-тик где Frame активен) → быстрорастущий сигнал, отражает реальную частоту реактивации.  
**Почему второй:** ~10 строк, снимает известное ограничение emergent primitive detection, разблокирует EMERGENT-TD-01.

---

### P3 — CR-TD-01: MAYA frontier fix
**Файл:** `engine.rs` (E1 fix), `axiom-arbiter/src/` (AshtiCore)  
**Что:** E1-fix токены не попадают в `Domain.frontier` → никогда не переходят в STATE_SLEEPING → накапливаются до 5000. Добавить `AshtiCore::push_to_frontier(domain_id, token_idx)` и вызывать после `inject_token`.  
**Почему третий:** убирает реальный баг накопления при долгих прогонах. Workaround работает, но правильный фикс маленький.

---

### P4 — OBS-TD-02: avg_shell_similarity rolling avg
**Файл:** `crates/axiom-observe/src/runner.rs`  
**Что:** `avg_candidate_shell_similarity()` всегда 0 при `snapshot_every=500` (кандидаты кристаллизуются за ~60 тиков). Rolling avg за последние N кристаллизаций вместо снапшота активных кандидатов.  
**Почему четвёртый:** улучшает observability quality, нужно для осмысленных OBS-метрик shell.

---

### P5 — CR-TD-04 persist: ActivityTrace → AutoSaver
**Файл:** `engine.rs` (init/shutdown), `axiom-persist`  
**Что:** serde уже есть ✅. Подключить к AutoSaver: сохранять ActivityTrace в bincode snapshot при DREAM-цикле; загружать при старте AxiomEngine.  
**Почему пятый:** V9 подготовка — observation sequence переживает рестарт. Инфраструктура готова, нужна только wire.

---

### P6 — word_signals() domain/layer expansion
**Файл:** `crates/axiom-agent/src/perceptors/decomposition_table.rs`  
**Что:** добавить domain (exec_*, shadow_*, ...) и layer (L1_*..L8_*) слова в `word_signals()`. id: и AnchorMatchTable уже готовы ✅. Path 2 (fallback) начнёт матчить domain/layer контекст.  
**Почему шестой:** механический, Path 1 уже покрывает эти слова через AnchorSet.match_text(). Расширяет fallback coverage.

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
