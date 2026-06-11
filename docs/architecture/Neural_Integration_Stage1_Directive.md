# AXIOM — Neural Integration: Этап 1 (директива)

**Дата:** 2026-06-11
**Статус:** Решение chrnv. Закрывает обсуждение V8 → Opus-спека → синтез Sonnet.
**Основной документ:** `Neural_Integration_V1_0.md` — остаётся в силе, эта страница сужает объём работ.

---

## Решение

Три этапа. Сейчас делаем ТОЛЬКО этап 1.

```
ЭТАП 1 (СЕЙЧАС)  — нейронка смотрит ВНУТРЬ
  Малая модель прикручена к Sensorium. Кристаллизует опыт, находит паттерны
  во внутреннем состоянии. Заменяет rule-based советников по одному.

ЭТАП 2 (ПОТОМ)   — понимание, голос, слух, зрение
  AudioPerceptor (среда + speech commands), Vision, выражение.
  Синтез Sonnet (два трека, один стек, rustfft) — сюда. Не потерян, отложен.

ЭТАП 3 (ПЛЮШКИ) — ультразвук-ветка, расширенный STT, прочее.
```

---

## Этап 1 — объём работ

Всё по `Neural_Integration_V1_0.md` §13, фазы 0–3, на ОДНОМ пилотном советнике:

```
1. Крейт axiom-neural:
   - rustfft (pure Rust, НЕ fftw)
   - ndarray с дефолтным matrixmultiply (pure Rust, НЕ ndarray-blas/OpenBLAS)
   - загрузка весов из .bin, static memory, inference

2. Пилот: ОДИН советник (предлагается ReactivationDepth — простейший вход)
   - вход: срез Sensorium (уровень 2, на t%11) + ActivityTrace rings [16/64/256]
   - архитектура: 1D-CNN (kernel 3–5) → GlobalAvgPool → MLP → AdvisorOutput
   - размер: 10–50K параметров для пилота
   - FFT над ActivityTrace как признаки — спектр внутренних ритмов системы

3. Дистилляция (спека §8): teacher = текущий rule-based, student = модель.
   Student воспроизводит teacher 95%+ → дообучение на DivergenceLog.

4. Калибровка confidence (спека §7). Без калибровки — только Ignore/Confirm.

5. Тренировка: ОФФЛАЙН или в DREAM (спека §6). Никогда на горячем пути.

6. Промоция доверия: Ignore → RequireConfirmation → AutoApply,
   только через genome, только chrnv (спека §5).
```

## Защита производительности (не поставить на колени)

```
- inference ТОЛЬКО на t%11, с таймаутом; превысил → пропуск, fallback
- ядро не ждёт модель никогда (advisory сбоку, tick идёт всегда)
- модель 10–50K параметров, inference << 100 µs
- fallback rule-based жив всегда; выдерни модель — AXIOM работает
- bench до/после: TickForward (50 tok) НЕ должен вырасти
  (hot path 24.8 µs — охраняемая цифра)
```

## Что ОТРЕЗАНО от этапа 1 (не делать, не проектировать)

```
- AudioPerceptor (все ветки: среда / speech commands / ультразвук) → этап 2
- rubato, захват микрофона, спектрограммы аудио → этап 2
- Vision-нейронка → этап 2
- Языковой выход / речевой орган → этап 2
- Остальные 4 советника → после успеха пилота (спека §13 фаза 4)
```

## Критерий успеха этапа 1

```
Пилотный советник на модели:
- воспроизводит rule-based teacher ≥ 95%
- после дообучения на DivergenceLog превосходит teacher на сложных случаях
- confidence калиброван (raw≠calibrated, calibrated отражает реальную точность)
- hot path не вырос, DREAM не затянут
- chrnv видит в Workstation: accuracy, divergence, состояние модели
→ тогда фаза 4 (остальные советники), потом этап 2.
```

---

## Архитектура крейта axiom-neural (Этап 1)

### Структура

```
crates/axiom-neural/
├── Cargo.toml           # rustfft, ndarray (matrixmultiply, NO blas feature)
├── src/
│   ├── lib.rs           # pub: Model, AdvisorInput, AdvisorOutput, NeuralError
│   ├── model.rs         # Model trait + Sequential impl
│   ├── layers.rs        # Conv1D, GlobalAvgPool, Linear, ReLU
│   ├── fft.rs           # FFT-frontend (rustfft wrapper, static буферы)
│   ├── normalize.rs     # Z-score нормализация
│   ├── calibration.rs   # ConfidenceCalibrator (таблица, обновляется из DivergenceLog)
│   └── weights.rs       # load_from_bin() / save_to_bin() — bincode
```

### Cargo.toml зависимости

```toml
[dependencies]
rustfft   = "6"          # pure Rust FFT, no C
ndarray   = "0.16"       # матрицы; feature matrixmultiply (default), NO "blas"
bincode   = { workspace = true }  # загрузка весов
serde     = { workspace = true }  # метаданные модели (размеры, версия)

# НЕТ: fftw, ndarray-blas, openblas-src, candle, burn, tract, onnx
```

### Ключевые типы

```rust
/// Вход советника на каждом t%11.
pub struct AdvisorInput {
    /// ActivityTrace rings: [short=16, mid=64, long=256] × N_subsystems
    /// Уже нормализованы (Z-score) и прошли FFT-frontend.
    pub features: Vec<f32>,        // pre-allocated, размер фиксирован при load
    pub tick: u64,
}

/// Выход любой нейронной модели советника.
pub struct AdvisorOutput {
    pub value: Vec<f32>,           // зависит от советника (напр. depth[8])
    pub raw_confidence: f32,       // до калибровки
    pub calibrated_confidence: f32,// после ConfidenceCalibrator
    pub computation_ns: u64,       // для мониторинга таймаута
}

pub trait Model: Send + Sync {
    fn infer(&self, input: &AdvisorInput) -> Result<AdvisorOutput, NeuralError>;
    fn load_from_bin(path: &Path) -> Result<Self, NeuralError> where Self: Sized;
    fn save_to_bin(&self, path: &Path) -> Result<(), NeuralError>;
    fn param_count(&self) -> usize;
}
```

### FFT-frontend (shared для всех советников)

```rust
// fft.rs
// FftFrontend: предвыделён при init, ноль alloc в infer()
pub struct FftFrontend {
    plan: Arc<dyn Fft<f32>>,       // rustfft plan, reuse
    scratch: Vec<Complex<f32>>,    // static scratch buffer
    output: Vec<f32>,              // magnitude spectrum
}

impl FftFrontend {
    /// ActivityTrace ring → magnitude spectrum (half = N/2+1 компонент).
    /// Работает inplace на предвыделённых буферах.
    pub fn compute(&mut self, ring: &[f32], out: &mut [f32]);
}
```

### ReactivationDepth — пилотная модель

```
Вход: ActivityTrace { short[16], mid[64], long[256] } × 9 подсистем
      → FFT каждого кольца → [9, 26] частотные признаки (8+32+128 / 2 + 1 ≈ 26 каждый)
      → flatten + Z-score → features[~2100]

Слои:
  Conv1D(in_ch=9, out_ch=16, kernel=3, stride=1) → ReLU  →  [16, ~2098]
  Conv1D(in_ch=16, out_ch=32, kernel=3, stride=1) → ReLU →  [32, ~2096]
  GlobalAvgPool                                           →  [32]
  Linear(32, 16)                                  → ReLU  →  [16]
  Linear(16, 8)                                           →  value[8]  (depth per octant)
  Linear(16, 1)                                  → Sigmoid →  raw_confidence

Параметры: ~16K (в пределах 10–50K директивы)
```

### Инварианты

| Правило | Значение |
|---------|----------|
| Нет alloc в infer() | все буферы в Model::load_from_bin() |
| Нет C-биндингов | rustfft + ndarray/matrixmultiply, pure Rust |
| Таймаут | caller (NeuralAdvisor) ждёт max 1ms, иначе fallback |
| Веса | `.bin` (bincode), не ONNX, не safetensors |
| Тренировка | НИКОГДА в axiom-neural production; отдельный `axiom-neural-train` бинарь |

---

## Примечания для этапа 2 (зафиксировать, не делать)

Из обсуждения Opus + Sonnet, чтобы не потерять к моменту этапа 2:

```
- "STT" на 100–200K параметров = keyword spotting (10–50 команд),
  НЕ свободная речь. Честное имя: Speech Commands.
- Ветки аудио V1: среда 20–300 Hz + команды 300–8000 Hz.
  Ультразвук >20 kHz = hardware-dependent, этап 3.
- Аудио-поток в отдельном потоке, в ядро через Gateway/Channel
  (инвариант единственного writer), wall-clock только в адаптере.
- Веса speech commands: Google Speech Commands dataset + дозапись,
  тренировка оффлайн, перенос .bin. Inference локальный — самодостаточность.
- Стек уже отработан этапом 1 (rustfft, ndarray, .bin, inference) —
  аудио ложится поверх готового.
```
