# Axiom — Отложенные задачи

**Версия:** 43.0
**Обновлён:** 2026-05-12

---

## Axiom Sentinel

### SENT-S6 — Speculative Layer (после бенчей S0–S5)

**Где:** `crates/axiom-space/src/lib.rs`, `crates/axiom-domain/src/domain_state.rs`, `crates/axiom-runtime/src/engine.rs`

Пока Arbiter обрабатывает тик N, свободные воркеры предвычисляют 2–3 вероятных состояния `SpatialHashGrid` для тика N+1. Zero-cost switch при совпадении (~9 µs vs ~40 µs полный rebuild).

**Что нужно:** отделить `SpatialHashGrid` от `DomainState` как самостоятельную speculatable единицу, добавить `SpatialHashGrid::snapshot/restore_from_grid_snapshot`. Высокая сложность — затрагивает ownership ~200+ тестов.

**Когда:** после закрытия SENT-S4b. Gravity 1M достигла цели (6.74 ms ✅), разблокировано.

---

## FrameWeaver

### FW-TD-01 — RequestFrameDetails не реализован

**Где:** `crates/axiom-protocol/src/commands.rs`, `crates/axiom-workstation/`

`EngineCommand::RequestFrameDetails { anchor_id }` и `EngineEvent::FrameDetails(FrameDetails)` определены в axiom-protocol, но handler нигде не реализован — ни в axiom-workstation, ни в axiom-broadcasting.

`FrameDetails` содержит `last_reactivated_at_tick: Option<u64>`, которое требует дополнительного per-anchor хранилища в FrameWeaver: `reactivation_counts` хранит счётчик, но не тик последней реактивации. `crystallized_at_tick` можно взять из `token.last_event_id` (он не сбрасывается при `ReinforceFrame`).

**Что нужно:** добавить `last_reactivated_at: HashMap<u32, u64>` в FrameWeaver + реализовать handler в axiom-workstation (Benchmarks или отдельная вкладка).

**Когда:** при добавлении детальной инспекции Frame в Workstation V2.0.

---

### FW-TD-02 — Per-pair co-activation не отслеживается

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

Текущая структура: `reactivation_counts: HashMap<u32, u32>` — глобальный счётчик реактиваций на Frame-анкер. Нет информации о том, какие Frame-ы активировались совместно (в одном скане или в соседних).

Нужно для: будущего CausalWeaver (причинные связи между Frame), AnalogyWeaver (похожие паттерны), рефлекторных сокращений опыта в EXPERIENCE. Конкретный вид структуры зависит от первого потребителя.

**Когда:** при проектировании CausalWeaver или AnalogyWeaver.

---

## Broadcasting

### BRD-TD-05 — Полнота полей build_system_snapshot()

**Где:** `crates/axiom-broadcasting/src/snapshot.rs`

Многие поля `SystemSnapshot` заполнены нулями/пустыми значениями: `capacity`, `temperature_decay`, `temperature_avg`, `recent_activity`, `layer_activations`, `guardian_stats`, `last_dream_report`, `adapter_progress`. Расширяется по мере добавления публичного API в axiom-runtime.

**Когда:** Постепенно, по мере интеграции Engine (axiom-node).

---

### BRD-TD-06 — Интеграционный тест pong timeout

**Где:** `crates/axiom-broadcasting/src/tests.rs`

Тест 2.7.e проверяет только что сервер отправляет Ping. Проверку разрыва соединения при отсутствии Pong сделать нельзя: `tokio-tungstenite` клиент автоматически отвечает на Ping без участия пользовательского кода.

**Когда:** При необходимости — через raw TCP клиент без WebSocket framing.

---

### BRD-TD-07 — Engine tick-loop → BroadcastHandle интеграция

**Где:** axiom-runtime (Engine), axiom-broadcasting (BroadcastHandle)

Спека предписывала `broadcasting` feature в axiom-runtime, который зависит от axiom-broadcasting. Невозможно: axiom-broadcasting уже зависит от axiom-runtime → цикл зависимостей. Правильный подход: интеграция делается на уровне бинарного crate (будущий `axiom-node` или демо-бинарник) который зависит на оба crate и вызывает `handle.publish(...)` из хука тик-цикла. Engine к этому готов: `snapshot_for_broadcast()` уже экспортируется через `BroadcastSnapshot`.

**Когда:** При добавлении `axiom-node` бинарника, который зависит на оба crate.

---

## Workstation

### WS4-TD-03 — System Map: неполные визуальные фичи спеки

**Где:** `crates/axiom-workstation/src/ui/system_map.rs`

Не реализованы три элемента из спеки (Документ 3A, раздел 2.4):
- **ASHTI sector fill** — активные домены должны заливать соответствующий сектор среднего кольца мандалы (цвет состояния). Сейчас только линии-разделители.
- **Flow lines** — линии между доменами должны подсвечиваться при `EngineEvent::DomainActivity` за последние ~500ms. Сейчас статические линии к центру.
- **Alert ring** — при `guardian_stats.vetoes_since_wake > 0` снаружи мандалы появляется тонкое красное кольцо. Не реализовано.

**Когда:** При наличии живых данных от Engine (axiom-node).

---

### WS4-TD-04 — SystemSnapshot: поля bottom-panel из спеки отсутствуют в протоколе

**Где:** `crates/axiom-protocol/src/snapshot.rs`, `crates/axiom-workstation/src/ui/system_map.rs`

Спека (Документ 3A, раздел 4.5) описывает в bottom-panel поля `last_hot_path_ns` (время горячего пути) и `promotions_today` / `last_dream_ago`. В `SystemSnapshot` их нет. Bottom-panel сейчас показывает state, fatigue%, tick, frames, events — всё что есть в протоколе.

**Что нужно:** Добавить в `SystemSnapshot`:
- `hot_path_ns: u64` — измеряется в tick-loop
- `promotions_today: u32` — счётчик в FrameWeaverStats
- `dream_phase_stats.last_dream_ended_at_tick` — для вычисления ago

**Когда:** При интеграции живого Engine (axiom-node).

---

## Ждут конкретного триггера

### Anchor-Fill — Наполнение якорных YAML-файлов

**Где:** `config/anchors/`

Сейчас загружены только:

- `axes.yaml` — 6 осевых якорей (X/Y/Z полюса)
- `layers/L5_cognitive.yaml` — 10 якорей когнитивного слоя
- `domains/D1_execution.yaml` — 6 якорей домена EXECUTION

Для полного семантического покрытия нужно заполнить:

| Файл                       | Слой / Домен | Рекомендуемых якорей |
|----------------------------|--------------|----------------------|
| `layers/L1_physical.yaml`  | L1 Physical  | 7+                   |
| `layers/L2_sensory.yaml`   | L2 Sensory   | 10+                  |
| `layers/L3_motor.yaml`     | L3 Motor     | 7+                   |
| `layers/L4_emotional.yaml` | L4 Emotional | 7+                   |
| `layers/L6_social.yaml`    | L6 Social    | 7+                   |
| `layers/L7_temporal.yaml`  | L7 Temporal  | 7+                   |
| `layers/L8_abstract.yaml`  | L8 Abstract  | 7+                   |
| `domains/D2_shadow.yaml`   | SHADOW       | 5+                   |
| `domains/D3_codex.yaml`    | CODEX        | 5+                   |
| `domains/D4_map.yaml`      | MAP          | 5+                   |
| `domains/D5_probe.yaml`    | PROBE        | 5+                   |
| `domains/D6_logic.yaml`    | LOGIC        | 5+                   |
| `domains/D7_dream.yaml`    | DREAM        | 6 (пример в спеке)   |
| `domains/D8_ethics.yaml`   | ETHICS       | 5+                   |

Формат: [docs/spec/Anchor_Tokens_V1_0.md](docs/spec/Anchor_Tokens_V1_0.md), раздел 7.
Диагностика через CLI: `:match "текст"` — показывает совпадения и вычисленную позицию.

**Когда:** По мере накопления понимания семантики системы (chrnv). Система работает без них — FNV-1a fallback.

---

## Workstation — расширения для V2.0

_Идеи, не реализованные в V1.0 по объёму или зависимостям. Не блокируют V1.0._

### WS-V2-01 — Long-term история Conversation

**Где:** `crates/axiom-workstation/src/app.rs` → `ConversationState.messages`

При рестарте Workstation лента чата пуста — история нигде не хранится. V1.0 этого не требует, но оператор теряет контекст предыдущих сессий.

**Что нужно:** Хранить историю в EXPERIENCE как часть нарратива Engine: каждый отправленный текст записывается в отдельный лог (файл или Engine API), загружается при старте.

**Когда:** V2.0 или при появлении narrative-log API в Engine.

---

### WS-V2-02 — Pause / Resume импорта

**Где:** `crates/axiom-workstation/src/ui/files.rs`, `crates/axiom-protocol/src/commands.rs`

Реализован Cancel (через `EngineCommand::CancelAdapter`). Pause/Resume нет — требует поддержки в адаптерах и соответствующих команд в протоколе.

**Что нужно:** `EngineCommand::PauseAdapter { run_id: String }` / `ResumeAdapter`, статус `AdapterStatus::Paused` в протоколе, кнопка Pause рядом с Cancel в `files.rs`.

**Когда:** При необходимости паузируемого импорта больших файлов.

---

### WS-V2-03 — Конструктор кастомных бенчмарков

**Где:** `crates/axiom-workstation/src/ui/benchmarks.rs`

V1.0 показывает историю предустановленных бенчмарков (6 вариантов из `BenchSpec`). Нет возможности собрать кастомный сценарий: нагрузка, длительность, выбор метрик.

**Что нужно:** Форма в Benchmarks tab — `BenchSpec` builder: тип нагрузки (dropdown), iterations, duration, domain selection, сохранение preset-ов локально.

**Когда:** V2.0 или при активном использовании бенчмарков.

---

### WS-V2-04 — Полный compatibility matrix Engine ↔ Workstation

**Где:** `crates/axiom-workstation/src/connection.rs`, `crates/axiom-protocol/src/lib.rs`

V1.0: проверяется только `major`-байт `PROTOCOL_VERSION`. Если Engine обновлён на minor — Workstation подключается, но поля протокола могут расходиться.

**Что нужно:** Матрица совместимости major.minor: graceful degradation для смежных minor-версий (игнорировать unknown enum variants), UI-индикатор "version mismatch, limited mode".

**Когда:** V2.0 / перед публичным релизом.

---

### WS-V2-05 — Сетевой режим (remote Engine)

**Где:** `crates/axiom-workstation/src/settings.rs`, `crates/axiom-workstation/src/connection.rs`

V1.0 рассчитан на локальный Engine (`127.0.0.1:9876`). Точки расширения уже помечены в архитектуре V1.0 — address конфигурируем, transport абстрагирован через WebSocket.

**Что нужно:** TLS поверх WS (`wss://`), аутентификация (token в Hello), обработка network timeouts, reconnect при network partition.

**Смотри:** `AXIOM_Workstation_02_Architecture.md` раздел 9 (Network Mode).

**Когда:** V2.0 / `axiom-node` (Engine на отдельном железе).

---

### WS-V2-06 — Sync между Workstation и Companion

Когда оба клиента (Workstation + Companion) подключены к одному Engine одновременно, нужна координация: не дублировать force-sleep запросы, видеть что другой клиент уже подключён. Синхронизация только через Engine, не напрямую.

**Когда:** Когда Companion будет реализован.

---

## Для проекта Companion

### COMP-01 — Vital Signs окно

Окно, спроектированное для постоянного отображения на физическом светильнике-банере рядом с рабочим столом. Считываемость с расстояния: доминирующий цвет состояния заполняет фон или ключевой элемент, базовые индикаторы активности, минимум текста.

**Ключевые требования:**
- Вертикальная композиция (форм-фактор светильника-банера)
- Гибкая адаптация под пропорции экрана (горизонтальный, квадратный, вертикальный)
- Может работать на отдельном hardware-устройстве (Raspberry Pi + HDMI дисплей)
- Физическое присутствие AXIOM в комнате — не рабочий инструмент, а ambient display

**Контекст:** Обсуждалось при проектировании Workstation как "светильник-банер на столе". Намеренно вынесено в Companion — Workstation покрывает потребность через System Map.

**Когда:** Первое окно Companion. Открыть этот раздел как стартовую точку при начале проектирования Companion.

---

## Внешние адаптеры

**Спецификация:** [docs/spec/External_Adapters_V3_0.md](docs/spec/External_Adapters_V3_0.md)
**Гайд:** [docs/guides/External_Adapters_Guide_V1_0.md](docs/guides/External_Adapters_Guide_V1_0.md)

| Адаптер         | Requires          | Фаза     | Статус    |
|-----------------|-------------------|----------|-----------|
| Рефактор CLI    | —                 | 0A/0B/0C | ✅        |
| WebSocket       | axum              | 1        | ✅        |
| REST API        | axum              | 2        | ✅        |
| egui Dashboard  | eframe            | 3        | ✅        |
| Telegram        | reqwest (feature) | 4        | ✅        |
| OpenSearch      | reqwest (feature) | 5        | ✅        |
| gRPC            | tonic + protobuf  | —        | не сейчас |
| Python bindings | pyo3              | —        | не сейчас |
