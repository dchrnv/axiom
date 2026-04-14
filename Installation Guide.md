# Axiom — Installation & Setup Guide

**Версия:** 1.0
**Дата:** 2026-04-14

---

## Требования

| Компонент | Минимум | Рекомендуется |
|-----------|---------|---------------|
| OS | Linux | Arch Linux |
| Rust | 1.75 | stable latest |
| RAM | 512 MB | 2 GB+ |
| CPU | x86_64 | любой, SIMD-опционален |

> Совместимость с Windows и macOS не тестировалась. Может заработать. Может нет.

---

## 1. Установка Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustc --version
```

---

## 2. Клонирование репозитория

```bash
git clone https://github.com/dchrnv/axiom.git
cd axiom
```

---

## 3. Сборка

```bash
# Debug (быстрая сборка, медленный запуск)
cargo build --workspace

# Release (медленная сборка, быстрый запуск — рекомендуется для работы)
cargo build --release
```

### Опциональные фичи

```bash
# С поддержкой ONNX-моделей (ML inference)
cargo build --release -p axiom-agent --features onnx

# С SIMD-физикой (автоматическая векторизация)
cargo build --release -p axiom-space --features simd
```

---

## 4. Тесты

```bash
# Все тесты (ожидаемый результат: 932 passing, 0 failures)
cargo test --workspace

# Конкретный crate
cargo test -p axiom-runtime

# Конкретный тест
cargo test -p axiom-runtime test_inject_anchor_tokens_axes

# С выводом println! (для отладки)
cargo test --workspace -- --nocapture
```

---

## 5. Запуск CLI

```bash
# Базовый запуск
cargo run --bin axiom-cli --release

# С флагами
cargo run --bin axiom-cli --release -- --verbose
cargo run --bin axiom-cli --release -- --adaptive        # адаптивная частота тиков
cargo run --bin axiom-cli --release -- --detail max      # подробный вывод
cargo run --bin axiom-cli --release -- --no-load         # чистый старт (игнорировать axiom-data/)
cargo run --bin axiom-cli --release -- --tick-hz 500     # 500 Гц вместо 100
cargo run --bin axiom-cli --release -- --data-dir /tmp/test  # нестандартная директория данных
```

---

## 6. Конфигурация

### axiom-cli.yaml (опционально)

Создайте в рабочей директории:

```yaml
# axiom-cli.yaml
tick_hz: 100
verbose: false
adaptive_tick_rate: false
detail_level: min      # off | min | mid | max
prompt: "axiom> "
hot_reload: false      # следить за config/axiom.yaml

tick_schedule:
  tension_check_interval: 10
  goal_check_interval: 10
  adaptation_interval: 50
  horizon_gc_interval: 500
  dream_interval: 100
  reconcile_interval: 200
  persist_check_interval: 1000   # 0 = autosave выкл

  adaptive_min_hz: 60
  adaptive_max_hz: 1000
  adaptive_step_up: 200
  adaptive_step_down: 20
  adaptive_cooldown: 50
```

### config/ (опционально)

Директория конфигурации движка:

```
config/
├── axiom.yaml          # Основной конфиг (runtime, presets)
├── genome.yaml         # Genome конституция (AccessRules, ProtocolRules)
├── anchors/
│   ├── axes.yaml       # 6 осевых якорей — семантика X/Y/Z
│   ├── layers/
│   │   └── L5_cognitive.yaml   # Якоря когнитивного слоя
│   └── domains/
│       └── D1_execution.yaml   # Якоря домена EXECUTION
└── presets/            # YAML-пресеты токенов и связей
```

Якоря загружаются автоматически при старте CliChannel. Если `config/anchors/` не найден — система работает с FNV-1a fallback.

---

## 7. Структура данных

Персистентное состояние хранится в `./axiom-data/` (bincode):

```
axiom-data/
├── engine_state.bin      # Полное состояние движка (tokens + connections + experience)
└── memory_manifest.yaml  # Метаданные: tick_count, traces, tokens
```

```bash
# Сброс состояния
rm -rf axiom-data/
cargo run --bin axiom-cli --release -- --no-load

# Сохранить состояние вручную
axiom> :save
axiom> :save /path/to/backup/

# Загрузить
axiom> :load /path/to/backup/
```

---

## 8. Бенчмарки

```bash
cargo bench -p axiom-bench
```

Результаты: [docs/bench/RESULTS.md](docs/bench/RESULTS.md)

---

## 9. Типичные проблемы

**Ошибка сборки — устаревшие зависимости:**
```bash
cargo clean && cargo build --release
```

**Конфликт версий Rust:**
```bash
rustup update stable
```

**Не загружается axiom-data/ при старте:**
```
load failed: ...
```
Используйте `--no-load` для чистого старта. Данные могут быть несовместимы с новой версией (bincode формат).

**ONNX-модель не загружается:**
Требуется фича `onnx` и наличие `.onnx` файла. По умолчанию MLEngine работает в mock-режиме.

---

## 10. Дерево crates

```
axiom-core        — Token (64B), Connection (64B), Event (64B)
axiom-genome      — Genome, AccessRules, ProtocolRules
axiom-config      — DomainConfig, ConfigLoader, AnchorSet, JsonSchema
axiom-space       — SpatialHashGrid, физика, SIMD-ready
axiom-shell       — Shell V3.0, семантические профили
axiom-frontier    — CausalFrontier V2.0, Storm Control
axiom-arbiter     — Arbiter, Experience, Reflector, SkillSet
axiom-heartbeat   — Heartbeat V2.0
axiom-upo         — UPO v2.2
axiom-ucl         — UCL команды (UclCommand, OpCode)
axiom-domain      — Domain, AshtiCore, FractalChain
axiom-runtime     — AxiomEngine, Guardian, Gateway, Channel
axiom-agent       — CLI, TextPerceptor, MLEngine
axiom-persist     — save/load/autosave (bincode)
axiom-bench       — Criterion бенчмарки
```
