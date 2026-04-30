# Axiom Roadmap

**Версия:** 39.0  
**Дата:** 2026-04-30

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

**DREAM Phase V1.0 + DreamConfig завершены.** 1088 тестов, 0 failures.  
**FrameWeaver V1.2:** промоция перенесена из on_tick → dream_propose() (вызов при FallingAsleep).  
**Over-Domain Layer:** traits `OverDomainComponent` + `Weaver`; `FrameWeaver V1.2` встроен в `AxiomEngine`.  
**DREAM (2026-04-28–29):** DreamScheduler, FatigueTracker, DreamCycle, DreamProposal; 4 состояния системы; GUARDIAN::check_frame_anchor_sutra_write(); CLI :dream-stats/:force-sleep/:wake-up; DreamConfig в axiom-config с hot-reload.  
**Онтология:** SUTRA / EXPERIENCE / MAYA. Frame живёт в EXPERIENCE (STATE_ACTIVE), промоция в SUTRA через CODEX только в DREAMING-состоянии.

---

## Активная задача

Нет активных задач. Система готова к следующему этапу.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
