# axiom-workstation

Графический интерфейс оператора AXIOM Engine. Десктопное приложение на [iced](https://github.com/iced-rs/iced) (v0.13), подключается к движку через WebSocket (`axiom-broadcasting`).

## Архитектура

```
main.rs
├── settings.rs      — UiSettings (engine_address), TOML-персистенция
├── connection.rs    — WebSocket-подписка (iced::Subscription), reconnect backoff
├── app.rs           — WorkstationApp: state, update(), view(), subscription()
└── ui/
    ├── header.rs    — строка заголовка, индикатор подключения
    ├── tabs.rs      — таб-бар (8 вкладок)
    ├── welcome.rs   — Welcome screen (4 состояния по ConnectionState)
    ├── system_map.rs — System Map: мандала ASHTI на canvas
    ├── config.rs    — Configuration: schema-driven двухпанельный UI
    ├── conversation.rs — Conversation: лента сообщений + domain selector
    ├── patterns.rs  — Patterns: sparklines L1-L8 + recent frames feed
    ├── dream_state.rs — Dream State: текущее состояние + fatigue + history
    ├── files.rs     — Files: import flow с прогресс-панелью
    ├── benchmarks.rs — Benchmarks: запуск + история результатов
    └── live_field.rs — Live Field: 3D-канвас с орбитальной камерой
```

## Запуск

```bash
cargo run -p axiom-workstation
```

По умолчанию подключается к `127.0.0.1:9876`. Адрес меняется в Configuration → Connection.

## Вкладки

| Вкладка        | Содержимое |
|----------------|------------|
| System Map     | Мандала ASHTI с пульсацией и анимацией состояния |
| Live Field     | 3D-визуализация токенов с орбитальной камерой |
| Conversation   | Подача текстовых команд движку |
| Patterns       | Sparklines активности слоёв L1-L8 |
| Dream State    | Состояние цикла сна + fatigue + история цикла |
| Configuration  | Schema-driven редактор конфигурации движка |
| Files          | Импорт данных через адаптеры |
| Benchmarks     | Запуск бенчмарков и история результатов |

## Протокол

Использует `axiom-protocol` — общие типы между workstation и движком:
- Handshake: `ClientMessage::Hello` → `EngineMessage::Hello(version)`
- Команды: `EngineCommand` (отправляются app → engine)
- События: `EngineEvent` (получаются engine → app через broadcast)
- Снапшоты: `SystemSnapshot` (полное состояние движка)

## Зависимости

- `iced 0.13` — UI framework (`canvas` + `tokio` features)
- `tokio-tungstenite 0.24` — WebSocket клиент
- `postcard` — бинарная сериализация протокола
- `axiom-protocol` — типы протокола

## Тесты

```bash
cargo test -p axiom-workstation
```

39 unit-тестов (логика update/state, без UI-рендера).
