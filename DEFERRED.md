# Axiom — Отложенные задачи

**Версия:** 14.0
**Обновлён:** 2026-04-06

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

## Отложенные функции (без срока)

### Справка: MLEngine input_size/output_size = 0

**Статус:** Отложен. НЕ реализовывать сейчас.

Проблема в `crates/axiom-agent/src/ml/engine.rs:120-123`. При загрузке ONNX через `tract` размеры тензоров не извлекаются — оба остаются 0. Проверка `if *input_size > 0` скрывает ShapeMismatch-ошибки.

**Когда исправлять:** При первой реальной ONNX-модели.

**Как исправлять:**
```rust
let input_fact = model.input_fact(0)
    .map_err(|e| MLError::ModelLoad(format!("No input fact: {}", e)))?;
let input_size = input_fact.shape.as_concrete()
    .map(|s| s.iter().product::<usize>())
    .unwrap_or(0);
```

---

## Memory Persistence V1.0 — технический долг

### D-04 — axiom-persist: serde_json вместо bincode

**Где:** `crates/axiom-persist/src/writer.rs`, `loader.rs`

При реализации bincode v2 не оказался в cargo-кэше (нет интернета при разработке). Перешли на `serde_json`. Формат JSON человекочитаемый, но:
- В 3–5× крупнее бинарного (критично при тысячах traces)
- В 2–4× медленнее сериализации/десериализации

**Как исправить:** Когда будет интернет — добавить `bincode = { version = "2", features = ["serde"] }` в `[workspace.dependencies]`, заменить `serde_json::to_string`/`from_slice` на `bincode::serde::encode_to_vec`/`decode_from_slice` в writer.rs и loader.rs. Расширение файла `.json` → `.bin`.

**Когда:** При следующей работе с axiom-persist или при появлении интернета.

---

### ~~D-05~~ — axiom-persist: DomainConfig не сохраняется **[RESOLVED 2026-04-06]**

Сохранение: `StoredDomain.config: Option<DomainConfig>` с `#[serde(default)]`.
Загрузка: `sd.config.unwrap_or_else(|| DomainConfig::factory_void(...))`.
Обратная совместимость сохранена — старые файлы десериализуются с `None → factory_void`.

---

### D-06 — axiom-persist: data_dir дублируется в CliConfig и PersistenceConfig

**Где:** `crates/axiom-agent/src/channels/cli.rs`

`CliConfig.data_dir` и `AutoSaver.config.data_dir` — два отдельных поля. После `:autosave on N` без явного пути `AutoSaver` использует `PersistenceConfig.data_dir` (инициализируется из `CliConfig.data_dir` при создании CliChannel), но если пользователь меняет `CliConfig.data_dir` в рантайме (сейчас нет такой команды) — `AutoSaver` не синхронизируется.

**Как исправить:** `AutoSaver` держать ссылку на `&str` из `CliConfig`, или убрать дублирование через единый `data_dir` в `CliChannel`.

**Когда:** При добавлении команды `:set data-dir <path>`.

---

### ~~D-07~~ — axiom-persist: AutoSaver не сбрасывает last_save_tick после :load **[RESOLVED 2026-04-06]**

Добавлен метод `AutoSaver::reset_save_tick(tick: u64)`.
В обработчике `:load`: `self.auto_saver.reset_save_tick(self.engine.tick_count)`.

---

### ~~D-08~~ — axiom-persist/skillset: дублирование import-логики **[RESOLVED 2026-04-06]**

Унифицировано через `import_skill_with_factor(skill, factor) -> bool`.
Константы `FRACTAL_IMPORT_FACTOR = 0.3`, `EXCHANGE_IMPORT_FACTOR = 0.7`.
`import_skill()` и `import_skill_exchange()` делегируют в общий метод.

---

### WebSocket / REST / gRPC / Python / JSON-schema

**Точки расширения:** `RuntimeAdapter` trait готов (`axiom-runtime/src/adapters.rs`).

| Адаптер | Требует | Статус |
|---|---|---|
| WebSocket | axum / actix-web | не начат |
| REST | axum / actix-web | не начат |
| gRPC | tonic + protobuf | не начат |
| Python bindings | pyo3 | не начат |
| JSON-schema валидация конфигов | — | не начат |
