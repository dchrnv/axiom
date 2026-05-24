# AXIOM — Roadmap

**Создан:** 2026-05-24  
**Обновлён:** 2026-05-24  
**Опирается на:** STATUS.md, BLUEPRINT.md, DEFERRED.md, архитектурные спеки  

---

## Ориентиры состояния (2026-05-24)

```
AxialEvaluator    V3.0 ✅  on_feedback calibration, OverrideOctant, NarrativeTracker
OverDomainArbiter V2.0 ✅  TrustConfig из yaml, TTL 1000, CognitiveProfile из yaml
NeuralAdvisor     V2.0 ✅  все 5 слотов, AdvisoryHistory ring-32, CognitiveProfile
ContextRecognizer V6.0 ✅  SyntacticBridge, ActivityTrace×3, SubsystemFatigue
FrameWeaver       V1.3 ✅
DREAM Phase       V1.0 ✅
1487 тестов, 0 failures
```

---

## Workstation V2: React SPA + Grafana

Спецификация: `docs/architecture/Workstation_V2_Plan.md`

| Фаза | Содержание | Объём |
|------|-----------|-------|
| WS-0 | Foundation: React scaffold + axum static + WS-клиент | 1 сессия |
| WS-1 | Advisory Queue: confirm/reject UI + REST endpoints | 1 сессия |
| WS-2 | Core Tabs: Conversation, Phase C, Patterns | 2 сессии |
| WS-3 | Grafana: /metrics endpoint + дашборды | 1 сессия |
| WS-4 | Tauri wrapper (нативный desktop) | 0.5 сессии, позже |

**iced Workstation:** замораживается после WS-2.2, удаляется после WS-2.3.  
**Гаджет:** PWA из коробки для устройств с браузером; отдельный минимальный клиент
для bare-metal экранов — пишется под конкретное железо, без изменений в бэкенде.

---

## Phase D — Замыкание документации и advisory-петли

**Цель:** привести документацию в соответствие с реализацией;
закрыть петлю обратной связи advisory-системы.

### D1 — Документация (1 сессия)

- [ ] STATUS.md: AxialEvaluator V2.0 → V3.0, Arbiter V1.0 → V2.0, тесты 1452 → 1487
- [ ] BLUEPRINT.md: обновить описание AE V3 + Arbiter V2
- [ ] DEFERRED.md: закрыть ARB-TD-01/02/03, PROFILE-01 как ✅
- [ ] Обновить ссылки в DEFERRED.md на AE-TD-01/02/03 (реализованы в AE V3)

### D2 — Advisory confirm/reject

Закрывается через WS-1 (React SPA) — REST endpoints + Advisory Queue UI.  
CLI-команды `:advisory confirm/reject` добавляются параллельно как fallback.

**Зависимости:** нет.  
**Ценность:** без этого advisory-система — read-only; AE V3, NA V2, Arbiter V2
              работают без реальной обратной связи от пользователя.

---

## Phase E — OBS-03: наблюдение и калибровка

**Цель:** собрать данные неоднородного корпуса для калибровки порогов.

### E1 — Подготовка корпуса

OBS-02 дал 312/312 emergent-кандидатов — все Frame прошли порог потому что все
тексты повторялись равномерно (415 инъекций = каждый текст ~50 раз).

Нужен **неоднородный корпус**:
- 10–15% текстов: 2–3 инъекции (не должны быть кандидатами)
- 50% текстов: 10–30 инъекций (средний уровень)
- 35% текстов: 100+ инъекций (потенциальные кандидаты)

### E2 — Прогон и анализ

- 50k+ тиков (больше чем OBS-02 для накопления истории)
- Логировать: октант по AE vs октант из NA advisory → divergence rate
- Логировать: качество калибровки Arbiter (quality_window per source/type)
- Результат: обновлённые пороги `DepthThresholdEmergentDetector`

**Зависимости:** D2 (нужны confirm/reject для ручной валидации emergent-кандидатов).  
**Выход:** откалиброванная система готова к NeuralAdvisor V3.

---

## Phase F — Arbiter V3: персистентность

**Цель:** закрыть ARB-TD-05 и ARB-TD-06.

### F1 — Персистентность автокалибровки (ARB-TD-05)

После рестарта `TrustConfig.min_confidence` сбрасывается к значениям из genome.yaml.
Накопленная калибровка теряется.

```rust
// axiom-persist: новый тип сохранения
ArbiterCalibrationState {
    entries: HashMap<(SourceId, AdvisoryType), f32>,  // (source, type) → min_confidence
}
```

`on_boot` → пробует загрузить из persist; поверх накладывает genome.yaml как floor.
`on_shutdown` → сохраняет текущие min_confidence.

### F2 — Персистентность CognitiveProfile (ARB-TD-06)

`octant_weights[8]` сбрасываются к начальному профилю (balanced/analytic) при рестарте.

```rust
ArbiterProfileState { octant_weights: [f32; 8] }
```

`on_boot` → загружает если есть; иначе from_yaml.  
`on_shutdown` → сохраняет.

### F3 — TrustConfig hot-reload (ARB-TD-04) — опционально

`genome.yaml` меняется → Arbiter подхватывает на лету без рестарта.
Через `ConfigWatcher` (уже в codebase). Низкий приоритет, приятная мелочь.

**Зависимости:** axiom-persist (уже есть).  
**Объём:** небольшой, 1 сессия.

---

## Phase G — NeuralAdvisor V3

**Цель:** диагностика расхождений + обучаемый ConflictResolver.

### G1 — Divergence logging

Когда `advisory_octant` расходится с `analytic_octant` на 2+ оси (Hamming distance ≥ 2)
— записывать в отдельный `DivergenceLog`:

```
DivergenceEntry { event_id, sutra_id, analytic_octant, advisory_octant,
                  distance: usize, advisor_confidence: f32 }
```

Доступен через `neural_advisor.divergence_log()`.  
В PhaseCSnapshot или отдельный endpoint для наблюдения.

Это главный инструмент калибровки из NA V2 §3 ("Workstation должна показывать
расхождение").

### G2 — PatternLearningResolver (conflict slot → V3)

Заменяет `RuleBasedCorpusCallosumResolver`. Учится на `AdvisoryHistory` per-sutra_id:

```
Вход: history_for_sutra (Vec<AdvisoryHistoryEntry>), текущий конфликт
Если history содержит ≥ MIN_SAMPLE (5) Confirmed/Rejected для этого Frame:
    pattern = dominant_diagnosis(history)
    confidence = acceptance_rate * len_factor
    return Some(pattern)
Иначе:
    fallback на RuleBasedCorpusCallosumResolver
```

**Требует OBS-03** — нужна реальная история advisories для обучения.

### G3 — Genome-per-advisor control (опционально)

```yaml
# genome.yaml
neural_advisor:
  emergent:
    enabled: false   # отключить в production конфиге
  conflict:
    enabled: true
```

`NeuralAdvisorRegistry::from_genome_config(genome)` — конфигурирует слоты из генома.  
Текущий `with_default_v2()` остаётся как fallback.

**Зависимости:** G1 требует никаких; G2 требует E2 (накопленные данные).  
**Объём:** G1 — 1 сессия; G2 — 1–2 сессии.

---

## Phase H — ContextRecognizer V7: генеративные подсистемы

**Цель:** система предлагает новые подсистемы на основе паттернов emergent primitives.

### H1 — SubsystemCandidate в DREAM Phase

```rust
pub struct SubsystemCandidate {
    pub emergent_primitives: Vec<u32>,   // sutra_ids
    pub centroid_position: [i16; 3],
    pub primary_octants: Vec<Octant>,
    pub evidence_strength: f32,
}
```

В `DreamPhase::dreaming_tick()` — этап `SubsystemDiscovery`:  
Кластеризация emergent primitives по co-activation → `SubsystemCandidate` →
emit `NotifySubsystemCandidate` UCL.

### H2 — SubsystemLifecycle

```
proposed → candidate → in_review → active → mature → deprecated → archived
```

YAML-файл для нового кандидата генерируется автоматически как черновик.
chrnv одобряет/редактирует через `:subsystem approve <id>`.

### H3 — Зависимости из H2 что переходят из V6

- **TransitionGraph** (граф переходов между подсистемами) — нужен для корректного
  Cascading в ActivitySignature. Строится поверх ActivityTrace.
- **FatigueStore** уже в axiom-experience ✅ (CR V6 Фаза B).

**Зависимости:** E2 + G2 (нужна реальная история для кластеризации).  
**Объём:** большой, 3–5 сессий. Самый крупный шаг перед V8.

---

## Backlog (V8–V9)

### V8 — Axiogenesis through Dilemmas

Аксиогенетические конфликты → новые якоря в подсистеме Values. Требует:
- Месяцев накопленной истории конфликтов
- WisdomStore в axiom-experience
- DeepConflictAnalysis в DREAM Phase

**Никогда не автоматический.** Требует явного одобрения.  
**Когда:** после H + реальной работы системы 6+ месяцев.

### V9 — Active NeuralAdvisor (нейронные модели)

Все 5 слотов → обученные модели (~1M параметров суммарно).  
Тренировка в DREAM Phase. ConfidenceCalibrator. Distillation от rule-based.

**Bootstrap problem:** нужна история от V3–V7.  
**Когда:** после H + накопленной истории.

---

## Зависимости (граф)

```
D1 (docs)
D2 (advisory CLI)
  └── E1+E2 (OBS-03)
        └── F1+F2 (Arbiter V3 persist)
              └── G1 (divergence log)     ← можно раньше, без OBS
                    └── G2 (PatternLearning)  ← требует OBS данных
                          └── H1+H2 (CR V7)
                                └── V8, V9
```

`F3` (hot-reload) и `G3` (genome-per-advisor) — независимы, делаются когда есть время.

---

## Открытые вопросы

| Вопрос | Когда решать |
|--------|--------------|
| Workstation: iced vs egui vs web vs CLI-only (см. отдельное обсуждение) | до D2 |
| NARRATIVE_WINDOW_SIZE=8 правильное? (AE-TD-06) | после OBS-03 |
| Пороги DepthThresholdEmergentDetector | после OBS-03 |
| axiom-agent: TextPerceptor Path B (embeddings) | после G2 |

---

## История

- **2026-05-24**: создан по итогам AE V3 + Arbiter V2.
