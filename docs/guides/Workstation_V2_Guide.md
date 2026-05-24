# Workstation V2 — Operator Guide

**Версия:** V2.0  
**Стек:** React 18 + Zustand + Vite · axum HTTP · WebSocket JSON  
**Источник:** `tools/axiom-web/`

---

## Запуск

### Dev (два терминала)

```bash
# Терминал 1
cargo run -p axiom-node --release
# → "http server on 127.0.0.1:8080"

# Терминал 2
cd tools/axiom-web
npm install   # первый раз
npm run dev
# → http://localhost:5173
```

Vite проксирует `/api` → `http://127.0.0.1:8080`, включая WebSocket (`ws: true`).

### Production

```bash
cd tools/axiom-web && npm run build && cd ../..
cargo run -p axiom-node --release
# http://127.0.0.1:8080  — axiom-node раздаёт dist/
```

---

## Интерфейс

### Заголовок

```
AXIOM  [Overview] [Conversation] [Phase C ⁿ] [Patterns]   Wake  ● live
```

- Состояние Engine: `Wake` / `FallingAsleep` / `Dreaming` / `Waking` — цвет меняется
- Статус соединения: `● live` (зелёный) / `○ reconnecting` (красный)
- Цифра на вкладке Phase C — количество pending advisories, ожидающих решения

---

## Вкладки

### Overview

Общий статус системы в реальном времени.

**Metrics row** (обновляются каждый snapshot):

| Метрика | Описание |
|---------|----------|
| Tick | Номер тика |
| Event | Номер события COM |
| Hot path | Время последнего тика (µs) |
| Fatigue | Текущая усталость (%) |
| Tokens | Суммарно по всем доменам |
| Connections | Суммарно |
| Dreams | Завершённых DREAM-циклов |
| Vetoes | Вето GUARDIAN всего |

**Fatigue bar** — прогресс-бар с маркером порога. Цвет: зелёный → жёлтый (>70% порога) → красный (≥ порога).

**FrameWeaver stats** — total frames / in sutra / promotions since wake (показывается при наличии данных).

**Advisory Queue** — появляется если есть pending advisories (дублирует Phase C для быстрого доступа).

---

### Conversation

Текстовый ввод в Engine через TextPerceptor.

- Пишем текст → `Ctrl+Enter` или кнопка **Send**
- Текст отправляется в `POST /api/text/submit` → инжектируется в SUTRA domain (100)
- Запись сразу появляется в ленте (kind = `user`)

**Виды записей в ленте:**

| Kind | Цвет | Триггер |
|------|------|---------|
| user | белый | отправленный текст |
| frame | синий | FrameCrystallized (WS событие) |
| dream | жёлтый | DreamPhaseTransition (WS событие) |
| alert | красный | Alert (WS событие) |

Лента не персистентна — при перезагрузке страницы очищается.

---

### Phase C

Аксиальное состояние системы.

**Axial State card:**

- **Dominant Octant** — текущий доминирующий октант AxialEvaluator (имя из 8 вариантов)
- **Dominant Subsystem** — текущая доминирующая подсистема CR (Writing/Mathematics/Philosophy/Code/Analysis/Unknown)
- **Emergent Candidates** — число ожидающих кандидатов на промоцию

**Octant Depth bars:**

Восемь полосок — средняя глубина `SutraDepthStore` по каждому октанту. Доминирующий октант выделен синим.

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

**Emergent Candidates таблица** — sutra_id, октант обнаружения, начальная глубина, кнопка **Approve**.

**Advisory Queue** — список pending advisories (см. ниже).

---

### Patterns

Паттерны активации слоёв.

**Current Layer Activations** — гистограмма L1–L8 over-domain, нормализованная по максимуму. Цвет уникален для каждого слоя.

**Layer History** — SVG-спарклайны для L1–L8. Хранятся последние 120 snapshot. Обновляются при каждом Snapshot через WebSocket.

**Domain Activity** — сетка карточек по доменам (SUTRA, EXECUTION, ..., MAYA). Каждая карточка: имя, token count, temp avg, recent activity, mini-bar-chart по слоям.

---

## Advisory Queue

Pending advisories — рекомендации Phase C системы, ожидающие подтверждения оператором.

**Колонки таблицы:**

| Колонка | Описание |
|---------|----------|
| Type | Тип advisory (0=DepthHint, 1=OctantCorrection, 2=ConflictDiagnosis, 3=SubsystemAttribution, 4=EmergentCandidate) |
| Subject | ID sutra (Frame) |
| Conf | Уверенность советника (0.0–1.0) |
| Label | Описание: что именно предлагает советник |
| TTL | Бар: синий → красный при истечении срока жизни (TTL = 1000 event_id) |
| — | Кнопки ✓ / ✗ |

**Действия:**

- **✓** (зелёный) — Confirm: `POST /api/advisory/confirm/{id}` → advisory применяется
- **✗** (красный) — Reject: `POST /api/advisory/reject/{id}` → advisory отклоняется

После подтверждения/отклонения запись исчезает из очереди при следующем snapshot.

---

## WebSocket-протокол

Клиент подключается к `ws://[host]/api/ws`. При подключении сразу получает полный `SystemSnapshot`.

**EngineMessage (JSON):**

```typescript
| { Hello:         { version: number; capabilities: number } }
| { Snapshot:      SystemSnapshot }
| { Event:         EngineEvent }
| { CommandResult: { command_id: number; result: unknown } }
| { Bye:           { reason: string } }
```

**EngineEvent:**

```typescript
| { Tick:                 { tick: number; event: number; hot_path_ns: number } }
| { DomainActivity:       { domain_id: number; recent_activity: number; layer_activations: number[] } }
| { DreamPhaseTransition: { from: EngineState; to: EngineState; trigger: string } }
| { FrameCrystallized:    { anchor_id: number; layers_present: number; participant_count: number } }
| { FrameReactivated:     { anchor_id: number; new_temperature: number } }
| { FramePromoted:        { anchor_id: number } }
| { Alert:                { level: string; message: string } }
```

Авто-переподключение: `setTimeout(connectWS, 2000)` при `onclose`.

---

## Структура исходников

```
tools/axiom-web/
├── src/
│   ├── App.tsx                  # корневой компонент: 4 таба, заголовок
│   ├── App.css                  # все стили (монохромная тема, monospace)
│   ├── store/
│   │   └── engine.ts            # Zustand store: snapshot, feed, layerHistory(120)
│   ├── ws/
│   │   ├── client.ts            # WS подключение, авто-reconnect, обработка событий
│   │   └── protocol.ts          # TypeScript типы протокола
│   └── components/
│       ├── AdvisoryQueue.tsx    # таблица advisories с confirm/reject
│       ├── Conversation.tsx     # лента + textarea + Send
│       ├── PhaseC.tsx           # аксиальное состояние + октанты + emergent
│       └── Patterns.tsx         # sparklines + domain grid
├── vite.config.ts               # proxy /api → 127.0.0.1:8080 (dev), outDir=dist
└── package.json                 # react 18, zustand 5, vite
```

---

## axiom-node — параметры

```
--http-addr   127.0.0.1:8080        адрес HTTP-сервера
--web-dist    tools/axiom-web/dist  путь к собранному React SPA
```

axiom-node сам раздаёт `dist/` как статику. В dev-режиме Vite сервер делает это сам (proxy `/api`).
