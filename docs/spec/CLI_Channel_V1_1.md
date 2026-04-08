# CLI Channel V1.1 — Спецификация и План реализации

**Версия:** 1.1 (ревизия после обратной связи исполнителя)  
**Дата:** 2026-04-05  
**Назначение:** Первый живой интерфейс к ядру AXIOM  
**Для:** Claude Sonnet (исполнитель)  
**Контекст:** Все этапы роадмапа реализованы. Cognitive Depth реализован. COM V1.1 (Event 64B) готов. Tick Scheduling работает. 813 тестов. Бенчмарки v6 подтверждают стабильность. Это первый выход за архитектурную границу ядра.  
**Связанные спеки:** External Integration Layer V1.0, UCL V2.0, API В AXIOM, Cognitive Depth V1.0

---

## Изменения V1.1 vs V1.0

- **Исправлено:** domain_id в ядре = `level_id * 100 + offset` (SUTRA=100, MAYA=110), не 0-10.
- **Добавлено:** Явное описание сборки InjectToken payload — сверяться с `parse_inject_token_payload()`.
- **Исправлено:** `AxiomEngine::new()` (не `try_new(genome)`) — использовать существующий API.
- **Уточнено:** Фаза 1 — основная работа: `last_routing` в Arbiter, `last_coherence` в MAYA.
- **Исправлено:** `[[bin]]` объявляется в Cargo.toml крейта axiom-agent.
- **Уточнено:** Парсинг CLI-аргументов через `std::env::args()`, без clap.
- **Уточнено:** Типы полей Token сверять с текущим кодом, не со спекой (спека может расходиться).

---

## 1. Что это и зачем

CLI Channel — тонкий адаптер между терминалом (stdin/stdout) и ядром AXIOM. Пользователь вводит текст → ядро обрабатывает его как паттерн → результат выводится в терминал.

Это **НЕ** чат-бот и **НЕ** интерпретатор естественного языка. Это диагностическое окно в живую когнитивную систему. Ты видишь:
- Как ядро превратило текст в токен
- Какой домен ответил
- Нашёлся ли рефлекс в EXPERIENCE
- Как работает Cognitive Depth (tension, multi-pass)
- Что система делает когда ты молчишь (internal impulses)

**Архитектурная граница:**
```
stdin → [CLI Channel] → UclCommand(64B) → [Ядро] → UclResult(32B) → [CLI Channel] → stdout
```
Ядро не знает что существует терминал. Ядро не импортирует tokio. Всё async живёт только в axiom-agent.

---

## 2. Компоненты

### 2.1 TextPerceptor — текст → токен

Превращает строку UTF-8 в `UclCommand(InjectToken)` с осмысленным Token.

**MVP-алгоритм кодирования (без ML):**

```rust
pub struct TextPerceptor;

impl TextPerceptor {
    pub fn perceive(&self, text: &str) -> UclCommand {
        let bytes = text.as_bytes();
        let len = bytes.len();

        // Position: хэш текста → 3D координаты
        let hash = fnv1a_hash(bytes);
        let x = ((hash >> 0) & 0xFFFF) as i16;
        let y = ((hash >> 16) & 0xFFFF) as i16;
        let z = ((hash >> 32) & 0xFFFF) as i16;

        // Shell-профиль L1-L8
        let shell: [u8; 8] = [
            0,                              // L1 physical — 0 (текст не физический)
            0,                              // L2 sensory — 0
            0,                              // L3 motor — 0
            estimate_emotion(text),         // L4 emotional
            200,                            // L5 cognitive — высокий
            180,                            // L6 social — высокий (коммуникация)
            estimate_temporal(text),        // L7 temporal
            estimate_abstraction(text),     // L8 abstract
        ];

        // Temperature, mass, valence — СВЕРИТЬ ТИПЫ С ТЕКУЩИМ Token в axiom-core!
        // В коде Token может использовать u8, i8, u16, i16 — не угадывать, проверить.
        let temperature = 200;  // Высокая пластичность (новый ввод)
        let mass = (50 + len.min(200)) as u16;
        let valence: i8 = 0;   // Нейтральный

        // === КРИТИЧЕСКИ ВАЖНО ===
        // Сборка payload: посмотреть parse_inject_token_payload() в engine.rs
        // и собрать payload ЗЕРКАЛЬНО тому как он парсится.
        // НЕ угадывать формат — читать код парсера.
        build_inject_token_command(
            100,  // target_domain_id = SUTRA(100), НЕ 0
            x, y, z,
            shell,
            temperature,
            mass,
            valence,
        )
    }
}
```

**build_inject_token_command() — ЗЕРКАЛО парсера:**

```rust
/// Собрать UclCommand(InjectToken) так чтобы parse_inject_token_payload() 
/// корректно прочитал все поля.
///
/// ПЕРЕД РЕАЛИЗАЦИЕЙ: открыть engine.rs, найти parse_inject_token_payload(),
/// и скопировать layout полей. Эта функция должна писать байты 
/// в ТОЧНОСТИ в том порядке и формате как парсер их читает.
fn build_inject_token_command(
    target_domain_id: u16,
    x: i16, y: i16, z: i16,
    shell: [u8; 8],
    temperature: /* тип из Token */,
    mass: /* тип из Token */,
    valence: /* тип из Token */,
) -> UclCommand {
    let mut cmd = UclCommand::default();
    cmd.opcode = OpCode::InjectToken as u16;
    cmd.target_id = target_domain_id as u32;

    // payload[0..48] — заполнить ЗЕРКАЛЬНО parse_inject_token_payload()
    // Пример (ПРОВЕРИТЬ по коду):
    // payload[0..2] = x.to_le_bytes()
    // payload[2..4] = y.to_le_bytes()
    // payload[4..6] = z.to_le_bytes()
    // payload[6..14] = shell
    // payload[14..N] = temperature, mass, valence, ...
    //
    // Единственный источник истины — parse_inject_token_payload() в engine.rs.
    
    todo!("Реализовать после проверки парсера InjectToken в engine.rs")
}
```

**Эвристики для Shell-профиля (MVP, без ML):**

```rust
/// L4 emotional: восклицательные знаки, капс, вопросы
fn estimate_emotion(text: &str) -> u8 {
    let mut score: u8 = 50;
    if text.contains('!') { score = score.saturating_add(50); }
    if text.contains('?') { score = score.saturating_add(30); }
    let upper_count = text.chars().filter(|c| c.is_uppercase()).count();
    if upper_count > text.len() / 2 && text.len() > 3 {
        score = score.saturating_add(80);
    }
    score
}

/// L7 temporal: слова связанные со временем
fn estimate_temporal(text: &str) -> u8 {
    let lower = text.to_lowercase();
    let markers = ["когда", "сейчас", "потом", "вчера", "завтра", "скоро",
                    "when", "now", "then", "yesterday", "tomorrow", "soon"];
    if markers.iter().any(|m| lower.contains(m)) { 180 } else { 30 }
}

/// L8 abstraction: средняя длина слов
fn estimate_abstraction(text: &str) -> u8 {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() { return 30; }
    let avg_len = words.iter().map(|w| w.len()).sum::<usize>() / words.len();
    (avg_len.min(15) as u8 * 15).min(255)
}

/// FNV-1a hash (детерминированный)
fn fnv1a_hash(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
```

### 2.2 MessageEffector — результат → текст

Форматирует ProcessingResult в диагностический вывод.

```rust
pub struct MessageEffector;

impl MessageEffector {
    pub fn format_result(&self, result: &ProcessingResult) -> String {
        let mut out = String::new();

        out.push_str(&format!("  path: {}\n", match result.path {
            ProcessingPath::Reflex => "⚡ reflex",
            ProcessingPath::SlowPath => "🧠 slow path",
            ProcessingPath::MultiPass(n) => return format!("  path: 🔄 multi-pass ({})\n{}", n, self.format_rest(result)),
        }));

        out.push_str(&self.format_rest(result));
        out
    }

    fn format_rest(&self, result: &ProcessingResult) -> String {
        let mut out = String::new();

        out.push_str(&format!("  dominant: domain {} ({})\n",
            result.dominant_domain_id,
            domain_name(result.dominant_domain_id)));

        if let Some(c) = result.coherence_score {
            out.push_str(&format!("  coherence: {:.2}\n", c));
        }

        if result.tension_count > 0 {
            out.push_str(&format!("  tension: {} active\n", result.tension_count));
        }

        if result.traces_matched > 0 {
            out.push_str(&format!("  traces matched: {}\n", result.traces_matched));
        }

        out.push_str(&format!("  shell: {:?}\n", result.output_shell));
        out.push_str(&format!("  position: ({}, {}, {})\n",
            result.output_position[0],
            result.output_position[1],
            result.output_position[2]));

        out
    }
}

/// Маппинг domain_id → имя.
/// В ядре domain_id = level_id * 100 + offset.
/// Уровень 1: SUTRA=100, EXECUTION=101, ..., MAYA=110.
fn domain_name(id: u16) -> &'static str {
    let offset = id % 100;
    match offset {
        0 => "SUTRA",
        1 => "EXECUTION",
        2 => "SHADOW",
        3 => "CODEX",
        4 => "MAP",
        5 => "PROBE",
        6 => "LOGIC",
        7 => "DREAM",
        8 => "ETHICS",
        9 => "EXPERIENCE",
        10 => "MAYA",
        _ => "UNKNOWN",
    }
}
```

### 2.3 CliChannel — stdin/stdout цикл

**Архитектура потоков:**

```
┌──────────────┐
│  stdin task   │ ← tokio::io::BufReader(stdin).read_line()
│  (async)      │ → mpsc::Sender<String>
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  tick loop    │ ← tokio::time::interval(1000/tick_hz ms)
│  (async)      │ 1. try_recv() — есть ввод?
│               │    → ':' prefix? → handle_meta_command()
│               │    → иначе: TextPerceptor → process_and_observe() → print
│               │ 2. engine.process_command(&tick_cmd) — ядро тикает
│               │ 3. if verbose: show_internal_activity()
└──────────────┘
```

```rust
pub struct CliChannel {
    engine: AxiomEngine,
    perceptor: TextPerceptor,
    effector: MessageEffector,
    config: CliConfig,
}

pub struct CliConfig {
    pub tick_hz: u32,       // default: 100
    pub verbose: bool,      // default: false
    pub prompt: String,     // default: "axiom> "
}

impl CliConfig {
    /// Парсинг аргументов через std::env::args(). Без clap.
    pub fn from_args_or_default() -> Self {
        let mut config = Self::default();
        let args: Vec<String> = std::env::args().collect();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--tick-hz" => {
                    i += 1;
                    if let Some(val) = args.get(i) {
                        config.tick_hz = val.parse().unwrap_or(100);
                    }
                }
                "--verbose" | "-v" => config.verbose = true,
                "--help" | "-h" => {
                    eprintln!("Usage: axiom-cli [--tick-hz N] [--verbose]");
                    std::process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }
        config
    }
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            tick_hz: 100,
            verbose: false,
            prompt: "axiom> ".to_string(),
        }
    }
}
```

---

## 3. Служебные команды

| Команда | Действие |
|---|---|
| `:quit` / `:q` | Завершить работу |
| `:status` | tick_count, кол-во токенов, трейсов, tension, goals |
| `:domains` | Список доменов (id, имя, кол-во токенов) |
| `:tokens <domain_id>` | Токены в домене (top-10 по weight) |
| `:traces` | Top-10 трейсов в EXPERIENCE по weight |
| `:tension` | Активные tension traces |
| `:goals` | Активные цели |
| `:verbose [on/off]` | Переключить подробный вывод |
| `:tick [N]` | Прокрутить N тиков без ввода |
| `:snapshot` | Сделать snapshot |
| `:schedule` | Текущий TickSchedule |
| `:help` | Список команд |

Всё что не начинается с `:` — отправляется в TextPerceptor → ядро.

---

## 4. ProcessingResult — что ядро возвращает для наблюдения

### 4.1 Структура

```rust
// Определить в axiom-runtime (или axiom-core если нужна видимость из agent)
pub struct ProcessingResult {
    pub ucl_result: UclResult,
    pub path: ProcessingPath,
    pub dominant_domain_id: u16,
    pub coherence_score: Option<f32>,
    pub tension_count: u32,
    pub output_shell: [u8; 8],
    pub output_position: [i16; 3],
    pub reflex_hit: bool,
    pub traces_matched: u32,
}

pub enum ProcessingPath {
    Reflex,
    SlowPath,
    MultiPass(u8),
}
```

### 4.2 Откуда данные — изменения в ядре (Фаза 1)

Это **основная работа** первой фазы. Данные для ProcessingResult сейчас не сохраняются — нужно добавить поля:

**В Arbiter:**
```rust
pub struct Arbiter {
    // ... существующие поля ...

    /// Результат последней маршрутизации (для диагностики)
    pub(crate) last_routing: Option<RoutingSnapshot>,
}

pub struct RoutingSnapshot {
    pub was_reflex: bool,
    pub traces_matched: u32,
    pub dominant_domain: u16,
}
```

Заполняется в `route_token()` перед возвратом. Одно присваивание — overhead ~2 ns.

**В MayaProcessor (или DomainState MAYA):**
```rust
// Если MayaProcessor — отдельная структура:
pub(crate) last_coherence: Option<f32>,
pub(crate) last_pass_count: u8,
pub(crate) last_output_shell: [u8; 8],
pub(crate) last_output_position: [i16; 3],
```

Заполняется при обработке в MAYA. Если multi-pass — `last_pass_count` > 1.

**В Experience (DomainState домена 9):**
```rust
// Уже может быть tension_traces.len() — проверить
// Если ExperienceExtension существует — читать оттуда
```

**process_and_observe():**
```rust
impl AxiomEngine {
    pub fn process_and_observe(&mut self, command: &UclCommand) -> ProcessingResult {
        // 1. Обычная обработка
        let ucl_result = self.process_command(command);

        // 2. Собрать диагностику из сохранённых полей
        // Arbiter: self.arbiter.last_routing
        // MAYA: DomainState(110).last_coherence, last_output_shell, ...
        // EXPERIENCE: DomainState(109).tension_traces.len()

        ProcessingResult {
            ucl_result,
            path: /* из arbiter.last_routing + maya.last_pass_count */,
            dominant_domain_id: /* из arbiter.last_routing.dominant_domain */,
            coherence_score: /* из maya.last_coherence */,
            tension_count: /* из experience.tension_traces.len() */,
            output_shell: /* из maya.last_output_shell */,
            output_position: /* из maya.last_output_position */,
            reflex_hit: /* из arbiter.last_routing.was_reflex */,
            traces_matched: /* из arbiter.last_routing.traces_matched */,
        }
    }
}
```

**Важно:** `process_and_observe()` НЕ замена `process_command()`. Это обёртка для наблюдения. Горячий путь не трогается. Overhead — чтение полей, ~10 ns.

---

## 5. Crate-структура

**axiom-agent уже существует.** CLI Channel добавляется туда.

```
crates/axiom-agent/
├── Cargo.toml              # + tokio
├── src/
│   ├── lib.rs
│   ├── channels/
│   │   ├── mod.rs
│   │   └── cli.rs          # CliChannel
│   ├── perceptors/
│   │   ├── mod.rs
│   │   └── text.rs         # TextPerceptor
│   ├── effectors/
│   │   ├── mod.rs
│   │   ├── shell.rs        # ShellEffector (существующий)
│   │   └── message.rs      # MessageEffector
│   ├── ml/
│   │   └── engine.rs       # MLEngine (существующий)
│   └── result.rs           # ProcessingResult, ProcessingPath
│
├── bin/
│   └── axiom-cli.rs         # Точка входа
```

**Cargo.toml (axiom-agent):**
```toml
[dependencies]
axiom-runtime = { path = "../axiom-runtime" }
axiom-core = { path = "../axiom-core" }
tokio = { version = "1", features = ["rt", "io-util", "macros", "sync", "time"] }

[[bin]]
name = "axiom-cli"
path = "bin/axiom-cli.rs"
```

**bin/axiom-cli.rs:**
```rust
use axiom_agent::channels::cli::{CliChannel, CliConfig};
use axiom_runtime::AxiomEngine;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Использовать СУЩЕСТВУЮЩИЙ API создания Engine
    // НЕ try_new(genome) — проверить актуальную сигнатуру в engine.rs
    let engine = AxiomEngine::new();

    let config = CliConfig::from_args_or_default();

    println!("AXIOM — Cognitive Architecture");
    println!("───────────────────────────────");
    // Вывести актуальное состояние из engine
    println!("Type :help for commands, :quit to exit\n");

    let mut cli = CliChannel::new(engine, config);
    cli.run().await;
}
```

---

## 6. План реализации (порядок фаз)

### Фаза 1: ProcessingResult + process_and_observe() [ЯДРО]

**Где:** `axiom-runtime`, `axiom-arbiter`

**Это самая инвазивная фаза. Изменения в ядре.**

1. Определить `ProcessingResult`, `ProcessingPath`, `RoutingSnapshot` в axiom-runtime.
2. Добавить `last_routing: Option<RoutingSnapshot>` в Arbiter. Заполнять в `route_token()`.
3. Добавить `last_coherence`, `last_pass_count`, `last_output_shell`, `last_output_position` в MayaProcessor/DomainState MAYA.
4. Реализовать `AxiomEngine::process_and_observe()`.
5. Тесты:
   - `process_and_observe()` при reflex → `path == Reflex`, `reflex_hit == true`.
   - `process_and_observe()` при slow path → `path == SlowPath`.
   - `process_and_observe()` при multi-pass → `path == MultiPass(N)`, `coherence_score.is_some()`.
   - `process_and_observe()` overhead < 1 µs vs `process_command()`.
   - `tension_count` корректно отражает количество tension traces.

`cargo test --workspace` зелёный после этой фазы.

### Фаза 2: TextPerceptor [AGENT]

**Где:** `axiom-agent/src/perceptors/text.rs`

1. Реализовать `TextPerceptor` с эвристиками.
2. **КРИТИЧЕСКИ:** Открыть `engine.rs`, найти `parse_inject_token_payload()`, скопировать layout. Реализовать `build_inject_token_command()` зеркально парсеру.
3. Проверить типы полей Token в `axiom-core/src/token.rs` — temperature, mass, valence могут быть u8/i8/u16/i16. Использовать реальные типы.
4. `target_domain_id` = 100 (SUTRA на уровне 1), НЕ 0.
5. Тесты:
   - Одинаковый текст → одинаковый command (детерминизм).
   - Разные тексты → разные position (хэш расходится).
   - Пустая строка → валидный command.
   - Юникод → валидный command (не паника).
   - Собранный command проходит через `parse_inject_token_payload()` без ошибок.
   - Shell L5 = 200 для любого текста.

### Фаза 3: MessageEffector [AGENT]

**Где:** `axiom-agent/src/effectors/message.rs`

1. Реализовать `MessageEffector::format_result()`.
2. `domain_name()` — использовать `id % 100` для логического offset.
3. Тесты:
   - Форматирование reflex result.
   - Форматирование multi-pass с coherence.
   - `domain_name(106)` → "LOGIC".
   - `domain_name(110)` → "MAYA".
   - `domain_name(999)` → "UNKNOWN".

### Фаза 4: CliChannel + служебные команды [AGENT]

**Где:** `axiom-agent/src/channels/cli.rs`

1. `CliChannel::new(engine, config)`.
2. `CliChannel::run()` — async: spawn stdin reader, запустить tick loop.
3. stdin reader: `tokio::io::BufReader::new(tokio::io::stdin())`, `read_line()` в цикле, send через mpsc.
4. tick loop: `tokio::time::interval()`, `try_recv()`, dispatch.
5. Служебные команды: `:status`, `:domains`, `:quit`, `:help`, `:verbose`, `:tick N`, `:traces`, `:tension`, `:snapshot`, `:schedule`.
6. Verbose mode: после каждого тика проверять tension traces, internal impulses — выводить если есть.
7. Graceful shutdown: `:quit` → break из tick loop → drop engine.

### Фаза 5: bin/axiom-cli.rs [AGENT]

**Где:** `crates/axiom-agent/bin/axiom-cli.rs`

1. `main()`: создать Engine (актуальный API), создать CliConfig, запустить CliChannel.
2. `CliConfig::from_args_or_default()` через `std::env::args()`.
3. Banner при старте.
4. `[[bin]]` в `crates/axiom-agent/Cargo.toml`.
5. `cargo run --bin axiom-cli` запускается, показывает промпт, принимает ввод.

### Фаза 6: Интеграционные тесты [AGENT + RUNTIME]

1. Inject текст → `process_and_observe()` → результат осмысленный.
2. Два одинаковых текста → второй может быть reflex (если weight > threshold).
3. Cognitive Depth: inject → coherence < threshold → multi-pass.
4. Tension trace появляется после незавершённой обработки.
5. `:status` возвращает корректные данные.
6. `:tick 100` — 100 тиков, Engine стабилен.
7. Бенчмарк: `process_and_observe()` overhead.

---

## 7. Инварианты

1. **Изоляция ядра.** Ядро не импортирует tokio, std::io. Граница = UclCommand(64B) → UclResult(32B).
2. **Детерминизм.** Одинаковый ввод → одинаковая обработка ядром. CLI может быть недетерминирован (timing), ядро — нет.
3. **Ядро живёт.** Tick loop крутится всегда. Cognitive Depth работает без ввода.
4. **Graceful shutdown.** `:quit` → чистый выход без паник.

---

## 8. Что НЕ входит в MVP

- ML-кодирование текста (BERT/embeddings)
- TelegramChannel, WebSocket, REST
- Ответы на естественном языке — вывод диагностический
- Persistent storage (сохранение EXPERIENCE между запусками)
- ANSI цветной вывод (nice to have)
- clap для парсинга аргументов

---

## 9. Критерий завершения

- [ ] `cargo run --bin axiom-cli` запускается и показывает промпт
- [ ] Ввод текста → токен инжектируется → результат выводится
- [ ] `:status` показывает состояние
- [ ] `:domains` показывает 11 доменов
- [ ] `:quit` корректно завершает
- [ ] Ядро тикает в фоне (tick_count растёт без ввода)
- [ ] Verbose mode показывает internal impulses
- [ ] `cargo test --workspace` зелёный (813+ тестов + новые)
- [ ] `process_and_observe()` overhead < 1 µs

---

## 10. История изменений

- **V1.1**: Ревизия после обратной связи исполнителя. Исправлены domain_id (100-110), API Engine (new() не try_new), layout payload (зеркало парсера), bin target (в Cargo.toml крейта), типы полей Token (сверять с кодом). Уточнена Фаза 1 как самая инвасивная.
- **V1.0**: Первая версия.
