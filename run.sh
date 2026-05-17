#!/usr/bin/env bash
# Axiom — запуск node + workstation одной командой
# Использование: ./run.sh [--build] [--release]
set -e

PROFILE="release"
BUILD=0

for arg in "$@"; do
    case $arg in
        --build) BUILD=1 ;;
        --debug) PROFILE="debug" ;;
    esac
done

if [[ $BUILD -eq 1 ]]; then
    echo "[axiom] building..."
    cargo build -p axiom-node -p axiom-workstation --release
fi

BIN_NODE="target/${PROFILE}/axiom-node"
BIN_WS="target/${PROFILE}/axiom-workstation"

if [[ ! -f "$BIN_NODE" || ! -f "$BIN_WS" ]]; then
    echo "[axiom] binaries not found, building..."
    cargo build -p axiom-node -p axiom-workstation --release
fi

echo "[axiom] starting node..."
"$BIN_NODE" &
NODE_PID=$!

# Ждём пока node откроет порт
for i in $(seq 1 20); do
    if grep -q "2694" /proc/net/tcp 2>/dev/null; then
        break
    fi
    sleep 0.2
done

echo "[axiom] starting workstation..."
"$BIN_WS"

echo "[axiom] workstation closed, stopping node (pid $NODE_PID)..."
kill "$NODE_PID" 2>/dev/null || true
wait "$NODE_PID" 2>/dev/null || true
