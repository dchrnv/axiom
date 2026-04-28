# PLAN: FrameWeaver V1.1 Stabilization

> **Статус: ЗАВЕРШЁН** (2026-04-27). Все этапы 0–4 выполнены. 1030 тестов, 0 failures.  
> Архивирован — актуальные результаты в `docs/spec/Weaver/erratas/FrameWeaver_V1_1_errata.md`.

**Дата:** 2026-04-26
**Контекст:** FrameWeaver V1.1 реализован (1017 тестов, 0 failures), но обнаружены критические пробелы. Этот документ — пошаговый план для имплементера. Перед каждым шагом — что делаем, как проверяем, критерии готовности.
**Не делаем в этой итерации:** второй Weaver, DREAM-фаза как этап обработки, демоция SUTRA→EXPERIENCE, межсистемный обмен Frame, live-тестирование через CLI. Всё это — в `deferred/`.
**Связанные документы:** `FrameWeaver_V1_1.md`, `Over_Domain_Layer_V1_1.md`, BLUEPRINT.md.

---

## Контекст: что не так сейчас

После реализации V1.1 обнаружено пять проблем разной критичности. Этот план их закрывает.

| # | Проблема                                              | Критичность | Этап в плане |
|---|-------------------------------------------------------|-------------|--------------|
| 1 | Hot path просел с 96.5 ns до ~310 ns (~3x)            | Критично    | Этап 3       |
| 2 | UnfoldFrame — заглушка, Frame нельзя использовать     | Критично    | Этап 2       |
| 3 | Промоция строит SUTRA-анкер с пустым списком участников | Критично  | Этап 2       |
| 4 | `Weaver::check_promotion` без `tick: u64`             | Серьёзно    | Этап 1       |
| 5 | `propose_to_dream` — заглушка (DREAM-фаза не существует) | Долг      | Deferred     |

---

## Этап 0 — гигиена документации (≈30 минут)

**Цель:** убрать риск спека/спека дрейфа. V1.0 не должна выглядеть актуальной для будущих сессий.

### 0.1 Пометить V1.0 как Superseded

В файлах `docs/specs/FrameWeaver_V1_0.md` и `docs/specs/Over_Domain_Layer_V1_0.md` в самом верху, после заголовка, добавить:

```
**Статус:** Superseded by V1.1 (онтологическая коррекция: Frame живёт в EXPERIENCE,
не в SUTRA). Этот документ сохранён для исторической прослеживаемости. Для
актуальной спецификации см. соответствующий V1.1 файл.
```

### 0.2 Создать errata-документ

Создать `docs/specs/erratas/FrameWeaver_V1_1_errata.md` со следующим содержимым:

```markdown
# FrameWeaver V1.1 — Errata

**Назначение:** фиксация неточностей и пробелов спецификации V1.1, обнаруженных
в процессе реализации. Все правки войдут в V1.2 после стабилизации.

## E1. Восстановление Frame из анкера не специфицировано

В V1.1 описано, как Frame создаётся (раздел 4.5: анкер + связи к участникам), но
не описано, как восстановить список участников из существующего анкера. Это
блокирует:
- UnfoldFrame (раздел 7) — handler не может развернуть Frame
- Promotion (раздел 5.4) — `build_promotion_commands` не знает, какие связи
  копировать в SUTRA

**Решение:** см. этап 2.1 этого плана. Алгоритм `restore_frame_from_anchor`.

## E2. Trait `Weaver::check_promotion` без tick

V1.1 раздел 8 указал сигнатуру:
```rust
fn check_promotion(&self, experience_state: &DomainState) -> Vec<PromotionProposal>;
```

Должна быть:
```rust
fn check_promotion(&self, tick: u64, experience_state: &DomainState) -> Vec<PromotionProposal>;
```

Без tick невозможно проверять `min_age_events` из `PromotionRule`. См. этап 1.

## E3. Hot path regression при интеграции

После добавления FrameWeaver в pipeline hot path tick вырос с ~96.5 ns до ~310 ns.
Природа просадки требует A/B измерения (этап 3). Подозрение на `mem::take` +
`drain_commands` каждый тик независимо от расписания Weavers.

## E4. propose_to_dream — заглушка

В V1.1 предполагалось наличие DREAM-фазы как этапа обработки. По факту такой фазы
не существует — есть только домен DREAM (107). FrameWeaver сейчас обходит DREAM
и кристаллизует напрямую. Это **известный долг**, не баг. Решение откладывается
до отдельного проектирования DREAM-фазы как механизма.
```

### 0.3 Создать deferred-документ

Создать `docs/specs/deferred/FrameWeaver_post_V1_1.md`:

```markdown
# Deferred: вне V1.1 stabilization

Эти задачи **не выполняются** в текущей итерации стабилизации.

## Архитектурные
- **DREAM-фаза как этап обработки.** Сейчас FrameWeaver кристаллизует напрямую,
  обходя DREAM. Полноценная DREAM-фаза должна быть спроектирована отдельно для
  всех Weavers, не только FrameWeaver.
- **Демоция SUTRA → EXPERIENCE.** Что делать с промоутнутым Frame, оказавшимся
  ошибкой. Принципиальный вопрос: истина не возвращается в опыт.
- **Межсистемный обмен Frame.** Экспорт/импорт Frame между экземплярами AXIOM
  через Memory Persistence. IMPORT_WEIGHT_FACTOR=0.7 уже есть, но протокол не
  определён.

## Реализационные
- **Live-тестирование через CLI/дашборд.** Откладывается до завершения этапов 1–4.
- **Второй Weaver (CausalWeaver, SpatialWeaver и др.).** Откладывается до
  стабилизации FrameWeaver.
- **Demotion операция RevokePromotion.** Связано с архитектурным вопросом выше.
```

### Критерий готовности этапа 0

- [ ] Заголовки V1.0 файлов помечены Superseded
- [ ] `errata` и `deferred` документы созданы и закоммичены
- [ ] Тесты остаются зелёными (изменений в коде на этом этапе нет)

---

## Этап 1 — правка trait и совместимость (≈1 час)

**Цель:** исправить `Weaver::check_promotion` и связанные пробелы в trait, не
ломая остальной код.

### 1.1 Изменить `Weaver` trait

В файле `axiom-runtime/src/over_domain/traits.rs` (или где живёт trait):

**Было:**
```rust
fn check_promotion(&self, experience_state: &DomainState) -> Vec<PromotionProposal>;
```

**Стало:**
```rust
fn check_promotion(&self, tick: u64, experience_state: &DomainState) -> Vec<PromotionProposal>;
```

### 1.2 Проверить, не пропущен ли tick в других методах

Открыть `traits.rs` и проверить каждый метод:
- `scan(&mut self, maya_state: &DomainState)` — нужен ли tick? **Да**, потому что в `FrameCandidate.detected_at` мы хотим записывать честный event_id (или tick) обнаружения. Сейчас, скорее всего, проставляется в момент сохранения, а не обнаружения. Добавить:

  ```rust
  fn scan(&mut self, tick: u64, maya_state: &DomainState) -> Vec<Self::Pattern>;
  ```

- `propose_to_dream(&self, pattern: &Self::Pattern)` — tick **не нужен**, это чистая трансформация.

### 1.3 Прокинуть tick во все вызовы

Найти все места вызова этих методов в `axiom-runtime` (вероятно, в `axiom_engine.rs` или в orchestrator-логике). Передать туда `tick: u64`.

Источник tick — это монотонный счётчик из `AxiomEngine` (тот же, что используется для `TickSchedule`).

### 1.4 Использовать tick в `FrameWeaver::check_promotion`

В реализации `check_promotion` для FrameWeaver — теперь честно проверять `min_age_events`:

```rust
let frame_age = tick.saturating_sub(frame_anchor.created_tick);
if frame_age < rule.min_age_events {
    continue; // ещё не дозрел
}
```

(Точное имя поля — что-то вроде `last_event_id` или отдельное `created_at`. Sonnet проверит по структуре Token V5.2.)

### 1.5 Тесты

Добавить или обновить unit-тесты:

**Test 1.5.a — `check_promotion_respects_min_age`:**
```
1. Создать Frame в EXPERIENCE с created_tick = 100
2. Вызвать check_promotion(tick=200, ...) с rule { min_age_events: 500 }
3. Ожидать пустой результат (Frame ещё не дозрел: 200 - 100 = 100 < 500)
4. Вызвать check_promotion(tick=700, ...) с тем же rule
5. Ожидать непустой результат (700 - 100 = 600 >= 500)
```

**Test 1.5.b — `scan_records_correct_detection_tick`:**
```
1. Создать узор в MAYA на тике 50
2. Вызвать scan(tick=50, ...)
3. Достать сохранённого кандидата
4. Проверить, что candidate.detected_at == 50
```

### Критерий готовности этапа 1

- [ ] Trait Weaver обновлён, tick передаётся в `scan` и `check_promotion`
- [ ] Все вызовы методов trait исправлены
- [ ] Тесты 1.5.a и 1.5.b добавлены и проходят
- [ ] 1017+ существующих тестов остаются зелёными
- [ ] `min_age_events` теперь реально проверяется

---

## Этап 2 — закрыть критические архитектурные дыры (≈4–6 часов)

**Цель:** UnfoldFrame и промоция должны полноценно работать. Frame перестаёт
быть write-only сущностью.

### 2.1 Спроектировать и реализовать `restore_frame_from_anchor`

Эта функция — **основа** и для UnfoldFrame, и для промоции. Она ходит по графу
связей в EXPERIENCE и собирает структуру Frame.

**Где жить:** в модуле `over_domain/weavers/frame.rs`, метод на `FrameWeaver` (или
свободная функция в том же модуле — на усмотрение Sonnet).

**Сигнатура:**
```rust
pub struct RestoredFrame {
    pub anchor: Token,                  // копия анкер-токена
    pub anchor_id: u32,                  // sutra_id анкера
    pub category: u16,                   // FRAME_CATEGORY_*
    pub participants: Vec<Participant>,  // восстановленный список
}

pub fn restore_frame_from_anchor(
    anchor_id: u32,
    source_state: &DomainState,
) -> Result<RestoredFrame, RestoreError>;

pub enum RestoreError {
    AnchorNotFound,
    NotAFrameAnchor,           // type_flags не содержит TOKEN_FLAG_FRAME_ANCHOR
    DanglingParticipant(u32),  // связь ведёт к несуществующему токену
    InvalidLinkType(u16),      // link_type не из категории 0x08
}
```

**Алгоритм:**
```
1. Найти токен с sutra_id == anchor_id в source_state.tokens.
   Если не найден → AnchorNotFound.
2. Проверить, что у токена type_flags содержит TOKEN_FLAG_FRAME_ANCHOR (0x0010).
   Если нет → NotAFrameAnchor.
3. Извлечь category из type_flags & FRAME_CATEGORY_MASK (0xFF00).
4. Пройти по source_state.connections, собирая все связи где:
   - source_id == anchor_id
   - flags & FLAG_ACTIVE != 0
   - (link_type >> 8) == 0x08  (синтаксическая категория)
   Если у связи link_type не из 0x08, но source_id == anchor_id — это ошибка
   данных, вернуть InvalidLinkType.
5. Для каждой такой связи c:
   - Проверить, что target_id существует в source_state.tokens.
     Если нет → DanglingParticipant.
   - Декодировать reserved_gate:
       origin_domain = u16::from_be_bytes([c.reserved_gate[0], c.reserved_gate[1]])
       role_id       = u16::from_be_bytes([c.reserved_gate[2], c.reserved_gate[3]])
   - Извлечь layer из старшего полубайта младшего байта link_type:
       Подтипы организованы как:
         S1 = 0x01..0x05  → layer = 1
         S2 = 0x10..0x14  → layer = 2
         S3 = 0x20..0x23  → layer = 3
         S4 = 0x30..0x34  → layer = 4
         S5 = 0x40..0x44  → layer = 5
         S6 = 0x50..0x54  → layer = 6
         S7 = 0x60..0x65  → layer = 7
         S8 = 0x70..0x74  → layer = 8
       layer = ((c.link_type & 0x00FF) >> 4) + 1
   - Создать Participant { sutra_id: c.target_id, origin_domain_id: origin_domain,
     role_id, role_link_type: c.link_type, layer }.
6. Вернуть RestoredFrame с заполненным списком.
```

**Важно:** функция read-only, не делает UCL-команд. Это чистый сбор данных.

### 2.2 Тесты для `restore_frame_from_anchor`

**Test 2.2.a — `restore_simple_frame`:**
```
1. Подготовить EXPERIENCE-state с:
   - Анкер-токен (sutra_id=1000, type_flags=TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX)
   - Участник 1 (sutra_id=1001, в SUTRA)
   - Участник 2 (sutra_id=1002, в SUTRA)
   - Связь анкер→1001, link_type=0x0801 (SUBJECT, S1), reserved_gate с origin=100
   - Связь анкер→1002, link_type=0x0802 (PREDICATE, S1), reserved_gate с origin=100
2. Вызвать restore_frame_from_anchor(1000, &state)
3. Ожидать: Ok(RestoredFrame), category == 0x0100, participants.len() == 2
4. Проверить layers (оба = 1), role_link_types правильные
```

**Test 2.2.b — `restore_returns_error_for_non_anchor`:**
```
1. Подготовить состояние с обычным токеном (без TOKEN_FLAG_FRAME_ANCHOR)
2. Вызвать restore_frame_from_anchor(token_id, &state)
3. Ожидать Err(NotAFrameAnchor)
```

**Test 2.2.c — `restore_detects_dangling_participant`:**
```
1. Анкер с одной связью к target_id=9999, которого нет в tokens
2. Ожидать Err(DanglingParticipant(9999))
```

**Test 2.2.d — `restore_extracts_correct_layers`:**
```
1. Анкер со связями всех 8 слоёв (по одной на каждый)
2. Восстановить
3. Проверить, что для каждого участника layer выставлен правильно (1..=8)
```

### 2.3 Реализовать UnfoldFrame handler

В engine, где сейчас handler возвращает `Success` без действий — заменить на
полноценную реализацию.

**Алгоритм:**
```
handle_unfold_frame(payload, ashti) -> UclResult:
    1. Определить source_domain:
       - если payload.source_domain == 0 → попробовать сначала EXPERIENCE (109),
         потом SUTRA (100); если в обоих не найдено — ошибка
       - иначе использовать указанный
    2. Получить snapshot source_state = ashti.peek_state(source_domain)
    3. restored = restore_frame_from_anchor(payload.frame_anchor_id, &source_state)
       - если Err → вернуть UclResult::Error
    4. Определить depth = payload.depth (0 → config.default_unfold_depth)
    5. Создать UCL-команды для копирования в payload.target_domain:
       - Сначала InjectToken для копии анкера (новый sutra_id в target)
       - Для каждого участника:
         - если depth > 0: рекурсивно — если участник тоже Frame-анкер
           (TOKEN_FLAG_FRAME_ANCHOR), развернуть его (depth-1)
         - иначе: просто создать ссылку (CreateConnection)
       - Вернуть UclResult::Success с количеством созданных токенов/связей
```

**Уточнение по recursion:** в первой итерации можно сделать **без рекурсии**.
То есть depth работает только на верхнем уровне: разворачиваем анкер + всех
непосредственных участников, не углубляясь. Это:
- проще
- безопаснее (нет stack overflow)
- покрывает 90% случаев

Полноценная рекурсивная развёртка — в deferred. Зафиксировать в errata.

### 2.4 Тесты для UnfoldFrame

**Test 2.4.a — `unfold_frame_to_target_domain`:**
```
1. Создать Frame в EXPERIENCE с 3 участниками
2. Послать UCL UnfoldFrame { frame_anchor_id, source_domain=109, target_domain=106, depth=1 }
3. Проверить:
   - В target_domain (106) появился новый токен с TOKEN_FLAG_FRAME_ANCHOR
   - Появились 3 новые связи с правильными link_type
   - reserved_gate сохранён (метаданные доменов участников не потеряны)
4. Frame в EXPERIENCE остался нетронутым
```

**Test 2.4.b — `unfold_frame_with_default_source`:**
```
1. Создать Frame только в EXPERIENCE (не в SUTRA)
2. Послать UCL с source_domain=0
3. Ожидать успех (нашёл в EXPERIENCE)

4. Создать Frame только в SUTRA (промоутнутый)
5. Удалить из EXPERIENCE (или подготовить состояние без него)
6. Послать UCL с source_domain=0
7. Ожидать успех (нашёл в SUTRA как fallback)
```

**Test 2.4.c — `unfold_returns_error_for_missing_anchor`:**
```
1. Послать UCL UnfoldFrame с frame_anchor_id, которого нет
2. Ожидать UclResult::Error с понятным сообщением
```

### 2.5 Починить промоцию через `restore_frame_from_anchor`

В `build_promotion_commands` (или эквиваленте) сейчас:
```rust
let dummy_candidate = FrameCandidate { participants: vec![], ... };
```

Заменить на:
```rust
let experience_state = ashti.peek_state(109);
let restored = restore_frame_from_anchor(frame_anchor_id, &experience_state)?;
let participants = restored.participants;
// дальше используем participants для построения команд
```

### 2.6 Тесты для исправленной промоции

**Test 2.6.a — `promotion_creates_sutra_frame_with_participants`:**
```
1. Создать Frame в EXPERIENCE с 3 участниками, состарить (manipulate created_tick)
   и активировать достаточно раз для срабатывания PromotionRule
2. Запустить on_tick на тике, когда промоция должна сработать
3. Проверить EXPERIENCE: оригинал на месте
4. Проверить SUTRA:
   - Есть новый анкер с TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE
   - state == STATE_LOCKED
   - 3 связи к тем же sutra_id участников
   - link_type сохранены
   - reserved_gate сохранены
```

**Test 2.6.b — `promotion_skipped_for_dangling_anchor`:**
```
1. Симулировать аномалию: анкер в EXPERIENCE есть, но участники потерялись
2. Запустить промоцию — она должна не упасть, а вернуть ошибку и пропустить этого
   кандидата (или залогировать предупреждение)
```

### Критерий готовности этапа 2

- [ ] `restore_frame_from_anchor` реализован, 4+ unit-теста проходят
- [ ] UnfoldFrame handler реализован (без рекурсии), 3+ теста
- [ ] Промоция использует `restore_frame_from_anchor`, 2+ теста
- [ ] В `errata` добавлена запись про "depth>1 recursion отложен"
- [ ] Все 1017+ существующих тестов остаются зелёными
- [ ] Frame теперь полноценен: можно создать, использовать (UnfoldFrame),
      продвинуть в SUTRA с правильными связями

---

## Этап 3 — расследование hot path (≈3–5 часов)

**Цель:** понять, куда ушли 200 ns/тик, и вернуться в зону 100–130 ns/тик.

### 3.1 Подготовить чистый A/B бенчмарк

Это **должно быть сделано до любой попытки оптимизации**. Иначе мы будем
оптимизировать вслепую.

**Создать новый бенчмарк** в `axiom-bench`:

```
benches/frameweaver_overhead.rs

Группа A — baseline (без FrameWeaver):
  - Собрать AxiomEngine с feature/флагом, который полностью отключает создание
    FrameWeaver (или вообще не добавляет его в over_domain_components)
  - Прогнать 50 токенов в LOGIC, измерить tick

Группа B — FrameWeaver есть, но scan_interval_ticks огромный:
  - FrameWeaver зарегистрирован, но scan_interval_ticks = u32::MAX
  - Это значит scan и check_promotion **никогда** не вызываются за разумное время
  - Этот замер показывает стоимость **самого факта наличия** FrameWeaver в
    pipeline (mem::take, drain_commands и т.д.)

Группа C — FrameWeaver работает на каждом тике:
  - scan_interval_ticks = 1
  - Но MAYA пуста (нечего сканировать)
  - Стоимость холостого scan

Группа D — FrameWeaver работает с реальными узорами:
  - scan_interval_ticks = 1
  - В MAYA подготовлены 3-5 синтаксических узоров
  - Стоимость "честной" работы
```

**Что мы хотим увидеть:**
- A → B: накладные расходы на интеграцию (целевое значение: ≤ 10 ns)
- B → C: стоимость холостого scan (целевое: ≤ 50 ns)
- C → D: стоимость реальной работы (нет жёсткой цели, главное чтобы
  scan_interval_ticks=20 давал амортизированную стоимость в норме)

### 3.2 Запустить и записать результаты

Прогнать `cargo bench`, собрать числа в таблицу:

```
| Группа | Описание                           | ns/tick | Δ от A   |
|--------|------------------------------------|---------|----------|
| A      | Без FrameWeaver                    | ?       | baseline |
| B      | FrameWeaver есть, scan отключён    | ?       | ?        |
| C      | FrameWeaver scan каждый тик, MAYA пуста | ?  | ?        |
| D      | FrameWeaver scan каждый тик, MAYA полная | ? | ?        |
```

Записать в `docs/specs/erratas/FrameWeaver_V1_1_errata.md` секцию **E3 Updated**.

### 3.3 Диагностировать на основе чисел

Решение принимается **по числам из 3.2**, не по интуиции. Возможные сценарии:

**Сценарий S1: A→B большой (>30 ns)**
Виновато само **наличие** FrameWeaver в pipeline. Это нездорово — холостой
интеграционный код не должен столько стоить. Подозреваемые:

- `mem::take` на `over_domain_components` каждый тик. Если так — поменять паттерн:
  ```rust
  // Было (примерно):
  let mut components = mem::take(&mut self.over_domain_components);
  for c in &mut components { c.on_tick(...) }
  self.over_domain_components = components;

  // Должно быть:
  for c in self.over_domain_components.iter_mut() {
      c.on_tick(...)
  }
  ```
  Если `mem::take` нужен из-за конфликта borrow checker (over_domain_components
  и ashti — поля одной структуры), решать через split borrow или метод-обёртку.

- `drain_commands` каждый тик возвращает `Vec` (даже пустой = аллокация на дроп).
  Поменять на `Option<SmallVec<[UclCommand; 8]>>` или возвращать `&mut Vec`,
  который дренируется самим engine.

- Проверки `tick % scan_interval_ticks` для каждого Weaver — копеечные, но если
  Weavers много в будущем, оптимизировать через wheel-расписание (TickSchedule
  заранее знает, кто на каком тике).

**Сценарий S2: B→C большой (>100 ns)**
Виноват сам холостой `scan`. Это значит, что FrameWeaver делает работу, даже
когда работать не над чем. Подозреваемые:

- Полный обход `maya_state.connections` без раннего выхода. Если
  syntactic-связей нет — должны выйти после первой итерации.
- Аллокация Vec для new_candidates даже если результат пуст.
- `HashMap` для candidates — даже пустой `HashMap::iter()` стоит. Если
  кандидатов часто 0, может быть `Option<HashMap>` или `Vec` с лимитом.

**Сценарий S3: C→D большой (это ожидаемо)**
Если C нормально, а D дорого — это **честная стоимость работы**, амортизируется
через `scan_interval_ticks`. Ничего чинить не надо, но стоит зафиксировать
характеристику.

### 3.4 Реализовать оптимизации

На основе сценария из 3.3 — точечные правки. **Не делать оптимизации, не
обоснованные числами.** Каждое изменение — после него прогнать бенчмарк
повторно, записать новое число.

### 3.5 Финальный замер

После всех оптимизаций — прогнать оригинальный criterion-бенчмарк:
```
TickForward / tokens_in_logic / 50
```

Целевая планка: **≤ 130 ns**. Если уложились — отлично. Если нет — обсудить:
вернулись в норму на 130-150 ns тоже допустимо, если 3x просадка превратилась в
1.3-1.5x.

### 3.6 Регрессионный бенчмарк

Добавить hot path tick в criterion как **постоянный** бенчмарк, который будет
прогоняться при каждом релизе. Это защитит от повторения проблемы:

```
benches/hot_path_regression.rs
- Замер tick на стандартной нагрузке
- Threshold: assert!(median_ns < 150) — если просядем выше, билд краснеет
```

### Критерий готовности этапа 3

- [ ] A/B/C/D бенчмарк запущен, числа записаны в errata
- [ ] Просадка диагностирована (S1/S2/S3 определён)
- [ ] Оптимизации реализованы по результатам диагностики
- [ ] Финальный hot path: ≤ 130 ns/tick (или явное обоснование, почему выше)
- [ ] Регрессионный бенчмарк добавлен в `axiom-bench`
- [ ] Все тесты остаются зелёными

---

## Этап 4 — финальная валидация (≈1 час)

**Цель:** убедиться, что после всех правок система целостна.

### 4.1 Полный прогон тестов

```
cargo test --workspace --all-features
cargo bench --bench frameweaver_overhead
cargo bench --bench hot_path_regression
```

Записать финальные числа.

### 4.2 Smoke-тест end-to-end

Прогнать сценарий, который:
1. Создаёт в MAYA синтаксический узор (через UCL)
2. Делает 30 тиков (FrameWeaver сканирует на 20-м)
3. Проверяет, что в EXPERIENCE появился Frame
4. Делает UCL UnfoldFrame в LOGIC
5. Проверяет, что в LOGIC появилась копия Frame
6. Проверяет метрики FrameWeaverStats

Это integration test, не unit-test. Положить в `crates/axiom-runtime/tests/`.

### 4.3 Обновить errata финальной сводкой

В `FrameWeaver_V1_1_errata.md` добавить раздел `## Resolution Summary`:

```
- E1: resolved в этапе 2 (`restore_frame_from_anchor`)
- E2: resolved в этапе 1 (trait-сигнатура исправлена)
- E3: resolved в этапе 3 (hot path: BEFORE → AFTER, см. таблицу A/B/C/D)
- E4: deferred (DREAM-фаза — отдельная задача)
```

### Критерий готовности этапа 4

- [ ] Все тесты зелёные
- [ ] Hot path в норме
- [ ] End-to-end smoke-тест проходит
- [ ] Errata обновлена с резолюциями

---

## После этого плана

После выполнения всех этапов FrameWeaver V1.1 будет стабилен и готов к двум
следующим шагам (которые **не входят в этот план**):

1. **Live-тестирование через CLI/дашборд.** Запуск с реальными входами через
   TextPerceptor, наблюдение за поведением.
2. **V1.2 спецификация.** На основе errata + наблюдений из live-теста — точечная
   ревизия V1.1.

После V1.2 можно переходить к проектированию следующего Weaver или DREAM-фазы.

---

## Резюме плана для Sonnet

| Этап | Что делаем                                       | Оценка времени | Блокер для следующего |
|------|--------------------------------------------------|-----------------|------------------------|
| 0    | Гигиена документов (Superseded, errata, deferred)| 30 минут        | нет                    |
| 1    | Trait сигнатура с tick                           | 1 час           | нет                    |
| 2    | restore_frame_from_anchor + UnfoldFrame + промоция| 4-6 часов      | этап 1                 |
| 3    | A/B бенчмарк + диагностика + оптимизация         | 3-5 часов       | этап 2                 |
| 4    | Финальная валидация                              | 1 час           | все предыдущие         |

**Итого:** 10-14 часов. Делать строго по порядку. После каждого этапа — фиксировать
прогресс в чате с chrnv для проверки.

---

## Что строго НЕ делаем

- Не трогаем DREAM-фазу
- Не пишем второго Weaver
- Не запускаем live-тестирование в CLI
- Не делаем рекурсивный UnfoldFrame depth>1
- Не реализуем демоцию SUTRA→EXPERIENCE
- Не делаем межсистемный обмен Frame
- Не оптимизируем то, что не подтверждено бенчмарком
