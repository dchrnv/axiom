# Axiom — Отложенные задачи

**Версия:** 26.0
**Обновлён:** 2026-04-24

---

## Ждут конкретного триггера

### Anchor-Fill — Наполнение якорных YAML-файлов (Фаза 4)

**Где:** `config/anchors/`

Сейчас загружены только:

- `axes.yaml` — 6 осевых якорей (X/Y/Z полюса)
- `layers/L5_cognitive.yaml` — 10 якорей когнитивного слоя
- `domains/D1_execution.yaml` — 6 якорей домена EXECUTION

Для полного семантического покрытия нужно заполнить:

| Файл                     | Слой / Домен | Рекомендуемых якорей |
| ---------------------------- | --------------------- | --------------------------------------- |
| `layers/L1_physical.yaml`  | L1 Physical           | 7+                                      |
| `layers/L2_sensory.yaml`   | L2 Sensory            | 10+                                     |
| `layers/L3_motor.yaml`     | L3 Motor              | 7+                                      |
| `layers/L4_emotional.yaml` | L4 Emotional          | 7+                                      |
| `layers/L6_social.yaml`    | L6 Social             | 7+                                      |
| `layers/L7_temporal.yaml`  | L7 Temporal           | 7+                                      |
| `layers/L8_abstract.yaml`  | L8 Abstract           | 7+                                      |
| `domains/D2_shadow.yaml`   | SHADOW                | 5+                                      |
| `domains/D3_codex.yaml`    | CODEX                 | 5+                                      |
| `domains/D4_map.yaml`      | MAP                   | 5+                                      |
| `domains/D5_probe.yaml`    | PROBE                 | 5+                                      |
| `domains/D6_logic.yaml`    | LOGIC                 | 5+                                      |
| `domains/D7_dream.yaml`    | DREAM                 | 6 (пример в спеке)          |
| `domains/D8_ethics.yaml`   | ETHICS                | 5+                                      |

Формат: [docs/spec/Anchor_Tokens_V1_0.md](docs/spec/Anchor_Tokens_V1_0.md), раздел 7.
Диагностика через CLI: `:match "текст"` — показывает совпадения и вычисленную позицию.

**Когда:** По мере накопления понимания семантики системы (chrnv). Система работает без них — FNV-1a fallback.

---

### D-06 — MLEngine: input_size/output_size = 0 при загрузке ONNX

**Где:** `crates/axiom-agent/src/ml/engine.rs:120-123`

Проверка `if *input_size > 0` скрывает ShapeMismatch-ошибки.

**Когда:** При первой реальной ONNX-модели.

---

### FW-TD-02 — FrameWeaver: min_participant_anchors не проверяется

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`, метод `qualifies_for_promotion`

Поле `PromotionRule::min_participant_anchors` (минимум участников, которые сами являются анкерами SUTRA) не проверяется — требует cross-domain lookup: найти sutra_ids участников Frame в SUTRA-домене. В текущей сигнатуре `check_promotion` нет доступа к AshtiCore.

**Что нужно:**
- Расширить сигнатуру `Weaver::check_promotion` (добавить `ashti: &AshtiCore`) или
- Передавать предвычисленный список SUTRA-анкеров снаружи

**Когда:** При реализации полного пути промоции.

---

### FW-TD-03 — Weaver::check_promotion без доступа к current_tick

**Где:** `crates/axiom-runtime/src/over_domain/traits.rs`, сигнатура `check_promotion`

Сигнатура `fn check_promotion(&self, experience_state: &DomainState, anchors: &[&Token]) -> Vec<PromotionProposal>` не передаёт текущий tick, поэтому `qualifies_for_promotion` использует `tick_proxy = 0` для проверки `min_age_ticks`.

**Что нужно:** Добавить `tick: u64` параметр в сигнатуру трейта (breaking change).

**Когда:** При первой реальной промоции EXPERIENCE → SUTRA.

---

### FW-TD-04 — on_boot не проверяет GENOME-права для FrameWeaver

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs:624`, метод `on_boot`

`_genome` игнорируется — не проверяется наличие `ModuleId::FrameWeaver` в GENOME access_rules и что выданные права соответствуют ожидаемым (EXPERIENCE/ReadWrite, MAYA/Read, SUTRA/Control). TODO-комментарий оставлен в коде.

**Что нужно:** Вызвать `genome.index().check_access(ModuleId::FrameWeaver, ...)` для каждого нужного ресурса; вернуть `Err(OverDomainError::GenomeDenied)` при отсутствии прав.

**Когда:** При добавлении runtime GENOME-enforcement (GenomeIndex уже реализован).

---

### FW-TD-05 — propose_to_dream возвращает пустые команды (DREAM не реализован)

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs:767`, метод `propose_to_dream`

`CrystallizationProposal.commands = Vec::new()` — команды предполагается заполнять DREAM-движком при принятии предложения. DREAM-фаза не существует; `Engine` не вызывает `propose_to_dream` нигде — метод используется только в тестах и будущей интеграции.

**Что нужно:**
1. Создать DREAM-компонент (Over-Domain или отдельный домен 107)
2. Engine подавать стабильные кандидаты через `propose_to_dream`, DREAM принимает/отклоняет и заполняет commands
3. Или упростить: убрать DREAM как посредника, вызывать `propose_to_dream` → `process_command` напрямую

**Когда:** При проектировании DREAM-фазы (когнитивный сон/рефлексия).

---

### FW-TD-06 — промоция EXPERIENCE→SUTRA без участников (dummy_candidate)

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs:727-734`, `on_tick` фаза 5

При срабатывании промоции из `on_tick` создаётся `dummy_candidate` с `participants: Vec::new()`. В результате `build_promotion_commands` генерирует только токен-анкер в SUTRA, но **не** генерирует `BondTokens` к участникам Frame. SUTRA-анкер изолирован — без связей к участникам паттерна.

**Что нужно:** Восстанавливать участников из EXPERIENCE — хранить `participants` в Connection-метаданных EXPERIENCE-анкера или в отдельном side-store (lineage_hash → Vec<sutra_id>).

**Когда:** При полной реализации пути EXPERIENCE → SUTRA.

---

### FW-TD-07 — три нереализованных RuleTrigger ветки (DreamCycle, HighConfidence, RepeatedAssembly)

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs:511-513`, `trigger_matches`

Три ветки `RuleTrigger` всегда возвращают `false`:
- `DreamCycle` — ждёт сигнала от DREAM-фазы (не существует)
- `HighConfidence(f32)` — нет confidence scoring у кандидатов
- `RepeatedAssembly { window_ticks }` — нет счётчика повторных сборок в скользящем окне

**Что нужно:**
- `DreamCycle`: сигнал от DREAM (флаг или канал)
- `HighConfidence`: добавить `confidence: f32` в `FrameCandidate`, вычислять из силы связей
- `RepeatedAssembly`: хранить `assembly_counts: HashMap<u64, (u64, u32)>` (hash → (last_tick, count))

**Когда:** По мере расширения модели кристаллизации.

---

### EA-TD-07 — Применение domain config при hot-reload к running engine

**Где:** `crates/axiom-agent/src/tick_loop.rs`, ветка `if let Some(_new_cfg) = watcher.poll()`

`ConfigWatcher` перенесён в `tick_loop` (EA-TD-05 ✅), поллинг работает, изменения axiom.yaml
обнаруживаются. Однако применение обновлённых domain-пресетов к уже запущенному `AxiomEngine`
не реализовано — `AxiomEngine` не имеет метода `apply_domain_config(&DomainConfig)`.

**Что нужно:**
1. Добавить `pub fn apply_domain_config(&mut self, domain_id: u16, cfg: &DomainConfig)` в `AxiomEngine`
2. В `tick_loop.rs` при обнаружении изменений перебрать `new_cfg.domains` и применить каждый
3. Логировать что именно изменилось (threshold, membrane, physics params)

**Когда:** Когда понадобится живая перенастройка доменных порогов без рестарта. Сейчас обход — рестарт axiom-cli с новыми конфигами.

---

## Внешние адаптеры

**Спецификация:** [docs/spec/External_Adapters_V3_0.md](docs/spec/External_Adapters_V3_0.md)
**Гайд:** [docs/guides/External_Adapters_Guide_V1_0.md](docs/guides/External_Adapters_Guide_V1_0.md)

| Адаптер         | Requires          | Фаза   | Статус |
|-----------------|-------------------|--------|--------|
| Рефактор CLI    | —                 | 0A/0B/0C | ✅   |
| WebSocket       | axum              | 1      | ✅     |
| REST API        | axum              | 2      | ✅     |
| egui Dashboard  | eframe            | 3      | ✅     |
| Telegram        | reqwest (feature) | 4      | ✅     |
| OpenSearch      | reqwest (feature) | 5      | ✅     |
| gRPC            | tonic + protobuf  | —      | не сейчас |
| Python bindings | pyo3              | —      | не сейчас |
