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
| 13A | Multi-pass MAYA | 14 ✅ |
| 13B | TensionTrace + Heartbeat | 12 ✅ |
| 13C | InternalImpulse + Dominance | ~12 |
| 13D | Goal + Curiosity | ~12 |
| **Итого** | | **26 готово, ~24 осталось (~783 total)** |

**Порядок:** 13A → 13B → 13C → 13D.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
