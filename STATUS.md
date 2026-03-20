# Axiom Status

**Версия:** v0.6.0
**Дата:** 2026-03-20

---

## ✅ v0.6.0 - Causal Time System (ЗАВЕРШЕНО)

**Выполнено:**
- Event-Driven Core: 12 семантических типов событий
- EventGenerator: decay, collision, stress, gravity checks
- CausalFrontier: O(active_entities), storm mitigation, FIFO
- Heartbeat V2.0: детерминистичная генерация по event count
- Domain::process_frontier(): полная интеграция компонентов
- Time Model V1.0 compliance: event_id вместо timestamps
- Cross-spec validation: 8 тестов
- **Factory Methods**: все 11 доменов (EXECUTION, SHADOW, MAP, PROBE, VOID)
- **Struct Layout**: UclCommand 64 bytes, UclResult 32 bytes (оптимизация padding)
- **Test Fixes**: 6 тестов исправлено → 100% success rate

**Архитектура:**
```
External Events → COM (event_id)
    ↓
Heartbeat (by event count)
    ↓
Causal Frontier (active only)
    ↓
EventGenerator (state checks)
    ↓
Generated Events → COM
```

**Тесты:** 173 pass, 0 fail ✅ (было: 167 pass, 6 fail)
**Новых модулей:** 4 (event_generator, causal_frontier, heartbeat, domain runtime)
**Спецификации:** Time Model V1.0, COM V1.0, Event-Driven V1, Causal Frontier V1, Heartbeat V2.0

**Коммиты:** 02282d1, e38e17b, ff9e5bf, 514d891, 745df1c, 8688439

---

## ✅ v0.5.0 Phase 3 - Dual-Path Architecture (2026-03-19)

**Выполнено:**
- Arbiter V1.0 (430 строк, 10 tests)
- ASHTI Processor (360 строк, 13 tests)
- MAYA Processor (270 строк, 12 tests)
- PhysicsProcessor: dual-path processing
- UCL opcodes 4000/4001

**Архитектура:** SUTRA(0) → EXPERIENCE(9) → Arbiter → ASHTI(1-8) / MAYA(10)

**Тесты:** 88 pass (было 52)

---

## 📊 Модули

| Модуль | Spec | Runtime | Status |
|--------|------|---------|--------|
| Token | V5.2 | V5.2 | ✅ Complete |
| Connection | V5.0 | V5.0 | ✅ Complete |
| COM | V1.0 | V1.0 | ✅ Complete |
| Domain | V2.1 | V2.1 | ✅ Complete |
| EventGenerator | V1 | V1 | ✅ Complete |
| CausalFrontier | V1 | V1 | ✅ Complete |
| Heartbeat | V2.0 | V2.0 | ✅ Complete |
| Experience | V1 | V1 | ✅ Complete |
| Arbiter | V1.0 | V1.0 | ✅ Complete |

---

## 🎯 Релизы

### v0.6.0 - Causal Time System ✅ (2026-03-20)
- Event-Driven архитектура
- Causal Frontier System
- Heartbeat V2.0
- Time Model V1.0 compliance

### v0.5.0 Phase 3 - Dual-Path ✅ (2026-03-19)
- Arbiter, ASHTI, MAYA
- 35 новых тестов

### v0.5.0 Phase 2 - EXPERIENCE ✅ (2026-03-19)
- Резонансный поиск, обучение

### v0.4.0 Phase 1 - COM ✅ (2026-03-19)
- Causal Order Model

### v0.3.1 - UCL V2.0 ✅ (2026-03-09)
- PhysicsProcessor, FFI

---

**Последнее обновление:** 2026-03-20
