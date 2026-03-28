# Axiom Roadmap

**Версия:** 10.3
**Дата:** 2026-03-28
**Спека:** [docs/spec/Roadmap_Ashti_Core_V2_1.md](docs/spec/Roadmap_Ashti_Core_V2_1.md)

---

## ~~✅ Этап 1: GENOME + GUARDIAN~~ — ЗАВЕРШЁН (426 тестов)

Бенчмарки (цели): `enforce_access` < 5 ns, `validate_reflex` < 500 ns, pipeline overhead vs ~40 µs.

---

## ~~✅ Этап 2: Storm Control~~ — ЗАВЕРШЁН (430 тестов)

**Цель:** Защита от причинных лавин. CausalFrontier получает state machine, budget, size limit.
**Важно:** CausalFrontier V2.0 уже в коде — это расширение.

### Шаг 1 — FrontierState machine
`Empty → Active → Storm → Stabilizing → Idle`. Переходы по правилам спеки (раздел 8).

### Шаг 2 — Causal Budget
`max_events_per_cycle` — жёсткий лимит. `pop()` → `None` при исчерпании. Frontier сохраняется до следующего цикла.

### Шаг 3 — Frontier Size Limit
`max_frontier_size` — предохранитель. `push()` отбрасывает при переполнении (Heartbeat подхватит позже).

### Шаг 4 — Batch Events *(опционально)*
При состоянии Storm: объединение однотипных событий (100× TokenMoved → 1× BatchTokenMoved).

### Шаг 5 — FrontierConfig пресеты
`tight / medium / wide`. Параметры: `max_frontier_size`, `max_events_per_cycle`, `storm_threshold`, `enable_batch_events`.

Тесты: state machine transitions, budget enforcement, size limit, storm detection, детерминизм.

**Бенчмарки:** TickForward с 1000+ токенами под Storm Control. Overhead state machine transitions.

**Критерий:** Система не зависает при каскаде из 10 000+ событий. Budget и size limit работают.

---

## 🔮 Этап 3: Configuration System

**Цель:** Снять hardcode. DomainConfig, Shell, Space — все параметры загружаются из YAML.
**Почему здесь:** Этап 6 меняет DomainConfig в рантайме — нужна YAML-инфраструктура до этого.
**Паттерн:** `Genome::from_yaml` уже отработан — применяем к остальным структурам.

### ~~✅ Шаг 1 — DomainConfig из YAML~~ — ЗАВЕРШЁН (443 тестов)
`DomainConfig::from_yaml(path)`. 11 YAML пресетов в `config/presets/domains/`. Валидация через `validate()`. 13 новых тестов.

### ~~✅ Шаг 2 — Shell semantic_contributions.yaml~~ — ЗАВЕРШЁН (448 тестов)
`config/schema/semantic_contributions.yaml`. `SemanticContributionTable::from_yaml(path)`. 5 новых тестов.

### ~~✅ Шаг 3 — Spatial YAML конфиг~~ — ЗАВЕРШЁН (460 тестов)
`SpatialConfig` + `from_yaml`. `SpatialHashGrid::with_config`. Пресеты: `tight/medium/loose`. 12 новых тестов.

### ~~✅ Шаг 4 — Единый ConfigLoader~~ — ЗАВЕРШЁН (469 тестов)
`ConfigLoader::load_all(axiom.yaml)` загружает все компоненты. `PresetsConfig` + `LoadedAxiomConfig`. 9 новых тестов.

**Критерий:** Все основные параметры системы загружаются из файлов.

---

## ~~✅ Этап 4: EXPERIENCE(9) — REFLECTOR + SKILLSET~~ — ЗАВЕРШЁН (496 тестов)

**Цель:** Домен 9 учится на опыте и кристаллизует навыки.

### Шаг 1 — REFLECTOR
`ReflexStats` per-reflex, `DomainProfile` по Shell-профилям (L1-L8). Обновление при обратной связи MAYA → Arbiter → EXPERIENCE. Интерфейс для GUARDIAN: данные для адаптации порогов.

### Шаг 2 — SKILLSET
`Skill` — кристаллизованный кластер токенов + связей. Критерии: min weight, N подтверждений, устойчивость кластера (все связи > threshold). Экспорт/импорт скиллов.

### Шаг 3 — resonance_search + SKILLSET
Если паттерн резонирует со скиллом — скилл возвращается как единый ответ.

Тесты: REFLECTOR статистика + обновление, кристаллизация кластера, активация скилла, экспорт/импорт.

**Бенчмарки:** resonance_search с SKILLSET vs без. Overhead REFLECTOR при обратной связи.

**Критерий:** Рефлексы имеют статистику. Кластеры кристаллизуются. Скиллы активируются.

---

## ~~✅ Этап 5: GridHash-индекс~~ — ЗАВЕРШЁН (519 тестов)

**Цель:** resonance_search от O(N) до O(1) для знакомых ситуаций.

### Шаг 1 — GridHash функция
Хэш Shell-профиля `[u8; 8]` + position → `u64`. Побитовый сдвиг (shift-фактор), XOR, rotate_left. Только целочисленная арифметика.

### Шаг 2 — AssociativeIndex
Предвыделённая хэш-таблица в DomainState домена 9. Ключ: `u64`. Значение: `SmallVec<[u32; 4]>`. Zero-alloc.

### Шаг 3 — Двухфазный resonance_search
Phase 1 (GridHash): O(1) lookup → ранний выход при Hit + weight ≥ threshold.
Phase 2 (физика): полный поиск при Miss. Физика сохраняется, GridHash дополняет.

### Шаг 4 — Обучение индекса
При записи опыта в EXPERIENCE(9): вычислить GridHash, добавить в AssociativeIndex. При затухании ниже порога — удалить.

### Шаг 5 — Shift-фактор
Конфигурируемый параметр. Подбирается на тестах: слишком мал → постоянные Miss, слишком велик → путает ситуации.

Тесты: детерминизм, распределение коллизий, Hit/Miss при разных shift-факторах, корректность при Hit и Miss.

**Бенчмарки:** GridHash computation < 30 ns. resonance_search с GridHash vs без (цель: 10x+ при Hit). Full pipeline < 35 µs (улучшение от ~40 µs).

**Критерий:** Знакомые ситуации за 30-50 ns. Незнакомые — как раньше.

---

## ~~✅ Этап 6: Адаптивные пороги~~ — ЗАВЕРШЁН (533 тестов)

**Цель:** Система адаптируется к опыту. REFLECTOR данные влияют на DomainConfig через GUARDIAN.

### Шаг 1 — Адаптация reflex_threshold
`Guardian::adapt_thresholds(stats, configs)` — читает REFLECTOR, корректирует `reflex_threshold` в DomainConfig. `AshtiCore::apply_experience_thresholds()` — передаёт новые пороги в Experience модуль.

### Шаг 2 — Динамическая реконфигурация доменов
`Guardian::adapt_domain_physics(stats, configs)` — адаптирует `temperature` и `resonance_freq` по success_rate.

### Шаг 3 — DREAM(7) как оптимизатор
`Guardian::dream_propose(candidates)` — принимает высокоактивные паттерны из Experience, возвращает `CodexAction::AddRule`. `AxiomEngine::dream_propose()` — извлекает кандидатов (weight ≥ 0.9, success_count ≥ 5).

`AxiomEngine::run_adaptation()` — единый цикл: Шаг 1 + Шаг 2 + apply_experience_thresholds. 14 новых тестов.

---

## 🔮 Этап 7: Causal Horizon + Масштабирование

**Цель:** Долгие запуски без роста памяти. Фрактальные уровни.

### ~~✅ Шаг 1 — Causal Horizon~~ — ЗАВЕРШЁН (548 тестов)
`CausalHorizon` в `axiom-domain`: `compute(states)` = min(token.last_event_id), монотонный `advance()`, `is_behind()`. `Experience::archive_behind_horizon(horizon)` — удаляет стары следы + чистит AssociativeIndex. `AshtiCore::compute_horizon()` + `run_horizon_gc()`. `AxiomEngine::causal_horizon()` + `run_horizon_gc()`. 15 новых тестов.

### ~~✅ Шаг 2 — Event Log pruning~~ — ЗАВЕРШЁН (558 тестов)
`EngineSnapshot::snapshot_event_id()` — граница pruning. `snapshot()` теперь заполняет `created_at = causal_horizon()`. `AxiomEngine::prune_after_snapshot(snap)` — удаляет Experience следы с `last_used < snap.created_at`. `AxiomEngine::snapshot_and_prune()` — атомарный snapshot + prune. `Experience::prunable_count(horizon)` — инспекция. 10 новых тестов.

### ~~Шаг 3 — Фрактальные уровни~~ → DEFERRED
Перенесено в DEFERRED.md. Преждевременное усложнение модели на данном этапе.

### ~~✅ Шаг 4 — Обмен скиллами~~ — ЗАВЕРШЁН (568 тестов)
`SkillSet::export()`, `import_batch(&[Skill]) -> usize` (дедупликация + вес × 0.3), `clear()`. `AshtiCore::export_skills()` + `import_skills()`. `AxiomEngine::export_skills()` + `import_skills()`. 10 новых тестов.

Тесты: horizon computation, event log pruning + restore, двухуровневая цепочка 10→0, экспорт/импорт скиллов.

**Критерий:** Система работает часами без роста памяти. Фрактальная цепочка функционирует.

---

## ~~✅ Этап 8: External Integration Layer~~ — ЗАВЕРШЁН (590 тестов)

**Цель:** Внешние системы взаимодействуют с AXIOM.

### ~~✅ Gateway + Channel~~ — ЗАВЕРШЁН
- **Gateway** — единая точка входа: владеет `AxiomEngine`, маршрутизирует команды, уведомляет `EventObserver`
- **Channel** — in-process очередь: `send(cmd)` + `drain_events()`, счётчик обработанных команд
- **Adapters** — `RuntimeAdapter` + `EventObserver` trait-границы для транспортных имплементаций
- **22 новых теста**: Gateway (process, process_with, observers), Channel (FIFO, drain, clear), process_channel

### ~~REST/gRPC/Python~~ → DEFERRED
Перенесено в DEFERRED.md. Требуют тяжёлых внешних crates (axum, tonic, pyo3).
Текущие trait-границы (`RuntimeAdapter`, `EventObserver`) — точки расширения для будущих транспортов.

---

## Сводка

| Этап | Название | Ключевой результат | Статус |
|------|----------|--------------------|--------|
| 1 | GENOME + GUARDIAN | Конституция, контроль доступа | ✅ 426 тестов |
| 2 | Storm Control | Защита от каскадов, state machine | ✅ 430 тестов |
| 3 | Configuration System | YAML для всего, снять hardcode | ✅ 469 тестов |
| 4 | REFLECTOR + SKILLSET | Статистика, кристаллизация скиллов | ✅ 496 тестов |
| 5 | GridHash | O(1) fast path, < 35 µs pipeline | ✅ 519 тестов |
| 6 | Адаптивные пороги | Самонастройка, DREAM(7) | ✅ 533 тестов |
| 7 | Causal Horizon | Долгий запуск, обмен скиллами | ✅ 568 тестов |
| 8 | External Integration | Gateway, Channel, adapter traits | ✅ 590 тестов |

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-28 (Этап 8 завершён)
