# FractalChain и SIMD-физика — гайд-разъяснение

**Версия:** 1.0
**Дата:** 2026-03-29
**Этап:** 12 (Фракталы и SIMD)
**Файлы:**
- [`crates/axiom-domain/src/fractal_chain.rs`](../../crates/axiom-domain/src/fractal_chain.rs)
- [`crates/axiom-domain/src/ashti_core.rs`](../../crates/axiom-domain/src/ashti_core.rs) (методы `take_maya_output`, `set_sutra_input`)
- [`crates/axiom-space/src/simd.rs`](../../crates/axiom-space/src/simd.rs)

---

## Часть 1 — FractalChain (Этап 12A)

### Зачем это нужно

Один `AshtiCore` — это 11 доменов, один уровень обработки. Но AXIOM предполагает
**иерархическое мышление**: грубое восприятие (уровень 0) рафинируется уровнем 1,
тот — уровнем 2, и так далее.

`FractalChain` — контейнер нескольких `AshtiCore`, связанных протоколом **10→0**:

```
MAYA(level 0) ──→ SUTRA(level 1) ──→ MAYA(level 1) ──→ SUTRA(level 2) ...
```

Токен, «выпавший» из MAYA одного уровня, автоматически становится входом
SUTRA следующего уровня.

---

### domain_id схема в цепочке

Каждый уровень получает `level_id`, из которого вычисляются domain_id всех 11 доменов:

```
domain_id = level_id * 100 + structural_role

level 0:  SUTRA=0,  ..., MAYA=10
level 1:  SUTRA=100, ..., MAYA=110
level 2:  SUTRA=200, ..., MAYA=210
```

Это исключает коллизии между уровнями — каждый живёт в своём пространстве ID.

---

### Два новых метода AshtiCore

#### `take_maya_output() -> Option<Token>`

Снимает последний токен с домена MAYA (structural_role = 10).
Синхронизирует `active_tokens` домена, чтобы следующий тик heartbeat видел
актуальное число токенов.

```rust
// MAYA уровня 0 имеет domain_id = 0*100 + 10 = 10
let token = core.take_maya_output(); // None если MAYA пуста
```

#### `set_sutra_input(token) -> Result<usize, CapacityExceeded>`

Впрыскивает токен в SUTRA текущего уровня. Эквивалентно
`inject_token(level_id * 100, token)`, но не требует знать domain_id снаружи.

```rust
// SUTRA уровня 1 имеет domain_id = 1*100 = 100
upper_core.set_sutra_input(token)?;
```

---

### FractalChain API

```rust
pub struct FractalChain { levels: Vec<AshtiCore> }
```

| Метод | Описание |
|-------|----------|
| `new(depth)` | Создать цепочку из `depth` уровней (level_id: 0..depth-1) |
| `depth()` | Число уровней |
| `inject_input(token)` | Впрыснуть токен в SUTRA первого уровня |
| `take_output()` | Забрать токен из MAYA последнего уровня |
| `tick()` | Один тик всех уровней + передача между ними |
| `exchange_skills()` | Обменяться навыками между всеми уровнями |
| `level(i)` / `level_mut(i)` | Прямой доступ к уровню по индексу |

---

### Как работает `tick()`

```
for i in 0..depth:
    levels[i].tick()           ← физика домена
    while token = levels[i].take_maya_output():
        levels[i+1].set_sutra_input(token)  ← передача вниз
```

Важно: тик уровня `i` завершается до того, как его выход передаётся уровню `i+1`.
Это детерминировано — нет гонок, нет асинхронности.

---

### Обмен навыками

```rust
let total_imported = chain.exchange_skills();
```

Алгоритм:
1. Собрать все `Skill` со всех уровней через `export_skills()`
2. Импортировать во все уровни через `import_batch()`

`import_batch` внутри отфильтровывает дубли по `skill_id`, поэтому повторный вызов
`exchange_skills()` безопасен — дубли не накапливаются.

---

### Пример: двухуровневая цепочка

```rust
let mut chain = FractalChain::new(2);

// Входной сигнал → уровень 0
chain.inject_input(token)?;

// Несколько тиков — данные проходят через оба уровня
for _ in 0..10 {
    chain.tick();
}

// Результат из MAYA уровня 1
if let Some(output) = chain.take_output() {
    // output — результат двойной обработки
}

// Синхронизировать навыки между уровнями
chain.exchange_skills();
```

---

## Часть 2 — SIMD batch-физика (Этап 12B)

### Проблема

`compute_gravity(x, y, z, mass, ...)` — скалярная функция для одного токена.
При 5000 токенов на поле = 5000 последовательных вызовов. Физика поля — самое
горячее место в цикле `AshtiCore::tick()`.

### Решение: batch-функция + авто-векторизация

Вместо явных intrinsics (`_mm256_...`) — **чистый Rust цикл** без ветвлений в
горячем пути, который компилятор авто-векторизует при `-C target-cpu=native`.

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

Компилятор видит: один цикл, нет зависимостей между итерациями, нет ветвлений →
раскладывает в AVX2 (8 × f32 за раз) или SSE4.2 (4 × f32).

---

### API

#### `apply_gravity_batch`

```rust
pub fn apply_gravity_batch(
    positions: &[[i16; 3]],  // координаты N токенов
    masses: &[u16],           // массы N токенов
    gravity_scale_shift: u32,
    model: GravityModel,
) -> GravityBatchResult
```

Возвращает `GravityBatchResult { accelerations: Vec<(i16, i16, i16)> }` —
ускорение для каждого токена. **Детерминированно идентично** N вызовам
`compute_gravity()`.

#### `apply_accelerations_to_velocities`

```rust
pub fn apply_accelerations_to_velocities(
    velocities: &mut [[i16; 3]],
    result: &GravityBatchResult,
)
```

In-place применяет ускорения к скоростям (saturating_add).

---

### Пример использования

```rust
use axiom_space::{apply_gravity_batch, apply_accelerations_to_velocities, GravityModel};

// Собрать позиции и массы из DomainState
let positions: Vec<[i16; 3]> = tokens.iter()
    .map(|t| t.position)
    .collect();
let masses: Vec<u16> = tokens.iter()
    .map(|t| t.mass as u16)
    .collect();

// Batch-вычисление
let result = apply_gravity_batch(&positions, &masses, 24, GravityModel::Linear);

// Применить к скоростям
let mut velocities: Vec<[i16; 3]> = tokens.iter()
    .map(|t| t.velocity)
    .collect();
apply_accelerations_to_velocities(&mut velocities, &result);
```

---

### Feature flag `simd`

В `Cargo.toml` потребителя:

```toml
axiom-space = { path = "...", features = ["simd"] }
```

Сам feature сейчас пуст — он служит **явным маркером** того, что потребитель
знает о batch-режиме и собирается компилировать с `-C target-cpu=native`.
В будущем сюда можно добавить явные intrinsics для платформ без авто-векторизатора.

---

### Детерминизм

`apply_gravity_batch` гарантированно возвращает тот же результат, что N вызовов
`compute_gravity`. Тест `test_batch_matches_scalar` проверяет это поэлементно.

Авто-векторизация не меняет результат для целочисленной арифметики (i16, i64):
нет floating point ассоциативности, нет переупорядочивания. Детерминизм сохраняется.

---

## Граница ответственности

| Компонент | Что делает |
|-----------|-----------|
| `AshtiCore::tick()` | Физика каждого домена по одному |
| `FractalChain::tick()` | Тик N уровней + MAYA→SUTRA transfer |
| `apply_gravity_batch` | Batch-ускорение для N токенов одного домена |

`apply_gravity_batch` не вызывается автоматически внутри `tick()` — это
инструмент для пользователя, который хочет вручную управлять физикой поля
в горячем цикле (например, в бенчмарках или специализированных симуляциях).
