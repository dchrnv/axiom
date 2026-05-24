# AXIOM Workspace — команды проверки и сборки
# Использование: just <команда>

# Запуск (production: axiom-node раздаёт dist/ на :8080)
run:
    ./run.sh

# Запуск в dev-режиме (axiom-node :8080 + npm run dev :5173)
dev:
    ./run.sh --dev

# Сборка + запуск (принудительная пересборка axiom-node и axiom-web)
run-build:
    ./run.sh --build

# Запуск с Grafana + Prometheus
run-grafana:
    ./run.sh --grafana

# Полная проверка workspace
check:
    cargo test --workspace
    cargo clippy --workspace -- -D warnings

# Тест конкретного crate
test crate:
    cargo test -p {{crate}}

# Все тесты
test-all:
    cargo test --workspace

# Clippy для всего workspace
clippy:
    cargo clippy --workspace -- -D warnings

# Проверка size assertions
size-check:
    cargo test --workspace -- size_assertion

# Бенчмарки (когда будут добавлены)
bench:
    cargo bench -p axiom-space

# Визуализация графа зависимостей
deps-graph:
    ./tools/visualize_deps.sh

# Проверка циклических зависимостей
deps-check:
    ./tools/check_deps.sh

# Билд всего workspace
build:
    cargo build --workspace

# Билд в release
build-release:
    cargo build --workspace --release

# Очистка
clean:
    cargo clean
