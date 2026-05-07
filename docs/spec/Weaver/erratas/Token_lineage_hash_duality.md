# Token.lineage_hash — Двойная семантика

**Дата:** 2026-05-07  
**Статус:** P0 закрыт (реализован в V1.3). Документ фиксирует дуальность как инвариант.  
**Затрагивает:** Token V5.2, FrameWeaver V1.3  
**Файлы:** `axiom-core/src/token.rs`, `axiom-arbiter/src/ashti_processor.rs`,
`axiom-arbiter/src/maya_processor.rs`, `axiom-runtime/src/over_domain/weavers/frame.rs`

---

## Суть проблемы

Token V5.2 определяет `lineage_hash: u64` в разделе "Фрактальная навигация":

> «Хеш пути: откуда пришёл (Sutra → Logic → Math...)»

FrameWeaver V1.3 переиспользует это же поле в Frame-анкерах для хранения
**идентификатора паттерна** — FNV-1a хэша от sorted sutra_id участников Frame.

Это не баг и не конфликт — это две легальные семантики для разных классов токенов,
разделённые через `type_flags`. Спека V5.2 не документирует второе значение.
Данный документ закрывает это расхождение.

---

## Семантика 1: Path provenance (общие токены)

**Класс токенов:** все, у которых `(type_flags & TOKEN_FLAG_FRAME_ANCHOR) == 0`.

**Начальное значение:** 0 (`Token::new()` инициализирует в 0).

**Как обновляется:**

1. `ashti_processor.rs::apply_causal()` — при прохождении через домен с ролью 6 (LOGIC):
   ```rust
   h ^= domain.domain_id as u64;
   h = h.wrapping_mul(0x100000001b3);  // FNV-1a множитель
   token.lineage_hash = h;
   ```
   Каждый домен оставляет след. Последовательность доменов кодируется в хэше.

2. `maya_processor.rs::consolidate_tokens()` — при консолидации MAYA выводит XOR всех входных:
   ```rust
   let lineage = tokens.iter().fold(0u64, |acc, t| acc ^ t.lineage_hash);
   ```
   Итоговый MAYA-токен несёт свёртку путей всех участников.

**Семантика:** "по каким доменам прошёл этот поток смысла". Используется для
фрактальной навигации в FractalChain (не реализовано полностью, поле зарезервировано).

---

## Семантика 2: Frame pattern identity (Frame-анкеры)

**Класс токенов:** `(type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0`.

**Как устанавливается:** в `FrameWeaver::build_crystallization_commands()` через
`InjectFrameAnchorPayload.lineage_hash = candidate.lineage_hash`, где:

```rust
candidate.lineage_hash = FNV-1a(sorted(sutra_ids всех участников Frame))
```

Вычисление в `FrameWeaver::fnv1a_lineage_hash(ids: &[u32])`:
```
1. sort_unstable(ids)
2. h = 0xcbf29ce484222325
3. for id in sorted: h ^= id as u64; h = h.wrapping_mul(0x100000001b3)
```

**Порядок участников не важен** — хэш детерминирован от состава паттерна.

**Как используется:**

- `find_existing_anchor(exp_state, lineage_hash)` — поиск существующего Frame в EXPERIENCE:
  ```rust
  exp_state.tokens.iter().find(|t|
      (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0
      && t.lineage_hash == lineage_hash
  )
  ```
- При промоции в SUTRA (`engine.rs:911`) — создание SUTRA-анкера с производным ID:
  ```rust
  (restored.anchor.lineage_hash ^ (target_domain as u64).wrapping_mul(0x9e3779b97f4a7c15)) as u32
  ```
- `dream_phase/cycle.rs:434` — DreamProposal читает `frame.anchor.lineage_hash`

**Семантика:** "какой набор участников образует этот Frame". Неизменен на всём
жизненном цикле Frame (EXPERIENCE → SUTRA). Не описывает историю пути.

---

## Инвариант разделения

```
type_flags & TOKEN_FLAG_FRAME_ANCHOR == 0   →   lineage_hash = path provenance
type_flags & TOKEN_FLAG_FRAME_ANCHOR != 0   →   lineage_hash = FNV-1a(sorted participant ids)
```

Перед чтением `lineage_hash` необходимо знать класс токена. Читать
`lineage_hash` Frame-анкера как путь — ошибка интерпретации.

---

## Почему конфликта нет на практике

**Архитектурная защита:**

Frame-анкеры живут в EXPERIENCE (109) и SUTRA (100) с `state = STATE_ACTIVE`
или `STATE_LOCKED` соответственно. Они **никогда не проходят через ASHTI-pipeline**
(домены 101–108), потому что:

1. Роутинг инжектирует токены в SUTRA (100) и проходит через ASHTI → MAYA.
   EXPERIENCE-токены (109) не участвуют в этом pipeline как входные.
2. `apply_causal()` вызывается только внутри `process_token_in_domain()` для токенов,
   активно проходящих pipeline. Locked/EXPERIENCE-токены туда не попадают.
3. MAYA consolidation (`maya_processor.rs`) работает с выводом ASHTI (101–108),
   не с EXPERIENCE/SUTRA токенами напрямую.

**Вывод:** `apply_causal()` и MAYA XOR-fold физически не могут затронуть
`lineage_hash` Frame-анкера при корректной работе системы.

---

## Affected code audit

| Файл | Строка | Что делает с lineage_hash | Класс токена |
|------|--------|--------------------------|-------------|
| `axiom-core/token.rs:157` | `lineage_hash: 0` | инициализация | все |
| `axiom-arbiter/ashti_processor.rs:133-136` | XOR domain_id, mul | обновление провенанса | общие (ASHTI pipeline) |
| `axiom-arbiter/maya_processor.rs:101,115` | XOR-fold из родителей | консолидация | общие (MAYA output) |
| `axiom-runtime/engine.rs:773` | `token.lineage_hash = p.lineage_hash` | копирование из InjectFrameAnchorPayload | Frame-анкеры |
| `axiom-runtime/engine.rs:911` | читает для деривации нового ID | Frame-анкеры (промоция) |
| `axiom-runtime/weavers/frame.rs:425` | `find_existing_anchor` — lookup по hash | Frame-анкеры |
| `axiom-runtime/weavers/frame.rs:503-509` | вычисление `candidate.lineage_hash` | Frame-кандидаты |
| `axiom-runtime/dream_phase/cycle.rs:434` | читает `frame.anchor.lineage_hash` | Frame-анкеры |

---

## Что нужно исправить в спеке Token V5.2

В секцию **3.4 Фрактальная навигация** добавить:

> **lineage_hash** — значение зависит от `type_flags`:
>
> - Если `(type_flags & TOKEN_FLAG_FRAME_ANCHOR) == 0` *(обычный токен)*:
>   Хэш истории пути домен-по-домену. Обновляется при прохождении ASHTI-pipeline
>   (`apply_causal`, Role 6) и при MAYA-консолидации. Default = 0.
>
> - Если `(type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0` *(Frame-анкер)*:
>   FNV-1a хэш от sorted sutra_id всех участников Frame.
>   Устанавливается при кристаллизации, не изменяется. Используется для
>   дедупликации (`find_existing_anchor`) и деривации ID при промоции.
>   Не несёт информации о пути.

В секцию **4. Инварианты** добавить:

> 5. **lineage_hash дуальность**: значение поля определяется классом токена.
>    Для Frame-анкеров (`TOKEN_FLAG_FRAME_ANCHOR`) — идентификатор паттерна.
>    Для остальных — провенанс пути (или 0). Смешение интерпретаций недопустимо.

---

## Альтернативы (рассмотрены и отклонены)

**Вариант A: Новое поле `frame_hash: u64` в Token.**  
Плюс: явная семантика. Минус: нарушает 64-byte layout — нет свободных байт в Token.
Отклонено.

**Вариант B: Хранить lineage_hash в `reserved_gate` Connection вместо Token.**  
Несовместимо с `find_existing_anchor()` (итерирует токены, не связи).
Отклонено.

**Вариант C: Отдельная HashMap в FrameWeaver для дедупликации.**  
Уже есть — `candidates: HashMap<u64, FrameCandidate>`. Но это кандидаты до
кристаллизации. После кристаллизации lookup идёт по EXPERIENCE-токенам.
Дублировать в HashMap означало бы держать второй индекс в sync с доменом. Отклонено.

**Принятое решение:** реиспользование `lineage_hash` корректно при соблюдении
инварианта разделения по `type_flags`. Требуется только документирование (этот документ).
