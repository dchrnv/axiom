#!/usr/bin/env bash
# Axiom Showcase — orchestrates OBS large-corpus run + benchmarks → SHOWCASE.md
#
# Usage:
#   ./scripts/showcase.sh                   # full run
#   ./scripts/showcase.sh --bench-only      # skip OBS, re-run benchmarks
#   ./scripts/showcase.sh --obs-only        # skip benchmarks
#
# Output: showcase/SHOWCASE.md + raw logs in showcase/

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

SHOWCASE_DIR="showcase"
OBS_OUT="$SHOWCASE_DIR/obs_out"
BENCH_OUT="$SHOWCASE_DIR/bench_out"
REPORT="$SHOWCASE_DIR/SHOWCASE.md"
CORPUS="config/obs/corpus_large.yaml"
ANCHORS="config/anchors"
GENERATED_AT="$(date '+%Y-%m-%d %H:%M')"

RUN_OBS=true
RUN_BENCH=true

for arg in "$@"; do
  case "$arg" in
    --bench-only) RUN_OBS=false ;;
    --obs-only)   RUN_BENCH=false ;;
  esac
done

mkdir -p "$OBS_OUT" "$BENCH_OUT"

# ─── 1. Build ────────────────────────────────────────────────────────────────
echo ""
echo "==> Building release binaries..."
cargo build --release -p axiom-observe -p axiom-bench 2>&1 | tail -5
echo "    Done."

# ─── 2. OBS run ──────────────────────────────────────────────────────────────
if $RUN_OBS; then
  echo ""
  echo "==> Running OBS large corpus (1M ticks)..."
  echo "    corpus: $CORPUS"
  echo "    output: $OBS_OUT"
  echo ""
  ./target/release/axiom-observe \
    "$CORPUS" \
    "$OBS_OUT" \
    "$ANCHORS" \
    2>&1 | tee "$SHOWCASE_DIR/obs_run.log"
  echo ""
  echo "    OBS complete."
fi

# ─── 3. Benchmarks ───────────────────────────────────────────────────────────
if $RUN_BENCH; then
  echo ""
  echo "==> Running benchmarks..."

  echo "    [1/2] hot_path_regression"
  cargo bench --bench hot_path_regression -- --noplot \
    2>&1 | tee "$BENCH_OUT/hot_path.txt"

  echo "    [2/2] over_domain_bench"
  cargo bench --bench over_domain_bench -- --noplot \
    2>&1 | tee "$BENCH_OUT/over_domain.txt"

  echo "    Benchmarks complete."
fi

# ─── 4. Assemble SHOWCASE.md ─────────────────────────────────────────────────
echo ""
echo "==> Assembling $REPORT..."

# Collect hardware / OS info
HW_CPU=$(grep -m1 "model name" /proc/cpuinfo 2>/dev/null | cut -d: -f2 | xargs || echo "unknown")
HW_CORES=$(nproc 2>/dev/null || echo "?")
HW_RAM=$(awk '/MemTotal/ {printf "%.0f GiB", $2/1048576}' /proc/meminfo 2>/dev/null || echo "?")
HW_OS=$(grep PRETTY_NAME /etc/os-release 2>/dev/null | cut -d= -f2 | tr -d '"' || uname -s)
HW_KERNEL=$(uname -r)
HW_RUSTC=$(rustc --version 2>/dev/null || echo "?")

{
  echo "# Axiom — Showcase Report"
  echo ""
  echo "> Generated: $GENERATED_AT  "
  echo "> Engine: V7 (ContextRecognizer V7, NeuralAdvisor V3, DREAM V1.1, FractalChain)  "
  echo "> Corpus: \`$CORPUS\`"
  echo ""
  echo "---"
  echo ""
  echo "## Environment"
  echo ""
  echo "| | |"
  echo "|---|---|"
  echo "| **OS** | $HW_OS |"
  echo "| **Kernel** | $HW_KERNEL |"
  echo "| **CPU** | $HW_CPU |"
  echo "| **Cores** | $HW_CORES |"
  echo "| **RAM** | $HW_RAM |"
  echo "| **Rust** | $HW_RUSTC |"
  echo ""
  echo "---"
  echo ""

  # ── OBS section ──────────────────────────────────────────────────────────
  if [ -f "$OBS_OUT/report.md" ]; then
    echo "## OBS — Live Corpus Run"
    echo ""
    cat "$OBS_OUT/report.md"
    echo ""
    echo "---"
    echo ""
  else
    echo "## OBS — results not available (run without --bench-only)"
    echo ""
    echo "---"
    echo ""
  fi

  # ── Bench section ─────────────────────────────────────────────────────────
  echo "## Benchmark Results"
  echo ""
  echo "All measurements: release build, Criterion 0.5, $HW_OS, $(uname -m)."
  echo ""

  if [ -f "$BENCH_OUT/hot_path.txt" ]; then
    echo "### Hot Path Regression (TickForward / 50 tokens)"
    echo ""
    echo "\`\`\`"
    grep -E "time:|thrpt:|change:" "$BENCH_OUT/hot_path.txt" | head -20 || true
    echo "\`\`\`"
    echo ""
  fi

  if [ -f "$BENCH_OUT/over_domain.txt" ]; then
    echo "### Over-Domain Layer (V7 pipeline)"
    echo ""
    echo "\`\`\`"
    grep -E "time:|thrpt:|change:" "$BENCH_OUT/over_domain.txt" | head -40 || true
    echo "\`\`\`"
    echo ""
  fi

  echo "---"
  echo ""

  # ── Summary ───────────────────────────────────────────────────────────────
  echo "## Summary"
  echo ""

  if [ -f "$OBS_OUT/report.md" ]; then
    TICKS=$(grep -oE "ticks_total: [0-9]+" "$CORPUS" | awk '{print $2}' || echo "?")
    TEXTS=$(grep -c "^  - id:" "$CORPUS" || echo "?")
    echo "| Parameter | Value |"
    echo "|-----------|-------|"
    echo "| Engine ticks | $TICKS |"
    echo "| Corpus texts | $TEXTS |"
    echo "| Subsystems covered | mathematics · writing · logic · music · time · values · morality · abstractions · dilemmas |"
    echo ""
  fi

  echo "Criterion HTML reports: \`target/criterion/\`  "
  echo "Raw bench logs: \`$BENCH_OUT/\`  "
  echo "OBS snapshots: \`$OBS_OUT/\`  "

} > "$REPORT"

echo ""
echo "==> Done."
echo "    Report: $REPORT"
echo ""
