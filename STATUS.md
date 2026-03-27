# AXIOM Status

**Обновлено:** 2026-03-27
**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

---

## Текущее состояние

**387 тестов, 0 failures, 0 warnings**

```
AxiomEngine
  ├── AshtiCore — 11 доменов (SUTRA=100..MAYA=110)
  │     ├── Arbiter (dual-path routing + Experience)
  │     ├── 11 × Domain (физика поля)
  │     └── 11 × DomainState (токены + связи)
  └── Guardian (CODEX-валидация рефлексов)
```

---

## Crates

| Crate | Тесты | Описание |
|-------|-------|----------|
| axiom-core | 24 | Token, Connection, Event |
| axiom-frontier | 28 | CausalFrontier V2.0, FrontierConfig, FrontierEntity, storm, budget |
| axiom-config | 33 | DomainConfig (11 factory методов), HeartbeatConfig |
| axiom-space | 83 | SpatialHashGrid, физика, координаты |
| axiom-shell | 43 | Shell V3.0, семантические профили |
| axiom-arbiter | 26 | Arbiter V1.0, Experience, AshtiProcessor, Maya, COM |
| axiom-heartbeat | 11 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-ucl | 5 | UCL commands |
| axiom-domain | 84 | Domain, DomainState, EventGenerator, AshtiCore |
| axiom-runtime | 31 | AxiomEngine, Guardian, Snapshot, orchestrator |
| axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| **Итого** | **387** | |
