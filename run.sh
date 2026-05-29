#!/usr/bin/env bash
# AXIOM Workstation V2 — запуск одной командой
#
#   ./run.sh           production: axiom-node раздаёт dist/ на :8080
#   ./run.sh --dev     dev:        axiom-node :8080 + npm run dev :5173
#   ./run.sh --build   принудительная пересборка перед запуском
#   ./run.sh --grafana запустить Grafana + Prometheus (docker compose)

set -euo pipefail

# nvm не загружается в non-interactive bash — подгружаем вручную
export NVM_DIR="${NVM_DIR:-$HOME/.var/app/com.vscodium.codium/config/nvm}"
[[ -s "$NVM_DIR/nvm.sh" ]] && source "$NVM_DIR/nvm.sh"

DEV=0
BUILD=0
GRAFANA=0

for arg in "$@"; do
    case $arg in
        --dev)     DEV=1 ;;
        --build)   BUILD=1 ;;
        --grafana) GRAFANA=1 ;;
    esac
done

BIN_NODE="target/release/axiom-node"
WEB_DIR="tools/axiom-web"
DIST_DIR="$WEB_DIR/dist"

# ── npm install если нужно ──────────────────────────────────────────────────
ensure_npm() {
    if [[ ! -d "$WEB_DIR/node_modules" ]]; then
        echo "[axiom] npm install..."
        (cd "$WEB_DIR" && npm install --silent)
    fi
}

# ── сборка axiom-node ───────────────────────────────────────────────────────
if [[ $BUILD -eq 1 || ! -f "$BIN_NODE" ]]; then
    echo "[axiom] building axiom-node..."
    cargo build -p axiom-node --release
fi

# ── Grafana (опционально) ───────────────────────────────────────────────────
if [[ $GRAFANA -eq 1 ]]; then
    if ! command -v docker &>/dev/null; then
        echo "[axiom] error: docker not found — install Docker to use --grafana"
        echo "[axiom]   Arch: sudo pacman -S docker && sudo systemctl enable --now docker"
        echo "[axiom]   Or:   https://docs.docker.com/engine/install/"
        exit 1
    fi
    echo "[axiom] starting Grafana + Prometheus..."
    (cd tools/grafana && docker compose up -d)
    echo "[axiom] → Grafana: http://localhost:3000"
fi

# ── dev режим ───────────────────────────────────────────────────────────────
if [[ $DEV -eq 1 ]]; then
    ensure_npm

    echo "[axiom] dev mode"
    echo "[axiom] → http://localhost:5173  (hot reload)"
    echo "[axiom] → http://127.0.0.1:8080  (API)"
    echo ""

    "$BIN_NODE" &
    NODE_PID=$!

    cleanup() {
        echo ""
        echo "[axiom] stopping..."
        kill "$NODE_PID" 2>/dev/null || true
        wait "$NODE_PID" 2>/dev/null || true
    }
    trap cleanup EXIT INT TERM

    (cd "$WEB_DIR" && npm run dev)
    exit 0
fi

# ── production режим ────────────────────────────────────────────────────────
if [[ $BUILD -eq 1 || ! -d "$DIST_DIR" ]]; then
    ensure_npm
    echo "[axiom] building axiom-web..."
    (cd "$WEB_DIR" && npm run build --silent)
fi

echo "[axiom] starting axiom-node..."
echo "[axiom] → http://127.0.0.1:8080"
echo ""
exec "$BIN_NODE"
