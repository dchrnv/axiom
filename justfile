# AXIOM Workspace — команды проверки и сборки
# Использование: just <команда>

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
