# Axiom Roadmap

**Версия:** 51.0  
**Дата:** 2026-05-13

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

**1201 тестов, 0 failures.** FrameWeaver V1.3, Workstation V1.0 + UI polish, protocol extensions, Axiom Sentinel V1.1 (S0–S6) завершены.

---

## Фазы работы

### Фаза A — axiom-node: Живая Workstation 🔑

**Главный приоритет.** Разблокирует BRD-TD-07, WS4-TD-03/04, BRD-TD-05, OBS-01.

Самодостаточный бинарный crate — живой Axiom без CLI. Собственный tick loop,
полная инициализация модели, graceful shutdown.

#### A1 — Каркас crate

- `crates/axiom-node` в workspace
- `Cargo.toml`: axiom-runtime, axiom-broadcasting, axiom-agent, axiom-persist,
  axiom-config, tokio (full), tracing + tracing-subscriber, clap
- `config.rs` — `NodeConfig`: port, data_dir, tick_hz, log_level, adaptive_tick

#### A2 — Startup

- `startup.rs` — полная последовательность инициализации:
  1. Загрузить `axiom.yaml` → Genome + DomainConfig + DreamConfig + HeartbeatConfig
  2. Загрузить `AnchorSet` из `config/anchors/`
  3. Создать `AxiomEngine` из конфига
  4. `inject_anchor_tokens` — якоря в пространство
  5. Восстановить состояние из persist (EXPERIENCE + SKILLSET)
- Ошибки старта — явные, с контекстом

#### A3 — Tick loop

- `tick.rs` — чистый loop без CLI-состояния:
  - tick engine (TickForward) → drain events → Dream phase → FrameWeaver
  - adaptive tick rate (TickBudget + AdaptiveTickRate)
  - каждые N тиков: `build_system_snapshot` → `handle.update_snapshot` + `handle.publish`
  - `speculate_grids` между тиком и reconcile
  - `auto_saver.tick`

#### A4 — Обработка команд Workstation

- `commands.rs` — `handle_engine_command(cmd, engine, handle, perceptor)`:
  - `SubmitText` → TextPerceptor → process → `CommandResult`
  - `RequestFullSnapshot` → `build_system_snapshot` → `handle.publish`
  - `RunBench` → запуск встроенных бенчмарков → `BenchProgress` events
  - `ForceSync`, `CancelAdapter`, прочие из axiom-protocol

#### A5 — Graceful shutdown

- `shutdown.rs` — перехват SIGINT/SIGTERM
- `force_save` состояния перед выходом
- Чистое завершение BroadcastServer

#### A6 — Smoke test + закрытие долгов

- Запустить axiom-node, подключить Workstation → убедиться что данные идут
- Закрыть BRD-TD-07 (Engine → BroadcastHandle интеграция ✅)
- Проверить WS4-TD-03/04 на живых данных → закрыть или уточнить

### Фаза S — Axiom Sentinel V1.1 ✅

**Реализовано 2026-05-13.** S0–S5 + S4b + S6 завершены. Gravity 1M: 6.74 ms (цель 8–10 ms ✅). Speculative Layer реализован.

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
