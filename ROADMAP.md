# Axiom Roadmap

**Версия:** 22.0
**Дата:** 2026-04-06

---

## Текущая задача: Memory Persistence V1.0

**Спека:** [docs/spec/Memory_Persistence_V1_0.md](docs/spec/Memory_Persistence_V1_0.md)

Новый crate `axiom-persist`. Граф зависимостей:
```
axiom-core (serde feature) ←── axiom-persist ──→ axiom-runtime
                                     ↑
                               axiom-agent (CLI команды :save/:load)
```
axiom-runtime НЕ зависит от axiom-persist. Persist читает из runtime, не наоборот.

---

### Фаза 1: Формат + `:save` + `:load` [DONE]

**Crates:** axiom-core, axiom-persist (новый), axiom-agent

**Подготовка:**
- [ ] `bincode = "2"` в `[workspace.dependencies]`
- [ ] `axiom-persist` добавить в workspace members
- [ ] `serde` feature в `axiom-core`:
  - `serde` как optional dependency
  - `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]` на Token, Connection, Event
  - Без feature — компилируется как раньше (zero deps)

**axiom-persist:**
- [ ] `MemoryManifest` — YAML-структура (version, checksum, tick_count, com_next_id, contents)
- [ ] `MemoryWriter::save(snapshot, path)` — сериализация по порядку (engine_state → tokens → connections → traces → manifest)
- [ ] `MemoryLoader::load(path)` — десериализация + import weight factor (0.7×) для traces
- [ ] Структура на диске: `axiom-data/{meta,tokens,traces,codex,map,context,reflector,config}/`
- [ ] Ошибки: `PersistError` (NotFound, CorruptManifest, VersionMismatch, Io, Decode)

**axiom-agent (CLI):**
- [ ] `:save [path]` — сохранить snapshot через `MemoryWriter`
- [ ] `:load [path]` — загрузить через `MemoryLoader`, применить к engine
- [ ] `:memory` — показать что в памяти (скиллы, трейсы, dirty)
- [ ] При `:quit` — автоматический `:save` если persistence включён

**Тесты (axiom-persist):**
- [ ] `save → load` — snapshot восстанавливается идентично
- [ ] Traces при загрузке получают weight × 0.7
- [ ] Повреждённый manifest → `PersistError::CorruptManifest`
- [ ] Несовместимая версия формата → `PersistError::VersionMismatch`
- [ ] Пустой engine → save → load → engine пустой

---

### Фаза 2: Boot sequence — загрузка при старте [DONE]

**Crates:** axiom-persist, axiom-agent (bin/axiom-cli.rs)

- [ ] `bin/axiom-cli.rs`: при старте проверять `axiom-data/manifest.yaml`
  - Есть и валидный → `MemoryLoader::load()` → применить к engine
  - Нет или повреждён → чистый старт (warning в stderr)
- [ ] Флаг `--data-dir <path>` в CLI (default: `./axiom-data`)
- [ ] Флаг `--no-load` для принудительного чистого старта
- [ ] После загрузки: пересчёт Shell из Connection (если есть API), перестройка SpatialHashGrid
- [ ] Banner: показывать "Loaded from axiom-data (tick=N, traces=M)" или "Fresh start"

**Тесты:**
- [ ] save → restart (новый engine) → load → рефлексы работают (traces подтягиваются)
- [ ] Отсутствие `axiom-data/` → чистый старт без паники
- [ ] `--no-load` → чистый старт даже при наличии axiom-data

---

### Фаза 3: Автосохранение (кристаллизация) [DONE]

**Crates:** axiom-persist, axiom-runtime (PersistenceConfig), axiom-agent

- [ ] `PersistenceConfig` в runtime config (enabled, data_dir, thresholds, intervals)
- [ ] Dirty buffer в axiom-persist: отслеживать изменённые traces/rules с момента последнего flush
- [ ] Интеграция с TickSchedule: новый интервал `persist_check_interval`
- [ ] Flush logic: `dirty_count > flush_threshold` ИЛИ `tick_count - last_flush > max_interval`
- [ ] Порядок записи (атомарность manifest): engine_state → skills → codex → context → reflector → traces → map → manifest
- [ ] Скилл записывается немедленно при кристаллизации (write_on_crystallize)

**Тесты:**
- [ ] Кристаллизация скилла → файл появляется на диске без явного `:save`
- [ ] Рестарт → скилл загружается из файла
- [ ] Прерванная запись (manifest не обновлён) → загрузка откатывается к предыдущему состоянию

---

### Фаза 4: Обмен знаниями [DONE]

**Crates:** axiom-persist, axiom-agent (CLI)

- [ ] `:export skills <path>` — экспорт SKILLSET в отдельную директорию
- [ ] `:export traces <path>` — экспорт трейсов > threshold
- [ ] `:import skills <path>` — импорт с `IMPORT_WEIGHT_FACTOR = 0.7`
- [ ] GUARDIAN-валидация импорта: проверка CODEX-совместимости
- [ ] `imported/` субдиректория: система распознаёт и применяет weight factor

**Тесты:**
- [ ] export из A → import в B → скиллы активируются (но с пониженным weight)
- [ ] GUARDIAN блокирует CODEX-нарушающий импорт

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
