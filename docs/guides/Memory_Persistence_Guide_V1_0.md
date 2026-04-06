# Memory Persistence — Руководство V1.0

**Версия:** 1.0  
**Дата:** 2026-04-06  
**Реализация:** Memory Persistence V1.0 (4 фазы)  
**Crate:** `axiom-persist`

---

## 1. Что это

Memory Persistence — слой сохранения и восстановления состояния AxiomEngine между запусками. Система живёт в отдельном crate `axiom-persist` чтобы не загрязнять ядро файловым I/O и serde-зависимостями.

**Граница ответственности:**

```
axiom-core   (Token, Connection — типы, serde feature)
axiom-arbiter (ExperienceTrace, Skill — типы, serde feature)
axiom-runtime (AxiomEngine, snapshot() / restore_from())
        ↑
axiom-persist ← читает состояние из runtime, пишет на диск
        ↑
axiom-agent  (CLI команды :save :load :autosave :export :import)
```

`axiom-runtime` НЕ зависит от `axiom-persist`. Направление зависимости — только вверх.

---

## 2. Архитектура хранилища

### Структура директории (`axiom-data/`)

```
axiom-data/
├── engine_state.json      ← основной файл: токены, связи, traces
└── manifest.yaml          ← маркер успешной записи (последний!)
```

Запись **атомарная**:
1. `engine_state.json.tmp` — сначала во временный файл
2. `rename(.tmp → engine_state.json)` — атомарно на Linux/POSIX
3. `manifest.yaml` — последним (маркер успеха)

Если процесс упал до записи manifest → при следующем старте load невозможен → чистый старт (корректное поведение).

### Формат `engine_state.json`

```json
{
  "tick_count": 42000,
  "com_next_id": 15300,
  "domains": [
    {
      "domain_id": 100,
      "tokens": [...],
      "connections": [...]
    }
  ],
  "traces": [
    {
      "pattern": { "sutra_id": 1, "domain_id": 100, ... },
      "weight": 0.73,
      "created_at": 1001,
      "last_used": 14500,
      "success_count": 12,
      "pattern_hash": 123456789
    }
  ],
  "tension": [...]
}
```

### Формат `manifest.yaml`

```yaml
version: axiom-memory-v1
created_at: '2026-04-06'
last_saved: '2026-04-06'
tick_count: 42000
com_next_id: 15300
axiom_version: 0.1.0
contents:
  domains: 11
  tokens: 127
  connections: 0
  traces: 8
  tension_traces: 2
```

`version` проверяется при загрузке — несовместимость → `PersistError::VersionMismatch`.

---

## 3. CLI — команды

### :save [path]

Сохранить текущее состояние Engine на диск.

```
:save                   → сохранить в axiom-data/
:save /tmp/my-session   → сохранить в /tmp/my-session/
```

Вывод: `saved to axiom-data (tick=1200, traces=5, tokens=33)`

### :load [path]

Загрузить состояние с диска. Engine заменяется целиком.  
TickSchedule после load восстанавливается из конфига (не перезаписывается из файла).

```
:load                   → загрузить из axiom-data/
:load /tmp/my-session   → загрузить из /tmp/my-session/
```

Вывод: `loaded from axiom-data (tick=1200, traces=5, tension=0)`

⚠️ **IMPORT_WEIGHT_FACTOR = 0.7** — загруженные traces получают `weight × 0.7`. Система должна подтвердить опыт новой обработкой перед полным усилением.

### :memory

Статистика текущего состояния памяти (без I/O).

```
  tick_count:  42000
  tokens:      127
  connections: 0
  traces:      8
  tension:     2
```

### :autosave [on N | off]

Автосохранение по интервалу тиков.

```
:autosave               → показать статус
:autosave on 1000       → сохранять каждые 1000 тиков
:autosave on 500        → сохранять каждые 500 тиков
:autosave off           → отключить
```

При `:quit` — автоматически сохраняет если autosave включён.

В `axiom-cli.yaml`:
```yaml
tick_schedule:
  persist_check_interval: 1000   # каждые 1000 тиков
```

---

## 4. Boot sequence — автозагрузка при старте

При запуске `axiom-cli` автоматически пытается загрузить состояние:

```
[1] Проверить <data_dir>/manifest.yaml
    ├── Есть и валиден → restore_from(snapshot) + import traces
    ├── Нет → чистый старт (fresh start)
    └── Повреждён/ошибка → warning в stderr + чистый старт
```

**Флаги запуска:**

```bash
# Стандартный запуск (ищет ./axiom-data/)
cargo run --bin axiom-cli

# Кастомная директория
cargo run --bin axiom-cli -- --data-dir /var/lib/axiom/session1

# Принудительный чистый старт (игнорировать axiom-data/)
cargo run --bin axiom-cli -- --no-load

# Комбинация
cargo run --bin axiom-cli -- --data-dir /tmp/axiom --tick-hz 10 --verbose
```

**Banner при старте:**
```
AXIOM — Cognitive Architecture
───────────────────────────────
tick_hz: 100 Hz  |  domains: 11  |  :help for commands
  mode: restored from axiom-data (tick=42000, traces=8, tension=2)
```
или
```
  mode: fresh start (no data at axiom-data)
```

---

## 5. Knowledge Exchange — обмен знаниями

Экспорт/импорт знаний между разными экземплярами AXIOM.

### :export traces [path]

Экспорт ExperienceTraces в JSON-файл.

```
:export traces                          → axiom-export-traces.json
:export traces /tmp/my-knowledge.json   → в указанный файл
```

Экспортирует все traces с weight ≥ 0 (все следы).

### :export skills [path]

Экспорт кристаллизованных навыков (SkillSet) в JSON-файл.

```
:export skills                          → axiom-export-skills.json
:export skills /tmp/my-skills.json
```

### :import traces [path]

Импорт traces из файла с GUARDIAN-валидацией.

```
:import traces                          → из axiom-export-traces.json
:import traces /tmp/my-knowledge.json
```

**Процесс импорта:**
1. Загрузить JSON-файл
2. Проверить `kind == "traces"` (иначе ошибка)
3. Для каждого trace → `guardian.validate_reflex(pattern)`:
   - `Veto` → пропустить (считается в `guardian_rejected`)
   - `Allow` → импортировать с `weight × 0.7`
4. Вывести отчёт

Вывод: `imported=5 rejected_by_guardian=1 skipped_dup=0 (total=6)`

### :import skills [path]

```
:import skills                          → из axiom-export-skills.json
:import skills /tmp/my-skills.json
```

Аналогично traces, но для кристаллизованных навыков. Дополнительно — дедупликация по similarity.

### Формат файла обмена

```json
{
  "header": {
    "kind": "traces",
    "version": "axiom-exchange-v1",
    "source_tick": 42000,
    "exported_at": "1744012800",
    "count": 8
  },
  "traces": [...]
}
```

---

## 6. GUARDIAN-валидация импорта

При `:import` каждый паттерн проходит CODEX-проверку:

| Проверка | Условие нарушения | VetoReason |
|---|---|---|
| GENOME access | Arbiter не имеет Execute на AshtiField | `GenomeDenied` |
| CODEX #1 | `token.state == STATE_LOCKED` | `TokenLocked` |
| CODEX #2 | `token.valence != 0 && token.mass == 0` | `ValenceWithoutMass` |
| CODEX #3 | `token.sutra_id == 0` | `ZeroSutraId` |

Отклонённые паттерны не попадают в Experience. Это защита от импорта CODEX-нарушающих знаний из внешних источников.

---

## 7. Программное API (Rust)

```rust
use axiom_persist::{save, load, WriteOptions, AutoSaver, PersistenceConfig};
use axiom_persist::{export_traces, export_skills, import_traces, import_skills};
use std::path::Path;

// Сохранение
let opts = WriteOptions::default();
let manifest = save(&engine, Path::new("axiom-data"), &opts)?;
println!("saved tick={}", manifest.tick_count);

// Загрузка
let result = load(Path::new("axiom-data"))?;
let engine = result.engine; // восстановленный engine
println!("traces imported: {}", result.traces_imported);

// AutoSaver
let cfg = PersistenceConfig::new("axiom-data", 1000); // каждые 1000 тиков
let mut saver = AutoSaver::new(cfg);

// В tick loop:
saver.tick(&engine); // проверяет интервал и сохраняет если нужно

// При завершении:
saver.force_save(&engine)?;

// Knowledge exchange
export_traces(&engine, Path::new("traces.json"), 0.0)?;
export_skills(&engine, Path::new("skills.json"))?;
let report = import_traces(&mut engine, Path::new("traces.json"))?;
println!("{}", report.summary_line());
```

### WriteOptions

```rust
pub struct WriteOptions {
    /// Минимальный weight trace для сохранения (0.0 = все)
    pub trace_weight_threshold: f32,
}
```

### PersistError

```rust
pub enum PersistError {
    NotFound(String),           // директория или файл отсутствует
    CorruptManifest(String),    // manifest.yaml не парсится
    VersionMismatch { expected, found },  // несовместимый формат
    Io(io::Error),              // ошибка файловой системы
    Decode(String),             // JSON не парсится
    Encode(String),             // JSON не сериализуется
}
```

---

## 8. Конфигурация в axiom-cli.yaml

```yaml
# axiom-cli.yaml
tick_hz: 100
verbose: false
data_dir: axiom-data           # директория по умолчанию для :save/:load

tick_schedule:
  persist_check_interval: 1000  # автосохранение каждые 1000 тиков (0 = откл)
  snapshot_interval: 5000
  adaptation_interval: 50
```

---

## 9. Константы

| Константа | Значение | Описание |
|---|---|---|
| `FORMAT_VERSION` | `"axiom-memory-v1"` | Версия формата хранилища |
| `IMPORT_WEIGHT_FACTOR` | `0.7` | Коэффициент снижения weight при load/import |
| `TickSchedule::persist_check_interval` | `0` | По умолчанию отключено |

---

## 10. Тесты

```bash
# Все тесты axiom-persist (35 тестов)
cargo test -p axiom-persist

# По группам
cargo test -p axiom-persist --test persist_tests    # Фаза 1: save/load
cargo test -p axiom-persist --test boot_tests       # Фаза 2: boot sequence
cargo test -p axiom-persist --test autosave_tests   # Фаза 3: автосохранение
cargo test -p axiom-persist --test exchange_tests   # Фаза 4: export/import
```
