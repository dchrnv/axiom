# Axiom Roadmap

**Версия:** 46.0  
**Дата:** 2026-05-06

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

**Workstation V1.0 + B1–E2 завершены (2026-05-06).** 1183 тестов, 0 failures.  
**B1–B6:** rfd file picker, multi-line editor, show-more, canvas::Cache, fade-in, MenuBar.  
**C1–C3:** syntactic_layer_activations, RunBench, TokenFieldPoint/token_field.  
**D1–D6:** check_promotion(tick), min_participant_anchors, RuleTrigger x3, GENOME on_boot, domain hot-reload, broadcasting throttle + Lagged resync.  
**E2:** MLEngine explicit ShapeMismatch (D-06 закрыт).  
**FrameWeaver V1.3** — все RuleTrigger реализованы, GENOME enforcement, корректный min_age_ticks.

---

## Фазы работы

### Фаза A — «Живая Workstation» 🔑

**Главный приоритет.** Все остальные фазы либо разблокируются после A, либо независимы от него.

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
