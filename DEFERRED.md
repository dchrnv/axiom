# Axiom — Отложенные задачи

**Версия:** 21.0
**Обновлён:** 2026-04-17

---

## Ждут конкретного триггера

### Anchor-Fill — Наполнение якорных YAML-файлов (Фаза 4)

**Где:** `config/anchors/`

Сейчас загружены только:

- `axes.yaml` — 6 осевых якорей (X/Y/Z полюса)
- `layers/L5_cognitive.yaml` — 10 якорей когнитивного слоя
- `domains/D1_execution.yaml` — 6 якорей домена EXECUTION

Для полного семантического покрытия нужно заполнить:

| Файл                     | Слой / Домен | Рекомендуемых якорей |
| ---------------------------- | --------------------- | --------------------------------------- |
| `layers/L1_physical.yaml`  | L1 Physical           | 7+                                      |
| `layers/L2_sensory.yaml`   | L2 Sensory            | 10+                                     |
| `layers/L3_motor.yaml`     | L3 Motor              | 7+                                      |
| `layers/L4_emotional.yaml` | L4 Emotional          | 7+                                      |
| `layers/L6_social.yaml`    | L6 Social             | 7+                                      |
| `layers/L7_temporal.yaml`  | L7 Temporal           | 7+                                      |
| `layers/L8_abstract.yaml`  | L8 Abstract           | 7+                                      |
| `domains/D2_shadow.yaml`   | SHADOW                | 5+                                      |
| `domains/D3_codex.yaml`    | CODEX                 | 5+                                      |
| `domains/D4_map.yaml`      | MAP                   | 5+                                      |
| `domains/D5_probe.yaml`    | PROBE                 | 5+                                      |
| `domains/D6_logic.yaml`    | LOGIC                 | 5+                                      |
| `domains/D7_dream.yaml`    | DREAM                 | 6 (пример в спеке)          |
| `domains/D8_ethics.yaml`   | ETHICS                | 5+                                      |

Формат: [docs/spec/Anchor_Tokens_V1_0.md](docs/spec/Anchor_Tokens_V1_0.md), раздел 7.
Диагностика через CLI: `:match "текст"` — показывает совпадения и вычисленную позицию.

**Когда:** По мере накопления понимания семантики системы (chrnv). Система работает без них — FNV-1a fallback.

---

### D-06 — MLEngine: input_size/output_size = 0 при загрузке ONNX

**Где:** `crates/axiom-agent/src/ml/engine.rs:120-123`

Проверка `if *input_size > 0` скрывает ShapeMismatch-ошибки.

**Когда:** При первой реальной ONNX-модели.

---

## Внешние адаптеры

**Спецификация:** [docs/spec/External_Adapters_V3_0.md](docs/spec/External_Adapters_V3_0.md)  
**План реализации:** [docs/spec/External_Adapters_Plan_V1_0.md](docs/spec/External_Adapters_Plan_V1_0.md)  
**Гайд:** [docs/guides/External_Adapters_Guide_V1_0.md](docs/guides/External_Adapters_Guide_V1_0.md)

| Адаптер         | Requires          | Фаза   | Статус    |
|-----------------|-------------------|--------|-----------|
| Рефактор CLI    | —                 | 0A/0B/0C | 0A ✅    |
| WebSocket       | axum              | 1      | не начат  |
| REST API        | axum              | 2      | не начат  |
| egui Dashboard  | eframe            | 3      | не начат  |
| Telegram        | teloxide (feature)| 4      | не начат  |
| OpenSearch      | reqwest (feature) | 5      | не начат  |
| gRPC            | tonic + protobuf  | —      | не сейчас |
| Python bindings | pyo3              | —      | не сейчас |

### Техдолг Phase 0C

**EA-TD-03 — CLI-фичи не перенесены в tick_loop**  
`watch_fields`, `event_log`, `PerfTracker`, `verbose`, `multipass_count/last_multipass_n`
удалены из tick loop в Phase 0C. Команды `:events`, `:perf`, `:watch` и verbose-вывод
работают с нулевыми/пустыми значениями вне CLI-контекста.  
**Когда:** Phase 1 — выделить `CliTickExtension` или передавать эти данные через
отдельный side-channel, либо оставить как CLI-only фичи с `Arc<Mutex<...>>`.

**EA-TD-04 — Adaptive tick rate не перенесён в tick_loop**  
`config.adaptive_tick_rate` и связанная логика (`TickRateReason`) остались в старом run().
В Phase 0C tick_loop работает с фиксированным интервалом.  
**Когда:** Phase 1 — добавить адаптивность в tick_loop как опциональный параметр.

**EA-TD-05 — hot_reload (ConfigWatcher) не перенесён в tick_loop**  
`config_watcher` остался в CliChannel но больше не вызывается из run().  
**Когда:** Phase 1 — передать ConfigWatcher в tick_loop через mpsc или Arc.

**EA-TD-06 — Inject output CLI: упрощённый формат**  
`process_adapter_command` для Inject возвращает `ServerMessage::Result` со структурными данными.
CLI-подписчик форматирует упрощённо (без `MessageEffector::format_result`).
В Phase 0B CLI использовал полный `MessageEffector` с DetailLevel.  
**Когда:** Phase 1 — добавить `CliAdapter` который знает про MessageEffector и DetailLevel,
либо добавить `ServerMessage::CommandResult` для уже-форматированного CLI-вывода из Inject.

### Техдолг Phase 0A

**EA-TD-01 — Дублирование `domain_name()`**  
`domain_name()` существует в `axiom-agent/src/effectors/message.rs` (pub fn) и продублирована в
`axiom-runtime/src/engine.rs` под `#[cfg(feature = "adapters")]`.  
**Когда:** При рефакторе axiom-runtime в Phase 0B/0C — вынести в `axiom-runtime` как публичную  
не-feature-gated `pub fn domain_name(id: u16) -> &'static str`, удалить дубли.

**EA-TD-02 — `shell` в `TokenSnapshot` — диагностическое приближение**  
`Shell [u8; 8]` не хранится в `Token` (вычисляется `axiom-shell::Shell`).  
`TokenSnapshot::shell` использует приближение: `[0,0,0,|valence|,temperature,mass,0,0]`.  
**Когда:** Если понадобится точный shell в broadcast — добавить `computed_shell: [u8;8]` в  
`DomainState` или пересчитывать через `Shell::from_token` при построении snapshot.
