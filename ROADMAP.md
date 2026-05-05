# Axiom Roadmap

**Версия:** 41.0  
**Дата:** 2026-05-05

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
                                                    ↑
                                               axiom-broadcasting
                                                    ↑
                                               axiom-workstation
```

**Workstation V1.0 завершён (2026-05-05).** 1174 тестов, 0 failures.  
**axiom-broadcasting** не подключён к Engine tick-loop (BRD-TD-07) — Workstation работает, но без живых данных.  
**DREAM Phase V1.0 + FrameWeaver V1.2** завершены. Промоция EXPERIENCE→SUTRA только в DREAMING.

---

## Фазы работы

### Фаза A — «Живая Workstation» 🔑

**Главный приоритет.** Все остальные фазы либо разблокируются после A, либо независимы от него.

#### A1 — BRD-TD-07: axiom-agent + BroadcastHandle (главный разблокировщик)

Добавить `axiom-broadcasting` в зависимости `axiom-agent`. Прокинуть `Option<BroadcastHandle>` в
`tick_loop`. Добавить флаг `--workstation-port 9876` в `axiom-cli`.

Почему через axiom-agent, а не axiom-node: цикла зависимостей нет (agent → broadcasting → runtime
— DAG). axiom-node остаётся планом для production-сервиса без CLI-багажа; этот шаг его не
блокирует и не усложняет.

**Результат:** `cargo run --bin axiom-cli -- --server --workstation-port 9876` → Workstation
подключается и получает живые тики, события, снапшоты.

**Затрагивает:** `crates/axiom-agent/Cargo.toml`, `src/tick_loop.rs`, `bin/axiom-cli.rs`

---

#### A2 — BRD-TD-05: build_system_snapshot() реальные данные

Сейчас `build_system_snapshot()` заполняет большинство полей `SystemSnapshot` нулями. После A1
появляется живой `AxiomEngine` — можно добирать реальные данные через публичный API
`axiom-runtime`: `guardian_stats`, `dream_phase` / `dream_stats`, `layer_activations` из
FrameWeaverStats, `temperature_avg` по доменам.

Делать итерационно: каждое добавленное поле сразу отображается в Workstation.

**Затрагивает:** `crates/axiom-broadcasting/src/snapshot.rs`

---

#### A3 — WS4-TD-03: System Map — сектор-заливка, flow lines, alert ring

После A1 приходят живые `DomainActivity` события и `guardian_stats`. Три элемента мандалы:
- **Sector fill** — заливка сектора ASHTI активным цветом при токенах > threshold
- **Flow lines** — подсветка линий при `DomainActivity` за последние ~500ms (хранить `last_active_at: [Instant; 8]` в AppData)
- **Alert ring** — тонкое красное кольцо снаружи при `guardian_stats.vetoes_since_wake > 0`

**Затрагивает:** `crates/axiom-workstation/src/ui/system_map.rs`, `src/app.rs`

---

#### A4 — WS4-TD-04: SystemSnapshot — недостающие поля bottom-panel

Добавить в протокол и заполнять из движка:
- `hot_path_ns: u64` — из tick-loop (уже измеряется в `PerfTracker`)
- `promotions_today: u32` — из `FrameWeaverStats.total_promotions`
- `last_dream_ended_at_tick: u64` — из `DreamPhaseStats`

**Затрагивает:** `crates/axiom-protocol/src/snapshot.rs`,
`crates/axiom-broadcasting/src/snapshot.rs`, `crates/axiom-workstation/src/ui/system_map.rs`

---

### Фаза B — «UI-доделки» (независимы от A, делать в любом порядке)

#### B1 — WS8-TD-01: файловый пикер rfd

`rfd = { version = "0.14", features = ["tokio"] }` → кнопка Browse в Files tab →
`rfd::AsyncFileDialog::new().pick_file()` через `Task::future` → `Message::FilesPickPath(path)`.

**Затрагивает:** `crates/axiom-workstation/Cargo.toml`, `src/ui/files.rs`, `src/app.rs`

---

#### B2 — WS6-TD-01: multi-line text_editor в Conversation

Заменить `text_input` на `iced::widget::text_editor` с `text_editor::Content` в
`ConversationState`. Enter = новая строка, Ctrl+Enter = submit (через `on_key_press`).

**Затрагивает:** `crates/axiom-workstation/src/ui/conversation.rs`, `src/app.rs`

---

#### B3 — WS7-TD-02: Show more / пагинация

Кнопка `[ Show more... ]` в лентах Patterns (max показывать 20, хранить 100) и Dream State
(max 5, хранить 20). `show_all: bool` флаг в `PatternsState` / `DreamWindowState`.

**Затрагивает:** `src/ui/patterns.rs`, `src/ui/dream_state.rs`, `src/app.rs`

---

#### B4 — WS4-TD-02: canvas::Cache в System Map

Разделить canvas на два слоя: статический (домены, кольца, разделители) через `canvas::Cache` +
динамический (пульсация мандалы, активные элементы) — отдельный `Frame` каждые 33ms.
Оправдано после A1, когда появятся реальные данные и нагрузка на canvas.

**Затрагивает:** `crates/axiom-workstation/src/ui/system_map.rs`

---

#### B5 — WS9-TD-02: Welcome screen fade-in

`welcome_opacity: f32` в `WorkstationApp`, накапливается в `AnimationTick` (+=0.04 до 1.0).
`container(...).style(|_| Style { opacity: self.welcome_opacity, .. })` или через color alpha.

**Затрагивает:** `src/app.rs`, `src/ui/welcome.rs`

---

#### B6 — WS9-TD-01 + WS9-TD-03: MenuBar + DetachTab кнопка

Исследовать `iced_aw` совместимость с iced 0.13 перед началом. Если несовместим — реализовать
кастомный dropdown через `stack` + условный overlay. После MenuBar — добавить пункт
"View → Detach current tab" → `Message::DetachTab(active_tab)`.

**Затрагивает:** `Cargo.toml`, `src/app.rs`, `src/ui/` (новый `menu.rs`)

---

### Фаза C — «Протокол-расширения» (после A1)

#### C1 — WS7-TD-01: syntactic_layer_activations в FrameWeaverStats

Добавить `syntactic_layer_activations: [u8; 8]` в `FrameWeaverStats` в `axiom-protocol`.
Заполнять из FrameWeaver при кристаллизации (какие синтаксические слои были активны).
Workstation: Patterns tab получает реальные S1-S8 sparklines.

**Затрагивает:** `axiom-protocol/src/snapshot.rs`, `axiom-runtime/src/over_domain/weavers/frame.rs`,
`axiom-workstation/src/ui/patterns.rs`

---

#### C2 — WS8-TD-02: EngineCommand::RunBench

Добавить `EngineCommand::RunBench { spec: BenchSpec }` в протокол.
Реализовать в axiom-agent: принять команду через broadcasting → запустить бенч →
отправлять `BenchStarted / BenchProgress / BenchFinished` events.
Воткнуть в Workstation: `Message::BenchRun` перестаёт быть no-op.

**Затрагивает:** `axiom-protocol/src/commands.rs`, `axiom-agent/src/tick_loop.rs`,
`axiom-workstation/src/app.rs`

---

#### C3 — WS10-TD-01: TokenFieldPoint в DomainSnapshot (Live Field)

Добавить `token_field: Vec<TokenFieldPoint>` в `DomainSnapshot`. `TokenFieldPoint` =
`{ position: [f32; 3], layer: u8, temperature: u8, anchor_membership: Option<u32> }`.

⚠️ Осторожно: при большом `token_count` снапшот становится тяжёлым. Решение: sampling
(max 300 токенов на домен) или отдельный on-demand запрос (не в базовом snapshot-цикле).

**Затрагивает:** `axiom-protocol/src/snapshot.rs`, `axiom-broadcasting/src/snapshot.rs`,
`axiom-workstation/src/ui/live_field.rs`

---

### Фаза D — «Engine tech debt» (независимый трек)

Не блокируется Workstation. Можно делать параллельно с любой фазой.

#### D1 — FW-TD-03: tick в Weaver::check_promotion (breaking change)

Добавить `tick: u64` в сигнатуру трейта `Weaver::check_promotion`. Сейчас
`qualifies_for_promotion` использует `tick_proxy = 0`, поэтому `min_age_ticks` никогда не
проверяется корректно.

**Затрагивает:** `axiom-runtime/src/over_domain/traits.rs` + все impl Weaver

---

#### D2 — FW-TD-02 + FW-TD-06: полный путь промоции EXPERIENCE→SUTRA

D2a — `min_participant_anchors`: передать `ashti: &AshtiCore` в `check_promotion` или предвычислять
список SUTRA-анкеров снаружи.

D2b — восстановление участников: `dummy_candidate` в `on_tick` генерирует изолированный SUTRA-анкер
без связей. Нужно хранить `lineage_hash → Vec<sutra_id>` и восстанавливать при промоции.

Делать вместе: оба касаются одного метода.

---

#### D3 — FW-TD-07: три RuleTrigger ветки

- `DreamCycle`: добавить флаг `dream_cycle_active: bool` в AxiomEngine, выставлять при входе в
  FallingAsleep — FrameWeaver проверяет его в `trigger_matches`
- `RepeatedAssembly`: `assembly_counts: HashMap<u64, (u64, u32)>` (lineage_hash → (last_tick, count))
  в FrameWeaver state
- `HighConfidence`: `confidence: f32` в `FrameCandidate`, вычислять из avg weight связей (V2+)

---

#### D4 — FW-TD-04: GENOME enforcement в on_boot

`genome.index().check_access(ModuleId::FrameWeaver, Resource, AccessLevel)` для каждого из 5
access rules. Вернуть `Err(OverDomainError::GenomeDenied)` при нарушении. GenomeIndex уже
реализован — это подключение, не новая реализация.

---

#### D5 — EA-TD-07: domain config hot-reload

`pub fn apply_domain_config(&mut self, domain_id: u16, cfg: &DomainConfig)` в `AxiomEngine`.
В `tick_loop` при `watcher.poll()` перебирать изменённые домены и применять.
Текущий обход: рестарт axiom-cli.

---

#### D6 — BRD-TD-01 + BRD-TD-03: broadcasting improvements (после A1)

BRD-TD-01: `should_send()` проверяет `domain_activity_threshold` (сравнивать дельту активности
из snapshot с порогом).

BRD-TD-03: при `RecvError::Lagged` отправлять клиенту полный `SystemSnapshot` для resync
(помечено `// SCALE-POINT`).

---

### Фаза E — «Контент и инфраструктура»

#### E1 — Anchor-Fill: якорные YAML-файлы

14 файлов (L1–L8 кроме L5, D2–D8). ~7–10 якорей каждый. Делать вручную — это семантический
контент, не код. Диагностика: `:match "слово"` в CLI. Система работает без них (FNV-1a fallback).

**Когда:** По мере понимания семантики. Без дедлайна.

---

#### E2 — D-06: MLEngine size check

`input_size > 0` guard скрывает ShapeMismatch. Заменить на явную проверку с ошибкой.
**Когда:** При первой реальной ONNX-модели.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing. Очень низкий приоритет.
- **WS-V2-***, **COMP-01** — V2.0 идеи и Companion. См. DEFERRED.md.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
