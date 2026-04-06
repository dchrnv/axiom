# AXIOM Status

**Обновлено:** 2026-04-06 (Memory Persistence V1.0 — Фаза 2)
**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

---

## Текущее состояние

**848 тестов, 0 failures, 0 warnings**

```
AxiomEngine (try_new + Arc<Genome>)
  ├── Genome (конституция, from_yaml, GenomeIndex O(1))
  ├── AshtiCore — 11 доменов (SUTRA=level*100 .. MAYA=level*100+10)
  │     ├── Arbiter (dual-path routing + Experience + Reflector + SkillSet + Internal Drive)
  │     ├── 11 × Domain (физика поля + CausalFrontier V2.0)
  │     └── 11 × DomainState (токены + связи)
  └── Guardian (CODEX + GENOME: enforce_access, validate_reflex, ML filters)

FractalChain — N уровней AshtiCore (MAYA[n] → SUTRA[n+1], skills exchange)
ConfigWatcher — горячая перезагрузка конфигов (inotify)
EventBus — pub/sub: типизированные и broadcast подписки

axiom-agent:
  ├── Perceptor/Effector: CLI, Telegram, Shell
  ├── MLEngine (mock + ONNX) → VisionPerceptor, AudioPerceptor (VAD)
  └── CLI Channel: :save/:load/:memory (через axiom-persist)

axiom-persist:
  └── save/load Engine state: Token+Connection+ExperienceTrace → JSON
      MemoryManifest (YAML), IMPORT_WEIGHT_FACTOR=0.7 для traces

axiom-space:
  └── apply_gravity_batch — batch-физика, авто-векторизация (feature "simd")
```

**Документация:** [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## Crates

| Crate | Тесты | Описание |
|-------|-------|----------|
| axiom-core | 24 | Token, Connection, Event |
| axiom-genome | 26 | Genome V1.0: конституция, GenomeIndex, from_yaml |
| axiom-frontier | 32 | CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection, budget |
| axiom-config | 75 | DomainConfig, ConfigLoader, YAML presets, ConfigWatcher (inotify hot reload) |
| axiom-space | 110 | SpatialHashGrid, физика, apply_gravity_batch (SIMD-ready, feature "simd") |
| axiom-shell | 48 | Shell V3.0, семантические профили, from_yaml |
| axiom-arbiter | 136 | Arbiter V1.0, Experience, REFLECTOR, SKILLSET, GridHash, AshtiProcessor, COM |
| axiom-heartbeat | 15 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-ucl | 5 | UCL commands |
| axiom-domain | 112 | Domain, DomainState, AshtiCore, CausalHorizon, FractalChain |
| axiom-runtime | 136 | AxiomEngine, Guardian, Gateway, Channel, EventBus, Adapters, TickSchedule |
| axiom-agent | 80 | CliPerceptor/Effector, TelegramPerceptor/Effector, ShellEffector, MLEngine, VisionPerceptor, AudioPerceptor |
| axiom-persist | 15 | MemoryWriter, MemoryLoader, MemoryManifest — save/load + boot sequence |
| axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| **Итого** | **848** | |

---

## Этапы

| Этап | Название | Статус |
|------|----------|--------|
| 1 | GENOME + GUARDIAN | ✅ |
| 2 | Storm Control | ✅ |
| 3 | Configuration System | ✅ |
| 4 | REFLECTOR + SKILLSET | ✅ |
| 5 | GridHash-индекс | ✅ |
| 6 | Адаптивные пороги | ✅ |
| 7 | Causal Horizon + Масштабирование | ✅ |
| 8 | External Integration Layer | ✅ |
| 9 | Tech Debt + EventBus + Config hot reload | ✅ |
| 10 | Agent Layer (CLI/Telegram/Shell) | ✅ |
| 11 | ML Inference | ✅ |
| 12 | FractalChain + SIMD-физика | ✅ |
| 13A | Cognitive Depth — Multi-pass + TensionTrace | ✅ |
| 13B | Cognitive Depth — Heartbeat Internal Drive | ✅ |
| 13C | Cognitive Depth — InternalImpulse + Dominance | ✅ |
| 13D | Cognitive Depth — Goal Persistence + Curiosity | ✅ |
| Cleanup | Plan_Cleanup_COM_V1_1 — 6 фаз (unwrap, Unknown, Event fields, COM, constants, TickSchedule) | ✅ |
| DEFERRED | D-06..D-09: tick_count snapshot, UCL no-op, reconcile_all, tolerances, StructuralRole маппинг | ✅ |
