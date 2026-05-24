# AXIOM — OverDomainArbiter V2.0

**Статус:** Спецификация  
**Версия:** 2.0  
**Дата:** 2026-05-24  
**Предыдущая версия:** `OverDomainArbiter_V1_0.md`  
**Категория:** Over-Domain Mechanism  
**Crate:** `axiom-runtime` / `over_domain/arbiter/`

---

## 1. Что меняется относительно V1

V1 заложил инфраструктуру: TrustConfig, PendingQueue, ArbiterLog, CognitiveProfile.
Но конфигурация жила только в коде, очередь не протухала, профиль не загружался.

**Три главных изменения V2:**

1. **TrustConfig из `genome.yaml` + автокалибровка `min_confidence`** (ARB-TD-01) —
   пороги задаются в конфиге; Arbiter корректирует их онлайн по истории подтверждений.

2. **TTL очереди** (ARB-TD-02) — `PendingAdvisory` истекает через N event_id;
   новый outcome `Expired` + `on_feedback(Expired)` источнику.

3. **`CognitiveProfile` из YAML** (PROFILE-01) — загрузка `octant_weights[8]` из
   `config/profiles/<name>.yaml`; hot-reload при смене файла.

Уже реализовано попутно в AE V3 / NA V2 (не входит в V2 Arbiter):
- `Advisory.octant_hint` + `CognitiveProfile` online learning (NA V2)
- `AxialStore::override_octant` / `AdvisoryAction::OverrideOctant` (AE V3)
- `unrouted_feedback` / `drain_octant_overrides` (AE V3 wiring)

---

## 2. TrustConfig из конфига (ARB-TD-01)

### Текущее состояние

`TrustConfig::default_v1()` — захардкожена в `trust.rs`. Изменить пороги без
перекомпиляции невозможно.

### V2: секция в genome.yaml

```yaml
# config/genome.yaml
arbiter:
  trust:
    - source: 0          # NeuralAdvisor
      type: DepthHint
      mode: AutoApply
      min_confidence: 0.75
    - source: 0
      type: OctantCorrection
      mode: RequireConfirmation
      min_confidence: 0.60
    - source: 1          # AxialEvaluator
      type: OctantCorrection
      mode: RequireConfirmation
      min_confidence: 0.70
    - source: 1
      type: NarrativeShift
      mode: RequireConfirmation
      min_confidence: 0.55
    # ... остальные записи
```

`AxialEvaluator::on_boot` читает секцию, строит `TrustConfig`.
Если секция отсутствует → fallback на `TrustConfig::default_v1()`.

### V2: автокалибровка min_confidence

После каждого `Confirmed` или `Rejected` Arbiter обновляет `min_confidence` для пары
`(source, type)` на основе скользящего качества из `ArbiterLog`:

```
quality(source, type) = confirmed / (confirmed + rejected)  [последние CALIBRATION_WINDOW записей]

если quality > CALIBRATION_HIGH (0.80) и min_confidence > CONFIDENCE_FLOOR (0.50):
    min_confidence -= CALIBRATION_STEP (0.02)   // советник хорош → снизить барьер

если quality < CALIBRATION_LOW (0.40) и min_confidence < CONFIDENCE_CEIL (0.95):
    min_confidence += CALIBRATION_STEP           // советник ошибается → поднять барьер
```

| Константа | Значение |
|-----------|----------|
| CALIBRATION_WINDOW | 20 |
| CALIBRATION_HIGH | 0.80 |
| CALIBRATION_LOW | 0.40 |
| CALIBRATION_STEP | 0.02 |
| CONFIDENCE_FLOOR | 0.50 |
| CONFIDENCE_CEIL | 0.95 |

Калибровка срабатывает в `confirm_pending` / `reject_pending` после записи в лог.
`ArbiterLog` получает метод `quality_window(source, type, window) -> Option<f32>`.

---

## 3. TTL очереди (ARB-TD-02)

### Текущее состояние

`PendingAdvisory` копится бесконечно. После долгого бездействия chrnv очередь
превращается в устаревший мусор.

### V2: expires_at_event

```rust
pub struct PendingAdvisory {
    pub advisory: Advisory,
    pub queued_at_event: u64,
    pub expires_at_event: u64,   // V2: queued_at_event + PENDING_TTL
}

pub const PENDING_TTL: u64 = 1000;
```

В начале каждого `tick_with_stores` перед обработкой новых advisories:

```
let now = event_id;
let expired: Vec<_> = pending
    .iter()
    .filter(|p| now >= p.expires_at_event)
    .collect();

for p in expired:
    log(Expired)
    feedback_source(p.advisory.source, p.advisory.id, AdvisoryOutcome::Expired)
    pending.remove(p)
```

Новый `ArbiterOutcome::Expired` и `AdvisoryOutcome::Expired` в `source.rs`.

Expired advisory не влияет на CognitiveProfile — только на лог и источник.
Источник получает `Expired` как сигнал что рекомендация устарела.

---

## 4. CognitiveProfile из YAML (PROFILE-01)

### Текущее состояние

`CognitiveProfile` реализован и работает (online learning в Arbiter).
`octant_weights[8]` инициализируются `[1.0; 8]` и корректируются онлайн.
Загрузки начального профиля из файла нет.

### V2: config/profiles/

```yaml
# config/profiles/balanced.yaml
name: balanced
octant_weights: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
```

```yaml
# config/profiles/analytic.yaml
name: analytic
octant_weights:
  # [CreativeAffirmation, EcstaticAffirmation, HeroicFatal, DestructiveActivating,
  #  IdealizedConsoling,  PassiveSentimental,  FormalDenying, SelfDestructiveApathic]
  - 1.2   # CreativeAffirmation  — синтетическая аффирмация
  - 0.8   # EcstaticAffirmation  — аффективно перегруженный
  - 1.4   # HeroicFatal          — высокое усилие при ясной структуре
  - 0.7   # DestructiveActivating— разрушение без цели
  - 0.9   # IdealizedConsoling   — идеализация
  - 0.6   # PassiveSentimental   — пассивность
  - 1.5   # FormalDenying        — чёткая формальная структура
  - 0.5   # SelfDestructiveApathic— апатия
```

`CognitiveProfile::from_yaml(path: &Path) -> Result<Self>` — новый конструктор,
читает файл, клампирует `[WEIGHT_MIN, WEIGHT_MAX]`.

`AxialEvaluator::on_boot` читает `config/genome.yaml → arbiter.profile` (имя файла):

```yaml
arbiter:
  profile: analytic   # config/profiles/analytic.yaml
```

Если не задано → `balanced` (все 1.0, текущее поведение).

Online learning продолжает работать поверх загруженного профиля — файл задаёт
начальные веса, опыт их корректирует.

---

## 5. ArbiterLog расширение

Для автокалибровки нужен метод скользящего качества:

```rust
impl ArbiterLog {
    /// Доля Confirmed среди (Confirmed + Rejected) за последние `window` записей
    /// для пары (source, advisory_type). None если нет данных.
    pub fn quality_window(
        &self,
        source: SourceId,
        advisory_type: AdvisoryType,
        window: usize,
    ) -> Option<f32> {
        let relevant: Vec<_> = self.entries.iter()
            .filter(|e| e.source == source && e.advisory_type == advisory_type)
            .filter(|e| matches!(e.outcome, ArbiterOutcome::Confirmed | ArbiterOutcome::Rejected))
            .rev()
            .take(window)
            .collect();
        if relevant.is_empty() {
            return None;
        }
        let confirmed = relevant.iter()
            .filter(|e| e.outcome == ArbiterOutcome::Confirmed)
            .count();
        Some(confirmed as f32 / relevant.len() as f32)
    }
}
```

---

## 6. Что в коде

Изменения относительно V1:

```
over_domain/arbiter/
├── mod.rs       — TTL sweep в tick_with_stores, автокалибровка в confirm/reject_pending,
│                  CognitiveProfile::from_yaml в on_boot
├── trust.rs     — TrustConfigLoader::from_genome_yaml(path), CALIBRATION_* константы,
│                  calibrate(source, type, accepted, log)
├── log.rs       — + quality_window(source, type, window) -> Option<f32>
│                  + ArbiterOutcome::Expired
├── source.rs    — + AdvisoryOutcome::Expired
└── profile.rs   — + CognitiveProfile::from_yaml(path) -> Result<Self>

config/
├── genome.yaml  — + секция arbiter.trust + arbiter.profile
└── profiles/
    ├── balanced.yaml
    └── analytic.yaml
```

---

## 7. Известные ограничения V2

- **ARB-TD-04** — `TrustConfig` не hot-reload. Изменение genome.yaml требует рестарта.
  V3: watch-поток + apply на лету.

- **ARB-TD-05** — автокалибровка не персистируется. После рестарта `min_confidence`
  сбрасывается к значениям из genome.yaml. V3: сохранять в `axiom-persist`.

- **ARB-TD-06** — `CognitiveProfile` (online learning веса) не персистируется.
  V3: сохранять в `axiom-persist` аналогично ARB-TD-05.

---

## 8. Что в V3+

- **V3:** TrustConfig hot-reload; автокалибровка и CognitiveProfile через axiom-persist.
- **V3:** Workstation — панель истории решений Arbiter с визуализацией quality per source.
- **V6+:** второй AdvisorySource (PatternAdvisor из DreamPhase); агрегация от
  нескольких источников с весами.
- **V9:** Arbiter — обучаемый; учится когда доверять каким источникам на основе
  накопленной истории.

---

## История

- **V1.0** (2026-05-19): Advisory-only инфраструктура. TrustConfig, PendingQueue,
  ArbiterLog, CognitiveProfile (online learning). ARB-TD-01/02/03 зафиксированы.
- **V1.x** (2026-05-23): CognitiveProfile wired в confirm/reject; Advisory.octant_hint;
  OverrideOctant action; unrouted_feedback / pending_overrides; NarrativeShift support.
  ARB-TD-03 закрыт через AE V3.
- **V2.0** (2026-05-24): TrustConfig из genome.yaml + автокалибровка (ARB-TD-01);
  TTL очереди с ArbiterOutcome::Expired (ARB-TD-02); CognitiveProfile::from_yaml
  с config/profiles/ (PROFILE-01).
