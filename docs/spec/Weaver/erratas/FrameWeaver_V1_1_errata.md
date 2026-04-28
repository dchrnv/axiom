# FrameWeaver V1.1 — Errata

**Дата:** 2026-04-26  
**Назначение:** фиксация неточностей и пробелов спецификации V1.1, обнаруженных в процессе реализации. Все правки войдут в V1.2 после стабилизации.

---

## E1. Восстановление Frame из анкера не специфицировано

В V1.1 описано, как Frame создаётся (анкер + связи к участникам), но не описано, как восстановить список участников из существующего анкера. Это блокирует:
- **UnfoldFrame** — handler не может развернуть Frame без знания участников
- **Promotion** — `build_promotion_commands` не знает, какие связи копировать в SUTRA

**Текущее состояние:** реализация обходит проблему через `dummy_candidate.participants = []` — SUTRA-анкер создаётся без BondTokens к участникам (FW-TD-06).

**Решение:** функция `restore_frame_from_anchor` — чтение графа связей EXPERIENCE-анкера. Реализуется в этапе 2 стабилизации.

---

## E2. Trait `Weaver::check_promotion` и `scan` без `tick: u64`

V1.1 не уточнил сигнатуры трейта. По факту реализованы без передачи текущего тика:

```rust
// Было:
fn scan(&mut self, maya_state: &DomainState) -> Vec<Self::Pattern>;
fn check_promotion(&self, experience_state: &DomainState, anchors: &[&Token]) -> Vec<PromotionProposal>;
```

```rust
// Должно быть:
fn scan(&mut self, tick: u64, maya_state: &DomainState) -> Vec<Self::Pattern>;
fn check_promotion(&self, tick: u64, experience_state: &DomainState, anchors: &[&Token]) -> Vec<PromotionProposal>;
```

Без `tick` невозможно проверять `min_age_events` из `PromotionRule` и записывать честный `detected_at_tick` в `FrameCandidate`. Сейчас используется `tick_proxy = 0`.

**Решение:** breaking change в trait, реализуется в этапе 1 стабилизации.

---

## E3. Hot path regression после интеграции FrameWeaver

После добавления FrameWeaver в pipeline hot path тик вырос с ~96.5 ns до ~310 ns (~3x).

**Результаты A/B/C/D бенчмарка** (50 токенов в LOGIC, `cargo bench --bench frameweaver_overhead`):

| Группа | Конфигурация | ns/тик | Δ от A |
|--------|-------------|--------|--------|
| A | FW disabled (scan_interval=u32::MAX) | ~280 ns | baseline |
| B | FW active, scan=1, MAYA пуста | ~451 ns | +171 ns |
| C | FW active, scan=1, MAYA 5 узоров | ~1,454 ns | +1174 ns |
| D | FW active, scan=1, MAYA 20 узоров | ~4,923 ns | +4643 ns |
| Reg | FW default (interval=20) ДО оптимизации | 311 ns | — |
| Reg | FW default (interval=20) ПОСЛЕ оптимизации | 238 ns | — |

**Диагноз:** FrameWeaver при default `scan_interval=20` добавлял только ~14 ns/тик амортизированно.
Основная регрессия (97→280 ns) — от других периодических задач engine (`tension_check_interval=10`,
`goal_check_interval=10`), добавленных параллельно с FrameWeaver.

**Реализованная оптимизация:** `drain_commands()` перенесён внутрь блока `if fw_interval > 0 && t % fw_interval == 0`.
До: вызывался каждый тик (311 ns). После: вызывается только когда on_tick реально отработал (238 ns, -24%).

**Итоговые характеристики** (50 токенов, default config):
- Hot path с FW: **~238 ns/тик**
- Вклад FW при scan_interval=20: **~7-14 ns/тик** (амортизированно, нормально)
- Изолированный scan_state при пустой MAYA: **~26 ns**
- Изолированный scan_state при 5 узорах: **~1,661 ns** → амортизируется через scan_interval

**Примечание:** цель ≤130 ns/тик требует оптимизации других периодических задач engine
(tension/goal checks), что выходит за рамки V1.1 стабилизации. Вынесено в deferred.

---

## E4. `propose_to_dream` — заглушка (DREAM-фаза не существует)

V1.1 предполагало наличие DREAM-фазы как этапа обработки. По факту такой фазы не существует — есть только домен DREAM (107). FrameWeaver сейчас обходит DREAM и кристаллизует напрямую в `on_tick`.

**Статус:** известный долг, не баг. Метод `propose_to_dream` возвращает `CrystallizationProposal` с пустыми `commands`, Engine его не вызывает.

**Решение:** откладывается до отдельного проектирования DREAM-фазы.

---

## Resolution Summary

*(заполняется по мере закрытия этапов стабилизации)*

| Errata | Статус | Этап |
|--------|--------|------|
| E1 | ✅ закрыто | Этап 2 — `restore_frame_from_anchor` реализована, UnfoldFrame handler активен, промоция использует восстановленных участников |
| E2 | ✅ закрыто | Этап 1 — trait Weaver обновлён, tick передаётся в `scan` и `check_promotion` |
| E3 | ✅ закрыто | Этап 3 — A/B/C/D бенчмарк запущен; `drain_commands` перенесён внутрь interval-guard (311→238 ns, -24%); вклад FW = ~7-14 ns амортизированно. Цель ≤130 ns требует оптимизации других engine tasks — deferred. |
| E4 | ⏸ deferred | — |

---

## Stage 4 Validation — финальный статус

**Дата:** 2026-04-26  
**Результат:** ✅ стабилизация V1.1 завершена

- Полный тест workspace: **1030 тестов, 0 failed**
- End-to-end smoke test `frameweaver_end_to_end_smoke`: ✅ проходит
  - MAYA синтаксический узор (head=10, targets=20/30) → 25 тиков → Frame в EXPERIENCE
  - UnfoldFrame в LOGIC: анкер + связи скопированы
  - FrameWeaverStats: scans=25, candidates≥1, crystallizations≥1, frames_in_experience≥1, unfold_requests=1
- Бенчмарки: `frameweaver_overhead`, `hot_path_regression` — зарегистрированы в `axiom-bench`
