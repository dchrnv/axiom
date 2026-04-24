# Axiom Roadmap

**Версия:** 37.0  
**Дата:** 2026-04-24

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent
                                                (axiom-cli)
```

**FrameWeaver V1.1 завершён (Фазы 1–5).** 1017 тестов, 0 failures.  
**Over-Domain Layer:** traits `OverDomainComponent` + `Weaver`; `FrameWeaver` встроен в `AxiomEngine` (on_tick + drain_commands).  
**Онтология:** SUTRA / EXPERIENCE / MAYA. Frame живёт в EXPERIENCE (STATE_ACTIVE), промоция в SUTRA через CODEX.

---

## Активная задача

Нет активных задач. Система готова к следующему этапу.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
