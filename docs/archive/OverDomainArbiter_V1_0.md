# OverDomainArbiter V1.0

**Статус:** проектирование (V1)  
**Дата:** 2026-05-19  
**Автор:** Chernov Denys

---

## 1. Назначение

OverDomainArbiter — седьмой над-доменный модуль. Он читает рекомендации от
*advisory*-источников (сейчас один: NeuralAdvisor) и принимает решение — применить
автономно, поставить в очередь на подтверждение chrnv, или проигнорировать.

Без Arbiter NeuralAdvisor работает в режиме Advisory-Only навсегда: советник видит
расхождения, обнаруживает паттерны, предлагает коррекции — но никто не слушает.
Arbiter — это слушатель.

**Чем НЕ является:**
- Не заменяет детерминированные компоненты (AxialEvaluator, ContextRecognizer)
- Не управляет доменами напрямую — действует только через UCL
- Не принимает решений без конфигурации доверия — всё управляется через `TrustConfig`

---

## 2. Позиция в архитектуре

```
AxialEvaluator ──────────────────────────────────────────┐
ContextRecognizer ──────────────────────────────────────┐ │
NeuralAdvisor (AdvisorySource) ────────────────────────┐│ │
                                                        ↓↓ ↓
                                             OverDomainArbiter
                                             (читает все источники,
                                              решает по TrustConfig,
                                              действует через UCL)
                                                        ↓
                                  UCL / SutraDepthStore / Workstation queue
```

Arbiter читает из:
- `AdvisoryResultStore` (от NeuralAdvisor, через `AdvisorySource` трейт)
- `AxialStore` — чтобы знать детерминированный результат для сравнения

Arbiter пишет в:
- `SutraDepthStore` — при AutoApply DepthHint
- UCL — команды в Workstation (очередь подтверждения)
- `ArbiterLog` — собственный лог решений (принято / отклонено / подтверждено)

---

## 3. AdvisorySource — трейт

Любой модуль который хочет предоставлять рекомендации реализует этот трейт.
В V1 единственный источник — NeuralAdvisor.

```rust
pub trait AdvisorySource: Send + Sync {
    fn source_id(&self) -> SourceId;

    /// Вернуть все активные рекомендации этого источника.
    /// Вызывается каждый тик Arbiter.
    fn poll_advisories(&self) -> Vec<Advisory>;

    /// Обратная связь: рекомендация была применена / отклонена / подтверждена позже.
    fn on_feedback(&mut self, id: AdvisoryId, outcome: AdvisoryOutcome);
}
```

`SourceId` — `u8`, уникальный идентификатор источника. Назначается при регистрации.

---

## 4. Advisory — единица рекомендации

```rust
pub struct Advisory {
    pub id: AdvisoryId,           // u64, уникальный в рамках источника
    pub source: SourceId,
    pub advisory_type: AdvisoryType,
    pub subject_id: u32,          // sutra_id Frame о котором рекомендация
    pub confidence: f32,          // 0.0..1.0
    pub action: AdvisoryAction,
    pub created_at_event: u64,
}

pub enum AdvisoryType {
    DepthHint,
    OctantCorrection,
    ConflictDiagnosis,
    SubsystemAttribution,
    EmergentCandidate,
}

pub enum AdvisoryAction {
    ApplyDepth { octant: usize, depth: u16 },
    NotifyWorkstation { message: String },
    // V2+: ApplyOctant, ApplySubsystem, ...
}
```

---

## 5. TrustConfig — конфигурация доверия

Для каждой пары `(SourceId × AdvisoryType)` задаётся режим работы.

```rust
pub struct TrustEntry {
    pub min_confidence: f32,   // ниже порога — игнорировать
    pub mode: TrustMode,
}

pub enum TrustMode {
    /// Игнорировать — источник зарегистрирован но данный тип не обрабатывается.
    Ignore,
    /// Автономно применить действие если confidence ≥ min_confidence.
    /// Требует Permission::Control в геноме для соответствующего ресурса.
    AutoApply,
    /// Поставить в очередь Workstation — chrnv подтверждает вручную.
    RequireConfirmation,
}
```

`TrustConfig` — `HashMap<(SourceId, AdvisoryType), TrustEntry>`.

В V1 конфигурация задаётся при конструировании Arbiter (не из файла).
Дефолт V1:

| Источник      | Тип                 | Режим               | min_confidence |
|---------------|---------------------|---------------------|----------------|
| NeuralAdvisor | DepthHint           | AutoApply           | 0.75           |
| NeuralAdvisor | OctantCorrection    | RequireConfirmation | 0.60           |
| NeuralAdvisor | ConflictDiagnosis   | Ignore              | —              |
| NeuralAdvisor | SubsystemAttribution| Ignore              | —              |
| NeuralAdvisor | EmergentCandidate   | RequireConfirmation | 0.60           |

**Обоснование:** DepthHint — наиболее безопасное автономное действие (глубина растёт
постепенно, decay откатывает ошибки). OctantCorrection и EmergentCandidate важны, но
требуют наблюдения chrnv на этапе калибровки.

---

## 6. Genome-интеграция

Arbiter в `on_boot` проверяет:
- `ExperienceMemory / Control` — необходим для AutoApply DepthHint
- `ExperienceMemory / Read` — минимум для RequireConfirmation

Если `ExperienceMemory / Control` не выдан геномом — Arbiter понижает все `AutoApply`
до `RequireConfirmation` автоматически. Это делает систему безопасной по умолчанию:
без явного разрешения автономных действий нет.

```yaml
# config/genome.yaml — секция Arbiter (добавить в V1)
- module: OverDomainArbiter
  id: 20
  resources:
    - resource: ExperienceMemory
      permission: Control
    - resource: AshtiField
      permission: Read
```

---

## 7. on_tick

```
Tick interval: 13 (простое, не совпадает с 5/7/11)
```

**on_tick:**
1. Для каждого зарегистрированного источника вызвать `poll_advisories()`
2. Для каждой рекомендации:
   - Найти `TrustEntry` по `(source_id, advisory_type)`
   - Если `confidence < min_confidence` → пропустить, `ArbiterLog::Skipped`
   - Если `Ignore` → пропустить тихо
   - Если `AutoApply` → выполнить `advisory.action`, `ArbiterLog::Applied`
   - Если `RequireConfirmation` → добавить в `PendingQueue`, `ArbiterLog::Queued`
3. Отправить источнику `on_feedback` для применённых и отклонённых рекомендаций

---

## 8. PendingQueue и Workstation

`PendingQueue: VecDeque<PendingAdvisory>` — очередь на подтверждение.

```rust
pub struct PendingAdvisory {
    pub advisory: Advisory,
    pub queued_at_event: u64,
    pub expires_at_event: Option<u64>,  // TTL — V2, в V1 не протухают
}
```

Workstation читает очередь через `PhaseCSnapshot` — добавить `pending_advisories` в V1.
chrnv нажимает "Применить" / "Отклонить" → UCL команда `ArbiterDecision` → Arbiter
исполняет или сбрасывает + отправляет `on_feedback` источнику.

---

## 9. ArbiterLog — обратная связь

Кольцевой буфер последних 500 решений. Не персистируется в V1.

```rust
pub struct ArbiterLogEntry {
    pub event_id: u64,
    pub source: SourceId,
    pub advisory_type: AdvisoryType,
    pub subject_id: u32,
    pub confidence: f32,
    pub outcome: ArbiterOutcome,   // Applied | Queued | Skipped | Confirmed | Rejected
}
```

Из лога считается качество советника: `confirmed / (confirmed + rejected)` для каждого
`(source, advisory_type)`. В V2 это станет основой для автоматической калибровки
`min_confidence` и промоции `RequireConfirmation → AutoApply`.

---

## 10. Что в коде

```
crates/axiom-runtime/src/over_domain/arbiter/
├── mod.rs          — OverDomainArbiter, on_tick, регистрация источников
├── source.rs       — AdvisorySource трейт, Advisory, AdvisoryType, AdvisoryAction
├── trust.rs        — TrustConfig, TrustEntry, TrustMode
└── log.rs          — ArbiterLog, ArbiterLogEntry, ArbiterOutcome
```

NeuralAdvisor реализует `AdvisorySource` в `neural_advisor/mod.rs` — тонкая обёртка
над существующим `AdvisoryResultStore`.

engine.rs:
```
if t % 13 == 0 {
    self.over_domain_arbiter.on_tick(t, &mut self.context_recognizer.depth_store_mut());
}
```

---

## 11. Известные ограничения V1

- **ARB-TD-01** — TrustConfig задаётся в коде, не в `config/genome.yaml`.
  **V2:** вынести в конфиг; `min_confidence` калибруется автоматически по `ArbiterLog`.

- **ARB-TD-02** — PendingQueue не протухает. Старые рекомендации копятся бесконечно.
  **V2:** добавить TTL на основе event_id.

- **ARB-TD-03** — AutoApply только для DepthHint. OctantCorrection требует записи
  в AxialStore, что сейчас невозможно без пересчёта позиции.
  **V2:** добавить `AxialStore::override_octant(sutra_id, octant)` с пометкой "advisory override".

---

## 12. Что в V2+

- **V2:** TrustConfig из конфига; автокалибровка min_confidence; TTL очереди;
  `AxialStore::override_octant`; AdvisoryHistory (тренды по источнику)
- **V3:** Workstation панель с историей решений Arbiter; расхождение advisor vs analytic
  отображается визуально
- **V6+:** второй AdvisorySource (PatternAdvisor из DreamPhase); Arbiter начинает
  агрегировать рекомендации от нескольких источников с весами
- **V9:** Arbiter сам является обучаемым — учится когда доверять каким источникам
  на основе накопленной истории подтверждений
