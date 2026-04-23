# BLUEPRINT.md — Patch для секции "Домены как зеркала"

**Дата:** 2026-04-22
**Назначение:** Обновление онтологической секции BLUEPRINT под трёхчастную онтологию (SUTRA/EXPERIENCE/MAYA) в соответствии с FrameWeaver V1.1 и Over-Domain Layer V1.1.

---

## Куда вставить

В `BLUEPRINT.md`, сразу после секции "Топология доменов (AshtiCore)" (после строки с `domain_name(id: u16) → &'static str — pub fn в axiom-runtime, по id % 100.`) и перед `---` разделителем, ведущим к секции "Domain & DomainState".

Если в текущей версии BLUEPRINT уже есть секция **"Фундаментальная архитектура: домены как зеркала"** (добавленная ранее, была в версии от 2026-04-20) — она **заменяется** на нижеследующий текст.

---

## Новый текст секции

```markdown
### Фундаментальная архитектура: трёхчастная онтология

AXIOM построен на разделении трёх онтологических слоёв, укоренённом в санскритской традиции:

**SUTRA (100) — нить, вечная истина.** Первичные сущности: anchor-токены, факты,
семантические оси, аксиомы. SUTRA не обрабатывает. Она хранит неизменное.

**EXPERIENCE (109) — накопленный опыт, история проявлений.** Удачные узоры, кристаллизованные
скиллы, закристаллизованный результат структурирования (Frame от FrameWeaver). Растёт,
меняется, обменивается между экземплярами AXIOM (Memory Persistence V1.0, IMPORT_WEIGHT_FACTOR=0.7).
Опыт может быть противоречив. Узоры здесь живут и стареют: часто используемые усиливаются,
редкие затухают.

**MAYA (110) — проявление, "сейчас".** Сборка узоров, генерация ответа, финальная
консолидация. Место живого плетения. MAYA получает либо рефлекс напрямую (быстрый путь
через EXPERIENCE), либо результат полного pipeline ASHTI 101–108 (медленный путь).

**ASHTI 101–108 — зеркала.** Они не создают новое содержание с нуля — они специализированные
линзы, через которые узоры преломляются в направлении MAYA. Каждый домен добавляет свою
перспективу (логика, симуляция, этика). У зеркал **два источника**: SUTRA ("что есть",
первичные сущности) и EXPERIENCE ("что бывало", накопленный опыт).

### Потоки между слоями

```
SUTRA (истина)  ──────┐
                      ├──► ASHTI 101-108 ─► MAYA (проявление)
EXPERIENCE (опыт) ────┤                           │
                      │                           │
EXPERIENCE (опыт) ◄───┴──── Weavers ◄──── MAYA (живые узоры)

                         [редкий путь]
EXPERIENCE ──► (GUARDIAN/CODEX) ──► SUTRA (промоция)
```

Следствия этой архитектуры:
- Добавление нового домена = новая линза/перспектива, не новое хранилище
- "Первичные истины" — в SUTRA; "накопленный опыт и скиллы" — в EXPERIENCE; "действие сейчас" — в MAYA
- Домены 101–108 можно заменить, добавить или убрать не меняя фундамента
- Weavers (FrameWeaver и будущие) пишут в EXPERIENCE по умолчанию; SUTRA — только через CODEX-санкционированную промоцию
- Межсистемный обмен идёт через EXPERIENCE; SUTRA у каждой системы своя
```

---

## Дополнительные правки в BLUEPRINT

### 1. В конце секции "Топология доменов", после списка доменов

Если у строки `109 — EXPERIENCE (role 9)  — ассоциативная память / рефлексы` есть желание уточнить — можно расширить до:

```
109 — EXPERIENCE (role 9)  — ассоциативная память, рефлексы, кристаллизованный опыт (Frame)
```

Это небольшое уточнение, показывающее, что EXPERIENCE — не только кэш рефлексов, но и слой онтологии.

### 2. В секции "Карта crates"

Если в карте crates уже есть `axiom-runtime`, дополнить его упоминанием Over-Domain Layer:

**Было:**
```
axiom-runtime    — AxiomEngine, Guardian, Gateway, Channel, EventBus, TickSchedule,
                   ProcessingResult, AdaptiveTickRate, Orchestrator, domain_name(),
                   BroadcastSnapshot + types (feature "adapters")
```

**Стало (добавить упоминание):**
```
axiom-runtime    — AxiomEngine, Guardian, Gateway, Channel, EventBus, TickSchedule,
                   ProcessingResult, AdaptiveTickRate, Orchestrator, domain_name(),
                   BroadcastSnapshot + types (feature "adapters"),
                   Over-Domain Layer (Guardians + Weavers, см. Over-Domain_Layer_V1_1.md)
```

### 3. Новая секция "Over-Domain Layer" (опционально, если есть желание)

Можно добавить короткую секцию перед или после раздела про Guardian, для плотного технического контекста:

```markdown
---

## Over-Domain Layer (axiom-runtime::over_domain)

Архитектурный слой компонентов над доменами. Две категории:

**Guardians** — контроль допустимости, veto-логика.
- GUARDIAN V1.0 — существующий (CODEX + GENOME enforcement)

**Weavers** — сборка реляционных структур, кристаллизация узоров в EXPERIENCE,
промоция в SUTRA через CODEX.
- FrameWeaver V1.1 — синтаксические/реляционные Frame (первый weaver)
- Deferred: CausalWeaver, SpatialWeaver, TemporalWeaver, AnalogyWeaver, NarrativeWeaver

### Инварианты
- Нет собственного хранилища (пишут в EXPERIENCE/SUTRA через UCL)
- Чтение только через `peek_state()`
- Авторизация через GENOME (ModuleId)
- Подчинены GUARDIAN

### Trait'ы
- `OverDomainComponent` — базовый (name, module_id, on_boot, on_tick, on_shutdown)
- `Weaver: OverDomainComponent` — scan, propose_to_dream, check_promotion, target_domain()=109

### Target domain
Weavers по умолчанию пишут в EXPERIENCE (domain_id=109). Промоция в SUTRA —
отдельный путь через CODEX-санкцию (раздел 5.4 FrameWeaver_V1_1.md).
```

---

## Суммарное воздействие патча

После применения этого патча BLUEPRINT.md:
1. Явно фиксирует трёхчастную онтологию (вместо двухчастной).
2. Документирует, что Weavers пишут в EXPERIENCE, не в SUTRA.
3. Показывает два источника у зеркал ASHTI.
4. Упоминает Over-Domain Layer как полноправный архитектурный слой.
5. Согласован с FrameWeaver V1.1 и Over-Domain Layer V1.1.

## Старые V1.0 документы

В списке `docs/specs/`:
- `FrameWeaver_V1_0.md` — пометить в заголовке `**Статус:** Superseded by V1.1 (онтологическая коррекция: Frame живёт в EXPERIENCE, не в SUTRA)`
- `Over_Domain_Layer_V1_0.md` — пометить в заголовке `**Статус:** Superseded by V1.1 (трёхчастная онтология)`

Не удалять — оставить как исторический след.
