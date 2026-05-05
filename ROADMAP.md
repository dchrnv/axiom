# Axiom Roadmap

**Версия:** 40.0  
**Дата:** 2026-05-05

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

**Workstation V1.0 завершён (2026-05-05).** 1174 тестов, 0 failures.  
**Три новых crate:** axiom-protocol (41 тест), axiom-broadcasting (6 тестов), axiom-workstation (39 тестов).  
**axiom-workstation:** iced 0.13 desktop-клиент, 8 вкладок (System Map, Live Field, Conversation, Patterns, Dream State, Configuration, Files, Benchmarks), bidirectional WebSocket, Welcome/Main фазы, alert overlay, keyboard shortcuts.  
**axiom-broadcasting** пока не подключён к реальному Engine tick-loop (BRD-TD-07 → axiom-node).  
**DREAM Phase V1.0 + DreamConfig завершены (2026-04-28–29).** FrameWeaver V1.2: промоция → dream_propose().  
**Онтология:** SUTRA / EXPERIENCE / MAYA. Frame живёт в EXPERIENCE, промоция в SUTRA только в DREAMING.

---

## Активная задача

Нет активных задач. Система готова к следующему этапу.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
