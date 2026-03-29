# Gateway, Channel, EventBus — гайд-разъяснение

**Версия:** 1.0
**Дата:** 2026-03-29
**Этапы:** 8 (Gateway + Channel), 9 (EventBus)
**Файлы:**
- [`crates/axiom-runtime/src/gateway.rs`](../../crates/axiom-runtime/src/gateway.rs)
- [`crates/axiom-runtime/src/adapters.rs`](../../crates/axiom-runtime/src/adapters.rs)

---

## Зачем это нужно

`AxiomEngine` — чистое ядро без ввода-вывода. Снаружи к нему нельзя обратиться напрямую:
нет HTTP, нет очередей, нет коллбэков. Это намеренно.

**Gateway** — единственная точка входа. Принимает `UclCommand`, возвращает `UclResult`.
**Channel** — in-process очередь: накапливает команды, сбрасывает батчем.
**EventBus** — pub/sub поверх событий: подписки по типу события или на всё.

---

## Gateway

```rust
pub struct Gateway {
    engine: AxiomEngine,
    bus: EventBus,
}
```

### Создание

```rust
use axiom_runtime::Gateway;

let mut gw = Gateway::with_default_engine();
```

### Методы

| Метод | Описание |
|-------|----------|
| `process(cmd)` | Одна команда → UclResult |
| `process_with(cmd, observer)` | Команда + временный наблюдатель |
| `process_channel(channel)` | Сбросить весь Channel за раз |
| `drain_and_notify(events)` | Разослать события всем подписчикам |
| `set_config_watcher(watcher)` | Горячая перезагрузка конфигов |
| `check_config_reload()` | Опросить watcher на изменения |
| `register_observer(obs)` | Псевдоним → `bus.subscribe_all()` |

### Пример: один запрос

```rust
use axiom_runtime::Gateway;
use axiom_ucl::{UclCommand, OpCode};

let mut gw = Gateway::with_default_engine();
let cmd = UclCommand::new(OpCode::TickForward, 0, 0, 0);
let result = gw.process(cmd);

assert_eq!(result.error_code, 0); // 0 = OK
```

### Пример: батч через Channel

```rust
use axiom_runtime::{Gateway, Channel};
use axiom_ucl::{UclCommand, OpCode};

let mut gw = Gateway::with_default_engine();
let mut ch = Channel::new();

// Накопить команды
for _ in 0..10 {
    ch.send(UclCommand::new(OpCode::TickForward, 0, 0, 0));
}
ch.send(UclCommand::new(OpCode::InjectToken, 106, 100, 0));

// Сбросить всё разом
gw.process_channel(&mut ch);

// Channel теперь пуст
assert_eq!(ch.len(), 0);
assert_eq!(ch.processed_count(), 11);
```

---

## Channel

```rust
pub struct Channel {
    commands: VecDeque<UclCommand>,
    events:   VecDeque<Event>,
}
```

Channel — двусторонний: команды идут **внутрь** движка, события выходят **наружу**.

### Методы команд

| Метод | Описание |
|-------|----------|
| `send(cmd)` | Добавить команду в очередь |
| `drain_commands()` | Забрать все команды (очередь очищается) |
| `len()` | Число команд |
| `processed_count()` | Кумулятивный счётчик обработанных |

### Методы событий

| Метод | Описание |
|-------|----------|
| `push_event(event)` | Добавить событие (движок → наружу) |
| `drain_events()` | Забрать все события |
| `clear()` | Очистить и команды, и события |

### Типичный цикл

```rust
let mut ch = Channel::new();

// Внешняя система: заполнить команды
ch.send(cmd1);
ch.send(cmd2);

// Движок: обработать
gw.process_channel(&mut ch);

// Внешняя система: забрать события
for event in ch.drain_events() {
    println!("event: {:?}", event.event_type);
}
```

---

## EventBus

EventBus — pub/sub поверх `EventObserver`. Живёт внутри Gateway.

```rust
pub struct EventBus {
    typed:     HashMap<u16, Vec<Box<dyn EventObserver>>>,
    broadcast: Vec<Box<dyn EventObserver>>,
}
```

### Подписка

```rust
use axiom_runtime::adapters::EventObserver;
use axiom_core::Event;

struct MyObserver;
impl EventObserver for MyObserver {
    fn on_event(&self, event: &Event) {
        println!("got event type={:#x}", event.event_type);
    }
}

// Подписка на все события
gw.register_observer(Box::new(MyObserver));

// Подписка на конкретный тип
use axiom_core::EventType;
gw.bus.subscribe(EventType::GravityUpdate as u16, Box::new(MyObserver));
```

### Публикация

`gw.drain_and_notify(events)` — принять Vec<Event> и разослать всем подписчикам.

Движок сам не публикует — это ответственность внешнего кода:
```rust
let result = gw.process(cmd);
// события лежат в Channel или возвращаются другим способом
gw.drain_and_notify(&collected_events);
```

### Счётчики

```rust
gw.bus.broadcast_count()           // число broadcast-подписчиков
gw.bus.typed_count(event_type)     // число подписчиков на тип
gw.bus.total_count()               // всего подписчиков
gw.bus.is_empty()                  // нет подписчиков вообще
```

---

## Config hot reload (ConfigWatcher)

```rust
use axiom_config::ConfigWatcher;

let watcher = ConfigWatcher::new("config/axiom.yaml")?;
gw.set_config_watcher(watcher);

// В игровом цикле
if let Some(new_cfg) = gw.check_config_reload() {
    // конфиг изменился — применить
    println!("config reloaded: {} domains", new_cfg.domains.len());
}
```

`check_config_reload()` неблокирующий — опрашивает inotify через канал.
Если файл не менялся — возвращает `None` за ~100 ns.

---

## Диаграмма потока данных

```
Внешняя система
      │
      │  UclCommand
      ▼
  [ Channel ]  ──send──▶  VecDeque<UclCommand>
      │
      │  process_channel()
      ▼
  [ Gateway ]
      │
      │  process_command()
      ▼
  [ AxiomEngine ]
      │  (Genome + AshtiCore + Guardian)
      │
      │  UclResult + Vec<Event>
      ▼
  [ EventBus ]  ──publish──▶  Observer 1, Observer 2, ...
      │
      │  push_event()
      ▼
  [ Channel ]  ──drain_events()──▶  Внешняя система
```

---

## Производительность (справочно)

| Операция | Время |
|----------|-------|
| `Gateway::process` (TickForward) | ~20 µs |
| `process_channel` (1 команда) | ~20 µs |
| `process_channel` (50 команд) | ~25 µs |
| `check_config_reload` (без изменений) | ~100 ns |

Overhead Gateway vs прямой `process_command` — в пределах шума (<1 µs).
Батч из 50 команд через Channel — ~0.1 µs/команду сверх первой.
