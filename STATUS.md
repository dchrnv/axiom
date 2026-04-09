# AXIOM Status

**Обновлено:** 2026-04-09 (Axiom Sentinel V1.0 — ЗАВЕРШЁН)
**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

---

## Текущее состояние

**900 тестов, 0 failures, 0 warnings**

```
AxiomEngine
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
  ├── TextPerceptor — текст → UclCommand(InjectToken) через FNV-1a hash → 3D position
  ├── MessageEffector — ProcessingResult → диагностический вывод
  ├── MLEngine (mock + ONNX) → VisionPerceptor, AudioPerceptor (VAD)
  └── CLI Channel: stdin/stdout loop, axiom-cli.yaml, :save/:load/:autosave/:export/:import

axiom-runtime:
  ├── process_and_observe() — обёртка process_command() с диагностикой (ProcessingResult)
  ├── Orchestrator — параллельная маршрутизация + Guardian check + apply_feedback
  └── AdaptiveTickRate — Variable Tick Rate (min_hz=60, max_hz=1000, cooldown=50)

axiom-persist:
  ├── save/load: Token+Connection+ExperienceTrace → JSON (атомарный rename)
  ├── MemoryManifest (YAML), IMPORT_WEIGHT_FACTOR=0.7
  ├── AutoSaver: интервальное автосохранение, force_save при :quit
  └── exchange: export/import traces+skills, GUARDIAN-валидация

axiom-space:
  └── apply_gravity_batch — batch-физика, авто-векторизация (feature "simd")
```

**Документация:** [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## Crates

| Crate | Тесты | Описание |
|-------|-------|----------|
| axiom-core | 36 | Token, Connection, Event |
| axiom-genome | 26 | Genome V1.0: конституция, GenomeIndex, from_yaml |
| axiom-frontier | 32 | CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection, budget |
| axiom-config | 93 | DomainConfig, ConfigLoader, YAML presets, ConfigWatcher (inotify hot reload) |
| axiom-space | 104 | SpatialHashGrid, физика, apply_gravity_batch (SIMD-ready, feature "simd") |
| axiom-shell | 48 | Shell V3.0, семантические профили, from_yaml |
| axiom-arbiter | 139 | Arbiter V1.0, Experience, REFLECTOR, SKILLSET, GridHash, AshtiProcessor, COM |
| axiom-heartbeat | 15 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-ucl | 9 | UCL commands |
| axiom-domain | 116 | Domain, DomainState, AshtiCore, CausalHorizon, FractalChain |
| axiom-runtime | 117 | AxiomEngine, Guardian, Gateway, Channel, EventBus, Adapters, TickSchedule, ProcessingResult, AdaptiveTickRate, Orchestrator |
| axiom-agent | 92 | TextPerceptor, MessageEffector, CliChannel, TelegramPerceptor/Effector, ShellEffector, MLEngine |
| axiom-persist | 35 | MemoryWriter, MemoryLoader, MemoryManifest, AutoSaver, exchange (export/import) |
| axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| **Итого** | **900** | |

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
| Cleanup | COM V1.1 — unwrap, Unknown, Event fields, COM, constants, TickSchedule | ✅ |
| Memory | Memory Persistence V1.0 — save/load/autosave/exchange (axiom-persist) | ✅ |
| CLI V1.1 | CLI Channel V1.1 — TextPerceptor, MessageEffector, process_and_observe, axiom-cli.yaml | ✅ |
| Sentinel | Axiom Sentinel V1.0 — Hardware Topology, Parallel Resonance Search, Variable Tick Rate | ✅ |
