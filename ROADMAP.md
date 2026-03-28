# Axiom Roadmap

**Версия:** 10.1
**Дата:** 2026-03-28
**Спека:** [docs/spec/Roadmap_Ashti_Core_V2_1.md](docs/spec/Roadmap_Ashti_Core_V2_1.md)

---

## ~~✅ Этап 1: GENOME + GUARDIAN~~ — ЗАВЕРШЁН (426 тестов)

---

## 🔄 Этап 2: Storm Control

**Цель:** Защита от причинных лавин. CausalFrontier получает state machine, budget, size limit.
**Важно:** CausalFrontier V2.0 уже в коде — это расширение.

### Шаг 1 — FrontierState machine
`Empty → Active → Storm → Stabilizing → Idle`. Переходы по правилам из спеки.

### Шаг 2 — Causal Budget
`max_events_per_cycle` — жёсткий лимит. `pop()` → `None` при исчерпании. Frontier сохраняется до следующего цикла.

### Шаг 3 — Frontier Size Limit
`max_frontier_size` — предохранитель. `push()` отбрасывает при переполнении (Heartbeat подхватит позже).

### Шаг 4 — FrontierConfig пресеты
`tight / medium / wide`. Параметры: `max_frontier_size`, `max_events_per_cycle`, `storm_threshold`.

Тесты: state machine transitions, budget enforcement, size limit, storm detection, детерминизм.

**Критерий:** Система не зависает при каскаде из 10 000+ событий.

---

## 🔮 Этап 3: Configuration System

**Цель:** Снять hardcode. DomainConfig, Shell, Space — все параметры загружаются из YAML.
**Почему здесь:** Этап 5 меняет DomainConfig в рантайме — нужна YAML-инфраструктура. Этапы 4-5 тестируются с разными конфигами доменов.
**Паттерн:** `Genome::from_yaml` уже отработан — применяем к остальным структурам.

### Шаг 1 — DomainConfig из YAML (DEFERRED 3.2)
`DomainConfig::from_yaml(path)`. Загрузка пресетов из `config/presets/`. Валидация через существующий `validate()`.

### Шаг 2 — Shell semantic_contributions.yaml (DEFERRED 3.5)
Снять hardcode из Shell V3.0. `config/schema/semantic_contributions.yaml`. `Shell::from_yaml(path)`.

### Шаг 3 — Spatial YAML конфиг (DEFERRED 3.1)
YAML конфигурация пространственных параметров SpatialHashGrid. Пресеты: tight/medium/loose. Согласование с DomainConfig 128-byte constraint.

### Шаг 4 — Единый ConfigLoader
Интеграция всех конфигов в `ConfigLoader`. Загрузка через `axiom.yaml`. Кэширование.

**Критерий:** Все основные параметры системы загружаются из файлов. Нет захардкоженных значений в production-пути.

---

## 🔮 Этап 4: EXPERIENCE(9) — REFLECTOR + SKILLSET

**Цель:** Домен 9 учится на опыте и кристаллизует навыки.

### Шаг 1 — REFLECTOR
`ReflexStats` per-reflex, `DomainProfile` по Shell-профилям. Обновление при обратной связи MAYA → Arbiter → EXPERIENCE.

### Шаг 2 — SKILLSET
`Skill` — кристаллизованный кластер токенов + связей. Критерии: min weight, N подтверждений, устойчивость кластера.

### Шаг 3 — resonance_search + SKILLSET
Если паттерн резонирует со скиллом — скилл возвращается как единый ответ. Экспорт/импорт скиллов.

**Критерий:** Рефлексы имеют статистику. Кластеры кристаллизуются. Скиллы активируются.

---

## 🔮 Этап 5: GridHash-индекс

**Цель:** resonance_search от O(N) до O(1) для знакомых ситуаций. Целевое время: 30-50 ns.

### Шаг 1 — GridHash функция
Хэш Shell-профиля `[u8; 8]` + position → `u64`. Только целочисленная арифметика.

### Шаг 2 — AssociativeIndex
Предвыделённая хэш-таблица в DomainState домена 9. Ключ: `u64`. Значение: `SmallVec<[u32; 4]>`.

### Шаг 3 — Двухфазный resonance_search
Phase 1 (GridHash): O(1) lookup → ранний выход при Hit + weight ≥ threshold.
Phase 2 (физика): полный поиск при Miss. Физика сохраняется.

**Критерий:** Знакомые ситуации за 30-50 ns. Полный pipeline < 35 µs (от текущих ~40 µs).

---

## 🔮 Этап 6: Адаптивные пороги

**Цель:** Система адаптируется к опыту. REFLECTOR данные влияют на DomainConfig через GUARDIAN.
**Требует:** Этапы 3 (config инфраструктура) + 4 (REFLECTOR статистика).

### Шаг 1 — Адаптация reflex_threshold
GUARDIAN читает статистику REFLECTOR → корректирует `reflex_threshold` в DomainConfig → COM-событие `DomainConfigUpdated` → Arbiter перечитывает.

### Шаг 2 — Динамическая реконфигурация доменов
Температура, гравитация, `resonance_freq` адаптируются к нагрузке и качеству обработки.

### Шаг 3 — DREAM(7) как оптимизатор
DREAM(7) в фоне (через Heartbeat) анализирует следы в EXPERIENCE(9), предлагает изменения CODEX через GUARDIAN.

**Критерий:** Пороги адаптируются. DomainConfig меняется в рантайме. DREAM генерирует предложения.

---

## 🔮 Этап 7: Causal Horizon + Масштабирование

**Цель:** Долгие запуски без роста памяти. Фрактальные уровни.

### Шаг 1 — Causal Horizon
Вычисление `horizon = min(last_event_id)` по всем активным сущностям. Архивация истории за горизонтом.

### Шаг 2 — Event Log pruning
Snapshot фиксирует состояние. События до `snapshot_event_id` удаляются. Опционально: сжатие на диск.

### Шаг 3 — Фрактальные уровни
Протокол 10→0: MAYA одного уровня → SUTRA следующего. Несколько уровней Ashti_Core.

### Шаг 4 — Обмен скиллами
Экспорт/импорт SKILLSET между экземплярами. Импортированные скиллы начинают с низким weight.

**Критерий:** Система работает часами без роста памяти. Фрактальная цепочка функционирует.

---

## 🔮 Этап 8: External Integration Layer

**Цель:** Внешние системы могут взаимодействовать с AXIOM.
**Спека:** После стабилизации Этапов 1-7.

- **Gateway** — единая точка входа для внешних запросов
- **Perceptors** — адаптеры входящих данных (REST, gRPC, Python bindings)
- **Effectors** — адаптеры исходящих действий
- **Channels** — асинхронные каналы между уровнями

---

## Сводка

| Этап | Название | Ключевой результат | Статус |
|------|----------|--------------------|--------|
| 1 | GENOME + GUARDIAN | Конституция, контроль доступа | ✅ |
| 2 | Storm Control | Защита от каскадов | 🔄 |
| 3 | Configuration System | YAML для всего, снять hardcode | 🔮 |
| 4 | REFLECTOR + SKILLSET | Статистика, кристаллизация скиллов | 🔮 |
| 5 | GridHash | O(1) fast path | 🔮 |
| 6 | Адаптивные пороги | Самонастройка системы | 🔮 |
| 7 | Causal Horizon | Долгий запуск, фракталы | 🔮 |
| 8 | External Integration | REST, gRPC, Python bindings | 🔮 |

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-28
