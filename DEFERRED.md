# Axiom — Отложенные задачи

**Версия:** 24.0
**Обновлён:** 2026-04-20

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
