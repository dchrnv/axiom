# AXIOM — Neural Integration Этап 1: Результат

**Дата:** 2026-06-12  
**Статус:** Этап 1 завершён (инфраструктура). Обучение модели — DEFERRED (NEURAL-TD-01..03).  
**Читатель:** следующая сессия Opus — контекст для Этапа 2 и для завершения обучения.

---

## Что сделано в Этапе 1

### Фаза 0 — axiom-neural (крейт инференса) ✅

```
crates/axiom-neural/src/
  lib.rs             — pub API: Model, AdvisorInput, AdvisorOutput, NeuralError
  model.rs           — Model trait + ModelMeta
  layers.rs          — Conv1D, Linear, GlobalAvgPool, relu, sigmoid
  fft.rs             — FftFrontend (rustfft, static scratch), ActivityFft (rings → magnitudes)
  normalize.rs       — zscore_inplace, minmax_inplace
  calibration.rs     — ConfidenceCalibrator (таблица + platt scaling) — РЕАЛИЗОВАН, не подключён
  weights.rs         — load_from_bin / save_to_bin (bincode)
  config.rs          — AdvisorMode {Rule, Neural}, ReactivationDepthConfig, ReactivationDepthArch
  reactivation_depth.rs — ReactivationDepthModel (пилот)
```

Инварианты выдержаны: pure Rust (rustfft + ndarray/matrixmultiply), нет C-биндингов,
нет alloc в `infer()` — все буферы предвыделены в `new_zeros()` / `load_from_bin()`.

### Фаза 1 — ReactivationDepthModel ✅

**Реальная архитектура** (отличается от спеки §13):

```
INPUT_SIZE = N_SUBSYSTEMS(9) × FFT_FEATURES_PER_SUB(171) = 1539
  FFT_FEATURES_PER_SUB = short(8+1=9) + mid(32+1=33) + long(128+1=129) = 171

Conv1D(9 → 32, kernel=3, stride=1) → ReLU
Conv1D(32 → 64, kernel=5, stride=1) → ReLU
GlobalAvgPool → [64]
Linear(64 → 32) → ReLU → [32]
Linear(32 → 8)           → value[8]     (depth-weight per octant)
Linear(32 → 1) + Sigmoid → raw_confidence

Параметры: ~13K (в пределах директивы 10–50K)
```

Спека предлагала CH1=16/CH2=32; реализовано CH1=32/CH2=64 — бо́льшая ёмкость, те же инварианты.

`from_arch(cfg)` — конфигурируемая архитектура из genome.yaml; `new_zeros()` — дефолт для тестов.  
`save/load_from_bin()` — roundtrip-тест проходит.

### Фаза 2 — Сбор training_data.jsonl ✅

OBS runner (`crates/axiom-observe/src/runner.rs`) пишет `training_data.jsonl` каждые
`TRAINING_SAMPLE_EVERY=200` тиков (только single-shard прогон).

**Формат** (`crates/axiom-observe/src/training.rs`):

```json
{
  "tick": 4200,
  "short": [/* 9×16=144 float */],
  "mid":   [/* 9×64=576 float */],
  "long":  [/* 9×256=2304 float */],
  "reactivation_weights": [0.97, 0.12, 0.0, 0.34, 0.0, 0.0, 0.0, 0.0],
  "teacher_confidence": 0.82,
  "meta": {
    "dominant_subsystem": 3,
    "dominant_octant": 0,
    "active_dilemmas": 2,
    "entropy_gradient": 0.15,
    "dominant_persistence": 0.73,
    "experience_traces": 312
  }
}
```

One-hot кольца: фиксированный порядок каналов (Writing=0..Dilemmas=8, канал 0 = absent).  
Teacher: `reactivation_weights[i] = max(0, 1 - avg_depth[i] / 3000)`.

### Фаза 3 — NeuralReactivationDepthAdvisor ✅

```
crates/axiom-runtime/src/over_domain/neural_advisor/neural_depth.rs
```

- Два режима: `AdvisorMode::Rule` (дефолт) и `AdvisorMode::Neural`
- `update_from_trace()` вызывается из `NeuralAdvisor::on_tick` раз в 11 тиков
- Таймаут: `INFER_TIMEOUT_NS=1_000_000` (1ms); при превышении — кеш не обновляется
- Fallback: если lock занят или timeout — `predict_depth()` читает старый кеш
- `from_config(cfg, repo_root)` загружает `.bin` если существует, иначе нулевые веса
- Подключён через `engine.rs:1317` — `sync_activity_trace`

### Фаза 4 — Workstation (Neural Depth Advisor панель) ✅

Bar chart по 8 октантам (cached_weights), mode индикатор, последний inference time.

---

## Открытые проблемы (DEFERRED)

### NEURAL-TD-02 — Мисматч формата (БЛОКИРУЕТ обучение) ⚠️

`training_data.jsonl` пишет one-hot: total **3024 признака**.  
`ReactivationDepthModel.infer()` ожидает FFT: `INPUT_SIZE = **1539**`.  
`neural_depth.rs` mode=Neural подаёт `extract_onehot_rings()` → `ShapeMismatch` → silent fallback.

**Итог: mode=Neural физически не работает в текущей реализации.**

Нужно выбрать один формат до тренировки:
- Вариант A (рекомендуется): FFT везде — `training.rs` прогоняет кольца через `ActivityFft` перед записью
- Вариант B: one-hot везде — `INPUT_SIZE=3024`, убрать FFT pipeline из модели

### NEURAL-TD-01 — Тренировка (ждёт NEURAL-TD-02 + реального корпуса)

Нужно ≥10K примеров `training_data.jsonl`. Синтетического corpus_showcase (~1000 примеров за 200K тиков)
недостаточно. Нужен CORPUS-TD-01 (реальные тексты).

**Скрипт тренировки (Python/torch)** — не написан, ожидается вне репо:
1. Загрузить `training_data.jsonl`
2. Дистилляция: supervised learning (вход=features, таргет=reactivation_weights[8])
3. Сохранить веса в `models/reactivation_depth.bin` через bincode-совместимый формат
4. `axiom-neural` подхватит при старте если файл существует

### NEURAL-TD-03 — ConfidenceCalibrator не подключён

`AdvisorOutput.calibrated_confidence = raw_confidence` (копирование, не калибровка).
`ConfidenceCalibrator` реализован в `calibration.rs` — нужно подключить после тренировки.

---

## Ключевые файлы для Этапа 2

```
crates/axiom-neural/src/
  fft.rs           — FftFrontend и ActivityFft: shared FFT для всех советников
  layers.rs        — Conv1D, Linear, GlobalAvgPool — building blocks для новых моделей
  weights.rs       — load/save .bin — стандарт для всех весов
  config.rs        — AdvisorMode — расширить новыми советниками

crates/axiom-runtime/src/over_domain/neural_advisor/
  neural_depth.rs  — паттерн интеграции: from_config + update_from_trace + predict_depth
  mod.rs           — NeuralAdvisor::on_tick — где регистрировать новые советники

crates/axiom-observe/src/
  training.rs      — TrainingExample: паттерн сбора данных для будущих советников
  runner.rs:221    — точка подключения TRAINING_SAMPLE_EVERY

crates/axiom-agent/src/perceptors/
  text.rs          — TextPerceptor: паттерн перцептора (воспроизвести для Audio/Vision)
  temporal.rs      — TemporalPerceptor: паттерн matching + stable_id
```

---

## Этап 2 — контекст (из директивы)

```
ЭТАП 2 — понимание, голос, слух, зрение:
  AudioPerceptor (среда 20–300 Hz + speech commands 300–8000 Hz)
  Vision-нейронка (поверх существующего L0VisionPerceptor)
  Языковое выражение (output — не реализован в этапе 1)

Стек уже отработан Этапом 1:
  rustfft, ndarray, .bin inference, ActivityFft, AdvisorMode, from_config паттерн
  — AudioPerceptor ложится поверх готового.

Аудио-поток: отдельный тред, в ядро через Gateway/Channel (инвариант единственного writer).
Wall-clock только в адаптере (AudioPerceptor), не в ядре.
Speech Commands: keyword spotting (10–50 слов), НЕ свободная речь.
  Веса: Google Speech Commands dataset + дозапись → .bin.
```

---

## Состояние тестов на момент завершения Этапа 1

1514 тестов (all features), 0 failures.  
TEST-TD-01 (`test_process_and_observe_slow_path_initially`) — pre-existing регрессия в DEFERRED.
