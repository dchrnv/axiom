# Axiom Roadmap

**Версия:** 12.0
**Дата:** 2026-03-28
**Спека:** [Roadmap_V3_0.md](Roadmap_V3_0.md)

---

## Сводка завершённых этапов

| Этап | Название | Статус |
|------|----------|--------|
| 1 | GENOME | ✅ 426 тестов |
| 2 | Storm Control | ✅ 430 тестов |
| 3 | Configuration System | ✅ 469 тестов |
| 4 | REFLECTOR + SKILLSET | ✅ 496 тестов |
| 5 | GridHash | ✅ 519 тестов |
| 6 | Адаптивные пороги | ✅ 533 тестов |
| 7 | Causal Horizon + Memory | ✅ 568 тестов |
| 8 | Gateway + Channel | ✅ 590 тестов |

Технический долг и будущие планы: [DEFERRED.md](DEFERRED.md)

---

## Этап 9: Технический долг + Event Bus

**Цель:** Закрыть DEFERRED.md. Подготовить инфраструктуру подписок для внешних каналов.

**Зависимости:** —

### 9A. Event Bus pub/sub

Подписочная модель поверх `EventObserver`. Модули подписываются на конкретные `EventType`, получают уведомления без polling.

**Что добавить в `axiom-runtime/src/adapters.rs`:**
- `EventBus` struct: `HashMap<u16, Vec<Box<dyn EventObserver>>>` — подписчики по типу события
- `EventBus::subscribe(event_type: u16, observer: Box<dyn EventObserver>)`
- `EventBus::publish(&[Event])` — рассылка по подпискам
- Интеграция в `Gateway`: `drain_and_notify` → `bus.publish(events)`

**Тесты:** subscribe/publish, фильтрация по event_type, несколько подписчиков на один тип.

### 9B. Token/Connection preset loading

Загрузка пресетов токенов и связей из YAML.

**Что добавить в `axiom-config`:**
- `TokenPreset` struct (поля Token + имя пресета)
- `ConnectionPreset` struct
- `ConfigLoader::load_token_presets(path) -> Vec<TokenPreset>`
- `ConfigLoader::load_connection_presets(path) -> Vec<ConnectionPreset>`
- Файлы `config/presets/tokens/*.yaml`, `config/presets/connections/*.yaml`

**Тесты:** загрузка YAML, валидация полей, round-trip.

### 9C. Config hot reload

Отслеживание изменений `config/*.yaml` без перезапуска. **Не применяется к GENOME** — конституция неизменна.

**Что добавить:**
- `ConfigWatcher` в `axiom-config`: `notify` crate (inotify на Linux)
- `ConfigWatcher::watch(path, callback: Box<dyn Fn(LoadedAxiomConfig)>)`
- Перезагрузка `DomainConfig`, `HeartbeatConfig`, пресетов
- `Gateway::set_config_watcher(watcher: ConfigWatcher)` — применяет новые конфиги через `engine_mut()`

**Тесты:** изменение файла → callback вызван, GENOME не перезагружается.

**Критерий:** DEFERRED.md пуст. Event Bus работает. Hot reload не ломает работающий Engine.

---

## Этап 10: External Integration — Agent Layer

**Цель:** AXIOM взаимодействует с внешним миром. CLI-агент отвечает на команды.

**Зависимости:** Этап 9 (Event Bus нужен для подписки Channel на события MAYA).

**Архитектура:**

```
stdin / Telegram / WebSocket
        │
   [ Perceptor ]  ──→  UclCommand  ──→  Gateway  ──→  AxiomEngine
                                             │
   [ Effector  ]  ←──  Event / Result  ←────┘
        │
stdout / ответ в чат
```

Новый бинарник `axiom-agent` (отдельный workspace или `src/bin/`), зависит от `axiom-runtime` + `tokio`.

### 10A. Trait-границы (axiom-runtime)

Добавить в `axiom-runtime/src/adapters.rs`:

```rust
/// Входящий адаптер: внешний сигнал → UclCommand
pub trait Perceptor: Send {
    fn receive(&mut self) -> Option<UclCommand>;
    fn name(&self) -> &str;
}

/// Исходящий адаптер: Event / UclResult → внешний мир
pub trait Effector: Send {
    fn emit(&mut self, event: &Event);
    fn emit_result(&mut self, result: &UclResult);
    fn name(&self) -> &str;
}
```

### 10B. CLI Channel (MVP)

`axiom-agent/src/channels/cli.rs`:
- `CliPerceptor`: читает строку из stdin → `InjectToken` или `TickForward` команда
- `CliEffector`: форматирует `UclResult` и события MAYA → stdout
- Минимальный REPL: `> tick`, `> inject <data>`, `> status`, `> quit`

**Тесты:** mock stdin → команда → mock stdout, round-trip без паники.

### 10C. Telegram Channel

`axiom-agent/src/channels/telegram.rs`:
- `TelegramPerceptor`: polling Telegram Bot API (reqwest), сообщение → `InjectToken`
- `TelegramEffector`: отправка ответа в чат при событии MAYA
- Конфиг: `config/channels.yaml` → `telegram.token`, `telegram.chat_id`

**Тесты:** mock HTTP → парсинг update → команда, формирование ответа.

### 10D. Shell Effector

`axiom-agent/src/channels/shell.rs`:
- `ShellEffector`: выполняет команду при событии с `event_type == ShellCommand`
- Белый список команд из `config/shell_whitelist.yaml`
- Guardian проверяет перед исполнением: `enforce_access(Effectors, ShellExec)`

**Тесты:** whitelist allow/deny, Guardian veto, команда из списка выполняется.

### 10E. axiom-agent бинарник

`axiom-agent/src/main.rs`:
```
1. Загрузить GENOME из config/genome.yaml
2. Создать AxiomEngine → Gateway
3. Загрузить config/channels.yaml → активировать каналы
4. Запустить EventBus подписки (MAYA events → Effectors)
5. Основной цикл: Perceptors → Gateway → Effectors
```

`config/channels.yaml`:
```yaml
channels:
  cli: true
  telegram:
    enabled: false
    token: ""
  shell:
    enabled: false
    whitelist: config/shell_whitelist.yaml
```

**Критерий:** `./axiom-agent` запускается, принимает команды из stdin, возвращает результат.

---

## Этап 11: ML Inference (Восприятие)

**Цель:** AXIOM видит и слышит через нейросети (ONNX, tract).

**Зависимости:** Этап 10 (нужен axiom-agent с Perceptor/Effector архитектурой).

### 11A. ML Engine wrapper

`axiom-agent/src/ml/engine.rs`:
- Обёртка над `tract-onnx` (чистый Rust, no Python)
- `MLEngine::load(path: &Path) -> Self`
- `MLEngine::infer(&[f32]) -> Vec<f32>` — синхронный вызов
- Асинхронная обёртка через `tokio::task::spawn_blocking`
- Загрузка моделей из `models/` при старте

### 11B. VisionPerceptor

`axiom-agent/src/channels/vision.rs`:
- Источник: файл изображения / USB-камера (через `v4l2` или `image` crate)
- Модель: YOLO tiny (ONNX)
- Выход: токены объектов с координатами → `InjectToken` в LOGIC(106) или MAP(104)
- Один обнаруженный объект = один Token: `temperature` = confidence × 255, `position` = bbox center

### 11C. AudioPerceptor

`axiom-agent/src/channels/audio.rs`:
- Источник: микрофон (ALSA) или аудиофайл
- VAD: Silero VAD (ONNX) — отсечение тишины
- STT: Whisper tiny (ONNX или `whisper-rs`)
- Выход: распознанный текст → токен в SUTRA(100)

### 11D. GUARDIAN фильтры для ML

Добавить в `axiom-runtime/src/guardian.rs`:
- `validate_ml_result(confidence: f32, threshold: f32) -> bool`
- Отклонение при `confidence < genome.config.ml_confidence_threshold`
- Adversarial defense: отклонение при аномально высоком `confidence` (> 0.99 на неизвестных классах)

**Тесты:** инференс на тестовых данных (mock), токенизация результата, GUARDIAN фильтрация.
**Критерий:** Агент распознаёт объекты на изображении и речь. GUARDIAN отсекает низкокачественный вывод.

---

## Этап 12: Фракталы и SIMD

**Цель:** Многоуровневая обработка. Максимальная производительность физики.

**Зависимости:** 8-11 (система должна быть стабильна при долгих запусках).

### 12A. Протокол 10→0 (Фрактальные уровни)

`crates/axiom-domain/src/fractal_chain.rs`:
- `FractalChain`: связывает два `AshtiCore` — `maya_output → sutra_input`
- `AshtiCore::set_sutra_input(token: Token)` — внешний впрыск в SUTRA(100)
- `AshtiCore::take_maya_output() -> Option<Token>` — забрать результат с MAYA(110)
- `FractalChain::tick()` — шаг по всей цепочке
- Обмен SkillSet между уровнями через `export/import_batch`

**Тесты:** двухуровневая цепочка, данные проходят от SUTRA-1 до MAYA-2, навыки переносятся.

### 12B. SIMD-оптимизация физики поля

`crates/axiom-space/src/simd.rs`:
- AVX2 / SSE4.2 batch-обработка позиций токенов
- Векторизованный `apply_gravity` для N токенов за раз
- Feature flag `simd` — включается через `RUSTFLAGS="-C target-cpu=native"`
- Scalar fallback при отсутствии поддержки

**Бенчмарк:** SpatialHashGrid rebuild с SIMD vs scalar на 5000 токенов.
**Критерий:** SIMD-путь не нарушает детерминизм. 2x+ ускорение физики поля.

---

## Граф зависимостей

```
9 (Tech Debt) ──→ 10 (Agent) ──→ 11 (ML) ──→ 12 (Фракталы)
                      │
                 axiom-agent
               (tokio, reqwest)
```

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
