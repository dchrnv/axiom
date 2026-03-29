# Ревизия наследия NeuroGraph → AXIOM

**Дата:** 2026-03-28  
**Назначение:** Выжимка по всем рассмотренным файлам из NeuroGraph. Что перенести, что уже покрыто, что архивировать.

---

## Сводная таблица

| Файл | Вердикт | Куда в AXIOM |
|------|---------|-------------|
| IntuitionEngine v3.0 | **Идеи перенесены** | Этап 5 роадмапа (GridHash) |
| IntuitionEngine v4.0 | **Идеи перенесены** | Этап 5 роадмапа (GridHash) |
| NeuroGraph стресс-тест 100M | **Справочный** | docs/history/ (baseline NeuroGraph) |
| KEY_V_2_0 | **Идеи перенесены** | GENOME V1.0 + GUARDIAN V1.0 |
| Genom | **Идеи перенесены** | GENOME V1.0 |
| LUT_ENGINE V1.0 | **Отложен** | Этап 10 роадмапа (SIMD/оптимизация) |
| Конкурирующие драйверы | **Уже покрыто** | ASHTI(1-8) + DomainConfig + адаптивные пороги |
| AxiomFrame | **Уже покрыто** | Token V5.2 + Shell V3.0 + UCL V2.0 |
| Универсальный Штрих-код | **Устарел** | Заменён Token V5.2 (полные поля вместо 4-bit) |
| Универсальный Язык НС | **Уже покрыто** | UCL V2.0 + External Integration Layer V1.0 |
| Axiom Semantic Core | **Design doc** | docs/architecture/semantic_foundation.md |
| Как объяснить машине | **Design doc** | docs/architecture/semantic_foundation.md |
| 12. Spatial Grid | **Уже покрыто** | Space V6.0 + Shell V3.0 |
| Симбиоз (AXIOM SHELL) | **Use case** | docs/applications/axiom_shell.md |

---

## Детали по каждому файлу

### IntuitionEngine v3.0 + v4.0

**Статус:** Ключевые идеи перенесены в Этап 5 роадмапа (GridHash).

**Что взято:**
- GridHash: Shell-профиль [u8; 8] + position → u64 через XOR + rotate_left + bit shift. O(1) lookup.
- Двухфазный поиск: Phase 1 (хэш, 30 ns) → Phase 2 (физика поля, 10 µs) при Miss.
- Confidence threshold: рефлекс не записывается в горячую таблицу после первого успеха — нужен счётчик подтверждений. Реализуется через REFLECTOR (Этап 4).
- Shift-фактор: конфигурируемый параметр квантования. Подбирается на тестах.

**Что адаптировано:**
- DashMap (lock-free) → предвыделённая таблица (zero-alloc). Нет многопоточности в ядре — не нужен lock-free.
- TTL для рефлексов → не нужен. Затухание через causal age (COM event_id). min_intensity > 0 гарантирует что следы не исчезают.
- Trusted Reflex (облегчённый Guardian) → бит GUARDIAN_CHECK_REQUIRED в DomainConfig V2.1. Если установлен — полная проверка. Если нет — доверенный рефлекс.
- "Ночной аналитик" (фоновое обучение) → DREAM(7) через Heartbeat (Этап 9 роадмапа).

**Файлы:** Архивировать в docs/history/neurograph/.

---

### KEY_V_2_0 + Genom

**Статус:** Ключевые идеи перенесены в GENOME V1.0 и GUARDIAN V1.0.

**Маппинг терминов:**

| NeuroGraph | AXIOM |
|---|---|
| CDNA (Core DNA) | GENOME — неизменяемый фундамент |
| ADNA (Active DNA) | CODEX (Домен 3) — пластичный закон |
| Genom (конституция) | GENOME (объединён с CDNA) |
| Guardian | GUARDIAN V1.0 — расширенный |
| Интуиция (мета-алгоритм) | DREAM(7) + обратная связь |
| DNA Cache | Не нужен — Arc<Genome> shared reference |
| Pub-Sub для обновлений | GenomeSubscriber trait (forward compatibility) |
| Evolution Manager | GUARDIAN (единственный писатель CODEX) |

**Что взято:**
- Иерархия стабильности: GENOME (неизменяемый) > CODEX (пластичный) > поведение доменов.
- Права доступа: матрица module × resource → permission.
- Протоколы: допустимые маршруты данных. Неописанный маршрут запрещён.
- Boot sequence: GENOME загружается первым, до всего остального.
- Подписка: модули получают обновления мгновенно, не polling.

**Файлы:** Архивировать в docs/history/neurograph/.

---

### LUT_ENGINE V1.0

**Статус:** Отложен до Этапа 10 (SIMD/оптимизация физики).

**Суть:** Замена f32::exp, деления, pow на предвычисленные таблицы 4096 элементов (16 КБ, помещается в L1 кэш). O(1) lookup с линейной интерполяцией, ~2 ns.

**Почему не сейчас:**
- check_decay = 109 ns, generate_gravity_update = 23 ns. Это уже быстро.
- LUT даст ускорение decay (если там f32::exp), но gravity уже 23 ns.
- Не bottleneck. Приоритет ниже чем GENOME, GridHash, Gateway.

**Что адаптировать при реализации:**
- `MAX_TICKS` и `ticks: f32` → заменить на causal_age (u64). LUT принимает u64 с конверсией на границе.
- `lazy_static!` → можно заменить на `const fn` или инициализацию при старте.

**Файл:** Сохранить в docs/specs/deferred/LUT_ENGINE_V1_0.md.

---

### Конкурирующие драйверы (потребности/контексты)

**Статус:** Уже покрыто архитектурой.

ASHTI(1-8) — это и есть конкурирующие "голоса". Каждый обрабатывает один паттерн по-своему. MAYA(10) собирает все ответы — конкуренция. DomainConfig определяет баланс (температура, гравитация). Адаптивные пороги через GUARDIAN (Этап 9) = переключение "режимов" системы.

Отдельный модуль не нужен.

---

### AxiomFrame + Универсальный Язык НС + Универсальный Штрих-код

**Статус:** Уже покрыто существующими спецификациями.

| Концепция NG | Реализация в AXIOM |
|---|---|
| AxiomFrame (универсальный контейнер) | Token V5.2 (64B) + Shell V3.0 (8B) |
| payload (сырые данные) | sutra_id (ссылка на SUTRA) |
| tags (семантика) | Shell-профиль L1-L8 |
| timestamp | COM event_id |
| format ("video/rgb") | type_flags в Token |
| ATP (JSON header + binary payload) | UCL Command (64B binary) |
| 4 диалекта (Stimulus→Percept→Cognition→Action) | Маршруты: Perceptor→SUTRA→EXPERIENCE→ASHTI→MAYA→Effector |
| Штрих-код (u32, 8×4 бита) | Token V5.2 (полные поля, не 4-bit) |
| Time Alignment | COM event_id (общий причинный порядок) |
| Zero-Copy (Apache Arrow) | repr(C, align(64)) + shared memory через UCL |

Отдельные модули не нужны.

---

### Axiom Semantic Core + Как объяснить машине

**Статус:** Design documents — философское обоснование осей.

**Ценность:** Объясняют *почему* оси именно такие:
- X (Аполлон/Дионис) = энтропия/предсказуемость.
- Y (Эрос/Танатос) = когерентность/разрушение.
- Z (Власть/Ничто) = магнитуда сигнала/пассивность.

**Применение:** Справочник при настройке DomainConfig (axis_x_ref, axis_y_ref, axis_z_ref) и SemanticContributionTable (Shell V3.0). Не код — обоснование.

**Файлы:** Объединить в docs/architecture/semantic_foundation.md.

---

### 12. Spatial Grid (NeuroGraph)

**Статус:** Уже покрыто Space V6.0 + Shell V3.0.

| NG Spatial Grid | AXIOM |
|---|---|
| 8 уровней координат (Point3D × 8) | position [i16; 3] в домене + Shell [u8; 8] |
| SparseGrid | SpatialHashGrid (zero-alloc) |
| R-tree | Не нужен — spatial hash достаточен для текущих масштабов |
| MultiCoordinate | Shell-профиль (8 измерений как u8, не как 3D координаты) |
| float координаты | i16 целочисленные (детерминизм) |

Потеря: в NG каждый уровень имел свои 3D координаты. В AXIOM — одно значение u8 на слой в Shell. Это осознанное сжатие: Shell — кэш для быстрого сравнения, не для навигации. Для навигации есть position в поле домена.

---

### Симбиоз (AXIOM SHELL Exoskeleton)

**Статус:** Use case для будущей реализации (после Этапа 7, External Integration Layer).

**Суть:** AXIOM как нервная система операционной среды. Hyprland/QTile управляется ядром. 8 рабочих столов = 8 октантов ASHTI. Eww виджеты показывают состояние "мозга".

**Как ложится на архитектуру:**
- Хоткеи → KeyboardPerceptor → UCL Command → ядро.
- Команды WM → HyprlandEffector → `hyprctl`.
- Виджеты → WebSocketChannel → подписка на события MAYA.
- SetFocus(domain_id) → изменение приоритетов Arbiter.

**Файл:** Сохранить в docs/applications/axiom_shell.md.

---

## Рекомендуемая структура docs/

```
docs/
├── specs/                          # Актуальные спецификации
│   ├── Token_V5_2.md
│   ├── GENOME_V1_0.md
│   ├── GUARDIAN_V1_0.md
│   ├── ... (все активные спеки)
│   └── deferred/
│       └── LUT_ENGINE_V1_0.md      # Отложен до Этапа 10
│
├── architecture/
│   ├── semantic_foundation.md      # Из Axiom Semantic Core + Как объяснить машине
│   ├── dependency_graph.md
│   └── migration_log.md
│
├── applications/
│   └── axiom_shell.md              # Из Симбиоз
│
└── history/
    └── neurograph/                 # Архив NG документов
        ├── IntuitionEngine_v3_0.md
        ├── IntuitionEngine_v4_0.md
        ├── KEY_V_2_0.md
        ├── Genom.md
        ├── AxiomFrame.md
        ├── ATP.md
        ├── SpatialGrid.md
        └── StressTest_100M.md
```

---

## Итог

Из 14 рассмотренных файлов:
- **4** содержали идеи, перенесённые в новые спецификации (IntuitionEngine → GridHash, KEY/Genom → GENOME/GUARDIAN).
- **1** отложен на будущее (LUT_ENGINE).
- **2** сохраняются как design/use case документы (Semantic Core, Симбиоз).
- **7** полностью покрыты существующей архитектурой AXIOM и могут быть архивированы.

Наследие NeuroGraph обработано. Всё ценное — в спецификациях или роадмапе.
