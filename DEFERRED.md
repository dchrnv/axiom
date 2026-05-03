# Axiom — Отложенные задачи

**Версия:** 30.0
**Обновлён:** 2026-05-03

---

## Workstation V1.0 — отложено из Stage 2

### BRD-TD-01 — DomainActivity throttle enforcement

**Где:** `crates/axiom-broadcasting/src/server.rs` → `should_send()`

`BroadcastingConfig::domain_activity_threshold` объявлен, но `should_send()` его не проверяет — событие `DomainActivity` всегда проходит фильтр. Полная реализация требует `recent_activity` дельты от Engine.

**Когда:** Stage 3+ (Engine integration).

---

### BRD-TD-03 — Snapshot resync при RecvError::Lagged

**Где:** `crates/axiom-broadcasting/src/server.rs`, ветка `RecvError::Lagged`

При переполнении broadcast-очереди клиент теряет события. Сейчас сервер логирует предупреждение и продолжает. По спеке нужно отправить клиенту полный `SystemSnapshot` для resync. Помечено `// SCALE-POINT` в коде.

**Когда:** Stage 3+ (после Engine integration — нужен живой snapshot).

---

### BRD-TD-05 — Полнота полей build_system_snapshot()

**Где:** `crates/axiom-broadcasting/src/snapshot.rs`

Многие поля `SystemSnapshot` заполнены нулями/пустыми значениями: `capacity`, `temperature_decay`, `temperature_avg`, `recent_activity`, `layer_activations`, `guardian_stats`, `last_dream_report`, `adapter_progress`. Расширяется по мере добавления публичного API в axiom-runtime.

**Когда:** Постепенно, по мере Stage 3–8.

---

### BRD-TD-07 — Engine tick-loop → BroadcastHandle интеграция

**Где:** axiom-runtime (Engine), axiom-broadcasting (BroadcastHandle)

Спека предписывала `broadcasting` feature в axiom-runtime, который зависит от axiom-broadcasting. Невозможно: axiom-broadcasting уже зависит от axiom-runtime → цикл зависимостей. Правильный подход: интеграция делается на уровне бинарного crate (будущий `axiom-node` или демо-бинарник) который зависит на оба crate и вызывает `handle.publish(...)` из хука тик-цикла. Engine к этому готов: `snapshot_for_broadcast()` уже экспортируется через `BroadcastSnapshot`.

**Когда:** Stage 8 или при добавлении `axiom-node` бинарника.

---

### BRD-TD-06 — Интеграционный тест pong timeout

**Где:** `crates/axiom-broadcasting/src/tests.rs`

Тест 2.7.e проверяет только что сервер отправляет Ping. Проверку разрыва соединения при отсутствии Pong сделать нельзя: `tokio-tungstenite` клиент автоматически отвечает на Ping без участия пользовательского кода.

**Когда:** При необходимости — через raw TCP клиент без WebSocket framing.

---

## Workstation V1.0 — отложено из Stage 4

### WS4-TD-01 — DetachTab: нет UI-триггера

**Где:** `crates/axiom-workstation/src/ui/tabs.rs`

`Message::DetachTab(tab)` и вся логика detach/re-attach реализованы в `app.rs`, но в таб-баре нет кнопки/жеста для вызова. По спеке — правый клик на табе или иконка «открепить».

**Когда:** Stage 9 (общие компоненты + контекстное меню).

---

### WS4-TD-02 — System Map canvas без Cache

**Где:** `crates/axiom-workstation/src/ui/system_map.rs`

Canvas перерисовывается каждые 33ms (AnimationTick) без разделения на статический и анимированный слой. По спеке — статические элементы (домены, подписи) кешируются через `canvas::Cache`, анимация мандалы — отдельный `Frame`. При большом количестве доменов это может просадить FPS.

**Когда:** Stage 10 (Live Field) — тогда появится реальная нагрузка на canvas, и оптимизация станет оправданной.

---

### WS4-TD-03 — System Map: неполные визуальные фичи спеки

**Где:** `crates/axiom-workstation/src/ui/system_map.rs`

Не реализованы три элемента из спеки (Документ 3A, раздел 2.4):
- **ASHTI sector fill** — активные домены должны заливать соответствующий сектор среднего кольца мандалы (цвет состояния). Сейчас только линии-разделители.
- **Flow lines** — линии между доменами должны подсвечиваться при `EngineEvent::DomainActivity` за последние ~500ms. Сейчас статические линии к центру.
- **Alert ring** — при `guardian_stats.vetoes_since_wake > 0` снаружи мандалы появляется тонкое красное кольцо. Не реализовано.

**Когда:** По мере Stage 5–8 — после Engine integration, когда будут живые данные для проверки.

---

### WS4-TD-04 — SystemSnapshot: поля bottom-panel из спеки отсутствуют в протоколе

**Где:** `crates/axiom-protocol/src/snapshot.rs`, `crates/axiom-workstation/src/ui/system_map.rs`

Спека (Документ 3A, раздел 4.5) описывает в bottom-panel поля `last_hot_path_ns` (время горячего пути) и `promotions_today` / `last_dream_ago`. В `SystemSnapshot` их нет. Bottom-panel сейчас показывает state, fatigue%, tick, frames, events — всё что есть в протоколе.

**Что нужно:** Добавить в `SystemSnapshot`:
- `hot_path_ns: u64` — измеряется в tick-loop
- `promotions_today: u32` — счётчик в FrameWeaverStats
- `dream_phase_stats.last_dream_ended_at_tick` — для вычисления ago

**Когда:** Stage 8 (Engine integration) — когда Engine будет публиковать живой snapshot.

---

## Workstation V1.0 — отложено из Stage 5

### WS5-TD-01 — Горячая перезагрузка WS-адреса

**Где:** `crates/axiom-workstation/src/app.rs`, `crates/axiom-workstation/src/connection.rs`

При изменении `engine_address` через Config вкладку и Apply — `settings.engine_address` обновляется и сохраняется, но `ws_subscription` запущен с **старым адресом** (id-ключ подписки = старый адрес). Новая подписка с новым адресом запустится только после перезапуска приложения.

Правильное решение: при изменении адреса принудительно завершить старую подписку и запустить новую. В iced это делается через смену `id` в `Subscription::run_with_id`.

**Что нужно:** После `ConfigApply` для `workstation.connection` триггерить пересоздание subscription (например, через поле `subscription_key: String` в стейте, которое обновляется → iced видит новый id → пересоздаёт).

**Когда:** Stage 9 или при реальной необходимости менять адрес без рестарта.

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

### FW-TD-02 — FrameWeaver: min_participant_anchors не проверяется

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`, метод `qualifies_for_promotion`

Поле `PromotionRule::min_participant_anchors` (минимум участников, которые сами являются анкерами SUTRA) не проверяется — требует cross-domain lookup: найти sutra_ids участников Frame в SUTRA-домене. В текущей сигнатуре `check_promotion` нет доступа к AshtiCore.

**Что нужно:**
- Расширить сигнатуру `Weaver::check_promotion` (добавить `ashti: &AshtiCore`) или
- Передавать предвычисленный список SUTRA-анкеров снаружи

**Когда:** При реализации полного пути промоции.

---

### FW-TD-03 — Weaver::check_promotion без доступа к current_tick

**Где:** `crates/axiom-runtime/src/over_domain/traits.rs`, сигнатура `check_promotion`

Сигнатура `fn check_promotion(&self, experience_state: &DomainState, anchors: &[&Token]) -> Vec<PromotionProposal>` не передаёт текущий tick, поэтому `qualifies_for_promotion` использует `tick_proxy = 0` для проверки `min_age_ticks`.

**Что нужно:** Добавить `tick: u64` параметр в сигнатуру трейта (breaking change).

**Когда:** При первой реальной промоции EXPERIENCE → SUTRA.

---

### FW-TD-04 — on_boot не проверяет GENOME-права для FrameWeaver

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs:624`, метод `on_boot`

`_genome` игнорируется — не проверяется наличие `ModuleId::FrameWeaver` в GENOME access_rules и что выданные права соответствуют ожидаемым (EXPERIENCE/ReadWrite, MAYA/Read, SUTRA/Control). TODO-комментарий оставлен в коде.

**Что нужно:** Вызвать `genome.index().check_access(ModuleId::FrameWeaver, ...)` для каждого нужного ресурса; вернуть `Err(OverDomainError::GenomeDenied)` при отсутствии прав.

**Когда:** При добавлении runtime GENOME-enforcement (GenomeIndex уже реализован).

---

### ~~FW-TD-05 — propose_to_dream возвращает пустые команды (DREAM не реализован)~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-04-29 в DREAM Phase V1.0 + FrameWeaver V1.2.

`propose_to_dream()` заменён на `dream_propose()` — вызывается однократно при входе в `FallingAsleep`. FrameWeaver сканирует кандидатов и передаёт `DreamProposal::Promotion` в `DreamCycle`. Интеграция через `DreamCycle` (Stabilization→Processing→Consolidation).

Оставшийся вопрос о заполнении `commands` снят: промоция идёт через `DreamCycle`, а не через `process_command` напрямую.

---

### FW-TD-06 — промоция EXPERIENCE→SUTRA без участников (dummy_candidate)

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs:727-734`, `on_tick` фаза 5

При срабатывании промоции из `on_tick` создаётся `dummy_candidate` с `participants: Vec::new()`. В результате `build_promotion_commands` генерирует только токен-анкер в SUTRA, но **не** генерирует `BondTokens` к участникам Frame. SUTRA-анкер изолирован — без связей к участникам паттерна.

**Что нужно:** Восстанавливать участников из EXPERIENCE — хранить `participants` в Connection-метаданных EXPERIENCE-анкера или в отдельном side-store (lineage_hash → Vec<sutra_id>).

**Когда:** При полной реализации пути EXPERIENCE → SUTRA.

---

### FW-TD-07 — три нереализованных RuleTrigger ветки (DreamCycle, HighConfidence, RepeatedAssembly)

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs:511-513`, `trigger_matches`

Три ветки `RuleTrigger` всегда возвращают `false`:
- `DreamCycle` — DREAM Phase V1.0 реализована, но сигнал из DreamCycle в FrameWeaver ещё не подключён
- `HighConfidence(f32)` — нет confidence scoring у кандидатов (см. также DreamPhase_V2_plus.md)
- `RepeatedAssembly { window_ticks }` — нет счётчика повторных сборок в скользящем окне

**Что нужно:**
- `DreamCycle`: сигнал от DREAM (флаг или канал)
- `HighConfidence`: добавить `confidence: f32` в `FrameCandidate`, вычислять из силы связей
- `RepeatedAssembly`: хранить `assembly_counts: HashMap<u64, (u64, u32)>` (hash → (last_tick, count))

**Когда:** По мере расширения модели кристаллизации.

---

### EA-TD-07 — Применение domain config при hot-reload к running engine

**Где:** `crates/axiom-agent/src/tick_loop.rs`, ветка `if let Some(_new_cfg) = watcher.poll()`

**Частично закрыто (2026-04-29):** `Gateway::check_config_reload()` теперь автоматически применяет `DreamConfig` при hot-reload через `engine.apply_dream_config(&cfg.dream)`. DreamScheduler и DreamCycle перенастраиваются без рестарта.

Оставшаяся часть: применение обновлённых **domain**-пресетов к уже запущенному `AxiomEngine` не реализовано — `AxiomEngine` не имеет метода `apply_domain_config(&DomainConfig)`.

**Что нужно:**
1. Добавить `pub fn apply_domain_config(&mut self, domain_id: u16, cfg: &DomainConfig)` в `AxiomEngine`
2. В `tick_loop.rs` при обнаружении изменений перебрать `new_cfg.domains` и применить каждый
3. Логировать что именно изменилось (threshold, membrane, physics params)

**Когда:** Когда понадобится живая перенастройка доменных порогов без рестарта. Сейчас обход — рестарт axiom-cli с новыми конфигами.

---

## Внешние адаптеры

**Спецификация:** [docs/spec/External_Adapters_V3_0.md](docs/spec/External_Adapters_V3_0.md)
**Гайд:** [docs/guides/External_Adapters_Guide_V1_0.md](docs/guides/External_Adapters_Guide_V1_0.md)

| Адаптер         | Requires          | Фаза   | Статус |
|-----------------|-------------------|--------|--------|
| Рефактор CLI    | —                 | 0A/0B/0C | ✅   |
| WebSocket       | axum              | 1      | ✅     |
| REST API        | axum              | 2      | ✅     |
| egui Dashboard  | eframe            | 3      | ✅     |
| Telegram        | reqwest (feature) | 4      | ✅     |
| OpenSearch      | reqwest (feature) | 5      | ✅     |
| gRPC            | tonic + protobuf  | —      | не сейчас |
| Python bindings | pyo3              | —      | не сейчас |
