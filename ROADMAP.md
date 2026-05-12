# Axiom Roadmap

**Версия:** 49.0  
**Дата:** 2026-05-12

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

**1192 тестов, 0 failures.** FrameWeaver V1.3, Workstation V1.0 + UI polish, protocol extensions, Axiom Sentinel V1.1 (S0–S5 + S4b) завершены.

---

## Фазы работы

### Фаза A — «Живая Workstation» 🔑

**Главный приоритет.** Все остальные фазы либо разблокируются после A, либо независимы от него.

### Фаза S — Axiom Sentinel V1.1 ✅

**Реализовано 2026-05-12.** S0–S5 + S4b завершены. Gravity 1M: 6.74 ms (цель 8–10 ms ✅). S6 → DEFERRED.md.

---

### Фаза E — «Контент и инфраструктура»

#### E1 — Anchor-Fill: якорные YAML-файлы

14 файлов (L1–L8 кроме L5, D2–D8). ~7–10 якорей каждый. Делать вручную — это семантический
контент, не код. Диагностика: `:match "слово"` в CLI. Система работает без них (FNV-1a fallback).

**Когда:** По мере понимания семантики. Без дедлайна.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing. Очень низкий приоритет.
- **WS-V2-***, **COMP-01** — V2.0 идеи и Companion. См. DEFERRED.md.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
