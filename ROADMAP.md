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

### Foundation Фаза 1 (продолжение)

- **SEED-TD-01** — TextPerceptor 2-path: интеграция кристаллических якорей для позиционирования.
  Детали в DEFERRED.md. После реализации — boot-инъекция crystal_c0.yaml.
- **C6** — OBS-прогон кристалла: матчинг графем, секторная раскладка, co-activation C0→C1.
  После SEED-TD-01.

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
