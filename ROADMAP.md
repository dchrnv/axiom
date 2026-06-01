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

**1667 тестов, 0 failures.**  
Phases E–H завершены. V7 (A–E) завершён. Performance & Tooling Sprint завершён (2026-05-29):  
token lifecycle (STATE_SLEEPING), parallel domain ticks, parallel OBS shards, streaming JSONL, Lab UI panel.  
DilemmaDetector V2.0 завершён (2026-05-31): Сигнал A, кристаллизация Frame в EXPERIENCE.  
OBS §9 верификация: 8/8 дилемм, DilemmaDetector рабочий.  
Фикс: SubsystemDependencies YAML wrapper, TextPerceptor subsystem-позиция, engine.rs record_injection_signal.  
Cross-Modal Binding V1.0 завершён (2026-05-31): ModalityStore, CrossModalDetector, CROSS_MODAL_BOND=0x0A01; 1623 тестов.

---

## Активные задачи


**Файл:** `crates/axiom-experience/src/sutra_depth_store.rs` → `apply_evidence`  
**Что:** `reactivation_count` считает DREAM-циклы (~10-15 за 30k тиков) — слишком грубо. Инкрементировать в `dream_activation_acc` (каждый Wake-тик где Frame активен) → быстрорастущий сигнал, отражает реальную частоту реактивации.  
**Почему второй:** ~10 строк, снимает известное ограничение emergent primitive detection, разблокирует EMERGENT-TD-01.

---

---

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
