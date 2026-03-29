# Agent Layer — гайд-разъяснение

**Версия:** 1.0
**Дата:** 2026-03-29
**Этапы:** 10 (Perceptor/Effector + каналы), 11 (ML Inference)
**Крейт:** [`crates/axiom-agent/`](../../crates/axiom-agent/)

---

## Концепция

AXIOM — замкнутая система. Снаружи она ничего не знает: нет stdin, нет HTTP, нет камеры.
`axiom-agent` — слой адаптеров, который переводит внешние сигналы в `UclCommand`
и выводит события движка во внешний мир.

```
Внешний мир  ──▶  Perceptor  ──▶  UclCommand  ──▶  Gateway  ──▶  AxiomEngine
AxiomEngine  ──▶  Event      ──▶  Effector    ──▶  Внешний мир
```

---

## Трейты Perceptor и Effector

```rust
pub trait Perceptor: Send {
    fn receive(&mut self) -> Option<UclCommand>;  // опрос — None если пусто
    fn name(&self) -> &str;
}

pub trait Effector: Send {
    fn emit(&mut self, event: &Event);
    fn emit_result(&mut self, result: &UclResult);
    fn name(&self) -> &str;
}
```

Оба трейта `Send` — совместимы с потоками. Polling-based (`receive` не блокирует).

---

## CLI-канал

Чтение команд из любого `Read` (stdin, файл, тест-буфер).

```rust
use axiom_agent::channels::cli::{CliPerceptor, CliEffector, parse_cli_command};

// Из stdin
let mut perc = CliPerceptor::from_reader(std::io::stdin());

// Из буфера (тесты)
let input = b"tick\ninject 106\nstatus\nquit\n";
let mut perc = CliPerceptor::from_reader(&input[..]);

while let Some(cmd) = perc.receive() {
    let result = gw.process(cmd);
    effector.emit_result(&result.into());
}
```

### Синтаксис команд

| Ввод | UclCommand |
|------|-----------|
| `tick` | `TickForward` |
| `status` | `TickForward` |
| `inject <id>` | `InjectToken(id)` |
| `inject` | `InjectToken(100)` — default SUTRA |
| `quit` / `exit` | → `None`, цикл завершается |

### CliEffector

```rust
let mut eff = CliEffector::from_writer(std::io::stdout());
eff.emit(&event);          // → "event type=0x1001 target=106"
eff.emit_result(&result);  // → "result code=0"
```

---

## Telegram-канал

HTTP polling или тесты без сети через `feed_update`.

### TelegramPerceptor

```rust
use axiom_agent::channels::telegram::{TelegramConfig, TelegramPerceptor, TelegramUpdate};

let config = TelegramConfig {
    token: "BOT_TOKEN".into(),
    chat_id: 123456789,
};
let mut perc = TelegramPerceptor::new(config);

// Реальный polling (reqwest::blocking)
perc.poll_blocking()?;

// Тестовый путь — без HTTP
perc.feed_update(TelegramUpdate {
    update_id: 1,
    message: Some(axiom_agent::channels::telegram::TelegramMessage {
        text: Some("tick".into()),
        ..Default::default()
    }),
});

while let Some(cmd) = perc.receive() {
    gw.process(cmd);
}
```

### parse_updates — чистая функция

```rust
use axiom_agent::channels::telegram::parse_updates;

let updates = parse_updates(r#"{"ok":true,"result":[{"update_id":1,"message":{"text":"/tick"}}]}"#);
```

Синтаксис команд — тот же, что у CLI.

### TelegramEffector

```rust
// Mock — захватывает сообщения в памяти (для тестов)
let mut eff = TelegramEffector::mock(config.clone());
eff.emit_result(&result);
assert_eq!(eff.sent_messages.len(), 1);

// Real — отправляет через Telegram Bot API
let mut eff = TelegramEffector::new(config);
```

---

## Shell-канал

Управляемое исполнение shell-команд по whitelist.

```rust
use axiom_agent::channels::shell::ShellEffector;

// Из кода
let mut eff = ShellEffector::new(vec![
    "echo ok".into(),
    "ls /tmp".into(),
]);

// Из YAML-файла
let mut eff = ShellEffector::from_whitelist_file(Path::new("config/shell_whitelist.yaml"))?;
```

### Принцип безопасности

Только **точное совпадение** — никаких glob, никакого partial match:

```rust
eff.is_allowed("echo ok")        // → true
eff.is_allowed("echo ok; rm -rf /")  // → false (разные строки)
eff.is_allowed("echo")           // → false
```

```rust
eff.execute_command("echo ok");  // true — выполнено
eff.execute_command("rm -rf /"); // false — заблокировано

println!("{:?}", eff.executed); // ["echo ok"]
println!("{:?}", eff.denied);   // ["rm -rf /"]
```

### YAML-whitelist

```yaml
# config/shell_whitelist.yaml
whitelist:
  - "echo ok"
  - "systemctl reload nginx"
  - "pg_dump --schema-only mydb"
```

---

## AudioPerceptor — VAD (Voice Activity Detection)

Обнаружение речи в аудиопотоке по энергетическому порогу.

```rust
use axiom_agent::channels::audio::AudioPerceptor;

let mut perc = AudioPerceptor::new()
    .with_domain(100)      // SUTRA первого уровня
    .with_threshold(0.02); // порог RMS (0.0–1.0)

// Из WAV-файла
let n = perc.process_wav(Path::new("audio.wav"))?;

// Из сырых сэмплов (тесты, стриминг)
perc.feed_samples(&samples);

while let Some(cmd) = perc.receive() {
    gw.process(cmd); // InjectToken в SUTRA при обнаружении речи
}
```

### Детали VAD

- Фрейм = 512 сэмплов (~32 мс при 16 kHz)
- `frame_rms(samples)` — энергия фрейма, нормализованная по `i16::MAX`
- Если `rms >= threshold` — это речь → `InjectToken`
- `priority` = `(rms * 255) as u8` — интенсивность кодируется в приоритет

---

## VisionPerceptor — детекция объектов

ONNX-модель → объекты → токены в MAP(104) или LOGIC(106).

```rust
use axiom_agent::channels::vision::VisionPerceptor;
use axiom_agent::ml::engine::MLEngine;

// Mock-режим (без ONNX)
let engine = MLEngine::mock(
    vec![3 * 224 * 224],
    vec![1.0, 0.85, 0.1, 0.1, 0.5, 0.5], // [class, conf, x, y, w, h]
);
let mut perc = VisionPerceptor::new(engine)
    .with_domain(104)       // MAP domain
    .with_threshold(0.5);   // порог уверенности

// Из файла изображения (image crate: PNG, JPEG, ...)
perc.process_image(Path::new("frame.jpg"))?;

// Из сырых детекций (тесты)
perc.feed_detections(vec![detection]);

while let Some(cmd) = perc.receive() {
    gw.process(cmd);
}
```

### Формат выхода ONNX

Каждый объект = 6 float: `[class_id, confidence, x, y, w, h]`

```
output[0..6]   → объект 1
output[6..12]  → объект 2
...
```

Объекты с `confidence < threshold` отфильтровываются.

Токен из детекции:
- `domain_id` = заданный (104 по умолчанию)
- `temperature` = `(confidence * 255) as u8`
- `mass` = `class_id as u8`

---

## Полный пример: CLI + Shell агент

```rust
use axiom_runtime::{Gateway, Channel};
use axiom_agent::channels::cli::{CliPerceptor, CliEffector};
use axiom_agent::channels::shell::ShellEffector;

let mut gw = Gateway::with_default_engine();
let mut ch = Channel::new();

let mut perceptor = CliPerceptor::from_reader(std::io::stdin());
let mut cli_eff   = CliEffector::from_writer(std::io::stdout());
let mut shell_eff = ShellEffector::new(vec!["echo status".into()]);

loop {
    // Получить команду
    let Some(cmd) = perceptor.receive() else { break };

    // Обработать
    ch.send(cmd);
    gw.process_channel(&mut ch);

    // Результаты → CLI
    for event in ch.drain_events() {
        cli_eff.emit(&event);
        shell_eff.emit(&event);
    }
}
```

---

## Guardian ML-фильтры

Перед тем как токен из ML попадёт в движок, Guardian его проверяет:

```rust
use axiom_runtime::Guardian;

// Принимает только [0.5, 0.99] — отсекает мусор и adversarial примеры
Guardian::validate_ml_confidence(0.85, 0.5)  // → true
Guardian::validate_ml_confidence(0.3, 0.5)   // → false (низкая уверенность)
Guardian::validate_ml_confidence(1.0, 0.5)   // → false (аномально высокая)

// Весь вектор выхода модели
Guardian::validate_ml_output(&output, 0.5)   // → true если все значения в диапазоне
```

Это делается **до** `gw.process(cmd)` при работе с VisionPerceptor/AudioPerceptor.
