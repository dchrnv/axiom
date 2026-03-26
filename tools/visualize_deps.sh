#!/bin/bash
# Генерация графа зависимостей

echo "📊 Generating dependency graph..."

# Проверка наличия cargo-deps
if ! command -v cargo-deps &> /dev/null; then
    echo "⚠️  cargo-deps not installed"
    echo "Install with: cargo install cargo-deps"
    echo "Also requires graphviz: sudo pacman -S graphviz (or apt/brew install graphviz)"
    exit 1
fi

# Проверка наличия dot (graphviz)
if ! command -v dot &> /dev/null; then
    echo "⚠️  graphviz (dot) not installed"
    echo "Install with: sudo pacman -S graphviz (or apt/brew install graphviz)"
    exit 1
fi

# Создание директории для графов
mkdir -p docs/architecture

# Генерация SVG графа
cargo deps --all-deps | dot -Tsvg > docs/architecture/dependency_graph.svg

if [ -f docs/architecture/dependency_graph.svg ]; then
    echo "✓ Graph saved to docs/architecture/dependency_graph.svg"
    echo "  Open it in a browser to view"
else
    echo "✗ Failed to generate graph"
    exit 1
fi
