# Axiom — Отложенные задачи

**Версия:** 20.0
**Обновлён:** 2026-04-14

---

## Ждут конкретного триггера

### Anchor-Fill — Наполнение якорных YAML-файлов (Фаза 4)

**Где:** `config/anchors/`

Сейчас загружены только:
- `axes.yaml` — 6 осевых якорей (X/Y/Z полюса)
- `layers/L5_cognitive.yaml` — 10 якорей когнитивного слоя
- `domains/D1_execution.yaml` — 6 якорей домена EXECUTION

Для полного семантического покрытия нужно заполнить:

| Файл | Слой / Домен | Рекомендуемых якорей |
|------|-------------|----------------------|
| `layers/L1_physical.yaml` | L1 Physical | 7+ |
| `layers/L2_sensory.yaml` | L2 Sensory | 10+ |
| `layers/L3_motor.yaml` | L3 Motor | 7+ |
| `layers/L4_emotional.yaml` | L4 Emotional | 7+ |
| `layers/L6_social.yaml` | L6 Social | 7+ |
| `layers/L7_temporal.yaml` | L7 Temporal | 7+ |
| `layers/L8_abstract.yaml` | L8 Abstract | 7+ |
| `domains/D2_shadow.yaml` | SHADOW | 5+ |
| `domains/D3_codex.yaml` | CODEX | 5+ |
| `domains/D4_map.yaml` | MAP | 5+ |
| `domains/D5_probe.yaml` | PROBE | 5+ |
| `domains/D6_logic.yaml` | LOGIC | 5+ |
| `domains/D7_dream.yaml` | DREAM | 6 (пример в спеке) |
| `domains/D8_ethics.yaml` | ETHICS | 5+ |

Формат: [docs/spec/Anchor_Tokens_V1_0.md](docs/spec/Anchor_Tokens_V1_0.md), раздел 7.
Диагностика через CLI: `:match "текст"` — показывает совпадения и вычисленную позицию.

**Когда:** По мере накопления понимания семантики системы (chrnv). Система работает без них — FNV-1a fallback.

---

### D-06 — MLEngine: input_size/output_size = 0 при загрузке ONNX

**Где:** `crates/axiom-agent/src/ml/engine.rs:120-123`

Проверка `if *input_size > 0` скрывает ShapeMismatch-ошибки.

**Когда:** При первой реальной ONNX-модели.

---

## Внешние адаптеры

**Точка расширения:** `RuntimeAdapter` trait в `axiom-runtime/src/adapters.rs`.

| Адаптер | Требует | Статус |
|---|---|---|
| WebSocket | axum / actix-web | не начат |
| REST API | axum / actix-web | не начат |
| gRPC | tonic + protobuf | не начат |
| Python bindings | pyo3 | не начат |

**Когда:** При конкретной задаче внешней интеграции.
