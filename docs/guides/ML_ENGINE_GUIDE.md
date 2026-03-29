# MLEngine — гайд-разъяснение

**Версия:** 1.0
**Дата:** 2026-03-29
**Этап:** 11 (ML Inference)
**Файл:** [`crates/axiom-agent/src/ml/engine.rs`](../../crates/axiom-agent/src/ml/engine.rs)

---

## Зачем это нужно

`MLEngine` — тонкая обёртка, которая скрывает разницу между:

- **тестовой средой** (нет ONNX файлов, нет GPU, нужна детерминированность)
- **production** (реальная ONNX модель, tract-onnx)

Весь остальной код (`VisionPerceptor`, `AudioPerceptor`) работает **только с `MLEngine`** — не знает, mock это или настоящая модель.

---

## MLError

```
MLError
 ├── ModelNotFound(String)          — файл .onnx не существует
 ├── LoadFailed(String)             — файл есть, но парсинг провалился
 ├── ShapeMismatch { expected, got } — дали 100 float, модель ждёт 150528
 ├── InferenceFailed(String)        — ошибка внутри tract во время run()
 └── NotEnabled                     — вызвали load() без --features onnx
```

`ShapeMismatch` — самая частая ошибка при интеграции. Модель ждёт тензор
`[3, 224, 224]` = 150528 float, а ты передал 100. Ошибка скажет точно что
ожидалось и что пришло.

`NotEnabled` — это не баг, а явная граница: если хочешь реальный ONNX, добавь
feature. Без него крейт компилируется легко и быстро.

---

## MLDetection

```rust
pub struct MLDetection {
    pub class_id: u32,      // какой класс (0 = person, 1 = car, ...)
    pub confidence: f32,    // уверенность [0.0, 1.0]
    pub bbox: [f32; 4],     // [x_center, y_center, width, height] в пикселях
}
```

Это **выходной формат** — то, что `VisionPerceptor` строит из сырого float-вектора
модели. Одна детекция = один обнаруженный объект.

`confidence` потом кодируется в `UclCommand::priority` — чем увереннее модель,
тем выше приоритет токена в движке.

---

## MLEngine как enum

```rust
pub enum MLEngine {
    Mock { input_shape: Vec<usize>, output: Vec<f32> },
    #[cfg(feature = "onnx")]
    Real { model: Box<dyn Runnable>, input_size: usize, output_size: usize },
}
```

**Почему enum, а не trait-объект?**

Trait-объект (`Box<dyn MLEngine>`) требовал бы `dyn` диспетчеризацию везде. Enum
даёт статическую диспетчеризацию через `match` — компилятор видит оба пути и
оптимизирует.

`#[cfg(feature = "onnx")]` на варианте `Real` — это ключевой момент. Без feature
этого варианта **физически не существует** в бинарнике. tract-onnx (~50+
зависимостей) не компилируется вообще.

---

## Два режима: как переключаться

**Mock (по умолчанию) — для тестов:**

```rust
// Модель "принимает" 150528 float (3×224×224) и "возвращает" 3 класса
let engine = MLEngine::mock(
    vec![3, 224, 224],
    vec![0.9, 0.07, 0.03],
);

let output = engine.infer(&vec![0.0f32; 150528]).unwrap();
// → [0.9, 0.07, 0.03] — всегда, детерминированно
```

Если `input_shape = vec![0]` — проверка размера отключена, принимает любой вход.
Удобно когда размер ещё не известен.

**Real (feature "onnx") — для production:**

```toml
# Cargo.toml потребителя
axiom-agent = { path = "...", features = ["onnx"] }
```

```rust
let engine = MLEngine::load(Path::new("models/yolo_tiny.onnx"))?;
let output = engine.infer(&tensor)?;
```

---

## Поток данных через VisionPerceptor

```
PNG файл
    │  image::open()
    ▼
RGBA пиксели [u8]
    │  pixels_to_tensor()  →  делит каждый байт на 255.0
    ▼
f32 тензор [0.0..1.0]
    │  MLEngine::infer()
    ▼
Vec<f32> — сырой выход модели
    │  parse_detections(output, threshold=0.5)
    │  chunks_exact(6): [class, conf, x, y, w, h]
    │  фильтрация по confidence
    ▼
Vec<MLDetection>
    │  detection_to_command()
    │  confidence → priority (×255)
    ▼
UclCommand::InjectToken → Gateway → AxiomEngine
```

---

## Что проверяет Guardian

```rust
Guardian::validate_ml_confidence(confidence, threshold) -> bool
```

| Ситуация | Результат | Почему |
|----------|-----------|--------|
| `confidence = 0.3`, threshold `0.5` | `false` | слишком низкая уверенность |
| `confidence = 0.9`, threshold `0.5` | `true` | в норме |
| `confidence = 0.999` | `false` | **adversarial defense** — реальные модели не дают > 0.99 на неизвестных классах; если дают, это атака или мусор |

Валидный диапазон: `[threshold, 0.99]` включительно.

---

## Что ещё нужно для production

Текущий `MLEngine::Real` — скелет, в нём `input_size: 0` и `output_size: 0`
(захардкожены). Для реального использования нужно будет прочитать форму из
`model.input_fact(0)` после загрузки. Это заготовка на Этап 12+.
