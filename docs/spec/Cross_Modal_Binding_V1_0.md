# AXIOM — Cross-Modal Binding V1.0

**Статус:** Спецификация
**Версия:** 1.0
**Дата:** 2026-05-31
**Категория:** Под-механизм ContextRecognizer (не отдельный over-domain модуль)
**Crate:** `axiom-runtime` / `over_domain/context_recognizer/cross_modal/`
**Опирается на:** `Universal_Grounding_Stack_V1_1` §12, `Primitive_Nature_and_Connections_V1_0`, `ContextRecognizer_V5_0`, `Connection_V5_0`, `INVARIANTS.md`
**Предпосылка:** composition bonds (V7-A1) и L0/L1 split (V7-A2) реализованы. Это единственный крупный незакрытый механизм UGS.

---

## 0. Зачем

Сейчас AXIOM понимает **символы про символы**. Текстовое "красный" — это Frame, связанный с другими текстовыми Frame. Система не знает, что "красный" имеет отношение к визуальному перцепту красного.

Cross-modal binding даёт **grounding** — привязку символа к перцепту. Текстовое "красный" связывается с визуальным кластером (длина волны ~700nm), когда они достаточно часто появляются вместе. Не hardcoded mapping — **emergent binding через co-activation**.

Это превращает AXIOM из системы про символы в систему про **мир, означенный символами**.

---

## 1. Принцип

Два Frame из **разных модальностей**, co-active в близком causal-time окне, получают **cross-modal bond** — связь особого типа. Через повторение. Не задаётся вручную.

```
"красный"     → TextPerceptor   → Frame[красный]       → sutra_id=X
красный цвет  → VisionPerceptor → Frame[700nm кластер] → sutra_id=Y

X и Y co-active 50+ раз в близком окне
  → CROSS_MODAL_BOND: X ←→ Y
  → система обнаружила: текстовое "красный" и визуальный кластер — одно
```

Это прямое применение `Primitive_Nature`: cross-modal bond — **примитив-отношение**, реализованный как Connection, не как якорь. Живёт со strength (укрепляется), stress (рассинхрон модальностей), elasticity.

---

## 2. Modality на Frame

Frame должен знать из какой модальности пришёл. **Не в Token** (64 байта HARD) — отдельный store.

```rust
// crates/axiom-experience/src/modality_store.rs — НОВОЕ

pub struct FrameModality {
    pub sutra_id: u32,
    pub modality: Modality,
    pub perceptor_source: u16,    // id перцептора, создавшего Frame
}

pub enum Modality {
    Text,        // TextPerceptor — работает
    Vision,      // L0VisionPerceptor — частично (Sobel→strokes)
    Audio,       // future
    Causal,      // future
    Spatial,     // future
    Internal,    // EXPERIENCE без внешнего входа (воспоминание, DREAM)
}
```

(enum Modality намечен в ContextRecognizer V6 roadmap §1.7 — здесь конкретизирован)

При кристаллизации Frame перцептор проставляет свою modality. Существующие текстовые Frame получают `Text` (миграция: дефолт Text для всех имеющихся, т.к. до сих пор был только TextPerceptor).

---

## 3. Co-activation детектор

Встроен в ContextRecognizer (он уже сканирует co-activation для дилемм — переиспользуем механизм).

```rust
pub struct CrossModalCandidate {
    pub frame_a: u32,
    pub frame_b: u32,
    pub modality_a: Modality,
    pub modality_b: Modality,
    pub co_activation_count: u32,
    pub first_seen_event: u64,
    pub last_seen_event: u64,
    pub avg_temporal_distance: u32,   // насколько близко во времени co-active
}
```

### Алгоритм

```
on_tick (под-шаг в CR, рядом с dilemma detector):

    active_frames = frames active in current window
    
    # Группировка по модальности
    by_modality = group_by_modality(active_frames)   # через modality_store
    
    # Только если активны минимум ДВЕ модальности
    if by_modality.len() < 2:
        return    # cross-modal требует разных модальностей
    
    # Пары между модальностями (не внутри одной)
    for (mod_a, frames_a) in by_modality:
        for (mod_b, frames_b) in by_modality where mod_b > mod_a:
            for fa in frames_a:
                for fb in frames_b:
                    update_candidate(fa, fb, mod_a, mod_b, current_event)


update_candidate(fa, fb, ma, mb, event):
    key = (min(fa,fb), max(fa,fb))
    cand = candidates.entry(key)
    cand.co_activation_count += 1
    cand.last_seen_event = event
    cand.avg_temporal_distance = running_avg(...)
    
    if cand.co_activation_count >= MIN_CROSS_MODAL_COACTIVATION:   # 50
        if not bond_exists(fa, fb):
            propose_cross_modal_bond(cand)
```

**Важно:** пары считаются **только между разными модальностями**. Внутри одной модальности co-activation — это обычная композиция (composition bonds), не cross-modal. Это разводит два механизма чётко.

---

## 4. Создание bond

```
propose_cross_modal_bond(candidate):
    # Только в DREAM Phase — не на горячем пути
    if not in_dream_phase():
        queue_for_dream(candidate)
        return
    
    # Через UCL, валидируется GUARDIAN
    submit_ucl(BondTokens {
        source_id: candidate.frame_a,
        target_id: candidate.frame_b,
        link_type: CROSS_MODAL_BOND,        # 0x0902 (рядом с COMPOSITION_BOND 0x0901)
        strength: normalize(candidate.co_activation_count),
        domain_id: 109,                      # EXPERIENCE
    })
    
    # chrnv видит в Workstation, может отозвать
    notify_workstation(candidate)
```

`BondTokens` opcode (2003) уже существует. `CROSS_MODAL_BOND = 0x0902` — новый link_type рядом с `COMPOSITION_BOND = 0x0901`.

### Категория связи

Cross-modal bond — это **новая семантическая категория** в Shell. Связь между модальностями вносит вклад в восприятие:

```yaml
# config/schema/semantic_contributions.yaml
categories:
  0x0A:  # CrossModal (новая)
    name: "CrossModal"
    base_profile: [10, 20, 0, 0, 10, 0, 0, 10]   # L2 sensory (перцепт) + L5 cognitive (символ)
```

L2 высокий потому что cross-modal привязывает к ощущению. L5 потому что связывает с понятием. Это и есть grounding — мост сенсорного и когнитивного.

---

## 5. Контроль и пороги

| Параметр | Значение | Зачем |
|----------|----------|-------|
| `MIN_CROSS_MODAL_COACTIVATION` | 50 | высокий — не связывать случайное (выше чем дилемма=2, т.к. ложная связь дороже) |
| Создание bond | только в DREAM Phase | не на горячем пути |
| Валидация | GUARDIAN | как все мутации |
| chrnv approval | да, как emergent primitives | значимое изменение |
| Отзыв bond | возможен | §6 — защита от ложных связей |
| Genome control | `allow_cross_modal_binding`, `min_co_activation`, `require_approval` | контроль через конституцию |

### Genome

```yaml
# genome.yaml fragment
cross_modal:
  allow_binding: true
  min_co_activation: 50
  require_chrnv_approval: true
  max_bonds_total: 10000
  allow_revocation: true
```

---

## 6. Отзыв ложной связи (риск из UGS §14.3)

Риск: два Frame случайно co-active часто (всегда читаешь про красное на красном фоне) → ложная связь.

Защита — **stress-driven revocation**:

```
Каждый cross-modal bond имеет current_stress (поле Connection, уже есть).

Когда модальности РАСХОДЯТСЯ (одна активна, другая нет, когда ожидалась):
    bond.current_stress += delta

Если stress превышает порог в течение N DREAM-циклов:
    предложить отзыв bond
    chrnv подтверждает в Workstation
    bond помечается INHIBITED (не удаляется сразу — может восстановиться)
```

Это использует `current_stress` ровно как задумано в `Primitive_Nature`: связь живёт, напряжение показывает что отношение под вопросом. Ложная связь сама накопит stress и будет отозвана.

---

## 7. Что это открывает

Когда работает:

- **Картинка → понятие.** Видит визуальный паттерн → активирует связанное текстовое понятие.
- **Слово → ожидание перцепта.** Читает "красный" → готов к визуальному паттерну.
- **Кросс-модальная проверка.** Текст говорит одно, картинка другое → stress → дилемма (связь с DilemmaDetector — сигнал B/C).
- **Путь к клинописи шаг 8** (UGS §13): когда приходит контекст-перевод, семантика проявляется поверх структуры через cross-modal bond символ↔значение.

Это grounding в прямом смысле: символ привязан к миру, не только к другим символам.

---

## 8. Инварианты

| Правило | Значение |
|---------|----------|
| Где живёт детектор | под-механизм ContextRecognizer |
| Когда тикает | в CR on_tick, рядом с dilemma detector |
| Modality | отдельный store, НЕ в Token (HARD) |
| Пары | только между разными модальностями |
| Bond link_type | CROSS_MODAL_BOND = 0x0902 |
| Категория связи | 0x0A CrossModal |
| Создание | только DREAM Phase, через UCL, GUARDIAN |
| Порог | MIN_CROSS_MODAL_COACTIVATION = 50 |
| Отзыв | stress-driven, chrnv подтверждает |
| chrnv approval | обязателен для создания |

---

## 9. Что в коде

```
crates/axiom-runtime/src/over_domain/context_recognizer/
└── cross_modal/                  — НОВОЕ
    ├── mod.rs                    — CrossModalDetector
    ├── candidate.rs              — CrossModalCandidate, co-activation учёт
    ├── bond.rs                   — создание bond через UCL
    └── revocation.rs             — stress-driven отзыв

crates/axiom-experience/src/
└── modality_store.rs             — НОВОЕ — FrameModality

crates/axiom-core/src/
└── connection.rs:
    + CROSS_MODAL_BOND: u16 = 0x0902

config/schema/semantic_contributions.yaml:
+ 0x0A CrossModal категория

genome.yaml:
+ cross_modal раздел
```

Перцепторы проставляют modality при кристаллизации (TextPerceptor → Text, VisionPerceptor → Vision).

---

## 10. Проверка успеха

Нужны **две модальности одновременно**. Сейчас реально работает Text, Vision частична. Минимальный тест:

- Подать текст "круг" + синтетическую картинку круга (через L0VisionPerceptor) в близком окне, многократно
- Проверить: возникает ли CrossModalCandidate, накапливается ли co_activation_count
- После порога: создаётся ли CROSS_MODAL_BOND в DREAM
- Ложный тест: подать несвязанные text+vision → bond НЕ должен создаться (порог 50 защищает)

Это **первый тест grounding** — связывания символа с перцептом. Если работает — UGS доказан не только структурно (клинопись), но и семантически (символ↔мир).

Зависимость: требует чтобы Vision pipeline был хотя бы минимально замкнут (UGS §11.1). Если Vision ещё не доводит до Frame — тест делается на двух text-под-модальностях как заглушка, потом на реальном Vision.

---

## 11. Связь с другими спеками

- **Universal Grounding Stack §12**: это реализация того раздела.
- **Primitive_Nature_and_Connections**: cross-modal bond = примитив-отношение, живёт как Connection со stress.
- **DilemmaDetector**: расхождение модальностей (text ≠ vision) → сигнал дилеммы.
- **ContextRecognizer**: co-activation механизм переиспользуется из dilemma detector.

---

# ПРИЛОЖЕНИЕ — Заготовка DilemmaDetector V2.1 (не потерять)

**Назначение приложения:** ветка 1 (углубление дилемм) отложена в пользу cross-modal, но зафиксирована здесь как готовое ТЗ. При возврате — развернуть в полную спеку за один присест, не восстанавливая с нуля.

## A. Что уже есть (готовая почва)

Из отчёта Sonnet 2026-05-31, V2.0 закрыта. Инфраструктура для V2.1 **вся на месте**:

- `current_stress` на Connection (V5.0) — для сигнала B
- AxialStore снапшот уже в CR — для сигнала C
- `DilemmaResolution::Dissolved` + `HeldInTension` уже определены в DilemmaStore
- co-activation механизм работает (путь 2, скользящее окно)

То есть V2.1 — снова **связка готовых частей**, как V2.0. Не новая инфраструктура.

## B. Четыре задачи V2.1

### B1. Сигнал B — напряжение связей
```
Сканировать current_stress на активных Connection.
Если связь A--[supports]-->B имеет высокий stress
  И одновременно активна A--[conflicts]-->B
  → противоречивые отношения → DilemmaCandidate { signal: ConnectionStress }
```
Опора: `current_stress` уже растёт в физике связей. Нужен порог STRESS_DILEMMA_THRESHOLD и скан активных связей в окне.

### B2. Сигнал C — Corpus Callosum
```
Читать AxialConflict из axial_store_snapshot (уже в CR).
Если analytic_octant != synthetic_octant для Frame
  → дилемма оценки (уровень 3) → DilemmaCandidate { signal: CorpusCallosum }
```
Опора: AxialEvaluator уже фиксирует конфликт. Нужно только читать снапшот и поднимать дилемму.

### B3. Отсев ложных (уровень 0) и трейд-оффов (уровень 1)
```
Уровень 0 (FalseDilemma): конфликт снимается уточнением факта
  → проверка: есть ли Frame, разрешающий противоречие в EXPERIENCE
  → если да: DilemmaResolution::Dissolved (не настоящая дилемма)

Уровень 1 (TradeOff): измеримый Парето-выбор
  → проверка: обе стороны измеримы и сравнимы по одной шкале
  → если да: пометить TradeOff, не эскалировать до глубокой дилеммы
```
Цель: не путать ложную/тривиальную дилемму с настоящей. Сейчас V2.0 всё считает RuleConflict.

### B4. HeldInTension (уровень 3)
```
Несовместимые модели реальности — НЕ разрешать, удерживать обе параллельно.
Из Дилеммы.md уровень 3: "оперировать обеими моделями как взаимодополнительными".

DilemmaResolution::HeldInTension:
  обе конфликтующие интерпретации остаются активными
  ни одна не подавляется
  Frame дилеммы помечается как "удерживаемый"
  периодически переоценивается (вдруг появится разрешение)
```
Опора: `HeldInTension` уже определён в DilemmaStore. Нужна логика удержания (не гасить вторую интерпретацию).

## C. Структуры (расширение существующих)

```rust
// DilemmaSignal уже имеет варианты — активировать неиспользуемые:
ConnectionStress,    // B1
CorpusCallosum,      // B2

// DilemmaResolution уже имеет — активировать:
Dissolved,           // B3 уровень 0
HeldInTension,       // B4 уровень 3

// Новые пороги:
STRESS_DILEMMA_THRESHOLD: f32      // B1
PARETO_COMPARABILITY_CHECK         // B3
```

## D. Инварианты V2.1

| Правило | Значение |
|---------|----------|
| Где | тот же dilemma под-механизм CR |
| Сигналы | A (есть) + B (stress) + C (Corpus Callosum) |
| Уровни покрытия | 0,1,2,3 (было 2,3 частично) |
| HeldInTension | не подавлять вторую интерпретацию |
| Новая инфраструктура | нет — всё уже определено, активируем |

## E. Что остаётся после V2.1

Из общего плана §6 спеки DilemmaDetector_V2_0:
- **V2.2** — уровень 4 (рефлексивные парадоксы, self-reference, fixed point)
- **V3.0** — уровень 5 (аксиогенез, рождение ценности, смыкается с ContextRecognizer V8 roadmap)

## F. Триггер возврата к ветке 1

Вернуться к V2.1 когда:
- cross-modal binding работает (эта спека закрыта), ИЛИ
- дилеммы начинают давать ложные срабатывания (нужен отсев B3), ИЛИ
- появляется потребность удерживать несовместимые модели (B4) — например при cross-modal расхождениях (text ≠ vision как дилемма)

Последний пункт — **прямой мост**: cross-modal расхождение (§7) естественно требует HeldInTension из V2.1. Так две ветки сходятся.

---

## История

- **V1.0** (2026-05-31): первая спека cross-modal binding. Реализует UGS §12. Modality на Frame (отдельный store), co-activation детектор между модальностями, CROSS_MODAL_BOND 0x0902, stress-driven отзыв ложных связей. Встроен в ContextRecognizer. Приложение: заготовка DilemmaDetector V2.1 (ветка 1) зафиксирована как готовое ТЗ чтобы не потерять — разворачивается в полную спеку при возврате.
