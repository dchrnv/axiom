# Axiom Roadmap

**Версия:** 80.0
**Дата:** 2026-06-13

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
axiom-corpus       axiom-neural   axiom-seed        ↑
                                               axiom-broadcasting
```

**1779 тестов (all features), TEST-TD-01 (DEFERRED).**

## Следующие задачи


### Store Optimization (ожидает завершения Foundation)

- **STORE-OPT-01** — HashMap→массив для bounded-хранилищ (axiom-experience):
  - `SubsystemId::index()` + `ALL_SUBSYSTEMS` в types.rs (фундамент)
  - `FatigueStore`: `HashMap<SubsystemId, SubsystemFatigue>` → `[SubsystemFatigue; 9]`
  - `InterpretationProfile.weights`: `HashMap<SubsystemId, u8>` → `[u8; 9]`
  - `MetaStore`: `HashMap<MetaSubsystemId, MetaActivation>` → `[Option<MetaActivation>; 7]`
  - Обновить `apply_to_weights` сигнатуру под новый тип weights.
  - Custom MetaSubsystemId (0x1100+) — в DEFERRED (STORE-TD-01).
- **STORE-OPT-02** — per-sutra консолидация: объединить AxialStore + SutraDepthStore +
  InterpretationProfileStore + ModalityStore + shell_registry в один `HashMap<u32, SutraRecord>`.
  Делать после OPT-01 (когда weights уже массив).

### Foundation Фаза 2 (следующая)

- **Seed Injection V1.0** — спека + реализация: grow-семена C1 (слоги) на слой C1,
  composition bonds C0→C1, пара аккумулятор→генератор.

---

## Не в активном плане

- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **COMP-01** — Vital Signs окно (Companion). См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **Neural Integration Этап 2** — AudioPerceptor, Speech Commands, Vision. После обучения модели (NEURAL-TD-01/02).
- **Neural Integration Этап 3** — ультразвук, расширенный STT. После этапа 2.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
