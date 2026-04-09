# Axiom Roadmap

**Версия:** 24.0
**Дата:** 2026-04-09

---

## Текущее состояние

Все этапы до Axiom Sentinel V1.0 включительно завершены. 900 тестов.

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent
                                                (axiom-cli)
```

---

## Следующий этап: Axiom Sentinel V1.0

**Спека:** [docs/architecture/Axiom Sentinel v1.0.md](docs/architecture/Axiom%20Sentinel%20v1.0.md)

Цель: адаптивный движок с аппаратной осведомлённостью и параллельной обработкой.  
Целевой показатель: «мыслительный акт» ≤ 1 ms при любой нагрузке.

**Доступные зависимости (в cargo-кэше):** `rayon 1.11.0`

---

### Фаза 1: Hardware-Aware Topology

**Crates:** axiom-runtime

- [ ] `AxiomEngine::new()`: `std::thread::available_parallelism()` → `worker_count`
- [ ] Хранить `worker_count` в `AxiomEngine` (или `EngineConfig`)
- [ ] `rayon::ThreadPool::new(worker_count - 1)` — 1 поток резервируется под ОС/Gateway
- [ ] Вынести `ThreadPool` в `AxiomEngine`, переиспользовать во всех параллельных операциях
- [ ] Тесты: `worker_count >= 1`, Engine не паникует при `worker_count = 1`

---

### Фаза 2: Parallel Resonance Search

**Crates:** axiom-arbiter

- [ ] `Experience::resonance_search_parallel(token, pool)` через `rayon::scope`
- [ ] Чанки по `L2_CACHE_BYTES / size_of::<ExperienceTrace>()` (L2 = 512 KB → ~8K traces/chunk)
- [ ] Reduce без mutex: `par_iter().max_by_key(|t| score(t, token))`
- [ ] Переключатель: `traces.len() < 512` → последовательный путь (без overhead rayon)
- [ ] Тесты: `parallel_result == sequential_result` для идентичных данных

---

### Фаза 3: Variable Tick Rate ✅

**Crates:** axiom-runtime, axiom-agent

- [x] `AdaptiveTickRate` в `TickSchedule`: `min_hz`, `max_hz`, `current_hz`
- [x] Повышение hz: `tension_count > threshold` ИЛИ `MultiPass` ИЛИ внешний ввод
- [x] Понижение hz: N тиков без ввода + без tension traces (cooldown)
- [x] `CliChannel`: пересчитывать `tokio::time::interval` при изменении `current_hz`
- [x] Команда `:tickrate` — показать текущий hz и причину
- [x] Тесты: hz повышается при tension, снижается при idle N тиков

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
