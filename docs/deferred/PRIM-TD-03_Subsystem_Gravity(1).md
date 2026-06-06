# AXIOM — PRIM-TD-03: Subsystem Gravity (Value / Abstraction)

**Документ:** инструкция для реализации (Sonnet)
**Дата:** 2026-06-03
**Контекст:** специфичная гравитация для Values и Abstractions. Якоря `val_beneficial`/`val_harmful` создают притяжение/отталкивание; высокоабстрактные якоря (A5 theory, A6 constructor) тянут токены из нижних уровней.
**Опирается на:** `BLUEPRINT.md` (apply_gravity_batch в axiom-space, AnchorSet в axiom-config), `Primitive_Nature_and_Connections`, `INVARIANTS.md`
**Согласовано:** chrnv + Opus

---

## 0. Главное решение до кода

**Subsystem-гравитация НЕ входит в hot-path `apply_gravity_batch`.**

`apply_gravity_batch` — это 23.4 µs/1K, SIMD/AVX2, горячий путь, его берегут. Если втащить туда правила подсистем (проверки sutra_id, direction, radius на каждый токен каждый тик) — убьём производительность и сложим SIMD.

Решение: **отдельная функция `apply_subsystem_gravity`, на reconcile-интервале, не каждый тик.** Она дополняет базовую гравитацию, не лезет в её горячий цикл. Базовая физика остаётся нетронутой и быстрой.

Это соответствует природе эффекта: притяжение к "благому" и абстрактному — **медленное смысловое смещение**, не покадровая физика. Ему не нужен каждый тик. Reconcile-интервал (или свой, более редкий) — правильный темп.

---

## 1. Интерфейс SubsystemGravityRule

```
// crates/axiom-space/src/lib.rs (или новый модуль subsystem_gravity.rs в space)

SubsystemGravityRule {
    anchor_sutra_id: u32,    // sutra_id якоря — центр pull/push
    anchor_position: [i16;3],// позиция якоря (кэш, чтобы не искать каждый раз)
    direction: f32,          // +1.0 притяжение, -1.0 отталкивание
    strength_factor: f32,    // множитель к базовой гравитации (умеренный!)
    radius: Option<u32>,     // None = без ограничения; Some(r) = только в радиусе
    target_domain: u16,      // к токенам какого домена применять (напр. MAYA 110)
    target_subsystem: Option<SubsystemId>,  // опц. сузить до подсистемы
}
```

`anchor_position` кэшируется в правиле при boot — якоря STATE_LOCKED, не двигаются (кроме медленного DREAM-сдвига, см. §5), позицию можно держать в правиле и не дёргать SUTRA каждый reconcile.

---

## 2. Сборка правил из AnchorSet (при boot)

```
// При inject_anchor_tokens или сразу после — собрать Vec<SubsystemGravityRule>.
// Вычисляется ОДИН раз при boot, хранится в AxiomEngine, не меняется в runtime.

VALUES:
  val_beneficial  → direction=+1.0 (pull),  strength_factor=УМЕРЕННЫЙ (см. §4)
  val_harmful     → direction=-1.0 (push),  strength_factor=УМЕРЕННЫЙ
  target_domain   = MAYA (110) — активные токены восприятия
  // ценностные якоря притягивают "благое", отталкивают "вредное"

ABSTRACTIONS:
  A5 (theory)     → direction=+1.0 (pull),  strength_factor=СЛАБЫЙ
  A6 (constructor)→ direction=+1.0 (pull),  strength_factor=СЛАБЫЙ
  radius          = Some(R) — только токены в пределах радиуса (локальное втягивание вверх)
  target_subsystem= Some(Abstractions) — тянет токены нижних абстрактных слоёв
  // высокоабстрактные якоря медленно подтягивают токены снизу вверх

// ТОЧНЫЕ ПОЗИЦИИ ЯКОРЕЙ (подтверждено chrnv, якоря НЕ двигаются — кэшировать навечно):
//   val_beneficial          : [8000, 12000, 13000]
//   val_harmful             : [3000,  1000, 11000]
//   abstraction_theory      : [13000, 10000, 14000]   (A5 = C5)
//   abstraction_constructor : [14000, 12000, 15000]   (A6 = C5+)
//
// ГЕОМЕТРИЯ (учесть при калибровке):
//   beneficial ↔ harmful  : ~12247 — ДАЛЕКО. Pull и push в разных зонах,
//                           не борются за одни токены. Чисто.
//   theory ↔ constructor  : ~2449  — БЛИЗКО. A5+A6 = по сути ОДИН центр
//                           притяжения вверх. Их radius перекроется →
//                           токен в зоне перекрытия получит ДВОЙНОЙ pull.
//                           → либо radius небольшой, либо factor ещё слабее
//                             (учесть удвоение в зоне A5+A6).
```

---

## 3. Функция apply_subsystem_gravity

```
// crates/axiom-space или вызов из axiom-runtime на reconcile.
//
// apply_subsystem_gravity(tokens, rules, event_id):
//   for rule in rules:
//     for token in tokens where token.domain == rule.target_domain:
//       // опц. фильтр по подсистеме
//       if rule.target_subsystem.is_some() && token не в той подсистеме: skip
//
//       delta = token.position - rule.anchor_position
//       dist  = length(delta)
//
//       // радиус-отсечка
//       if let Some(r) = rule.radius { if dist > r: continue }
//
//       // сила: базовая гравитация × factor × direction, спад с дистанцией
//       force = base_gravity_term(dist) * rule.strength_factor * rule.direction
//
//       // притяжение (+) двигает токен К якорю, отталкивание (-) ОТ
//       apply_displacement(token, normalize(delta) * force * (-rule.direction... ))
//       // знак направления: pull сокращает delta, push увеличивает
//
//   // event_id — для причинности (если смещение порождает событие). НЕ wall-clock.
```

Ключевое:
- работает по тем же законам что базовая гравитация (спад с дистанцией), только с множителем и направлением
- НЕ внутри `apply_gravity_batch` — отдельный проход
- radius-отсечка экономит: большинство токенов вне радиуса абстрактных якорей пропускаются сразу

---

## 4. Калибровка силы — осторожно

```
// strength_factor должен быть УМЕРЕННЫМ/СЛАБЫМ. Почему критично:
//
// Риск 1 — коллапс к якорю. Слишком сильный pull val_beneficial стянет
//   все токены в одну точку → потеря пространственной структуры.
//   Это та самая проблема "center collapse" (открытый вопрос проекта про
//   гравитацию к Anchor(0,0,0)). Subsystem-гравитация НЕ должна её усугублять.
//
// Риск 2 — борьба с базовой гравитацией. Если factor сравним с базовой —
//   токены задёргаются между естественным притяжением и subsystem-pull.
//
// Стартовые значения: factor 0.1–0.3 от базовой. Push (harmful) и pull
//   (beneficial) симметричны по модулю. Abstractions ещё слабее (0.05–0.15) —
//   это лёгкое подтягивание вверх, не насос.
//
// Откалибровать ПОСЛЕ первого прогона по факту — смотреть не схлопывается ли
//   распределение. min_intensity>0 инвариант: ничто не должно стянуться в ноль.
```

---

## 5. Хранение и инварианты

```
// Хранение: Vec<SubsystemGravityRule> в AxiomEngine. Boot-time, immutable в runtime.
//
// ЯКОРЯ НЕ ДВИГАЮТСЯ (подтверждено chrnv). Поэтому:
//   - anchor_position кэшируется в правиле при boot — валиден навечно
//   - механизм обновления правил НЕ НУЖЕН
//   - val_*/A5/A6 НЕ двигаются в DREAM (зафиксированы)
//
// ИНВАРИАНТЫ:
//   - wall-clock запрещён, только event_id
//   - не в hot-path apply_gravity_batch (отдельная функция, reconcile-интервал)
//   - min_intensity>0 — ничто не схлопывается (контроль через умеренный factor)
//   - правила immutable после boot
//   - STATE_LOCKED якоря не двигаются от этой гравитации (они центры, не цели)
```

---

## 6. Где вызывать

```
// На reconcile-интервале (НЕ каждый тик). Варианты:
//   а) внутри существующего reconcile-прохода (если он есть на интервале)
//   б) свой интервал в TickSchedule (напр. subsystem_gravity_interval,
//      реже базовой гравитации — раз в N тиков)
//
// Рекомендация: свой интервал, редкий. Смысловое смещение медленное,
//   ему не нужна частота физики. Это и бережёт hot path.
//
// Порядок: после базовой гравитации (subsystem-pull корректирует поверх).
```

---

## 7. Тесты

```
test_beneficial_pulls_nearby_token
  токен рядом с val_beneficial → после apply сместился К якорю

test_harmful_repels_nearby_token
  токен рядом с val_harmful → сместился ОТ якоря

test_no_effect_beyond_radius
  токен за пределами radius абстрактного якоря → позиция не изменилась

test_rules_loaded_from_anchor_set
  после boot Vec<SubsystemGravityRule> непустой, содержит val_beneficial/harmful
  и A5/A6 с правильными direction

// ДОБАВИТЬ:
test_no_collapse_under_repeated_application
  N применений подряд → токены НЕ схлопнулись в точку якоря
  (проверка min_intensity>0, защита от коллапса §4)

test_not_in_hot_path
  apply_gravity_batch не вызывает subsystem-логику
  (раздельность горячего пути и медленной гравитации)
```

---

## 8. Bench — обязательно перед мержем

```
// Это физика, hot path рядом. Перед мержем:
//   1. apply_gravity_batch (1K) — должен остаться ~23.4 µs (НЕ вырасти).
//      Если вырос — subsystem-логика просочилась в hot path, чинить.
//   2. apply_subsystem_gravity (свой bench) — измерить отдельно.
//      Это на reconcile, не каждый тик, поэтому амортизируется.
//   3. TickForward (50 tok) — не должен просесть от нового интервала.
//
// Если subsystem_gravity дорогая — увеличить интервал (реже применять),
//   она смысловая, терпит редкость.
```

---

## 9. Порядок работ

```
1. SubsystemGravityRule + сборка из AnchorSet (boot) + хранение в AxiomEngine
   позиции якорей кэшировать из §2 (навечно, не двигаются)
   тест: test_rules_loaded_from_anchor_set
2. apply_subsystem_gravity (отдельная функция, НЕ в batch)
   тесты: pull / push / radius
3. Вызов на reconcile-интервале (свой, редкий)
4. Калибровка factor (умеренный §4; учесть двойной pull в зоне A5+A6 §2)
   + тест на коллапс
5. Bench: hot path не вырос, TickForward не просел
```

---

## 10. Что НЕ делать

```
- НЕ втаскивать в apply_gravity_batch (hot path сохранить)
- НЕ делать factor сильным (коллапс к якорю)
- НЕ двигать сами якоря этой гравитацией (они центры pull/push, STATE_LOCKED)
- НЕ применять каждый тик (reconcile/свой редкий интервал)
- НЕ использовать wall-clock (event_id только)
- НЕ мержить без bench (физика рядом с горячим путём)
```

---

## История

- **2026-06-03**: инструкция по PRIM-TD-03. Главное решение — subsystem-гравитация ИЗОЛИРОВАНА от hot-path apply_gravity_batch (отдельная функция на reconcile-интервале). Values: beneficial pull / harmful push. Abstractions: A5/A6 подтягивают снизу с radius. Акцент на умеренный factor (защита от коллапса, связь с открытым вопросом center-collapse) и обязательный bench перед мержем.
