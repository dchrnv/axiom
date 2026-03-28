# AXIOM MODULE SPECIFICATION: GENOME V1.0

**Статус:** Актуальная спецификация (foundational)  
**Версия:** 1.0.0  
**Дата:** 2026-03-28  
**Назначение:** Неизменяемый конституционный слой системы — единый источник истины  
**Crate:** `axiom-genome` (новый crate в workspace)  
**Модель времени:** Загружается до COM. Не генерирует событий. Статичен в рантайме.  
**Связанные спеки:** Guardian V1.0, Ashti_Core V2.0, Configuration System V1.0, DomainConfig V2.1, Arbiter V1.0  
**Наследие:** CDNA + Genom из NeuroGraph KEY V2.0

---

## 1. Назначение

**GENOME** — неизменяемый набор фундаментальных правил, определяющих архитектуру, права доступа и протоколы взаимодействия всех модулей AXIOM.

GENOME — это **не домен**. У него нет поля, токенов, мембраны или физики. Это конфигурационный слой, загружаемый первым при инициализации, до создания доменов, до запуска COM, до любого события.

Если **CODEX (Домен 3)** — это пластичный закон, который может эволюционировать (аналог ADNA), то **GENOME** — это конституция, которая определяет пространство допустимых состояний (аналог CDNA). CODEX не может нарушить GENOME. Никто не может.

**Иерархия:**

```
GENOME (неизменяемый, загружается первым)
  ↓ определяет правила для
GUARDIAN (над-доменный, читает GENOME + CODEX)
  ↓ контролирует
CODEX (Домен 3, пластичный закон)
  ↓ один из
ASHTI (1-8) — процессоры
```

---

## 2. Что содержит GENOME

GENOME состоит из четырёх разделов. Каждый раздел — read-only структура, загружаемая при старте.

### 2.1 Структурные инварианты (Invariants)

Неизменяемые физические и архитектурные ограничения:

```rust
pub struct GenomeInvariants {
    /// Размеры core-структур (compile-time, но дублируются для валидации)
    pub token_size: u16,           // Всегда 64
    pub connection_size: u16,      // Всегда 64
    pub event_size: u16,           // Всегда 32
    pub domain_config_size: u16,   // Всегда 128

    /// Фундаментальные ограничения
    pub max_domains: u8,           // Максимум доменов в одном уровне Ashti_Core (11)
    pub min_intensity: u8,         // min_intensity > 0 для EXPERIENCE(9), никогда не ноль
    pub sutra_write_exclusive: bool, // Только SUTRA(0) имеет право WRITE на токены

    /// Временная модель (Time Model V1.0)
    pub no_wall_clock_in_core: bool, // Запрет std::time внутри ядра — всегда true
    pub event_id_monotonic: bool,    // event_id строго возрастает — всегда true
}
```

Эти значения **никогда не меняются** в рантайме. Попытка их изменить — критическая ошибка, немедленная остановка системы.

### 2.2 Права доступа (Access Control)

Определяет, какой модуль к какому ресурсу имеет доступ:

```rust
pub struct AccessRule {
    pub module: ModuleId,          // Кто
    pub resource: ResourceId,      // К чему
    pub permission: Permission,    // Что может делать
}

#[derive(Clone, Copy)]
pub enum Permission {
    None,           // Нет доступа
    Read,           // Только чтение
    ReadWrite,      // Чтение и запись
    Execute,        // Выполнение (для процессоров)
    Control,        // Управление (для GUARDIAN)
}
```

Базовая таблица прав:

| Модуль | SUTRA(0) | ASHTI(1-8) | EXPERIENCE(9) | MAYA(10) | CODEX(3) | GENOME |
|--------|----------|------------|----------------|----------|----------|--------|
| Arbiter | Read | Execute | Read | Execute | Read | Read |
| GUARDIAN | Read | Control | Control | Read | **ReadWrite** | **Read** |
| Heartbeat | — | Read | Read | — | — | Read |
| Shell | — | Read | Read | — | — | — |
| Адаптеры | — | — | — | Read | — | — |

Ключевые правила:
- SUTRA(0) — единственный модуль с правом WRITE на создание токенов. Все остальные работают через REFERENCE (ссылки на sutra_id).
- GUARDIAN — единственный модуль с правом WRITE на CODEX(3). Он может добавлять, изменять и ингибировать правила в CODEX.
- GENOME — read-only для всех, включая GUARDIAN. Никто не может изменить GENOME в рантайме.

### 2.3 Протоколы взаимодействия (Protocols)

Определяет допустимые маршруты данных между модулями:

```rust
pub struct ProtocolRule {
    pub source: ModuleId,          // Откуда
    pub target: ModuleId,          // Куда
    pub data_type: DataType,       // Что передаётся
    pub mandatory: bool,           // Обязательный маршрут?
}
```

Маршруты Ashti_Core (из Ashti_Core V2.0, раздел 6):

| Маршрут | Содержание | Обязательный |
|---------|-----------|--------------|
| SUTRA(0) → EXPERIENCE(9) | Ссылка на токен (новый паттерн) | Да |
| EXPERIENCE(9) → Arbiter | Результат резонансного поиска | Да |
| Arbiter → MAYA(10) | Рефлекс (если одобрен GUARDIAN) | Нет |
| Arbiter → ASHTI(1-8) | Паттерн + подсказка | Да |
| ASHTI(1-8) → MAYA(10) | Результат обработки | Да |
| ASHTI(1-8) → EXPERIENCE(9) | Новый опыт | Да |
| MAYA(10) → Arbiter | Результат сравнения рефлекса | Условный |
| Arbiter → EXPERIENCE(9) | Обратная связь | Условный |

Любой маршрут, не описанный в GENOME, запрещён. Если модуль A пытается отправить данные модулю B вне протокола — GUARDIAN блокирует.

### 2.4 Модульная конфигурация (Module Config)

Глобальные параметры, общие для всех доменов или всего уровня:

```rust
pub struct GenomeConfig {
    /// Arbiter: глобальные параметры (из Arbiter V1.0, раздел 5)
    pub arbiter_response_timeout: u64,    // Max event_id ожидания ответа от 1-8
    pub arbiter_storm_threshold: u32,     // Max рефлексов в очереди на MAYA

    /// Frontier: глобальные лимиты (из Causal Frontier V2.0)
    pub default_max_events_per_cycle: u32,
    pub default_storm_threshold: u32,

    /// Heartbeat: базовый пресет
    pub default_heartbeat_interval: u32,

    /// Ashti_Core: количество доменов
    pub ashti_domain_count: u8,           // 11 (0=SUTRA, 1-8=ASHTI, 9=EXPERIENCE, 10=MAYA)
}
```

---

## 3. Формат хранения

### 3.1 Файл

GENOME хранится в YAML-файле, загружаемом через Configuration System V1.0:

```yaml
# config/genome.yaml — Конституция системы AXIOM

version: 1

invariants:
  token_size: 64
  connection_size: 64
  event_size: 32
  domain_config_size: 128
  max_domains: 11
  min_intensity_nonzero: true
  sutra_write_exclusive: true
  no_wall_clock_in_core: true
  event_id_monotonic: true

access_control:
  - module: guardian
    resource: codex
    permission: read_write
  - module: guardian
    resource: genome
    permission: read
  - module: arbiter
    resource: experience
    permission: read
  - module: arbiter
    resource: codex
    permission: read
  # ... полная таблица

protocols:
  - source: sutra
    target: experience
    data_type: token_reference
    mandatory: true
  - source: experience
    target: arbiter
    data_type: resonance_response
    mandatory: true
  # ... все маршруты

config:
  arbiter_response_timeout: 10000
  arbiter_storm_threshold: 100
  default_max_events_per_cycle: 1000
  default_storm_threshold: 5000
  default_heartbeat_interval: 1024
  ashti_domain_count: 11
```

### 3.2 In-memory структура

При загрузке YAML парсится в typed struct:

```rust
pub struct Genome {
    pub version: u32,
    pub invariants: GenomeInvariants,
    pub access_rules: Vec<AccessRule>,       // Предвыделён, не растёт
    pub protocol_rules: Vec<ProtocolRule>,   // Предвыделён, не растёт
    pub config: GenomeConfig,
}
```

После загрузки и валидации, `Genome` замораживается: `Arc<Genome>` раздаётся всем модулям. Никто не может получить `&mut Genome`.

```rust
// При инициализации:
let genome = Arc::new(Genome::load_and_validate("config/genome.yaml")?);

// Каждый модуль получает Arc<Genome> (shared, immutable reference):
let guardian = Guardian::new(Arc::clone(&genome));
let arbiter = Arbiter::new(Arc::clone(&genome));
let engine = AxiomEngine::new(Arc::clone(&genome), ...);
```

---

## 4. Порядок загрузки (Boot Sequence)

GENOME решает "проблему курицы и яйца": он загружается первым, до всего остального.

```
1. Загрузить genome.yaml → Genome struct
2. Валидировать инварианты (compile-time assertions дублируются runtime-проверкой)
3. Заморозить Genome в Arc<Genome>
4. Загрузить DomainConfig для каждого домена (валидация через Genome.invariants)
5. Создать GUARDIAN (получает Arc<Genome>)
6. Создать домены ASHTI (0-10), включая CODEX(3)
7. Создать Arbiter (получает Arc<Genome>)
8. Инициализировать COM (CausalClock)
9. Система готова к приёму UCL-команд
```

Если валидация GENOME на шаге 2 не проходит — система не запускается. Нет fallback. Нет "работаем с тем что есть". GENOME корректен или система не существует.

---

## 5. Механизм подписки (Pub-Sub)

Модули не опрашивают GENOME повторно — они получают его один раз при создании (`Arc<Genome>`). GENOME не меняется в рантайме, поэтому классический Pub-Sub не нужен.

Однако для **будущей эволюции** (когда появится контролируемый механизм обновления GENOME) предусмотрен trait:

```rust
pub trait GenomeSubscriber {
    /// Вызывается при обновлении GENOME (будущая функциональность).
    /// Модуль получает новую версию и адаптируется.
    fn on_genome_update(&mut self, new_genome: &Genome);

    /// Возвращает список ресурсов, на которые подписан модуль.
    fn subscribed_resources(&self) -> &[ResourceId];
}
```

Текущая реализация: `on_genome_update` не вызывается никогда (GENOME неизменяем). Trait существует для forward compatibility.

---

## 6. Валидация

### 6.1 При загрузке

```rust
impl Genome {
    pub fn validate(&self) -> Result<(), GenomeError> {
        // Инварианты размеров
        if self.invariants.token_size != 64 { return Err(GenomeError::InvariantViolation("token_size")); }
        if self.invariants.connection_size != 64 { return Err(GenomeError::InvariantViolation("connection_size")); }
        if self.invariants.event_size != 32 { return Err(GenomeError::InvariantViolation("event_size")); }
        if self.invariants.domain_config_size != 128 { return Err(GenomeError::InvariantViolation("domain_config_size")); }

        // Протоколы: нет дублирующих маршрутов
        // Права: GUARDIAN имеет ReadWrite на CODEX
        // Конфиг: значения в допустимых диапазонах
        // ...

        Ok(())
    }
}
```

### 6.2 Runtime-проверки (через GUARDIAN)

GUARDIAN использует GENOME для проверки каждого решения:

```rust
impl Guardian {
    pub fn check_access(&self, module: ModuleId, resource: ResourceId, operation: Permission) -> bool {
        self.genome.access_rules.iter().any(|rule|
            rule.module == module
            && rule.resource == resource
            && rule.permission >= operation
        )
    }

    pub fn check_protocol(&self, source: ModuleId, target: ModuleId, data_type: DataType) -> bool {
        self.genome.protocol_rules.iter().any(|rule|
            rule.source == source
            && rule.target == target
            && rule.data_type == data_type
        )
    }
}
```

---

## 7. Связь с CODEX (Домен 3)

CODEX — пластичный закон внутри домена 3. Он содержит правила, которые система "выучила" или получила. В отличие от GENOME:

| Свойство | GENOME | CODEX (Домен 3) |
|----------|--------|-----------------|
| Изменяемость | Неизменяем в рантайме | Пластичен, эволюционирует |
| Природа | Конфигурационный слой | Домен с полем, токенами, физикой |
| Кто может менять | Никто | GUARDIAN (единственный с правом Write) |
| Что содержит | Архитектурные правила | Поведенческие правила |
| Аналог в НГ | CDNA + Genom | ADNA |
| Загрузка | Первым, до всего | Как обычный домен |

GUARDIAN проверяет рефлексы по двум источникам:
1. **GENOME** — нарушает архитектурный инвариант? → вето, без обсуждений.
2. **CODEX** — нарушает поведенческое правило? → вето, но правило может быть обновлено через обратную связь.

---

## 8. Производительность

GENOME — это in-memory read-only структура. Обращение к ней — обычное чтение через `&Arc<Genome>`, стоимость = разыменование указателя (~1 ns).

Для горячего пути (check_access, check_protocol): таблицы прав и протоколов предварительно индексируются по `ModuleId` при загрузке, что даёт O(1) lookup вместо линейного поиска:

```rust
pub struct GenomeIndex {
    /// Предвычисленная матрица прав: [ModuleId][ResourceId] -> Permission
    access_matrix: [[Permission; MAX_RESOURCES]; MAX_MODULES],

    /// Предвычисленная матрица протоколов: [SourceId][TargetId] -> bool
    protocol_matrix: [[bool; MAX_MODULES]; MAX_MODULES],
}
```

Lookup в матрице — один доступ к памяти. Это быстрее HashMap и быстрее линейного поиска по Vec.

---

## 9. Инварианты

1. **Неизменяемость.** GENOME не изменяется после загрузки. `Arc<Genome>` без `Mutex`.
2. **Первичность.** GENOME загружается первым. Все остальные модули зависят от него.
3. **Полнота.** Каждый маршрут данных описан в GENOME. Неописанный маршрут запрещён.
4. **Валидация.** GENOME валидируется при загрузке. Невалидный GENOME → система не запускается.
5. **CDNA-приоритет.** Любое правило GENOME имеет абсолютный приоритет над CODEX. Конфликт GENOME vs CODEX всегда решается в пользу GENOME.
6. **GUARDIAN-привилегия.** GUARDIAN — единственный модуль с расширенными правами (ReadWrite на CODEX, Read на всё остальное).

---

## 10. Будущее: Эволюция GENOME

В текущей версии GENOME неизменяем. В будущем возможен контролируемый механизм обновления:

1. Модуль или GUARDIAN предлагает изменение.
2. Изменение проверяется на непротиворечивость (нет циклических зависимостей, нет нарушения инвариантов).
3. Если проходит — применяется атомарно.
4. Все подписчики получают `on_genome_update()`.
5. Модули адаптируются к новым правилам.

Этот механизм **не реализуется в V1.0**. Trait `GenomeSubscriber` существует как forward compatibility.

---

## 11. История изменений

- **V1.0**: Первая версия. Четыре раздела: инварианты, права доступа, протоколы, конфигурация. Неизменяемый в рантайме. Boot sequence. Связь с CODEX(3) и GUARDIAN.
