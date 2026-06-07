# Axiom Roadmap

**Версия:** 77.0
**Дата:** 2026-06-05

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
axiom-corpus                                        ↑
                                               axiom-broadcasting
```

**1721 тестов, 0 failures.**
Shell-TD-02, SEN-TD-01 (V2.0), BRD-TD-06 завершены (2026-06-05).
PRIM-TD-03 Subsystem Gravity завершён (2026-06-04).
Sensorium V1.0, Waves V1.0, Cross-Modal Binding pipeline замкнуты (2026-06-03).

---

## Активные задачи

### Очередь диагностики (по OBS-сигналам)

Каждый шаг: исправить → OBS Quick → проверить метрику.
При сложной сопутствующей работе (п.2 Октанты/модули) → DEFERRED.

| # | Метрика OBS | Что сломано | Статус |
|---|---|---|---|
| 1 | ShellSim = 0.000 | avg_crystallized_shell_similarity в FrameWeaverStats (EMA α=0.3) | ✅ |
| 2 | 0% accuracy: abstractions, morality, writing | якоря не матчатся на текстах | ✅ |
| 3 | Tension traces = 0 | TensionTrace не создаётся после resolution | ✅ |
| 4 | Октанты O2/O4–O8 = 0 всегда | AxialEvaluator не заполняет октанты | **→ сейчас** |

---

## Завершено (текущая сессия)

### DIL-TD-01 ✅ — Dilemma Resolution Pipeline

**Цель:** дилеммы наконец разрешаются и попадают в EXPERIENCE.

**Диагноз:** инфраструктура полностью готова (`resolve()`, `drain_pending_crystallizations()`,
`crystallize_to_experience_commands()`), но в `ContextRecognizer.on_tick()` нет ни одного
вызова `resolve()` в production-коде. 8 дилемм накапливаются до лимита и застывают навсегда.

**Шаги:**

#### Шаг 1 — Resolution conditions в `ContextRecognizer.on_tick()` (Type III/IV)
`crates/axiom-runtime/src/over_domain/context_recognizer/mod.rs`

Добавить в конец on_tick() после detection, scan active dilemmas:

- **Type III (ValueConflict):** если `dominant_persistence > 0.8` И один из конфликтующих якорей
  относится к доминирующей подсистеме → `resolve(id, ContextualPriority { winner })`.
  Fallback: intensity decay (0.995/тик), при intensity < 0.1 → `ContextualPriority` по энергии.

- **Type IV (OntologicalConflict):** если дилемма активна > 500 тиков И entropy < 0.1 (стабильное
  состояние) → `resolve(id, Complementarity)`. Обе модели сосуществуют.

#### Шаг 2 — Crystallization drain в on_tick()
Вызывать после resolution scan:
```rust
let crystallization_cmds = self.dilemma_store
    .drain_pending_crystallizations()
    .into_iter()
    .flat_map(|r| crystallize_to_experience_commands(&r, position, exp_domain_id))
    .collect::<Vec<_>>();
cmds.extend(crystallization_cmds);
```

#### Шаг 3 — Type V (Axiogenic) в DreamCycle
`crates/axiom-runtime/src/engine.rs` — в `apply_dream_depth_update()`, рядом с
`drain_cross_modal_bond_commands()`:
- drain Type V диlemmas из store → кристаллизовать как Frame anchors в EXPERIENCE

#### Шаг 4 — Тесты
- `test_value_conflict_resolves_on_dominant_persistence`
- `test_ontological_resolves_on_complementarity`
- `test_crystallization_generates_ucl_commands`
- integration test: OBS-Quick показывает resolved > 0

**Результат:** OBS corpus_showcase: `Dilemmas resolved: 64` (MAX_RESOLVED), `active: 0`. ✅
Type V (Axiogenic) перенесён в DEFERRED (только DREAM Phase).
Калибровка compute_confidence: avg coherence 1.000→0.750, multi-pass events появились.

---

## Не в активном плане

- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **COMP-01** — Vital Signs окно (Companion). См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **V9** — Active NeuralAdvisor (нейронные модели). После накопленной истории.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
