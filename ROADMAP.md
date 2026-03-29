# Axiom Roadmap

**Версия:** 16.0
**Дата:** 2026-03-29

---

Все 12 этапов завершены. **731 тест, 0 failures.**

Текущее состояние: [STATUS.md](STATUS.md)
Технический долг и будущие планы: [DEFERRED.md](DEFERRED.md)

---

## Этап 13 — Когнитивная глубина V1.0 (Internal Drive)

**Цель:** Перевести систему от реактивного поведения к когнитивному.
**Дизайн:** [docs/deferred/Cognitive_Depth_V1_0.md](docs/deferred/Cognitive_Depth_V1_0.md)
**Крейты:** `axiom-arbiter` (основной), `axiom-heartbeat`, `axiom-config`.
**Новых крейтов нет. Новых доменов нет.**

### 13A — Multi-pass (MAYA + Arbiter)

MAYA возвращает confidence вместе с токеном. Arbiter повторяет проход если confidence < порога.

**Файлы:**
- `axiom-arbiter/src/maya_processor.rs` — добавить `consolidate_with_confidence(results, domain) -> (Token, f32)`, реиспользовав существующую `compute_confidence()`
- `axiom-arbiter/src/lib.rs` — добавить `route_from_experience_multipass()` с параметрами `max_passes: u8`, `min_coherence: u8`
- `axiom-config` — добавить `max_passes: u8`, `min_coherence: u8` в Arbiter-секцию конфига

**Поведение:**
```
route_from_experience_multipass(pattern, pass=0)
  → ASHTI(1-8) → MAYA.consolidate_with_confidence()
  → if confidence < min_coherence && pass < max_passes:
      enrich pattern (temperature += result.temperature / 2)
      → retry with pass+1
  → else: return final result
```

**Тесты (~10):** pass=1 при высоком coherence, retry при низком, max_passes соблюдается, итоговое качество растёт.

---

### 13B — TensionTrace + Heartbeat Internal Drive

При низком coherence в EXPERIENCE(9) создаётся tension след с горячим токеном. Heartbeat его находит и генерирует внутренний импульс.

**Файлы:**
- `axiom-arbiter/src/experience.rs` — `add_tension_trace(pattern, coherence_score)` и `drain_pending_impulses() -> Vec<Token>`; tension = ExperienceTrace с `type_flags |= TENSION_FLAG`, высоким temperature, отрицательным valence
- `axiom-heartbeat/src/lib.rs` — добавить `enable_internal_drive: bool` в `HeartbeatConfig`; при пульсе вызывать `drain_pending_impulses()`

**Тесты (~12):** tension создаётся после низкого coherence, горячие следы drain'ятся, холодные остаются, интеграция с Heartbeat.

---

### 13C — InternalImpulse + Internal Dominance (Arbiter)

Arbiter получает два потока — внешние сигналы и внутренние импульсы. `internal_dominance_factor` управляет балансом.

**Файлы:**
- `axiom-arbiter/src/lib.rs` — добавить `ImpulseSource`, `InternalImpulse`, `select_next(external, internal, factor) -> Pattern`
- `axiom-config` — добавить `internal_dominance_factor: u8` (0..255 → 0.0..1.0) в Arbiter-секцию

```rust
pub enum ImpulseSource { External, Tension, Incompletion, Curiosity, Goal }
pub struct InternalImpulse { pub source: ImpulseSource, pub weight: f32, pub pattern: Token }
```

**Тесты (~12):** factor=0 → только external, factor=255 → internal побеждает, None/None → idle, граничные случаи.

---

### 13D — Goal Persistence + Curiosity

CODEX генерирует impulse если цель не достигнута. DREAM генерирует impulse для следов near crystallization threshold.

**Файлы:**
- `axiom-arbiter/src/ashti_processor.rs` — расширить CODEX(3) physics: `type_flags & GOAL_FLAG` + weight > goal_threshold → пометить goal_pending
- `axiom-arbiter/src/experience.rs` — `check_curiosity_candidates(skill_threshold) -> Vec<Token>`: следы с weight ∈ [0.8·threshold, threshold)
- `axiom-arbiter/src/lib.rs` — `generate_goal_impulses()` и `generate_curiosity_impulses()`, вызываются из Heartbeat

**Тесты (~12):** цель без достижения → impulse, достигнутая цель → нет impulse, curiosity при near-threshold кластере.

---

### Итого Этап 13

| Подэтап | Что | Ожид. тестов |
|---------|-----|-------------|
| 13A | Multi-pass MAYA | ~10 |
| 13B | TensionTrace + Heartbeat | ~12 |
| 13C | InternalImpulse + Dominance | ~12 |
| 13D | Goal + Curiosity | ~12 |
| **Итого** | | **~46 новых (~777 total)** |

**Порядок:** 13A → 13B → 13C → 13D.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
