# Axiom Roadmap

**Версия:** 9.8
**Дата:** 2026-03-28

---

## 🔄 Этап 1: GENOME + GUARDIAN

**Спека:** [docs/spec/GENOME_V1_0.md](docs/spec/GENOME_V1_0.md), [docs/spec/GUARDIAN_V1_0.md](docs/spec/GUARDIAN_V1_0.md)
**Архитектура:** [docs/spec/Ashti_Core_V2_1.md](docs/spec/Ashti_Core_V2_1.md)

### Шаг 1 — Crate `axiom-genome` (Фаза A)

Новый crate: `axiom-genome` → зависит только от `axiom-core`.

```
axiom-genome
  ├── Genome { version, invariants, access_rules, protocol_rules, config }
  ├── GenomeInvariants  — размеры структур, архитектурные ограничения
  ├── AccessRule / Permission / ModuleId / ResourceId
  ├── ProtocolRule / DataType
  ├── GenomeConfig      — глобальные параметры Arbiter, Frontier, Heartbeat
  ├── GenomeIndex       — предвычисленные матрицы для O(1) lookup
  └── Genome::default_ashti_core() — захардкоженная конфигурация (без serde)
```

Тесты: O(1) access/protocol lookup, валидация инвариантов, `validate()` → Ok/Err.

### Шаг 2 — Guardian расширяется GENOME

В `axiom-runtime/src/guardian.rs` добавить:
- `genome: Arc<Genome>` + `genome_index: GenomeIndex`
- `enforce_access(module, resource, operation) -> bool` — O(1) через матрицу
- `enforce_protocol(source, target) -> bool` — O(1) через матрицу
- `scan_domain(domain, domain_id) -> Vec<InhibitAction>`
- `update_codex(codex_state, action) -> Result`
- Существующая CODEX-валидация остаётся, `validate_reflex()` расширяется GENOME-проверками

Тесты: enforce_access разрешает/блокирует, enforce_protocol, validate_reflex с GENOME.

### Шаг 3 — `Arc<Genome>` через цепочку конструкторов

```
AxiomEngine::try_new(genome: Arc<Genome>) -> Result<Self, AxiomError>
  → Guardian::new(Arc::clone(&genome))
  → Arbiter::new(Arc::clone(&genome), ...)
  → AshtiCore::new(Arc::clone(&genome), ...)
```

Существующие тесты: `AxiomEngine::new()` → `AxiomEngine::try_new(...).unwrap()`.

### Шаг 4 — Arbiter ↔ GUARDIAN интеграция

При `GUARDIAN_CHECK_REQUIRED` бите в DomainConfig — Arbiter вызывает
`Guardian::validate_reflex()` перед отправкой рефлекса в MAYA.

### Шаг 5 — Фаза B: `config/genome.yaml` + serde_yaml (опционально)

`Genome::from_yaml(path)`. Тест: `from_yaml` == `default_ashti_core()`.

**Критерий завершения:** `cargo test --workspace` зелёный. Количество тестов ≥ 400.
GENOME создаётся. GenomeIndex строится. Guardian проверяет по GENOME + CODEX.
Pipeline работает с `try_new()`.

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
