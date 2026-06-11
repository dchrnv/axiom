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

**1514 тестов (all features), 0 failures.**
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
| 4 | Октанты O2/O4–O8 = 0 всегда | AxialEvaluator не заполняет октанты | ✅ (O3=Dionysus/Thanatos/Will 12896★, O7=4306★, O8=4310★ через мембраны) |

---

## Завершено (текущая сессия)

### Domain_Membrane_Profiles_V1_0 ✅ — мембранная трансформация входящих токенов

**Цель:** entropy≈0 и Dionysus недостижим → физика работает с уже «окрашенным» токеном.

Реализовано по спеке `docs/architecture/Domain_Membrane_Profiles_V1_0.md`:
- `MembraneProfile` в axiom-genome + секция в genome.yaml (8 доменов 101–108).
- `membrane_transform()` — чистая функция blend (mass/valence/temp) + clamp.
- `Arbiter::configure_membranes()` + применение в `route_to_ashti` (slow path, не fast path).
- `AshtiCore::apply_membrane_profiles()` → вызов из `AxiomEngine::try_new`.
- Пресеты откалиброваны: logic membrane_state→ADAPTIVE, void quantum_noise→150,
  shadow resonance_freq→400, logic resonance_freq→200.
- 9 unit-тестов, `test_from_yaml_matches_default` расширен.

**Ожидаемый эффект (§9 спеки):** entropy>0, октанты Dionysus/Thanatos/Nothing активируются,
active_dilemmas>0 при конфликтных текстах. Проверить следующим OBS-прогоном с BLEND_FACTOR=0.5.

---

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

## Активный план: Neural Integration — Этап 1

**Директива:** `docs/architecture/Neural_Integration_Stage1_Directive.md`  
**Спека:** `docs/architecture/Neural_Integration_V1_0.md`  
**Охранная цифра:** TickForward hot path 24.8 µs — не должен вырасти.

### Фаза 0 — axiom-neural: каркас инференса

```
Новый крейт: crates/axiom-neural/
Зависимости: rustfft (pure Rust), ndarray + matrixmultiply (pure Rust, NO BLAS)
Нет C-биндингов. Нет OpenBLAS. Нет fftw.

Что реализовать:
  - Model trait: load_from_bin(path) + infer(&[f32]) → Vec<f32>
  - Layer1DConv { weights, bias, kernel_size, stride }
  - GlobalAvgPool
  - Linear { weights, bias }
  - Static memory: все буферы предвыделены при load_from_bin()
  - FFT-frontend: fft_features(slice: &[f32], out: &mut [f32]) — rustfft
  - Z-score нормализация входа

Критерий: крейт компилируется, Model::new_zeros().infer() работает.
```

### Фаза 1 — ReactivationDepth пилот (архитектура)

```
Вход (t%11, Sensorium уровень 2):
  ActivityTrace rings: short[16] + mid[64] + long[256] по каждой из N подсистем
  → FFT над каждым кольцом → частотные признаки
  → конкатенация → нормализация (Z-score)

Модель (10–50K параметров):
  Conv1D(in=N_subsystems, out=16, kernel=3) → ReLU
  Conv1D(in=16, out=32, kernel=3)           → ReLU
  GlobalAvgPool → [32]
  Linear(32, 8)                              → AdvisorOutput.value[8]
  Linear(32, 1)                              → AdvisorOutput.confidence

Критерий: модель с нулевыми весами проходит через pipeline без паники/alloc.
```

### Фаза 2 — Дистилляция (teacher → student)

```
Источник данных: OBS-прогон с corpus_showcase → snapshot Sensorium каждые N тиков
  → сохраняем пары (sensorium_slice, teacher_output) в training_data.bin

Teacher = текущий ReactivationDepthAdvisor (rule-based, в коде)
Student = модель из фазы 1

Тренировка ОФФЛАЙН (Python + torch → ONNX → конвертация в .bin):
  or чистый Rust train loop в отдельном бинарнике axiom-neural-train

Критерий: student воспроизводит teacher ≥ 95% на holdout.
```

### Фаза 3 — Интеграция в NeuralAdvisor

```
NeuralAdvisorConfig (genome.yaml):
  reactivation_depth:
    mode: rule   # → rule | neural | distill
    trust: ignore

Интеграция:
  - t%11: если mode=neural → model.infer(sensorium_slice), timeout 1ms, иначе fallback
  - AdvisorOutput.confidence → CalibrationTable → calibrated_confidence
  - TrustConfig использует только calibrated_confidence

Критерий: bench TickForward не вырос.
         DivergenceLog пишет пары (advice, actual) при mode=neural.
```

### Фаза 4 — Workstation + промоция

```
Новый блок в Workstation (Internals или отдельный таб):
  - advisor: reactivation_depth | mode | accuracy | divergence_rate
  - кнопка Switch mode (rule/neural) — через genome update
  - калиброванный confidence vs raw

Промоция: Ignore → RequireConfirmation → AutoApply через genome (chrnv решает).
Критерий: chrnv видит accuracy, решает промотировать ли.
```

---

## Не в активном плане

- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **COMP-01** — Vital Signs окно (Companion). См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **Neural Integration Этап 2** — AudioPerceptor, Speech Commands, Vision. После успеха этапа 1.
- **Neural Integration Этап 3** — ультразвук, расширенный STT. После этапа 2.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
