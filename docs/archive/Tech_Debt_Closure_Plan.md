# Axiom Technical Debt Closure — Детальный план

**Версия:** 1.0  
**Дата:** 2026-04-10  
**Для:** Claude Sonnet (исполнитель)  
**Контекст:** 900 тестов, CLI Channel реализован, Memory Persistence V1.0 специфицирован. Закрываем технический долг перед переходом к persistence.

---

## Порядок выполнения

| Шаг | Что | Crates | Сложность | Зависимости |
|-----|-----|--------|-----------|-------------|
| 1 | ~~D-05: data_dir дублирование~~ | axiom-agent | малая | ✅ Сделано |
| 2 | D-02: Event._pad → event_subtype | axiom-core | малая | — |
| 3 | D-03: Token.reserved_phys → origin | axiom-core | малая | — |
| 4 | D-01: domain_id u16 vs u32 | весь стек | большая | Шаги 2-3 |

**Принцип:** `cargo test --workspace` зелёный после каждого шага. Каждый шаг — отдельный коммит.

---

## Шаг 2: D-02 — Event._pad → event_subtype: u16

### Что делаем

Переименовать анонимный padding в семантическое поле. Миграция бесплатная: дефолт = 0, сигнатуры не меняются.

### Изменения в коде

**Файл:** `crates/axiom-core/src/event.rs`

1. Переименовать поле:
```rust
// Было:
pub _pad: u16,

// Стало:
pub event_subtype: u16,
```

2. Добавить константы подтипов (в том же файле или в отдельном `event_subtypes.rs`):

```rust
// === Event subtypes ===
// Подтипы для event_type = TokenMove (0x0004)
pub const SUBTYPE_NONE: u16 = 0;          // Не указан (обратная совместимость)
pub const SUBTYPE_GRAVITY: u16 = 1;       // Движение от гравитации
pub const SUBTYPE_MANUAL: u16 = 2;        // Ручное перемещение (ApplyForce)
pub const SUBTYPE_COLLISION: u16 = 3;     // Отскок от столкновения
pub const SUBTYPE_IMPULSE: u16 = 4;       // Внутренний импульс (Cognitive Depth)
pub const SUBTYPE_INERTIA: u16 = 5;       // Инерционное движение
pub const SUBTYPE_ATTRACTOR: u16 = 6;     // Движение к target (аттрактор)

// Подтипы для event_type = InternalImpulse (0x4001)
pub const SUBTYPE_TENSION: u16 = 1;       // Tension trace
pub const SUBTYPE_CURIOSITY: u16 = 2;     // DREAM curiosity
pub const SUBTYPE_GOAL: u16 = 3;          // Goal persistence
pub const SUBTYPE_INCOMPLETION: u16 = 4;  // Incompletion trace

// Подтипы для event_type = ConnectionCreate (0x1001)
pub const SUBTYPE_RESONANCE: u16 = 1;     // Создана резонансом
pub const SUBTYPE_COLLISION_LINK: u16 = 2; // Создана столкновением
pub const SUBTYPE_IMPORTED: u16 = 3;      // Импортирована (persistence/exchange)

// Подтипы для SystemCheckpoint (0xF001)
pub const SUBTYPE_MANUAL_SAVE: u16 = 1;   // Ручное сохранение (:save)
pub const SUBTYPE_AUTO_SAVE: u16 = 2;     // Автосохранение (кристаллизация)
pub const SUBTYPE_SHUTDOWN_SAVE: u16 = 3; // Сохранение при :quit
```

3. Обновить конструкторы — **НЕ менять сигнатуру**:

```rust
impl Event {
    pub fn new(event_id: u64, domain_id: u16, event_type: EventType) -> Self {
        Self {
            // ... существующие поля ...
            event_subtype: SUBTYPE_NONE,  // Было: _pad: 0
            // ... остальное ...
        }
    }
}
```

4. Найти все места где `_pad` упоминается:
```bash
grep -rn "_pad" crates/axiom-core/src/event.rs
```
Заменить `_pad` → `event_subtype` во всех инициализациях.

5. **Опционально:** В EventGenerator (axiom-domain), где генерируются конкретные события, начать проставлять subtype. Например в `generate_gravity_update()`:
```rust
event.event_subtype = SUBTYPE_GRAVITY;
```

Это не обязательно для MVP — можно делать постепенно. Все существующие события будут иметь `event_subtype: 0` (SUBTYPE_NONE) что корректно.

### Обновить спецификацию

В `docs/spec/` — обновить документ COM (если есть COM V1.1.md) или Event-Driven V1.md. Добавить раздел "Event Subtypes" с таблицей выше.

### Тесты

- [ ] `size_of::<Event>() == 64` — compile-time assert (не изменился)
- [ ] `Event::new()` → `event_subtype == 0`
- [ ] Создать Event с subtype → прочитать → значение сохранилось
- [ ] Все существующие тесты проходят без изменений

### Бенчмарки

Не нужны — поле уже было в структуре как padding, размер не изменился.

---

## Шаг 3: D-03 — Token.reserved_phys → origin: u16

### Что делаем

Переименовать физический резерв в семантическое поле. Миграция бесплатная: дефолт = 0.

### Изменения в коде

**Файл:** `crates/axiom-core/src/token.rs`

1. Переименовать поле:
```rust
// Было:
pub reserved_phys: u16,

// Стало:
pub origin: u16,
```

2. Добавить константы:

```rust
// === Token origin ===
// Кодирует откуда пришёл токен

/// Рождён в текущем уровне (создан в SUTRA)
pub const TOKEN_ORIGIN_LOCAL: u16 = 0x0000;

// 0x0001..=0x00FF — пришёл с уровня N FractalChain
// Пример: origin = 3 означает "пришёл с уровня 3 через MAYA[3] → SUTRA[4]"

/// Восстановлен из persistence (загружен с диска)
pub const TOKEN_ORIGIN_PERSISTED: u16 = 0xFE00;

/// Импортирован из другого экземпляра AXIOM (будущее)
pub const TOKEN_ORIGIN_EXTERNAL_BASE: u16 = 0xFF00;
// 0xFF00..=0xFFFF — зарезервировано для внешних источников
// Конкретный source_id кодируется в младших 8 битах: 0xFF00 | source_id
```

3. Обновить `Token::new()`:
```rust
impl Token {
    pub fn new(/* существующие параметры */) -> Self {
        Self {
            // ... существующие поля ...
            origin: TOKEN_ORIGIN_LOCAL,  // Было: reserved_phys: 0
            // ...
        }
    }
}
```

4. Найти все места где `reserved_phys` упоминается:
```bash
grep -rn "reserved_phys" crates/
```
Заменить → `origin` (или `origin: 0` / `origin: TOKEN_ORIGIN_LOCAL`).

### Интеграция с FractalChain

**Файл:** `crates/axiom-runtime/src/fractal.rs` (или где живёт FractalChain)

При переходе MAYA[n] → SUTRA[n+1]:

```rust
// Когда токен переходит с уровня n на уровень n+1
fn transfer_to_next_level(token: &mut Token, source_level: u16) {
    token.origin = source_level;  // Пришёл с уровня N
    // ... остальная логика переноса ...
}
```

Найти место где FractalChain передаёт выход MAYA одного уровня во вход SUTRA следующего. Добавить `token.origin = level_id` перед инъекцией.

### Интеграция с Persistence (будущее)

При загрузке токенов из файла (Memory Persistence, Фаза 2):

```rust
fn load_tokens(path: &Path) -> Vec<Token> {
    let mut tokens = deserialize_tokens(path);
    for token in &mut tokens {
        token.origin = TOKEN_ORIGIN_PERSISTED;
    }
    tokens
}
```

Это будет реализовано в axiom-persist. Сейчас только добавить константу и подготовить поле.

### Обновить спецификацию

В `docs/spec/Token V5.2.md`:
- Переименовать `reserved_phys` → `origin: u16` в разделе 2 (структура)
- Добавить раздел 3.5 "Origin" с описанием кодировки
- Обновить раздел 4 "Инварианты" — добавить: `origin = 0 для локально созданных токенов`

**Версию обновить до V5.3** (или оставить V5.2 с пометкой "origin added").

### Тесты

- [ ] `size_of::<Token>() == 64` — compile-time assert (не изменился)
- [ ] `Token::new()` → `origin == TOKEN_ORIGIN_LOCAL` (== 0)
- [ ] FractalChain: токен переходящий уровень → `origin == source_level`
- [ ] Все существующие тесты проходят

---

## Шаг 4: D-01 — domain_id: u16 vs u32 унификация

### Что делаем

Унифицировать тип `domain_id` в `u16` по всему стеку. Это не меняет layout структур (Token/Connection уже u16), только сигнатуры методов Engine/AshtiCore.

### Масштаб изменений

Это **самое большое изменение** в техдолге. Затрагивает сигнатуры публичных методов → каскад изменений в тестах и бенчмарках.

### Стратегия

**Поиск всех мест:**
```bash
grep -rn "domain_id: u32" crates/
grep -rn "as u32" crates/ | grep -i "domain"
```

**Изменение сверху вниз:**

1. **axiom-runtime** (Engine API — точка входа):

```rust
// Было:
impl AxiomEngine {
    pub fn token_count(&self, domain_id: u32) -> usize { ... }
    pub fn spawn_domain(&mut self, domain_id: u32) -> Result<...> { ... }
}

// Стало:
impl AxiomEngine {
    pub fn token_count(&self, domain_id: u16) -> usize { ... }
    pub fn spawn_domain(&mut self, domain_id: u16) -> Result<...> { ... }
}
```

2. **axiom-runtime** (AshtiCore):

```rust
// Было:
impl AshtiCore {
    pub fn inject_token(&mut self, domain_id: u32, token: Token) -> ... { ... }
    pub fn index_of(&self, domain_id: u32) -> Option<usize> { ... }
    pub fn config_of(&self, domain_id: u32) -> Option<&DomainConfig> { ... }
}

// Стало:
impl AshtiCore {
    pub fn inject_token(&mut self, domain_id: u16, token: Token) -> ... { ... }
    pub fn index_of(&self, domain_id: u16) -> Option<usize> { ... }
    pub fn config_of(&self, domain_id: u16) -> Option<&DomainConfig> { ... }
}
```

3. **Убрать все касты:**
```rust
// Убрать:
let domain_id = token.domain_id as u32;
self.inject_token(token.domain_id as u32, token);

// Заменить на:
let domain_id = token.domain_id;
self.inject_token(token.domain_id, token);
```

4. **axiom-agent** — обновить вызовы:

Найти все места где agent вызывает Engine API с `as u32` и убрать каст. TextPerceptor уже использует `u16` для domain_id (SUTRA = 100u16).

5. **Бенчмарки** — обновить вызовы в bench файлах:

```bash
grep -rn "domain_id.*u32\|as u32" benches/
```

### Внутренние HashMap

Проверить: если Engine/AshtiCore использует `HashMap<u32, ...>` для доменов — заменить на `HashMap<u16, ...>`.

Если используется Vec с индексом — `index_of()` возвращает `usize`, это не затронуто.

### Чеклист

- [ ] `grep -rn "domain_id: u32" crates/` → 0 результатов после рефакторинга
- [ ] `grep -rn "as u32" crates/ | grep -i domain` → 0 результатов (кроме внутренних вычислений где u32 нужен для арифметики)
- [ ] `cargo test --workspace` — все 900+ тестов зелёные
- [ ] `cargo bench` — компилируется (бенчмарки тоже обновлены)
- [ ] Обновить DEFERRED.md — пометить D-01 закрытым

### Что НЕ менять

- `Token.domain_id: u16` — уже u16, не трогать
- `Connection.domain_id: u16` — уже u16
- `DomainConfig.domain_id: u16` — уже u16
- `Event.domain_id: u16` — уже u16
- Внутренняя арифметика где u32 нужен для промежуточных вычислений (например `domain_id as u32 * 100` — если такое есть, каст допустим ВНУТРИ функции)

---

## После завершения

Когда все 4 шага выполнены:

1. **DEFERRED.md** — обновить: D-01, D-02, D-03 помечены ✅ с датой
2. **Спецификации** — обновлены: COM V1.1 (event_subtype), Token V5.2→V5.3 (origin)
3. **900+ тестов** зелёные
4. Технический долг по структурным несоответствиям = **закрыт**

Следующий этап: **Memory Persistence V1.0 Фаза 1** (`:save` + `:load`).

---

## Резюме решений (для быстрой справки)

| DEFERRED ID | Поле | Решение | Почему |
|---|---|---|---|
| D-02 | Event._pad | `event_subtype: u16` | domain_id уже = target; subtype классифицирует причину события |
| D-03 | Token.reserved_phys | `origin: u16` | Шире чем layer_id: покрывает FractalChain + persistence + multi-instance |
| D-01 | domain_id типы | Унифицировать в u16 | Структуры уже u16, только API методы используют u32 |
