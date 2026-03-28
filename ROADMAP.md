# Axiom Roadmap

**Версия:** 10.0
**Дата:** 2026-03-28
**Спека:** [docs/spec/Roadmap_Ashti_Core_V2_1.md](docs/spec/Roadmap_Ashti_Core_V2_1.md)

---

## ~~✅ Этап 1: GENOME + GUARDIAN~~ — ЗАВЕРШЁН (426 тестов)

---

## 🔄 Этап 2: Storm Control

**Цель:** Защита от причинных лавин. CausalFrontier получает state machine, budget, size limit.

**Важно:** Базовая структура CausalFrontier V2.0 уже в коде. Это расширение.

### Шаг 1 — FrontierState machine

Добавить состояния: `Empty → Active → Storm → Stabilizing → Idle`. Переходы по правилам из спеки.

### Шаг 2 — Causal Budget

`max_events_per_cycle` — жёсткий лимит. `pop()` возвращает `None` при исчерпании бюджета. Frontier сохраняется до следующего цикла.

### Шаг 3 — Frontier Size Limit

`max_frontier_size` — предохранитель. `push()` отбрасывает при переполнении (Heartbeat подхватит позже).

### Шаг 4 — FrontierConfig пресеты

Конфигурация: `tight / medium / wide`. Параметры: `max_frontier_size`, `max_events_per_cycle`, `storm_threshold`.

Тесты: state machine transitions, budget enforcement, size limit, storm detection, детерминизм.

**Критерий:** Система не зависает при каскаде из 10 000+ событий.

---

## 🔮 Этап 3: EXPERIENCE(9) — REFLECTOR + SKILLSET

**Цель:** Домен 9 учится на опыте и кристаллизует навыки.

### Шаг 1 — REFLECTOR

`ReflexStats` per-reflex, `DomainProfile` по Shell-профилям. Обновление при обратной связи MAYA → Arbiter → EXPERIENCE.

### Шаг 2 — SKILLSET

`Skill` — кристаллизованный кластер токенов + связей. Критерии кристаллизации: min weight, N подтверждений, устойчивость кластера.

### Шаг 3 — resonance_search + SKILLSET

Если паттерн резонирует со скиллом — скилл возвращается как единый ответ. Экспорт/импорт скиллов.

**Критерий:** Рефлексы имеют статистику. Кластеры кристаллизуются. Скиллы активируются.

---

## 🔮 Этап 4: GridHash-индекс

**Цель:** resonance_search от O(N) до O(1) для знакомых ситуаций. Целевое время: 30-50 ns.

### Шаг 1 — GridHash функция

Хэш Shell-профиля [u8; 8] + position → u64. Только целочисленная арифметика, без аллокаций.

### Шаг 2 — AssociativeIndex

Предвыделённая хэш-таблица в DomainState домена 9. Ключ: u64. Значение: `SmallVec<[u32; 4]>`.

### Шаг 3 — Двухфазный resonance_search

Phase 1 (GridHash): O(1) lookup → ранний выход при Hit + weight ≥ threshold.
Phase 2 (физика): полный поиск при Miss. Физика сохраняется, GridHash дополняет.

**Критерий:** Знакомые ситуации за 30-50 ns. Полный pipeline < 35 µs (улучшение от текущих ~40 µs).

---

## 🔮 Этап 5: Адаптивные пороги

**Цель:** Система адаптируется к опыту. REFLECTOR данные влияют на DomainConfig через GUARDIAN.

- Адаптация `reflex_threshold` по статистике REFLECTOR
- `DomainConfig` изменяется в рантайме через GUARDIAN (COM-событие `DomainConfigUpdated`)
- DREAM(7) как фоновый оптимизатор через Heartbeat

---

## 🔮 Этап 6: Causal Horizon + Масштабирование

**Цель:** Долгие запуски без роста памяти. Фракталы.

- Causal Horizon: вычисление горизонта, архивация истории
- Event Log pruning после snapshot
- Фрактальные уровни: MAYA одного уровня → SUTRA следующего
- Экспорт/импорт SKILLSET между экземплярами

---

## 🗂 Configuration System (параллельный трек)

Не блокирует этапы выше, реализуется по мере готовности.

### Шаг 1 — DomainConfig из YAML (DEFERRED 3.2)

`DomainConfig::from_yaml(path)` + загрузка пресетов из `config/presets/`. Паттерн отработан на Genome::from_yaml.

### Шаг 2 — Shell semantic_contributions.yaml (DEFERRED 3.5)

Снять hardcode из Shell V3.0. `config/schema/semantic_contributions.yaml`. Shell::from_yaml.

### Шаг 3 — Spatial YAML конфиг (DEFERRED 3.1)

YAML конфигурация пространственных параметров. Пресеты: tight/medium/loose.

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-28
