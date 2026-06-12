# AXIOM — Neural Training Guide

**Версия:** 1.0  
**Дата:** 2026-06-12  
**Модель:** ReactivationDepthModel (пилот, Neural Integration Этап 1)  
**Крейт:** `crates/axiom-neural/`

---

## Главное — что видит нейронка

**Нейронка НЕ видит токены.** Она видит **ритмы активности подсистем во времени.**

Конкретно: три кольцевых буфера `ActivityTrace` — история того насколько каждая из 9
подсистем была активна за последние 16 / 64 / 256 тиков.

```
short_ring[16]  — последние ~160ms (при 100 Hz)  — "что прямо сейчас"
mid_ring[64]    — последние ~640ms               — "что в ближайшем прошлом"
long_ring[256]  — последние ~2.5 сек             — "общий фон"
```

Из этих рингов FFT извлекает **частотный спектр** — какие ритмы есть у системы:
быстрые переключения, медленные доминирование, регулярные паттерны.

Это **паттерны второго порядка**: нейронка учится понимать не "что за токен",
а "как система дышит в целом". Отдельный токен ей ничего не скажет.

---

## Архитектура (должна совпадать Python ↔ Rust)

```
Вход: [9 подсистем, 171 FFT-признаков] = 1539 float32
  (171 = 9 компонент из short + 33 из mid + 129 из long)

Conv1D(in_ch=9, out_ch=32, kernel=3, stride=1) → ReLU  → [32, 169]
Conv1D(in_ch=32, out_ch=64, kernel=5, stride=1) → ReLU → [64, 165]
GlobalAvgPool                                           → [64]
Linear(64 → 32)                                 → ReLU → [32]
Linear(32 → 8)                                         → value[8]   (depth per octant)
Linear(32 → 1)                                + Sigmoid → confidence  (0..1)
```

**Параметры (~13K):**
- Conv1: (9×3+1)×32 = 896
- Conv2: (32×5+1)×64 = 10304
- FC1: 64×32+32 = 2080
- FC_value: 32×8+8 = 264
- FC_conf: 32×1+1 = 33
- **Итого: 13577**

Архитектура читается из `genome.yaml` → `neural_advisor.depth.arch`.
При изменении arch старый `.bin` несовместим, нужна перетренировка.

---

## Отображение каналов (SubsystemId → индекс)

**КРИТИЧНО:** порядок каналов фиксирован. Нарушение = бессмыслица на выходе.

| Индекс канала | Подсистема | SubsystemId |
|---------------|------------|-------------|
| 0 | Writing | 1 |
| 1 | Mathematics | 2 |
| 2 | Logic | 3 |
| 3 | Time | 4 |
| 4 | Music | 5 |
| 5 | Values | 6 |
| 6 | Morality | 7 |
| 7 | Abstractions | 8 |
| 8 | Dilemmas | 9 |

Этот порядок соответствует `SubsystemId` в `axiom-experience/src/lib.rs`.

---

## Формат тренировочных данных

### Файл: `obs_out/training_data.jsonl`

Каждая строка — один тренировочный пример. JSON:

```json
{
  "tick": 12500,
  "rings": {
    "short": [[f32×16], [f32×16], ..., [f32×16]],
    "mid":   [[f32×64], [f32×64], ..., [f32×64]],
    "long":  [[f32×256], [f32×256], ..., [f32×256]]
  },
  "teacher_output": {
    "reactivation_weights": [f32×8],
    "confidence": f32
  },
  "metadata": {
    "dominant_subsystem": 2,
    "dominant_octant": 1,
    "active_dilemmas": 0,
    "entropy_gradient": 0.12
  }
}
```

**`rings`**: матрицы `[9 подсистем][длина кольца]`.
`rings.short[0]` = история Writing за 16 тиков, `rings.short[1]` = Mathematics, и т.д.

**`teacher_output.reactivation_weights`**: что сказал rule-based советник.
8 float32 в диапазоне [0, 1], один на каждый октант (O1..O8).
Значение = "насколько сильно нужно реактивировать кадры в этом октанте".

**`teacher_output.confidence`**: уверенность rule-based советника (0..1).
Для rule-based это детерминировано (часто 1.0 или фиксированные значения).

---

## Предобработка (должна совпадать с Rust)

Pipeline строго совпадает с `ReactivationDepthModel::extract_features()`:

```python
import numpy as np
from scipy.fft import rfft

def extract_features(rings: dict) -> np.ndarray:
    """
    rings: {'short': array [9, 16], 'mid': array [9, 64], 'long': array [9, 256]}
    returns: array [9, 171] float32
    """
    features = np.zeros((9, 171), dtype=np.float32)
    
    for ch in range(9):
        # FFT каждого кольца (magnitude, half-spectrum)
        short_fft = np.abs(rfft(rings['short'][ch])) / np.sqrt(16)   # 9 компонент
        mid_fft   = np.abs(rfft(rings['mid'][ch]))   / np.sqrt(64)   # 33 компоненты
        long_fft  = np.abs(rfft(rings['long'][ch]))  / np.sqrt(256)  # 129 компонент
        
        features[ch, :9]      = short_fft
        features[ch, 9:42]    = mid_fft
        features[ch, 42:171]  = long_fft
    
    # Z-score по всему вектору (как в zscore_inplace)
    flat = features.flatten()
    mean, std = flat.mean(), flat.std()
    if std > 1e-7:
        features = (features - mean) / std
    else:
        features[:] = 0.0
    
    return features
```

**Важно:** Z-score применяется к **плоскому вектору** (всем 1539 числам вместе),
не по каждому каналу отдельно. Так работает `zscore_inplace` в Rust.

---

## Python-модель (PyTorch)

Должна быть архитектурно идентична Rust-модели:

```python
import torch
import torch.nn as nn

class ReactivationDepthModel(nn.Module):
    def __init__(self, cfg):
        super().__init__()
        ch1, ch2 = cfg['conv1_channels'], cfg['conv2_channels']
        k2, fc1  = cfg['conv2_kernel'],   cfg['fc1_size']
        
        self.conv1    = nn.Conv1d(9, ch1, kernel_size=3, stride=1)
        self.conv2    = nn.Conv1d(ch1, ch2, kernel_size=k2, stride=1)
        self.fc1      = nn.Linear(ch2, fc1)
        self.fc_value = nn.Linear(fc1, 8)
        self.fc_conf  = nn.Linear(fc1, 1)
    
    def forward(self, x):
        # x: [batch, 9, 171]
        x = torch.relu(self.conv1(x))     # [batch, ch1, 169]
        x = torch.relu(self.conv2(x))     # [batch, ch2, ...]
        x = x.mean(dim=-1)               # GlobalAvgPool → [batch, ch2]
        x = torch.relu(self.fc1(x))      # [batch, fc1]
        value = self.fc_value(x)         # [batch, 8]
        conf  = torch.sigmoid(self.fc_conf(x))  # [batch, 1]
        return value, conf

# Конфиг из genome.yaml
cfg = {'conv1_channels': 32, 'conv2_channels': 64, 'conv2_kernel': 5, 'fc1_size': 32}
model = ReactivationDepthModel(cfg)
```

---

## Функция потерь

```python
def loss_fn(pred_value, pred_conf, target_value, target_conf):
    # MSE на reactivation weights
    loss_value = nn.functional.mse_loss(pred_value, target_value)
    # BCE на confidence
    loss_conf  = nn.functional.binary_cross_entropy(pred_conf, target_conf)
    return loss_value + 0.1 * loss_conf
```

Коэффициент 0.1 на confidence — confidence вторична, основная задача = правильные веса.

---

## Метрики качества

| Метрика | Что значит | Порог |
|---------|-----------|-------|
| `teacher_agreement` | % случаев где top-3 октанта модели = top-3 советника | ≥ 95% (критерий дистилляции) |
| `octant_mse` | MSE по 8 весам реактивации | < 0.05 |
| `conf_calibration_error` | |E[conf] - P(top-1 correct)| | < 0.10 |
| `inference_ns` | Время инференса (release mode) | < 500 µs |

**Критерий перехода из дистилляции в дообучение:** `teacher_agreement ≥ 0.95`
**Критерий промоции Ignore→RequireConfirmation:** держится неделю без деградации

---

## Экспорт весов в .bin

После тренировки нужно конвертировать PyTorch-веса в формат bincode (Rust).

Используй утилиту `axiom-neural-convert` (создаётся отдельно):

```bash
# Сохранить веса из PyTorch
torch.save({
    'conv1.weight': model.conv1.weight.detach().cpu().numpy(),  # [ch1, 9, 3]
    'conv1.bias':   model.conv1.bias.detach().cpu().numpy(),
    'conv2.weight': model.conv2.weight.detach().cpu().numpy(),
    'conv2.bias':   model.conv2.bias.detach().cpu().numpy(),
    'fc1.weight':   model.fc1.weight.detach().cpu().numpy(),
    'fc1.bias':     model.fc1.bias.detach().cpu().numpy(),
    'fc_value.weight': model.fc_value.weight.detach().cpu().numpy(),
    'fc_value.bias':   model.fc_value.bias.detach().cpu().numpy(),
    'fc_conf.weight':  model.fc_conf.weight.detach().cpu().numpy(),
    'fc_conf.bias':    model.fc_conf.bias.detach().cpu().numpy(),
}, 'weights.npz')

# Конвертировать в .bin
cargo run -p axiom-neural-convert -- weights.npz models/reactivation_depth.bin
```

`axiom-neural-convert` — отдельный бинарник в `crates/axiom-neural/` (ещё не реализован,
следующий шаг). Он читает `.npz`, строит структуру `Weights` и сохраняет через bincode.

---

## Проверка совместимости

После конвертации проверь что Rust и Python дают одинаковый результат на одном входе:

```bash
# В Rust
cargo test -p axiom-neural test_save_load_roundtrip

# Численная проверка (отдельный скрипт)
python3 scripts/verify_weights.py \
    --weights models/reactivation_depth.bin \
    --test-input obs_out/training_data.jsonl \
    --tolerance 1e-5
```

Допустимая погрешность: < 1e-5 на каждом выходе (float32 округление).

---

## Типичные ошибки

| Проблема | Причина | Решение |
|----------|---------|---------|
| Все reactivation_weights ≈ одинаковые | Z-score не применён к входу | Проверить preprocessing |
| `teacher_agreement` = 50% | Порядок каналов перепутан | Сверить SubsystemId → channel mapping |
| Confidence всегда ≈ 0.5 | Слишком мало данных для calibration | Нужно ≥ 1000 примеров на бин |
| Inference в Rust ≠ PyTorch | Conv1D порядок осей | PyTorch: [batch, ch, len], Rust: то же |
| `.bin` не загружается | Несовместимость arch | Пересобери модель с arch из genome.yaml |

---

## Сбор данных (пошаговый сценарий)

```bash
# 1. Запустить OBS — он теперь собирает training_data.jsonl
cargo build --release -p axiom-observe
./target/release/axiom-observe \
    config/obs/corpus_showcase.yaml \
    obs_out \
    config/anchors

# 2. Данные в obs_out/training_data.jsonl
wc -l obs_out/training_data.jsonl  # должно быть >> 1000 записей

# 3. Тренировать (Python)
python3 scripts/train_reactivation_depth.py \
    --data obs_out/training_data.jsonl \
    --epochs 50 \
    --output weights.npz

# 4. Конвертировать (Rust)
cargo run -p axiom-neural-convert -- weights.npz models/reactivation_depth.bin

# 5. Переключить режим
# В genome.yaml: mode: neural

# 6. Запустить и проверить через Workstation → Internals
./run.sh
```

---

## История

- **V1.0** (2026-06-12): первый гайд. Дистилляция rule-based → neural.
  Формат JSONL, Python/PyTorch зеркальная архитектура, bincode .bin обмен.
  `axiom-neural-convert` и `scripts/train_reactivation_depth.py` — следующий шаг.
