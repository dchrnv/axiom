#!/bin/bash
# Проверка циклических зависимостей в workspace

echo "🔍 Checking for cyclic dependencies..."

if command -v jq &> /dev/null; then
    if cargo metadata --format-version 1 2>/dev/null | jq '.resolve' > /dev/null 2>&1; then
        echo "✓ No cyclic dependencies found"
        exit 0
    else
        echo "✗ Cyclic dependencies detected or metadata error!"
        exit 1
    fi
else
    echo "⚠️  jq not installed, using simple cargo check"
    if cargo check --workspace 2>&1 | grep -i "cyclic"; then
        echo "✗ Cyclic dependencies detected!"
        exit 1
    else
        echo "✓ No obvious cyclic dependencies found"
        exit 0
    fi
fi
