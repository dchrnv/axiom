# AXIOM WORKSTATION V2.0

**Статус:** Реализован (WS-0..3 завершены, 2026-05-24)  
**Заменяет:** axiom-workstation (iced, V1.0) — заморожен после WS-2.2  
**Реализация:** `tools/axiom-web/`, `crates/axiom-node/`  
**Руководство оператора:** `docs/guides/Workstation_V2_Guide.md`

---

## 1. Контекст пивота

V1.0 (iced, 2026-05-05) был построен как нативное Rust-приложение в эстетике школы
Cupertino: светлый фон, скруглённые формы, 8 вкладок, мандала как символический центр.
Спецификации V1 явно включали браузерный интерфейс и HTML/JS в список
**архитектурных анти-паттернов**.

V2.0 отступает от этого решения. Причины:

1. **advisory-петля не замыкалась через iced.** OverDomainArbiter V2 требует confirm/reject
   от оператора. Добавить REST-endpoint в iced-клиент означало бы встроить HTTP-сервер
   внутрь GUI. Это инверсия архитектуры.

2. **Grafana не интегрируется с нативным приложением.** Prometheus /metrics — естественный
   вывод для Grafana. В iced это невозможно.

3. **axiom-node уже был HTTP-сервером.** После добавления WebSocket JSON bridge (WS-0)
   браузер получал правильный протокол без дополнительных усилий. Прокси в Vite закрывал
   CORS и WebSocket в dev.

4. **Скорость итераций.** React + hot reload vs. `cargo build` каждый раз.

**Что осталось неизменным из V1:** цель — окно в работу системы. Наблюдение как главный
режим. Оператор смотрит, понимает, корректирует. Conversation — утилитарный отладочный
канал, не центр интерфейса.

**Что изменилось:** эстетика (тёмная монохромная вместо Cupertino), стек (React вместо iced),
масштаб (4 сфокусированных таба вместо 8 развёрнутых окон), отсутствие detach/multi-window
в V2 (запас для WS-4 Tauri).

---

## 2. Эстетический язык V2

V2 осознанно использует **тёмную монохромную терминальную эстетику**.

```
--bg:       #0d0e11  (почти чёрный)
--surface:  #16181e  (карточки, заголовок)
--surface2: #1c1f28  (вложенные карточки)
--border:   #2a2d36
--text:     #e2e4ea
--text-dim: #7a7f94
--accent:   #5b8dee  (синий — активные элементы, доминирующий октант)
--green:    #4caf72  (Wake, confirm)
--red:      #e05c5c  (reject, critical, conn-off)
--yellow:   #d4a32a  (FallingAsleep, Waking)

Шрифт: JetBrains Mono / Fira Code / Cascadia Code (monospace)
Font-size: 13px base
```

**Принцип цвета:** строго функциональный, совпадает с V1. Цвет несёт состояние:
зелёный = нормально/подтверждено, жёлтый = переходное состояние, красный = требует
внимания/отклонения, синий = доминирующий/активный.

**Отказ от Cupertino:** V2 — инструмент разработчика, живущий рядом с терминалом.
Тёмная монохромная тема органична для этой роли. Cupertino-эстетика — в будущем
Companion, ориентированном на пользователя, а не разработчика.

---

## 3. Архитектура

```
                    ┌─────────────────────────────────────┐
                    │            axiom-node                │
                    │                                      │
                    │  AxiomEngine ← NodeCmd channel       │
                    │  tick loop (60 Hz)                   │
                    │                                      │
                    │  axum HTTP :8080                     │
                    │   GET  /api/ws          WS bridge    │
                    │   POST /api/text/submit              │
                    │   POST /api/advisory/confirm/{id}    │
                    │   POST /api/advisory/reject/{id}     │
                    │   GET  /metrics         Prometheus   │
                    │   GET  *                ServeDir     │
                    └──────────┬──────────────────────────┘
                               │ WebSocket JSON
                               │ REST
                    ┌──────────▼──────────────────────────┐
                    │         tools/axiom-web              │
                    │         React 18 SPA (Vite)          │
                    │                                      │
                    │  ws/client.ts    авто-reconnect 2s   │
                    │  store/engine.ts Zustand store       │
                    │  App.tsx         4 таба              │
                    │                                      │
                    │  Overview | Conversation | Phase C   │
                    │  Patterns                            │
                    └─────────────────────────────────────┘

                    ┌─────────────────────────────────────┐
                    │       tools/grafana (опционально)    │
                    │                                      │
                    │  Prometheus :9090 — scrape /metrics  │
                    │  Grafana     :3000 — 3 дашборда      │
                    └─────────────────────────────────────┘
```

### 3.1 NodeCmd channel

Ключевое архитектурное решение: HTTP-хендлеры **не владеют AxiomEngine**.
Вместо `Arc<Mutex<AxiomEngine>>` используется unbounded mpsc channel:

```rust
pub enum NodeCmd {
    AdvisoryConfirm(u64),
    AdvisoryReject(u64),
    SubmitText(String),
}
```

HTTP-хендлер отправляет команду в канал. Tick loop дренирует канал и применяет
синхронно на каждой итерации. Нет mutex-ов на горячем пути.

### 3.2 WebSocket JSON bridge

Существующий BroadcastServer использует postcard бинарный протокол (для axiom-workstation
iced, axiom-dashboard egui). Он остаётся нетронутым.

Новый аксиум-node `/api/ws` подписывается на тот же `broadcast::Receiver<EngineMessage>`,
но ре-сериализует как JSON для браузера:

```
BroadcastHandle.subscribe_events() → Receiver<EngineMessage>
     ↓
axum WS handler → serde_json::to_string() → browser
```

При подключении нового клиента — немедленно отправляется полный SystemSnapshot
из `BroadcastHandle.latest_snapshot()`.

### 3.3 Zustand store

```typescript
interface EngineStore {
  snapshot: SystemSnapshot | null;
  connected: boolean;
  feed: FeedMessage[];          // Conversation лента
  layerHistory: number[][];     // последние 120 snapshots, rolling
  
  setSnapshot(s: SystemSnapshot): void;  // также вызывает pushHistory
  setConnected(b: boolean): void;
  addFeedMessage(msg: Omit<FeedMessage, 'id'>): void;
}
```

`layerHistory` хранит `over_domain.layer_activations` из каждого Snapshot —
это источник данных для SVG-спарклайнов в Patterns.

---

## 4. WebSocket-протокол (JSON)

Зеркалит `axiom-protocol` Rust типы. Serde encoding:
- unit variant → `"VariantName"`
- struct variant → `{ "VariantName": { ...fields } }`

### EngineMessage

```typescript
type EngineMessage =
  | { Hello:         { version: number; capabilities: number } }
  | { Snapshot:      SystemSnapshot }
  | { Event:         EngineEvent }
  | { CommandResult: { command_id: number; result: unknown } }
  | { Bye:           { reason: string } }
```

### EngineEvent

```typescript
type EngineEvent =
  | { Tick:                 { tick: number; event: number; hot_path_ns: number } }
  | { DomainActivity:       { domain_id: number; recent_activity: number; layer_activations: number[] } }
  | { DreamPhaseTransition: { from: EngineState; to: EngineState; trigger: string } }
  | { FrameCrystallized:    { anchor_id: number; layers_present: number; participant_count: number } }
  | { FrameReactivated:     { anchor_id: number; new_temperature: number } }
  | { FramePromoted:        { anchor_id: number } }
  | { Alert:                { level: string; message: string } }
```

### Поведение клиента

- При открытии соединения: получает `Snapshot` с текущим полным состоянием
- После: получает `Event` по мере возникновения (DomainActivity, Tick, etc.)
- При разрыве: `setTimeout(connectWS, 2000)` в `onclose`

---

## 5. Четыре вкладки

### 5.1 Overview

Метрики системы в реальном времени. Приоритет — быстрое считывание состояния без
необходимости переключаться.

**Metrics row:** Tick, Event, Hot path (µs), Fatigue (%), Tokens, Connections, Dreams,
Vetoes. Обновляются каждый Snapshot.

**Fatigue bar:** прогресс-бар с маркером порога. Цвет: зелёный → жёлтый (>70% порога)
→ красный (≥ порога). Числа: `current% / threshold%`.

**FrameWeaver stats:** total frames / in sutra / promotions since wake.

**Advisory Queue:** появляется если есть pending advisories. Дублирует Phase C для
быстрого доступа без переключения вкладки.

### 5.2 Conversation

Текстовый ввод в Engine через TextPerceptor.

- Textarea + `Ctrl+Enter` или кнопка Send
- `POST /api/text/submit` → NodeCmd::SubmitText → TextPerceptor → SUTRA domain (100)
- Запись немедленно появляется в ленте (kind=`user`)

**Лента** — не персистентная (очищается при перезагрузке):

| Kind | Цвет | Источник |
|------|------|---------|
| user | белый | отправленный текст |
| frame | синий | FrameCrystallized WS event |
| dream | жёлтый | DreamPhaseTransition WS event |
| alert | красный | Alert WS event |

**Domain selector** — присутствует в UI, но текущая реализация всегда injecting в
SUTRA (100). Функциональный selector — деферред.

### 5.3 Phase C

Аксиальное состояние: доминирующий октант, подсистема, глубины, emergent candidates,
advisory queue.

**Axial State:** dominant octant (имя из 8), dominant subsystem (Writing/Mathematics/
Philosophy/Code/Analysis/Unknown), pending emergent count.

**Octant Depth bars:** 8 полосок, нормализованных по максимуму. Доминирующий октант:
имя в `--accent`, заливка `--accent` opacity 1.0.

Октанты по индексу:
```
0  CreativeAffirmation
1  EcstaticAffirmation
2  HeroicFatal
3  DestructiveActivating
4  IdealizedConsoling
5  PassiveSentimental
6  FormalDenying
7  SelfDestructiveApathic
```

**Emergent Candidates:** таблица sutra_id / octant / initial_depth / [Approve].
Approve → `POST /api/text/submit` body `{ text: ":approve {sutra_id}" }`.

**Badge на вкладке:** `<span class="tab-badge">{pendingCount}</span>` при
`pending_advisories.length > 0`.

### 5.4 Patterns

Паттерны активации слоёв — единственная вкладка где данные накапливаются во времени.

**Current Layer Activations:** гистограмма L1–L8 over-domain, нормализованная по
максимуму. Цвет уникален для каждого слоя (8 предустановленных цветов).

**Layer History sparklines:** SVG `<polyline>` без внешних зависимостей. Rolling буфер
120 снапшотов в `layerHistory`. Min-max нормализация по истории каждого слоя.

**Domain Activity grid:** карточки каждого домена (SUTRA→MAYA). Каждая: имя,
token_count, temperature_avg, recent_activity, mini-bar-chart по слоям.

---

## 6. Advisory Queue

Ключевой элемент V2 — закрытие advisory-петли.

```
┌──────┬──────────────┬──────┬──────────────────────────────┬────────────────┬───┐
│ Type │ Subject      │ Conf │ Label                        │ TTL            │   │
├──────┼──────────────┼──────┼──────────────────────────────┼────────────────┼───┤
│ OctC │ Frame #1042  │ 0.82 │ OctantCorrection: HeroicFat… │ [████░░░] 120  │✓ ✗│
│ NarS │ Frame #0573  │ 0.71 │ NarrativeShift: Mathematics  │ [██████░] 450  │✓ ✗│
│ OctC │ Frame #0891  │ 0.68 │ OctantCorrection: Formal...  │ [████████] 890 │✓ ✗│
└──────┴──────────────┴──────┴──────────────────────────────┴────────────────┴───┘
```

**TTL bar:** `age / 1000` (где 1000 = `PENDING_TTL` event_id). При `age/TTL > 0.8` —
заливка меняется на `--red` (`data-critical` атрибут).

**Confirm (✓):** `POST /api/advisory/confirm/{advisory_id}` → NodeCmd::AdvisoryConfirm
→ `engine.confirm_pending_advisory(id)` → `over_domain_arbiter.confirm_pending(id, depth_store)`.

**Reject (✗):** `POST /api/advisory/reject/{advisory_id}` → NodeCmd::AdvisoryReject
→ `engine.reject_pending_advisory(id)`.

**Busy state:** кнопки дизаблятся во время inflight-запроса.

---

## 7. Prometheus /metrics

~30 метрик в text format. Manual `write!` в String — без зависимости от crate `prometheus`.

Группы:

```
# ENGINE
axiom_engine_tick_total, axiom_engine_event_total, axiom_hot_path_ns
axiom_engine_state{state="Wake"} 1

# MEMORY
axiom_tokens_total, axiom_connections_total
axiom_cross_domain_events_recent

# FATIGUE & DREAM
axiom_fatigue_current, axiom_fatigue_threshold
axiom_dream_cycles_total, axiom_ticks_since_dream

# GUARDIAN
axiom_guardian_vetoes_total, axiom_guardian_vetoes_since_wake

# FRAMEWEAVER
axiom_frames_total, axiom_frames_in_sutra
axiom_frame_promotions_since_wake
axiom_last_crystallization_tick

# PHASE C
axiom_phase_c_dominant_octant, axiom_phase_c_dominant_subsystem
axiom_phase_c_emergent_candidates
axiom_phase_c_advisory_pending
axiom_octant_depth{octant="0"} ... axiom_octant_depth{octant="7"}

# DOMAINS (per-domain)
axiom_domain_tokens{domain="SUTRA"} ...
axiom_domain_temperature{domain="SUTRA"} ...
axiom_domain_activity{domain="SUTRA"} ...
```

---

## 8. Grafana (tools/grafana/)

Docker Compose: Grafana :3000 + Prometheus :9090.
`extra_hosts: host.docker.internal:host-gateway` — доступ к axiom-node на хосте.
Scrape interval: 5s.

**3 provisioned дашборда** (JSON в git, schemaVersion=39, refresh=5s):

| Дашборд | Панели |
|---------|--------|
| `engine.json` | Engine State, Tick rate, Hot path, Tokens, Fatigue bar, Dreams |
| `phase_c.json` | Dominant Octant, Advisory pending, Octant depth time-series |
| `domains.json` | Per-domain tokens, temperature, activity (стаки по доменам) |

---

## 9. Состояние реализации (2026-05-24)

| Фаза | Содержание | Статус |
|------|-----------|--------|
| WS-0 | axiom-node axum HTTP, WS JSON bridge, React scaffold, Zustand store | ✅ |
| WS-1 | Advisory Queue UI, confirm/reject buttons + TTL bar, REST endpoints | ✅ |
| WS-2 | Core Tabs: Conversation, Phase C (octant depth, emergent, advisory), Patterns (sparklines, domain grid) | ✅ |
| WS-3 | /metrics Prometheus, tools/grafana docker-compose, 3 provisioned дашборда | ✅ |
| WS-4 | Tauri wrapper (нативный desktop) | Deferred |

**iced Workstation (V1.0):** заморожен после WS-2.2. Продолжает работать параллельно.
Дата удаления: после WS-4 + стабильность 2+ сессий.

---

## 10. Deferred

### WS-4 — Tauri wrapper

```bash
cd tools/axiom-web
npm install @tauri-apps/cli
npx tauri init
npx tauri build  # .deb / .AppImage
```

`tauri.conf.json` указывает на `dist/index.html`. UI-код остаётся тем же.
Добавляет: нативное окно, системный трей, offline-работу без браузера.

### Conversation domain selector

Domain selector присутствует в UI но сейчас косметический — всегда injecting в SUTRA (100).
Для functional selector нужна поддержка `target_domain` в `POST /api/text/submit`.

### Conversation persistence

История чата не сохраняется между перезагрузками браузера. Для persistence нужен
либо narrative-log API в Engine, либо localStorage в клиенте.

### FW-TD-01 — RequestFrameDetails

UCL-команда `RequestFrameDetails { anchor_id }` существует в протоколе, но обработчик
не написан. Нужна для детального просмотра участников Frame в Phase C.

### WS-V2-01..05

Идеи из V1 DEFERRED (история Conversation, Pause/Resume импорта, custom bench,
TLS, Companion sync) — см. [AXIOM_Workstation_DEFERRED.md](AXIOM_Workstation_DEFERRED.md).

---

## 11. Структура файлов

```
tools/axiom-web/
├── index.html
├── package.json              react 18, zustand 5, vite
├── vite.config.ts            proxy /api → :8080, ws:true; outDir=dist
├── tsconfig*.json
└── src/
    ├── App.tsx               корневой компонент + FatigueBar + Metric
    ├── App.css               все стили (CSS custom properties)
    ├── main.tsx
    ├── store/
    │   └── engine.ts         Zustand: snapshot, connected, feed, layerHistory
    ├── ws/
    │   ├── client.ts         connectWS(), handleMessage(), handleEvent()
    │   └── protocol.ts       TypeScript типы: EngineMessage, EngineEvent, SystemSnapshot, ...
    └── components/
        ├── AdvisoryQueue.tsx  таблица advisories, confirm/reject, TTL bar
        ├── Conversation.tsx   feed + textarea + send
        ├── PhaseC.tsx         axial state + octant bars + emergent + advisory
        └── Patterns.tsx       Sparkline(SVG) + layer bars + domain grid

crates/axiom-node/
├── Cargo.toml                axum, tower-http[fs,cors], serde_json, tokio
└── src/
    ├── main.rs               создаёт NodeCmd channel, запускает http::run + tick::run
    ├── config.rs             --http-addr, --web-dist CLI args
    ├── http.rs               NodeCmd enum, create_cmd_channel(), run(), format_metrics()
    └── tick.rs               дренирует NodeCmd channel, обрабатывает AdvisoryConfirm|Reject|SubmitText

tools/grafana/
├── docker-compose.yml        grafana:latest + prom/prometheus:latest
├── provisioning/
│   ├── datasources/          prometheus.yml (host.docker.internal:9090)
│   └── dashboards/           dashboard.yml provider
└── dashboards/
    ├── engine.json
    ├── phase_c.json
    └── domains.json
```
