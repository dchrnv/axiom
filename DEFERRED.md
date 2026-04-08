# Axiom — Отложенные задачи

**Версия:** 15.0
**Обновлён:** 2026-04-08

---

## Структурные несоответствия

### D-01 — domain_id: u16 vs u32 на границе Engine API

**Проблема:** Тип `domain_id` не согласован по всему стеку.

| Место | Тип |
|---|---|
| `Token.domain_id` | `u16` |
| `Connection.domain_id` | `u16` |
| `DomainConfig.domain_id` | `u16` |
| `InjectTokenPayload.target_domain_id` | `u16` |
| `AshtiCore.inject_token(domain_id: u32)` | `u32` |
| `AshtiCore.index_of(domain_id: u32)` | `u32` |
| `AxiomEngine.token_count(domain_id: u32)` | `u32` |

На каждой точке входа происходит неявный каст `token.domain_id as u32`. Безопасно при текущих значениях (max ≈ 65510), но семантически нечистно.

**Варианты:**
- Унифицировать в `u32` — значительный рефакторинг, +2 байта на Token/Connection (нарушает 64B)
- Унифицировать в `u16` — движок API принимает `u16`
- Оставить, явно задокументировать слой конвертации

**Когда решать:** При следующем изменении Token/Connection layout.

---

### D-02 — Event._pad: u16 — 2 байта анонимного выравнивания

**Где:** `crates/axiom-core/src/event.rs`

Layout после Cleanup Phase 3:
```
source_domain:      u16  // 2B
_pad:               u16  // 2B — только выравнивание u32
snapshot_event_id:  u32  // 4B
payload:          [u8;8] // 8B
```

**Кандидаты для _pad:**
- `target_domain: u16` — домен-получатель (Event сейчас знает только источник)
- `event_subtype: u16` — подтип внутри `event_type` (ShellExec, InternalImpulse)

**Когда решать:** При следующем расширении семантики Event.

---

### D-03 — Token.reserved_phys: u16 — 2 байта физического резерва

**Где:** `crates/axiom-core/src/token.rs:62`

Паддинг между `target: [i16; 3]` и `valence: i8`. Возможные применения:
- `layer_id: u16` — номер фрактального уровня-владельца
- `hop_count: u16` — счётчик переходов в FractalChain

**Когда решать:** При проектировании multi-level fractal routing.

---

## Отложенные функции

### D-04 — axiom-persist: serde_json вместо bincode

**Где:** `crates/axiom-persist/src/writer.rs`, `loader.rs`

При реализации bincode v2 не оказался в cargo-кэше (нет интернета при разработке). Перешли на `serde_json`. Формат JSON человекочитаемый, но:
- В 3–5× крупнее бинарного (критично при тысячах traces)
- В 2–4× медленнее сериализации/десериализации

**Как исправить:** Добавить `bincode = { version = "2", features = ["serde"] }` в `[workspace.dependencies]`, заменить `serde_json::to_string`/`from_slice` на `bincode::serde::encode_to_vec`/`decode_from_slice`. Расширение файла `.json` → `.bin`.

**Когда:** При появлении интернета.

---

### D-05 — axiom-persist: data_dir дублируется в CliConfig и PersistenceConfig

**Где:** `crates/axiom-agent/src/channels/cli.rs`

`CliConfig.data_dir` и `AutoSaver.config.data_dir` — два отдельных поля. `AutoSaver` инициализируется из `CliConfig.data_dir` при создании, но не синхронизируется при runtime-изменении.

**Как исправить:** Единый `data_dir` в `CliChannel`, или `AutoSaver` держит `&str` из `CliConfig`.

**Когда:** При добавлении команды `:set data-dir <path>`.

---

### D-06 — MLEngine: input_size/output_size = 0 при загрузке ONNX

**Где:** `crates/axiom-agent/src/ml/engine.rs:120-123`

При загрузке ONNX через `tract` размеры тензоров не извлекаются. Проверка `if *input_size > 0` скрывает ShapeMismatch-ошибки.

**Как исправить:**
```rust
let input_fact = model.input_fact(0)?;
let input_size = input_fact.shape.as_concrete()
    .map(|s| s.iter().product::<usize>())
    .unwrap_or(0);
```

**Когда:** При первой реальной ONNX-модели.

---

### D-07 — JSON-schema валидация конфигов

**Где:** `axiom-cli.yaml`, `crates/axiom-config/src/loader.rs`

Сейчас при невалидном YAML в `axiom-cli.yaml` или конфигах доменов — panic или молчаливый дефолт. Нет чёткого сообщения что именно неверно.

**Что нужно:**
- Crate `schemars` — генерация JSON Schema из Rust-структур через `#[derive(JsonSchema)]`
- Crate `jsonschema` — валидация документа против схемы
- Добавить `#[derive(JsonSchema)]` на `CliConfig`, `TickScheduleConfig`, `DomainConfig`, `PersistenceConfig`
- В `ConfigLoader::load()` и `CliConfig::from_yaml()` — валидировать перед десериализацией, выводить human-readable ошибку с путём к проблемному полю
- Опционально: команда `axiom-cli --dump-schema` — выводит актуальную JSON Schema для `axiom-cli.yaml`

**Когда:** При появлении интернета (нужны `schemars`, `jsonschema` из crates.io).

---

## Внешние адаптеры (требуют интернета)

**Точка расширения:** `RuntimeAdapter` trait в `axiom-runtime/src/adapters.rs`.

| Адаптер | Требует | Статус |
|---|---|---|
| WebSocket | axum / actix-web | не начат |
| REST API | axum / actix-web | не начат |
| gRPC | tonic + protobuf | не начат |
| Python bindings | pyo3 | не начат |

**Когда:** При появлении интернета и конкретной задачи интеграции.
