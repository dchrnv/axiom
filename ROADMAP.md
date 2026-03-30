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

### ✅ 13A — Multi-pass (MAYA + Arbiter) — завершено

- `MayaProcessor::consolidate_with_confidence() -> (Token, f32)`
- `DomainConfig`: `max_passes: u8`, `min_coherence: u8` (вместо `reserved_arbiter`)
- `Arbiter::route_with_multipass()` — повторный проход при низком confidence
- `RoutingResult`: поля `confidence` и `passes`
- `TensionTrace` создаётся при итоговом confidence < порога
- **14 тестов**

---

### ✅ 13B — TensionTrace + Heartbeat Internal Drive — завершено

- `HeartbeatConfig::enable_internal_drive` (false для weak/disabled, true для medium/powerful)
- `Arbiter::on_heartbeat_pulse(pulse, enable_internal_drive) -> Vec<Token>`
  — остужает следы (decay=10/пульс), сливает горячие (threshold=128) в импульсы
- **12 тестов**

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

### ✅ 13C — InternalImpulse + Internal Dominance — завершено

- `ImpulseSource`: External, Tension, Incompletion, Curiosity, Goal
- `InternalImpulse`: source, weight, pattern
- `Arbiter::select_next(external, internal, dominance_factor)` — 0=реактивная, 128=равновесие, 255=задумчивая
- `DomainConfig::internal_dominance_factor: u8` (reserved_meta → [2])
- **14 тестов**

---

### ✅ 13D — Goal Persistence + Curiosity — завершено

- `TOKEN_FLAG_GOAL: u16 = 0x0001` — флаг цели в type_flags
- CODEX(3) physics: goal-токен получает +20 mass, +15 temperature
- `Experience::check_goal_traces(threshold)` — незавершённые цели
- `Experience::check_curiosity_candidates(threshold)` — следы в зоне [0.8·t, t)
- `Arbiter::generate_goal_impulses(pulse, interval)` — Goal-импульсы каждые N пульсов
- `Arbiter::generate_curiosity_impulses(threshold)` — Curiosity-импульсы
- **14 тестов**

---

### ✅ Этап 13 — Когнитивная глубина V1.0 — ЗАВЕРШЁН

| Подэтап | Что | Тестов |
|---------|-----|--------|
| 13A | Multi-pass MAYA + TensionTrace | 14 ✅ |
| 13B | Heartbeat Internal Drive | 12 ✅ |
| 13C | InternalImpulse + Dominance | 14 ✅ |
| 13D | Goal Persistence + Curiosity | 14 ✅ |
| **Итого** | | **54 новых, 785 total** |

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
