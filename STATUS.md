# AXIOM Status

**Обновлено:** 2026-03-28 (Этап 6 завершён)
**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

---

## Текущее состояние

**568 тестов, 0 failures, 0 warnings**

```
AxiomEngine (try_new + Arc<Genome>)
  ├── Genome (конституция, from_yaml, GenomeIndex O(1))
  ├── AshtiCore — 11 доменов (SUTRA=100..MAYA=110)
  │     ├── Arbiter (dual-path routing + Experience)
  │     ├── 11 × Domain (физика поля + CausalFrontier V2.0)
  │     └── 11 × DomainState (токены + связи)
  └── Guardian (CODEX + GENOME: enforce_access/protocol, validate_reflex)
```

**Документация:** [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## Crates

| Crate | Тесты | Описание |
|-------|-------|----------|
| axiom-core | 24 | Token, Connection, Event |
| axiom-genome | 26 | Genome V1.0: конституция, GenomeIndex, from_yaml (Фаза B) |
| axiom-frontier | 32 | CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection, budget |
| axiom-config | 48 | DomainConfig (11 factory методов + from_yaml), HeartbeatConfig, ConfigLoader::load_all |
| axiom-space | 95 | SpatialHashGrid, физика, координаты, SpatialConfig from_yaml |
| axiom-shell | 48 | Shell V3.0, семантические профили, from_yaml |
| axiom-arbiter | 86 | Arbiter V1.0, Experience, REFLECTOR, SKILLSET (export/import_batch), GridHash, AshtiProcessor, Maya, COM |
| axiom-heartbeat | 11 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-ucl | 5 | UCL commands |
| axiom-domain | 99 | Domain, DomainState, EventGenerator, AshtiCore, CausalHorizon |
| axiom-runtime | 79 | AxiomEngine (try_new, snapshot_and_prune, run_adaptation), Guardian, RoleStats |
| axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| **Итого** | **568** | |
