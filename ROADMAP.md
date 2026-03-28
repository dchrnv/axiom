# Axiom Roadmap

**Версия:** 9.9
**Дата:** 2026-03-28

---

## 🔄 Этап 1: GENOME + GUARDIAN

**Спека:** [docs/spec/GENOME_V1_0.md](docs/spec/GENOME_V1_0.md), [docs/spec/GUARDIAN_V1_0.md](docs/spec/GUARDIAN_V1_0.md)
**Архитектура:** [docs/spec/Ashti_Core_V2_1.md](docs/spec/Ashti_Core_V2_1.md)

### ~~Шаг 1 — Crate `axiom-genome` (Фаза A)~~ ✅ ГОТОВО

### ~~Шаг 2 — Guardian расширяется GENOME~~ ✅ ГОТОВО

### ~~Шаг 3 — `Arc<Genome>` через цепочку конструкторов~~ ✅ ГОТОВО

### ~~Шаг 4 — Arbiter ↔ GUARDIAN интеграция~~ ✅ ГОТОВО

### Шаг 5 — Фаза B: `config/genome.yaml` + serde_yaml (опционально)

`Genome::from_yaml(path)`. Тест: `from_yaml` == `default_ashti_core()`.

**Критерий завершения:** `cargo test --workspace` зелёный. Количество тестов ≥ 400.
GENOME создаётся. GenomeIndex строится. Guardian проверяет по GENOME + CODEX.
Pipeline работает с `try_new()`.

### Шаг 6 — Функциональный гайд

`docs/guides/AXIOM_GUIDE.md` — детальное описание функционала системы:
- Архитектура: крейты, зависимости, роли
- GENOME: конституция, boot sequence, валидация
- GUARDIAN: CODEX + GENOME проверки, enforce_access/enforce_protocol
- AshtiCore: 11 доменов, tick(), drain_events()
- Causal Frontier V2.0: FrontierConfig, pop(), begin/end_cycle
- Arbiter: dual-path routing, Experience, Maya
- UCL: команды, AxiomEngine::try_new(), process_command()
- Примеры кода для ключевых сценариев

---

## 🔮 Долгосрочные цели

### Configuration System
YAML-загрузка пространственных параметров и semantic_contributions. Требует согласования с DomainConfig 128-byte constraint.

### Адаптеры
Python bindings, REST API, gRPC — нужны для внешней интеграции.

### Производительность
SIMD (AVX-512), incremental spatial hash rebuild — после стабилизации архитектуры.

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-27
