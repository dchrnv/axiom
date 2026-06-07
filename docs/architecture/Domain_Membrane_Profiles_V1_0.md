# AXIOM — Domain Membrane Profiles V1.0

**Статус:** Спецификация
**Версия:** 1.0
**Дата:** 2026-06-05
**Категория:** Физика доменов · мембранная трансформация входящих токенов
**Crate:** `axiom-domain` + `axiom-config` (профили) + `axiom-runtime` (точка применения)
**Опирается на:** `Domain_V1_3`, `DomainConfig`, `GENOME_V1_0`, `GUARDIAN`, `Arbiter`, `AxialEvaluator`, `INVARIANTS.md`

> **Контекст:** идея родилась из наблюдения что entropy ≈ 0 и Dionysus недостижим, потому что все 8 ASHTI-доменов обрабатывают токен, но результаты сходятся в одну точку. AxialEvaluator видит только позиционные якоря, не дифференцированные доменные паттерны. Решение: мембрана домена трансформирует входящий токен согласно природе домена — физика работает с уже «окрашенным» материалом.

---

## 1. Проблема которую это решает

```
Сейчас:
  Токен проходит EXECUTION(+,+,+) и DREAM(-,-,-) →
  выходит с одинаковыми mass/valence/temperature.
  В EXPERIENCE кристаллизуются фреймы с плоской подписью.
  AxialEvaluator: entropy ≈ 0, Dionysus недостижим, 8 октантов не живые.

После мембран:
  Токен в EXECUTION → тяжёлый, горячий, притягивает (+окрашен).
  Тот же токен в DREAM → лёгкий, холодный, отталкивает (-окрашен).
  В EXPERIENCE — фреймы с РАЗНЫМИ подписями по доменам.
  AxialEvaluator читает настоящую дифференциацию → все 8 октантов живые.

Физика рождает узор органически из природы домена, а не приклеивается снаружи.
```

---

## 2. Природа 8 ASHTI доменов — оси и значения

Каждый домен имеет природу (X, Y, Z) по трём осям пространства AXIOM. Это **закодировано в координатах** доменов и теперь становится **физической подписью токена**.

| Домен | ID | Природа (X,Y,Z) | X:Apollo/Dionysus | Y:Eros/Thanatos | Z:Will/Nothing |
|-------|----|----------------|---------------------|-----------------|----------------|
| EXECUTION | 101 | (+,+,+) | Apollo | Eros | Will |
| SHADOW | 102 | (-,+,-) | Dionysus | Eros | Nothing |
| CODEX | 103 | (-,-,+) | Dionysus | Thanatos | Will |
| MAP | 104 | (+,-,+) | Apollo | Thanatos | Will |
| PROBE/INTNT | 105 | (+,+,-) | Apollo | Eros | Nothing |
| LOGIC | 106 | (-,+,+) | Dionysus | Eros | Will |
| DREAM | 107 | (-,-,-) | Dionysus | Thanatos | Nothing |
| VOID | 108 | (+,-,-) | Apollo | Thanatos | Nothing |

---

## 3. Параметры мембраны — три поля токена

Три поля Token уже несут семантику осей:

| Ось | Поле токена | Тип | Полюс А | Полюс Б |
|-----|------------|-----|---------|---------|
| X: Apollo/Dionysus | `mass: u8` | 0..255 | Apollo = **200** (тяжёлый/порядок) | Dionysus = **55** (лёгкий/хаос) |
| Y: Eros/Thanatos | `valence: i8` | -128..127 | Eros = **+40** (притяжение) | Thanatos = **-40** (отталкивание) |
| Z: Will/Nothing | `temperature: u8` | 0..255 | Will = **220** (горячий/активный) | Nothing = **80** (холодный/пассивный) |

**Симметрия значений:**
- mass: (200+55)/2 ≈ 128 (середина u8 = 128) ✓
- valence: +40/-40 (симметрично i8) ✓
- temperature: (220+80)/2 = 150 (умеренная середина) ✓

Значения умеренные — не экстремальные, чтобы токен оставался валидным для всех механизмов (гравитация, Shell, resonance) и не вызывал GUARDIAN-вето.

---

## 4. Профили мембран — 8 доменов

```
// Вычисляется из таблицы §2 + значений §3:

EXECUTION (101) — (+,+,+) — порядок · притяжение · активность
  mass_in = 200, valence_in = +40, temp_in = 220
  "Тяжёлый горячий магнит. Структурированные плотные паттерны."

SHADOW (102) — (-,+,-) — хаос · притяжение · пассивность
  mass_in = 55,  valence_in = +40, temp_in = 80
  "Лёгкий холодный магнит. Рыхлые но связанные паттерны."

CODEX (103) — (-,-,+) — хаос · отталкивание · активность
  mass_in = 55,  valence_in = -40, temp_in = 220
  "Лёгкий горячий отталкиватель. Динамичная разреженность."

MAP (104) — (+,-,+) — порядок · отталкивание · активность
  mass_in = 200, valence_in = -40, temp_in = 220
  "Тяжёлый горячий отталкиватель. Структурированное расталкивание."

PROBE/INTNT (105) — (+,+,-) — порядок · притяжение · пассивность
  mass_in = 200, valence_in = +40, temp_in = 80
  "Тяжёлый холодный магнит. Плотные медленные паттерны."

LOGIC (106) — (-,+,+) — хаос · притяжение · активность
  mass_in = 55,  valence_in = +40, temp_in = 220
  "Лёгкий горячий магнит. Быстрые рыхлые паттерны."

DREAM (107) — (-,-,-) — хаос · отталкивание · пассивность
  mass_in = 55,  valence_in = -40, temp_in = 80
  "Лёгкий холодный отталкиватель. Максимально разреженно и тихо."

VOID (108) — (+,-,-) — порядок · отталкивание · пассивность
  mass_in = 200, valence_in = -40, temp_in = 80
  "Тяжёлый холодный отталкиватель. Плотная холодная пустота."
```

---

## 5. Механизм трансформации

### 5.1 Где применяется — на ВХОДЕ в домен

```
// Трансформация применяется ПЕРЕД тем как домен начинает обработку.
// Не на выходе — на входе. Это принципиально: физика работает с уже
// окрашенным токеном → узор рождается органически из природы домена.

Точка применения в Arbiter (dual-path):
  fast path (resonance) — проходит без трансформации (latency-critical, § 5.3)
  slow path (domain pipeline ASHTI 1→8) — ЗДЕСЬ применяем мембрану
  
Конкретно: перед вызовом domain.apply(token) или аналогом —
  token = membrane_transform(token, domain_id)
```

### 5.2 Функция трансформации

```
// membrane_transform(token, domain_id) → token:
//   profile = GENOME.membrane_profiles[domain_id]
//
//   Трансформация — НЕ замена, а УПРАВЛЯЕМОЕ СМЕЩЕНИЕ:
//   new_mass = blend(token.mass, profile.mass_in, MEMBRANE_BLEND_FACTOR)
//   new_valence = blend_signed(token.valence, profile.valence_in, MEMBRANE_BLEND_FACTOR)
//   new_temp = blend(token.temperature, profile.temp_in, MEMBRANE_BLEND_FACTOR)
//
//   blend(current, target, factor):
//       (current as f32 * (1.0 - factor) + target as f32 * factor) as u8
//
// MEMBRANE_BLEND_FACTOR : f32 — конфигурируемый, 0.0..1.0
//   0.0 = мембрана не влияет (fallback к старому поведению)
//   1.0 = полная замена (токен полностью перекрашивается)
//   рекомендуемый старт: 0.5 (половина от природы домена)
//
// Не жёсткая замена (target = profile.mass_in) а blend. Почему:
//   - сохраняет семантику исходного токена частично
//   - постепенный переход, легко отлаживать
//   - BLEND_FACTOR = конфиг, можно подбирать по бенчмарку

// ВАЖНО: трансформированные значения clamp-ются в безопасные границы
// ДО применения (GUARDIAN не должен ловить очевидные артефакты):
//   mass: max(1, new_mass)              — не ноль (min_intensity > 0)
//   valence: clamp(new_valence, -127, 127)
//   temperature: max(1, new_temp)       — не ноль (иначе Frozen)
```

### 5.3 Исключение — fast path

```
// Fast path Arbiter (resonance_search) — latency-critical, ~20 µs.
// Мембранная трансформация НЕ применяется на fast path.
// Причина: resonance работает с исходными параметрами токена,
//   это рефлекторный путь, не когнитивный.
// Мембрана = когнитивная обработка = только slow path (ASHTI pipeline).
```

---

## 6. Хранение профилей — GENOME

```
// Природа доменов — конституция системы. Хранится в GENOME (frozen после boot).
// Нельзя менять в runtime (как все остальные конституционные правила).

// genome.yaml:
domain_membrane_profiles:
  execution_101: {mass_in: 200, valence_in:  40, temp_in: 220}
  shadow_102:    {mass_in:  55, valence_in:  40, temp_in:  80}
  codex_103:     {mass_in:  55, valence_in: -40, temp_in: 220}
  map_104:       {mass_in: 200, valence_in: -40, temp_in: 220}
  probe_105:     {mass_in: 200, valence_in:  40, temp_in:  80}
  logic_106:     {mass_in:  55, valence_in:  40, temp_in: 220}
  dream_107:     {mass_in:  55, valence_in: -40, temp_in:  80}
  void_108:      {mass_in: 200, valence_in: -40, temp_in:  80}

membrane_blend_factor: 0.5   // старт, подбирается по benchmarks + OBS

// MEMBRANE_BLEND_FACTOR в GENOME — не в DomainConfig.
// Почему GENOME а не конфиг доменов: природа домена = конституция,
//   не оперативная настройка. Изменить профиль = изменить суть домена.
//   Это требует генома-эволюции (будущее), не рутинного конфига.

// Загрузка: genome.yaml → struct Genome → Arc<Genome> → AxiomEngine::new.
//   membrane_profiles: HashMap<u16, MembraneProfile>  // key = domain_id
```

---

## 7. CODEX + GUARDIAN — красная зона

```
// CODEX (домен 103) = пластичный закон, задаёт ограничения.
// GUARDIAN = единственный исполнитель.
//
// Красная зона — экстремальные параметры после трансформации:
//   mass > 250 или mass < 1
//   |valence| > 120
//   temperature > 250 или temperature < 1
//
// При нарушении: GUARDIAN применяет clamping перед применением UCL.
// НЕ вето (трансформация полезная), только ограничение крайностей.
// Логируется в GuardianLog для наблюдения.
//
// Конфиг красной зоны — в CODEX-секции genome.yaml (не в GENOME.membrane):
codex_constraints:
  max_mass_after_membrane: 250
  min_mass_after_membrane: 1
  max_abs_valence_after_membrane: 120
  max_temp_after_membrane: 250
  min_temp_after_membrane: 1
```

---

## 8. Что НЕ меняется

```
// Token 64B — HARD. mass/valence/temperature уже там. Структура не меняется.
// Трансформация работает с СУЩЕСТВУЮЩИМИ полями.

// Connection, Event, DomainConfig — не трогаем.
// domain_position_hash — не трогаем (sutra_id диапазоны остаются как есть).
// Fast path resonance_search — не трогаем (§5.3).
// AxialEvaluator — не трогаем. Он автоматически начнёт читать правильные
//   signatures, когда токены придут уже окрашенными.
// FrameWeaver — не трогаем. Кристаллизует из того что получит в MAYA.
// SUTRA (100), EXPERIENCE (109), MAYA (110) — не трогаем. Трансформация
//   только в pipeline 101–108.
```

---

## 9. Ожидаемый эффект — что изменится в метриках

```
После включения мембран с BLEND_FACTOR=0.5, следующий OBS-прогон должен показать:

  ActivityDynamics:
    entropy > 0 (была ≈ 0) — КЛЮЧЕВОЙ ПОКАЗАТЕЛЬ
    signature ≠ "Steady always" — появится Cascading/Oscillating
    oscillation_score > 0 — переключения между режимами

  AxialEvaluator:
    октанты Dionysus(-), Thanatos(-), Nothing(-) начинают активироваться
    не только Apollo/Eros/Will (как сейчас)
    Corpus Callosum конфликты появляются естественно (без искусственного добавления)

  ContextRecognizer:
    active_dilemmas > 0 при конфликтных текстах (DREAM и EXECUTION создают
    противоположные подписи → DilemmaDetector ловит)

Если этого нет после включения → BLEND_FACTOR слишком мал, поднять до 0.7.
Если система нестабильна (entropy слишком высокий, лавина реактивации) →
  BLEND_FACTOR слишком велик, опустить до 0.3.
```

---

## 10. Тесты

```
test_membrane_transform_execution
  token с mass=128, valence=0, temp=128 → через EXECUTION мембрану →
  mass > 128 (ближе к 200), valence > 0 (ближе к +40), temp > 128 (ближе к 220)
  с factor=0.5: mass ≈ 164, valence ≈ 20, temp ≈ 174

test_membrane_transform_dream
  тот же токен → через DREAM мембрану →
  mass < 128, valence < 0, temp < 128

test_membrane_symmetry
  EXECUTION и DREAM трансформации токена-128 симметричны (delta ≈ ±равная)

test_membrane_clamping
  токен с mass=0 → не остаётся 0 после трансформации (clamp to 1)
  токен с valence=-127 через Thanatos → не уходит за -127

test_fast_path_unchanged
  resonance_search путь: токен НЕ трансформируется (нет membrane на fast path)

test_no_token_struct_change
  Token.sutra_id, Token.position, Token.flags — не изменились после трансформации
  (только mass/valence/temperature)

test_genome_profiles_loaded
  после AxiomEngine::new все 8 профилей загружены из genome.yaml
  execution_101.mass_in == 200, dream_107.temp_in == 80
```

---

## 11. Порядок реализации

```
1. MembraneProfile struct + genome.yaml секция
   {mass_in: u8, valence_in: i8, temp_in: u8, blend_factor: f32}
   (blend_factor per-domain для гибкости, переопределяет global)
   Загрузка в Genome, доступ AxiomEngine::new

2. membrane_transform(token, profile) — чистая функция, без side effects
   + clamp safety bounds
   тесты: transform_execution, transform_dream, symmetry, clamping

3. Точка применения в Arbiter slow path
   перед domain.apply(token) — вызов membrane_transform
   тест: fast_path_unchanged

4. Прогон OBS с BLEND_FACTOR=0.5
   смотреть entropy, Dionysus активность, диапазон октантов
   калибровать factor по наблюдению (§9)

5. CODEX красная зона — добавить проверку в GUARDIAN
   при выходе за bounds: clamping + лог
```

---

## 12. Связь с SUTRA и SyntacticBridge

```
// Из IDE-диалога также обсуждалась проблема SyntacticBridge:
//   target_id = domain_position_hash(domain_id, position) →
//   такие токены не существуют в EXPERIENCE → participants = [] →
//   AxialEvaluator всегда позиционный fallback.
//
// Мембранная трансформация ЧАСТИЧНО решает это: токены в MAYA теперь несут
// подпись домена через mass/valence/temperature → FrameWeaver кристаллизует
// фреймы с реальной дифференциацией → EXPERIENCE получает подписанные якоря.
//
// Но SyntacticBridge domain_position_hash → это ОТДЕЛЬНАЯ задача (ASHTI-TD-01).
// Мембраны = шаг 1 (физическая окраска).
// SyntacticBridge = шаг 2 (правильные участники фрейма).
// Оба нужны для полного решения. Но мембраны дают немедленный измеримый эффект
// и не зависят от SyntacticBridge.
```

---

## 13. Инварианты

| Правило | Значение |
|---------|----------|
| Token 64B | HARD — не меняется, трансформация работает с существующими полями |
| Трансформация | только slow path (ASHTI 1→8), НЕ fast path (resonance) |
| Профили мембран | в GENOME (frozen после boot), не в DomainConfig |
| Blend factor | 0.0..1.0, конфигурируемый, глобальный или per-domain |
| mass после трансформации | min 1 (clamp, min_intensity>0 инвариант) |
| temperature после | min 1 (иначе token сразу frozen) |
| valence после | clamp(-127, 127) |
| SUTRA (100) | не трогается |
| Домены 109 (EXPERIENCE), 110 (MAYA) | не трогаются |
| Fast path | нет трансформации |
| GUARDIAN | clamp красной зоны + лог, не вето |

---

## История

- **V1.0** (2026-06-05): первая спека. Решает проблему entropy≈0 и Dionysus-недостижимости через мембранную трансформацию входящих токенов по природе домена (+/-,+/-,+/-). Три параметра: mass/valence/temperature → оси Apollo/Eros/Will. Значения симметричны (200/55, ±40, 220/80). BLEND_FACTOR конфигурируемый. Профили в GENOME (конституция). CODEX+GUARDIAN ограничивают красную зону. Не ломает Token 64B, не трогает fast path, не трогает AxialEvaluator (он начнёт видеть настоящую дифференциацию сам). ASHTI-TD-01 (SyntacticBridge) — смежная задача, шаг 2 полного решения.
```
