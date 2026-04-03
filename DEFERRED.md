# Axiom — Отложенные задачи

**Версия:** 11.0
**Обновлён:** 2026-04-02

---

## Структурные несоответствия (найдены в Cleanup V1.1)

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

Во внутреннем ядре все токены и конфиги используют `u16`. Внешний API Engine принимает `u32`. На каждой точке входа происходит неявный каст `token.domain_id as u32`. Это безопасно при текущих значениях (max domain_id ≈ 65510), но семантически нечистно.

**Варианты:**
- Унифицировать всё в `u32` — значительный рефакторинг, +2 байта на Token/Connection (нарушает 64B)
- Унифицировать всё в `u16` — движок API принимает `u16`
- Оставить как есть, явно задокументировать слой конвертации

**Когда решать:** При следующем изменении Token/Connection layout.

---

### D-02 — Event._pad: u16 — 2 байта анонимного выравнивания

**Где:** `crates/axiom-core/src/event.rs`

**Проблема:** После Phase 3 cleanup `_reserved[16]` стал:
```
source_domain:      u16  // 2B
_pad:               u16  // 2B — только выравнивание u32
snapshot_event_id:  u32  // 4B
payload:          [u8;8] // 8B
```
Поле `_pad: u16` нужно для выравнивания `snapshot_event_id: u32` по адресу кратному 4. Но эти 2 байта могут нести полезную нагрузку:

**Кандидаты:**
- `target_domain: u16` — домен-получатель (сейчас Event не знает куда идёт, только откуда пришёл)
- `event_subtype: u16` — подтип события внутри `event_type` (для ShellExec, InternalImpulse)
- Оставить `_pad` и перенести `snapshot_event_id: u32` в `payload[4..8]`, освободив 4 байта

**Когда решать:** При следующем расширении семантики Event.

---

### D-03 — Token.reserved_phys: u16 — 2 байта физического резерва

**Где:** `crates/axiom-core/src/token.rs:62`

Между `target: [i16; 3]` и `valence: i8` лежат 2 байта паддинга. Сейчас `reserved_phys: u16 = 0`. Возможные применения:
- `layer_id: u16` — номер фрактального уровня-владельца (сейчас вычисляется как `domain_id / 100`)
- `hop_count: u16` — счётчик переходов между уровнями в FractalChain
- Оставить резервом для будущего физического поля (спин, заряд)

**Когда решать:** При проектировании multi-level fractal routing.

---

### D-04 — DomainConfig.reserved_id: u64 — 8 байт полностью свободны

**Где:** `crates/axiom-config/src/domain_config.rs:93`

Первое поле — `reserved_id: u64`, всегда 0, нигде не читается. Это 8 байт в 128-байтной структуре.

**Кандидат:** Разместить здесь `compare_tokens` tolerance-пороги (D-05), которые сейчас захардкожены как константы модуля:
```
token_compare_temp_tolerance:    i16  // 2B, default 10
token_compare_mass_tolerance:    i16  // 2B, default 5
token_compare_valence_tolerance: i16  // 2B, default 2
// + 2B остаток
```
Тогда пороги станут per-domain конфигурируемыми, что было оригинальной целью Phase 5.

**Когда решать:** Вместе с D-05.

---

### D-05 — compare_tokens tolerance: константы модуля, не DomainConfig

**Где:** `crates/axiom-arbiter/src/lib.rs`

Phase 5 добавила именованные константы:
```rust
pub const TOKEN_COMPARE_TEMP_TOLERANCE:    i16 = 10;
pub const TOKEN_COMPARE_MASS_TOLERANCE:    i16 = 5;
pub const TOKEN_COMPARE_VALENCE_TOLERANCE: i16 = 2;
```
По ROADMAP Phase 5 они должны были попасть в `DomainConfig`. Этого не произошло — `DomainConfig` уже был заполнен до `reserved_meta: [u8; 2]`.

**Решение:** Использовать `reserved_id: u64` из D-04 (6 из 8 байт). Тогда `compare_tokens()` читает из конфига, а не из глобальных констант.

**Когда решать:** Вместе с D-04. Связанные задачи.

---

### D-06 — tick_count не сохраняется в EngineSnapshot

**Где:** `crates/axiom-runtime/src/engine.rs`, `crates/axiom-runtime/src/snapshot.rs`

`AxiomEngine.tick_count: u64` не входит в `EngineSnapshot`. После `restore_from()` счётчик сбрасывается в 0. Последствие: все периодические задачи TickSchedule срабатывают на первом же тике после восстановления (при `interval=10` — на тике 10, при `interval=5000` — на тике 5000 вместо ожидаемого момента).

**Решение:** Добавить `tick_count: u64` в `EngineSnapshot` и восстанавливать его в `restore_from()`.

**Сложность:** Минимальная. 1 поле + 2 строки.

**Когда решать:** До первого production restore в живой системе.

---

### D-07 — UCL: 9 опкодов разобраны, но не диспатчатся

**Где:** `crates/axiom-runtime/src/engine.rs:process_command`

`opcode_from_u16()` успешно распознаёт все 19 опкодов. Но в `match opcode { ... }` ветка `_ =>` ловит 9 из них:

| OpCode | Числовое | Статус |
|---|---|---|
| `LockMembrane` | 1002 | не диспатчится → UNKNOWN_OPCODE |
| `ReshapeDomain` | 1003 | не диспатчится → UNKNOWN_OPCODE |
| `ApplyForce` | 2001 | не диспатчится → UNKNOWN_OPCODE |
| `AnnihilateToken` | 2002 | не диспатчится → UNKNOWN_OPCODE |
| `BondTokens` | 2003 | не диспатчится → UNKNOWN_OPCODE |
| `SplitToken` | 2004 | не диспатчится → UNKNOWN_OPCODE |
| `ChangeTemperature` | 3001 | не диспатчится → UNKNOWN_OPCODE |
| `ApplyGravity` | 3002 | не диспатчится → UNKNOWN_OPCODE |
| `PhaseTransition` | 3003 | не диспатчится → UNKNOWN_OPCODE |

Клиент отправляет валидный опкод, получает `SystemError / UNKNOWN_OPCODE`. Это silent breakage.

**Быстрое решение:** Явные ветки → `Success` с `events=0` (текущее поведение CollapseDomain). Закрывает несоответствие без реализации физики.

**Когда решать:** Перед публичным UCL API / gRPC-адаптером.

---

### D-08 — StructuralRole vs UCL factory_preset: две системы нумерации

**Где:** `crates/axiom-ucl/src/lib.rs:258`, `crates/axiom-config/src/domain_config.rs`

Числовые коды не совпадают:

| Роль | StructuralRole (enum) | UCL factory_preset (комментарий) |
|---|---|---|
| Sutra | 0 | 1 |
| Void | 8 | 0 |
| Maya | 10 | 10 (совпадает) |

`UclBuilder::spawn_domain` сейчас присваивает `structural_role = factory_preset` (Phase 5: задокументировано как проблема). Если SpawnDomain начнёт реально создавать домены через `DomainConfig`, маппинг сломает роли.

**Решение:** Либо привести UCL к StructuralRole нумерации (breaking change API), либо добавить явный маппинг в `handle_spawn_domain`.

**Когда решать:** До реализации динамического SpawnDomain.

---

### D-09 — AshtiCore::reconcile_all() и reconcile_interval

**Где:** `crates/axiom-domain/src/ashti_core.rs`, `crates/axiom-runtime/src/engine.rs`

`TickSchedule.reconcile_interval = 200` существует как поле, но в `handle_tick_forward` нет соответствующего вызова — метода `AshtiCore::reconcile_all()` не существует.

**Семантика reconcile в AXIOM:**

Примирение (reconcile) — периодическая операция согласованности семантического пространства после серии тиков. Включает три задачи:

1. **Пространственная перестройка** — на каждом `Domain` уже есть `rebuild_spatial_grid()` и `should_rebuild_spatial_grid()` (через `events_since_rebuild >= rebuild_frequency`). `reconcile_all()` форсирует перестройку для доменов, у которых флаг поднят, но rebuild ещё не произошёл.

2. **Обрезка осиротевших связей** — `Connection.source_id` / `target_id` ссылаются на `sutra_id` токенов. После удаления токенов из домена связи могут ссылаться на несуществующие токены. Нужна итерация по connections каждого домена с проверкой наличия обоих концов.

3. **Проверка соответствия domain_id** — токен может оказаться в DomainState, чей domain_id не совпадает с `token.domain_id`. Это семантический инвариант: токен обязан знать свой домен.

**Оценка сложности:** Средняя. Задачи 1 и 3 — простые итерации. Задача 2 требует построения множества живых `sutra_id` перед проверкой связей.

**Реализация:**
```rust
// ashti_core.rs
pub fn reconcile_all(&mut self) {
    for i in 0..11 {
        if let (Some(state), Some(domain)) =
            (self.state_mut(i), self.domains.get_mut(i))
        {
            // 1. Пространственная перестройка
            if domain.should_rebuild_spatial_grid() {
                domain.rebuild_spatial_grid(&state.tokens);
            }
            // 2. Осиротевшие связи
            let live: std::collections::HashSet<u32> =
                state.tokens.iter().map(|t| t.sutra_id).collect();
            state.connections.retain(|c| {
                live.contains(&c.source_id) && live.contains(&c.target_id)
            });
            // 3. domain_id токенов
            let did = self.domain_id_at(i).unwrap_or(0) as u16;
            for token in &mut state.tokens {
                token.domain_id = did;
            }
        }
    }
}
```

**Когда решать:** Перед первым использованием семантической маршрутизации в живой системе. Без reconcile осиротевшие связи накапливаются, пространственный индекс устаревает.

---

## Отложенные функции (без срока)

### Справка: MLEngine input_size/output_size = 0

**Статус:** Отложен. НЕ реализовывать сейчас.

Проблема в `crates/axiom-agent/src/ml/engine.rs:120-123`. При загрузке ONNX-модели через `tract` размеры тензоров не извлекаются:

```rust
Ok(MLEngine::Real {
    model: Box::new(model),
    input_size: 0,  // Должно быть: model.input_fact(0).shape
    output_size: 0, // Должно быть: model.output_fact(0).shape
})
```

Последствие: проверка `if *input_size > 0` скрывает ShapeMismatch-ошибки.

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
