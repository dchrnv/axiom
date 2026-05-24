# AXIOM — Workstation V2: React SPA + Grafana

**Статус:** План реализации  
**Дата:** 2026-05-24  
**Заменяет:** `axiom-workstation` (iced) — постепенно, не сразу  

---

## Архитектура

```
tools/axiom-web/          ← React SPA (Vite + React + TypeScript)
  src/
  ├── ws/client.ts        ← WebSocket клиент (axiom-broadcasting protocol)
  ├── api/client.ts       ← REST клиент (/api/*)
  ├── components/
  └── App.tsx

axiom-node (axum)         ← существующий HTTP сервер, дополняется:
  GET  /                  ← отдаёт tools/axiom-web/dist/index.html
  GET  /assets/*          ← статика React
  GET  /ws                ← WebSocket (уже есть)
  POST /api/advisory/confirm/:id   ← новый endpoint
  POST /api/advisory/reject/:id    ← новый endpoint
  GET  /metrics           ← Prometheus формат (новый)

Grafana                   ← отдельный процесс (docker)
  scrapes /metrics каждые 15s
  дашборды: Activity, Arbiter, Dream, Octants
```

**Разработка:** `npm run dev` в `tools/axiom-web/` — hot-reload в браузере.  
**Продакшн:** `npm run build` → `dist/` → axum раздаёт.  
**Desktop:** Tauri wrapper (Phase WS-3, позже) — обернуть существующий React в нативное окно.

---

## Phase WS-0: Foundation (1 сессия)

Минимальная основа: React приложение подключается к движку.

### WS-0.1 — Scaffold

```
tools/axiom-web/
├── package.json          (vite, react, typescript, zustand)
├── vite.config.ts        (proxy /ws → localhost:3001, /api → localhost:3001)
├── index.html
└── src/
    ├── App.tsx
    ├── ws/
    │   ├── protocol.ts   (типы из axiom-protocol: SystemSnapshot, etc.)
    │   └── client.ts     (WebSocket + reconnect)
    └── store/
        └── engine.ts     (zustand store: SystemSnapshot)
```

Зависимости (npm):
- `react` + `react-dom` + TypeScript
- `vite` — dev server + bundler
- `zustand` — состояние (проще Redux, не нужен Context boilerplate)
- `@tanstack/react-query` — для REST запросов

### WS-0.2 — axum: static files + API scaffold

В `axiom-node/src/main.rs` добавить:

```rust
// Static files (React build)
.route("/", get(serve_index))
.route("/assets/*path", get(serve_asset))

// Advisory API
.route("/api/advisory/confirm/:id", post(api_confirm_advisory))
.route("/api/advisory/reject/:id",  post(api_reject_advisory))

// Prometheus metrics
.route("/metrics", get(serve_metrics))
```

### WS-0.3 — Минимальный UI

- Строка состояния: `engine_state`, `current_tick`, `current_event`
- Список доменов: имя, token_count, temperature_avg
- WebSocket статус (connected / reconnecting)

**Критерий готовности:** браузер открывает `localhost:8080`, показывает живые данные.

---

## Phase WS-1: Advisory Queue (1 сессия)

Главная задача Phase D из ROADMAP — закрыть advisory-петлю.

### WS-1.1 — REST endpoints в axiom-node

```rust
async fn api_confirm_advisory(
    State(engine): State<Arc<Mutex<Gateway>>>,
    Path(id): Path<u64>,
) -> StatusCode {
    engine.lock().await.engine_mut().confirm_pending_advisory(id);
    StatusCode::OK
}

async fn api_reject_advisory(
    State(engine): State<Arc<Mutex<Gateway>>>,
    Path(id): Path<u64>,
) -> StatusCode {
    engine.lock().await.engine_mut().reject_pending_advisory(id);
    StatusCode::OK
}
```

`AxiomEngine::confirm_pending_advisory(id)` — вызывает
`over_domain_arbiter.confirm_pending(id, &mut depth_store)`.

### WS-1.2 — Advisory Queue компонент

```
┌─────────────────────────────────────────────────────┐
│ Pending Advisories (3)                               │
├──────┬──────────────┬──────────┬────────┬───────────┤
│ Type │ Subject      │ Conf     │ Age    │           │
├──────┼──────────────┼──────────┼────────┼───────────┤
│ OctC │ Frame #1042  │ 0.82     │ 120ev  │ ✓  ✗     │
│ NarS │ global       │ 0.71     │  45ev  │ ✓  ✗     │
│ OctC │ Frame #0891  │ 0.68     │ 890ev  │ ✓  ✗     │
└──────┴──────────────┴──────────┴────────┴───────────┘
```

- `✓` → `POST /api/advisory/confirm/:id`
- `✗` → `POST /api/advisory/reject/:id`
- TTL-бар: визуальный прогресс `age / PENDING_TTL` (истекает — красный)
- Auto-refresh: данные приходят по WS, не polling

**Критерий готовности:** открыть браузер, нажать Confirm на advisory, увидеть
что оно исчезло из очереди и появилось в log с outcome=Confirmed.

---

## Phase WS-2: Core Tabs (2 сессии)

Портируем самое нужное из iced Workstation.

### WS-2.1 — Conversation Tab

- Поле ввода + кнопка Submit (Ctrl+Enter)
- Лента сообщений (user/system)
- Domain selector (какой домен injecting)
- Привязка к Frame-событиям: когда Frame кристаллизуется — подсветить строку

### WS-2.2 — Phase C Panel

```
┌─────────────────────────────────┐
│ Dominant Octant: HeroicFatal    │
│ Dominant Subsystem: Mathematics │
│                                 │
│ Octant Depth                    │
│ [████░░░░] Heroic     1842      │
│ [███░░░░░] Creative   1203      │
│ [██░░░░░░] Formal      891      │
│ ...                             │
│                                 │
│ Emergent Candidates (2)         │
│ Frame #1042 oct=2 depth=1891    │
│ Frame #0573 oct=6 depth=1204    │
│ [Approve]                       │
└─────────────────────────────────┘
```

### WS-2.3 — Patterns Tab

Sparklines для layer activations (L1–L8).
Маленькие time-series через `recharts` или `uplot` (легковесный).

### WS-2.4 — System Map (опционально)

Live Field 3D — Three.js или `@react-three/fiber`.
Точки = токены (position, temperature → цвет, layer → форма).
Орбитальная камера.

---

## Phase WS-3: Grafana (1 сессия)

Наблюдаемость без UI кода.

### WS-3.1 — Prometheus endpoint в axiom-node

```
GET /metrics → text/plain; version=0.0.4

# Arbiter
arbiter_quality_window{source="NeuralAdvisor",type="OctantCorrection"} 0.73
arbiter_quality_window{source="AxialEvaluator",type="NarrativeShift"} 0.61
arbiter_pending_count 3
arbiter_min_confidence{source="NeuralAdvisor",type="DepthHint"} 0.73

# ContextRecognizer
cr_subsystem_fatigue{subsystem="Mathematics"} 0.42
cr_activity_entropy_gradient 0.18
cr_dominant_subsystem{name="Mathematics"} 1

# AxialEvaluator
ae_narrative_window_octant 2
ae_narrative_confidence 0.75

# DREAM
dream_fatigue 0.31
dream_cycles_total 14

# NeuralAdvisor
na_divergence_rate_7d 0.12      ← advisory vs analytic, расхождение ≥ 2 оси
na_advisories_total{type="OctantCorrection",outcome="Confirmed"} 47
na_advisories_total{type="OctantCorrection",outcome="Rejected"} 12

# Engine
engine_tick_total 142330
engine_event_total 89012
engine_tokens_total 8841
```

В Rust: библиотека `prometheus` (crates.io) или ручной format string — второе проще.

### WS-3.2 — Grafana setup

```yaml
# docker-compose.yml (в tools/grafana/)
services:
  grafana:
    image: grafana/grafana:latest
    ports: ["3000:3000"]
    volumes:
      - ./dashboards:/var/lib/grafana/dashboards
      - ./provisioning:/etc/grafana/provisioning
```

Дашборды (JSON экспорт, хранится в git):

**1. Advisory & Trust** — quality_window per source/type, pending count, confirm/reject rate,
   min_confidence trend (калибровка)

**2. Activity** — entropy_gradient, subsystem_fatigue per subsystem, dominant_subsystem,
   activity_signature distribution

**3. Dream & Memory** — fatigue curve, dream cycles, tokens/frames total

**4. Octant** — распределение октантов по времени, narrative_confidence

**Критерий готовности:** открыть `localhost:3000`, видеть живые графики.

---

## Phase WS-4: Tauri wrapper (позже, 0.5 сессии)

Когда React SPA стабилен:

```bash
cd tools/axiom-web
npm install @tauri-apps/cli
npx tauri init
npx tauri dev    # нативное окно
npx tauri build  # .deb / .AppImage
```

`tauri.conf.json` указывает на `dist/index.html`. Весь UI-код остаётся тем же.
Добавляет: нативное меню, системный трей, offline работа.

---

## Что происходит с iced Workstation

`axiom-workstation` не удаляем сразу — он продолжает работать параллельно.
Переключаемся когда React SPA покрывает минимум: Conversation + Advisory + Phase C.

Дата заморозки iced: после WS-2.2.  
Дата удаления iced: после WS-2.3 + стабильность 2+ сессий без регрессий.

---

## Гаджет с малым экраном (заложено)

React SPA работает как PWA из коробки (`vite-plugin-pwa`).
На устройстве с браузером (Pi + Chromium) открывается как есть.

Для устройства без браузера: отдельный клиент под конкретное железо.
Пишется когда известны характеристики гаджета. Использует тот же REST/WS API.
Никаких изменений в Rust-бэкенде не потребуется.

---

## Итоговая последовательность

```
WS-0  Foundation + axum static + scaffold           ← 1 сессия
WS-1  Advisory Queue (confirm/reject)               ← 1 сессия  ← D2 из ROADMAP
WS-2  Core Tabs (Conversation, Phase C, Patterns)   ← 2 сессии
WS-3  Grafana + /metrics endpoint                   ← 1 сессия
WS-4  Tauri wrapper                                 ← 0.5 сессии, позже
```

Суммарно: ~5.5 сессий для полного перехода.  
После WS-1 система уже функционально превосходит iced Workstation по главному параметру.
