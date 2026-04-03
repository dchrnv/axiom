# Axiom — Отложенные задачи

**Версия:** 12.0
**Обновлён:** 2026-04-02

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

### WebSocket / REST / gRPC / Python / JSON-schema

**Точки расширения:** `RuntimeAdapter` trait готов (`axiom-runtime/src/adapters.rs`).

| Адаптер | Требует | Статус |
|---|---|---|
| WebSocket | axum / actix-web | не начат |
| REST | axum / actix-web | не начат |
| gRPC | tonic + protobuf | не начат |
| Python bindings | pyo3 | не начат |
| JSON-schema валидация конфигов | — | не начат |
