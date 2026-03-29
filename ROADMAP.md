# Axiom Roadmap

**Версия:** 14.0
**Дата:** 2026-03-29
**Спека:** [Roadmap_V3_0.md](Roadmap_V3_0.md)

---

## Сводка завершённых этапов

| Этап | Название | Статус |
|------|----------|--------|
| 1 | GENOME | ✅ 426 тестов |
| 2 | Storm Control | ✅ 430 тестов |
| 3 | Configuration System | ✅ 469 тестов |
| 4 | REFLECTOR + SKILLSET | ✅ 496 тестов |
| 5 | GridHash | ✅ 519 тестов |
| 6 | Адаптивные пороги | ✅ 533 тестов |
| 7 | Causal Horizon + Memory | ✅ 568 тестов |
| 8 | Gateway + Channel | ✅ 590 тестов |
| 9 | Tech Debt + Event Bus | ✅ 629 тестов |
| 10 | Agent Layer | ✅ 674 тестов |

Технический долг и будущие планы: [DEFERRED.md](DEFERRED.md)

---

## Этап 11: ML Inference (Восприятие)

**Цель:** AXIOM видит и слышит через нейросети (ONNX, tract).

**Зависимости:** Этап 10 (Perceptor/Effector архитектура готова).

### 11A. ML Engine wrapper

`axiom-agent/src/ml/engine.rs`:
- Обёртка над `tract-onnx` (чистый Rust, no Python)
- `MLEngine::load(path: &Path) -> Result<Self, MLError>`
- `MLEngine::infer(&[f32]) -> Result<Vec<f32>, MLError>` — синхронный вызов
- `MLEngine::input_shape() -> &[usize]` — ожидаемая форма входного тензора
- Mock-режим для тестов без реальных ONNX файлов

### 11B. VisionPerceptor

`axiom-agent/src/channels/vision.rs`:
- Источник: файл изображения (через `image` crate)
- Модель: ONNX (подключается через MLEngine)
- Выход: токены объектов → `InjectToken` в LOGIC(106) или MAP(104)
- Один объект = один Token: `temperature` = confidence × 255, `position` = bbox center
- `VisionPerceptor::from_image_file(path)` — тестируется без камеры

### 11C. AudioPerceptor

`axiom-agent/src/channels/audio.rs`:
- Источник: аудиофайл (WAV через `hound` crate)
- VAD: энергетический порог (без ONNX зависимости для VAD)
- Выход: обнаруженная речь → токен в SUTRA(100)
- `AudioPerceptor::from_wav_file(path)` — тестируется без микрофона

### 11D. GUARDIAN фильтры для ML

Добавить в `axiom-runtime/src/guardian.rs`:
- `validate_ml_confidence(confidence: f32, threshold: f32) -> bool`
- Отклонение при `confidence < threshold` (по умолчанию 0.5)
- Adversarial defense: отклонение при `confidence > 0.99` (аномально высокое)

**Тесты:** mock инференс, токенизация результата, GUARDIAN фильтрация по порогу.
**Критерий:** MLEngine инициализируется и принимает f32-тензоры. GUARDIAN отсекает низкокачественный вывод.

---

## Этап 12: Фракталы и SIMD

**Цель:** Многоуровневая обработка. Максимальная производительность физики.

**Зависимости:** 8-11 (система должна быть стабильна при долгих запусках).

### 12A. Протокол 10→0 (Фрактальные уровни)

`crates/axiom-domain/src/fractal_chain.rs`:
- `FractalChain`: связывает два `AshtiCore` — `maya_output → sutra_input`
- `AshtiCore::set_sutra_input(token: Token)` — внешний впрыск в SUTRA(100)
- `AshtiCore::take_maya_output() -> Option<Token>` — забрать результат с MAYA(110)
- `FractalChain::tick()` — шаг по всей цепочке
- Обмен SkillSet между уровнями через `export/import_batch`

**Тесты:** двухуровневая цепочка, данные проходят от SUTRA-1 до MAYA-2, навыки переносятся.

### 12B. SIMD-оптимизация физики поля

`crates/axiom-space/src/simd.rs`:
- AVX2 / SSE4.2 batch-обработка позиций токенов
- Векторизованный `apply_gravity` для N токенов за раз
- Feature flag `simd` — включается через `RUSTFLAGS="-C target-cpu=native"`
- Scalar fallback при отсутствии поддержки

**Бенчмарк:** SpatialHashGrid rebuild с SIMD vs scalar на 5000 токенов.
**Критерий:** SIMD-путь не нарушает детерминизм. 2x+ ускорение физики поля.

---

## Граф зависимостей

```
9 (Tech Debt) ──→ 10 (Agent) ──→ 11 (ML) ──→ 12 (Фракталы)
                      │
                 axiom-agent
               (tokio, reqwest)
```

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
