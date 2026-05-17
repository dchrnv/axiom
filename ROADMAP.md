# Axiom Roadmap

**Версия:** 57.0  
**Дата:** 2026-05-17

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
                                                    ↑
                                               axiom-broadcasting
                                                    ↑
                                               axiom-workstation
```

**1344 тестов, 0 failures.**  
Phase C (C1–C5), Phase I (I1–I4, I6), Phase E (E1) завершены.  
Workstation V1.0, axiom-node, Axiom Sentinel V1.1 в продакшне.

---

## Активные задачи

### I5 — OBS-01: живое наблюдение

**Проблема:** система ещё не запускалась с Phase C + полным якорным словарём на живых данных.

**Что сделать:** запустить `./run.sh`, подавать тексты через Conversation. Зафиксировать:

1. Какие Frame кристаллизуются? На каких текстах?
2. Какие SubsystemId определяет ContextRecognizer? Правильно ли?
3. Есть ли конфликты octant analytic vs synthetic в AxialEvaluator?
4. Появляются ли emergent-кандидаты в NeuralAdvisor?
5. Первый `NotifyEmergentCandidate` — при каких условиях?
6. Корректно ли пороги DepthThresholdEmergentDetector (8000/30/100)?

Результат: список наблюдений → tuning порогов → возможные errata.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **WS-V2-***, **COMP-01** — V2.0 идеи и Companion. См. DEFERRED.md.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
