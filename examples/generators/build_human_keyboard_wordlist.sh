#!/usr/bin/env bash
set -euo pipefail

EXAMPLES_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=$(cd -- "$EXAMPLES_DIR/.." && pwd)
SPEC_ROOT=${1:-"$EXAMPLES_DIR/specs/human-keyboard-v2"}
OUT=${2:-target/wordlists/human-keyboard-walks-priority.txt}
WORK_DIR=$(mktemp -d)
PARALLEL=${PARALLEL:-$(nproc)}
SORT_MEM=${SORT_MEM:-50%}

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

mkdir -p "$WORK_DIR"
mkdir -p "$(dirname -- "$OUT")"

CURATED="$WORK_DIR/curated.txt"
BROAD="$WORK_DIR/broad.txt"
MERGED="$WORK_DIR/merged-numbered.tsv"
FIRSTS="$WORK_DIR/firsts.tsv"

python3 "$EXAMPLES_DIR/generators/gen_human_keyboard_walks.py" -o "$CURATED"
"$ROOT/bench_v2_quick.sh" "$SPEC_ROOT" "$BROAD"

awk '{ print NR "\t" $0 }' "$CURATED" "$BROAD" > "$MERGED"

LC_ALL=C sort --parallel="$PARALLEL" -S "$SORT_MEM" -t $'\t' -k2,2 -k1,1n "$MERGED" \
  | awk -F '\t' '$2 != prev { print $1 "\t" $2; prev = $2 }' \
  > "$FIRSTS"

LC_ALL=C sort --parallel="$PARALLEL" -S "$SORT_MEM" -n -t $'\t' -k1,1 "$FIRSTS" \
  | cut -f2- \
  > "$OUT"

wc -l "$CURATED" "$BROAD" "$OUT"
du -h "$OUT"
sha256sum "$OUT"
