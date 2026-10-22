#!/usr/bin/env bash
# Compares row-major, column-major and tiled (block-major) timings for 90 and
# 180 degree rotations.
#
# Usage: ./bench.sh [image] [runs]
#   image  PPM file to rotate (default: ../mallory.ppm)
#   runs   how many times to time each case (default: 5)
#
# Cases run one after another, never in parallel: concurrent runs would
# compete for the same caches and memory bandwidth and skew the timings.


# Had CLAUDE build a script to run the benchmarks and compare timings

set -euo pipefail

cd "$(dirname "$0")"

IMAGE="${1:-../mallory.ppm}"        # put in file .ppm here
RUNS="${2:-5}"
BIN=./target/release/ppmtrans

if [[ ! -f "$IMAGE" ]]; then
    echo "Image not found: $IMAGE" >&2
    exit 1
fi

echo "Building release binary..."
cargo build --release -q

CASES=(
    "-r 90 --row-major"
    "-r 90 --col-major"
    "-r 90 --block-major"
    "-r 180 --row-major"
    "-r 180 --col-major"
    "-r 180 --block-major"
)

# Prints the transformation time in ms reported by one run of ppmtrans.
time_once() {
    # stdout (the image) is discarded; the timing line is on stderr.
    # shellcheck disable=SC2086  # $1 is intentionally word-split into flags
    "$BIN" $1 "$IMAGE" 2>&1 >/dev/null |
        sed -n 's/.*took \([0-9.]*\) ms.*/\1/p'
}

echo "Image: $IMAGE   Runs per case: $RUNS"
echo
printf "%-22s %10s %10s %10s\n" "case" "min (ms)" "avg (ms)" "max (ms)"
printf "%-22s %10s %10s %10s\n" "----" "--------" "--------" "--------"

for c in "${CASES[@]}"; do
    # One untimed warm-up run so the image file is in the OS page cache.
    time_once "$c" >/dev/null
    for ((i = 0; i < RUNS; i++)); do
        time_once "$c"
    done | awk -v name="$c" '
        { t = $1; sum += t; if (NR == 1 || t < min) min = t; if (t > max) max = t }
        END { printf "%-22s %10.2f %10.2f %10.2f\n", name, min, sum / NR, max }'
done
