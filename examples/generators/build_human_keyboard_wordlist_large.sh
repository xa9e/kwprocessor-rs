#!/usr/bin/env bash
set -euo pipefail

EXAMPLES_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=$(cd -- "$EXAMPLES_DIR/.." && pwd)
SPEC_ROOT=${1:-"$EXAMPLES_DIR/specs/human-keyboard-v2"}
OUT=${2:-target/wordlists/human-keyboard-walks-large.txt}
TARGET_SIZE=${3:-5G}
WORK_DIR=$(mktemp -d)
POLICY_SOURCE_LINES=${POLICY_SOURCE_LINES:-2000000}
COMPRESS=${COMPRESS:-0}
MIN_LINE_BYTES=3

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

to_bytes() {
  python3 - "$1" <<'PY'
import re
import sys

value = sys.argv[1].strip()
match = re.fullmatch(r"([0-9]+)([KkMmGgTt]?[Bb]?)?", value)
if not match:
    raise SystemExit(f"invalid size: {value}")

num = int(match.group(1))
unit = (match.group(2) or "").lower().rstrip("b")
scale = {"": 1, "k": 1024, "m": 1024**2, "g": 1024**3, "t": 1024**4}[unit]
print(num * scale)
PY
}

TARGET_BYTES=$(to_bytes "$TARGET_SIZE")
KWP="$ROOT/target/release/kwp-rs"
KEYMAP="${KEYMAP:-$SPEC_ROOT/keymaps/en.keymap}"
BD="$SPEC_ROOT/basechars"
RD="$SPEC_ROOT/routes"
PRIORITY="$EXAMPLES_DIR/wordlists/human-keyboard-walks-priority.txt"

cargo build --release --manifest-path "$ROOT/Cargo.toml" >/dev/null

mkdir -p "$(dirname -- "$OUT")"
: > "$OUT"

bytes_written() {
  stat -c '%s' "$OUT"
}

remaining_bytes() {
  local current
  current=$(bytes_written)
  if (( current >= TARGET_BYTES )); then
    echo 0
  elif (( TARGET_BYTES - current < MIN_LINE_BYTES )); then
    echo 0
  else
    echo $((TARGET_BYTES - current))
  fi
}

log_status() {
  local label="$1"
  printf '# %-42s %12s / %s\n' "$label" "$(numfmt --to=iec --suffix=B "$(bytes_written)")" "$TARGET_SIZE" >&2
}

append_limited_stdin() {
  local label="$1"
  local remaining
  remaining=$(remaining_bytes)
  if (( remaining <= 0 )); then
    return 0
  fi

  awk -v limit="$remaining" '
    BEGIN { used = 0 }
    {
      line = $0 "\n"
      len = length(line)
      if (used + len > limit) exit 0
      printf "%s", line
      used += len
    }
  ' >> "$OUT"

  log_status "$label"
}

append_file() {
  local label="$1"
  local file="$2"
  append_limited_stdin "$label" < "$file"
}

append_kwp() {
  local label="$1"
  local opts="$2"
  local base="$3"
  local route="$4"
  local -a opt_argv=()
  if [[ -n "$opts" ]]; then
    read -r -a opt_argv <<< "$opts"
  fi

  local remaining
  remaining=$(remaining_bytes)
  if (( remaining <= 0 )); then
    return 0
  fi

  local err_file="$WORK_DIR/kwp-stderr.$$"
  set +o pipefail
  "$KWP" "${opt_argv[@]}" "$base" "$KEYMAP" "$route" 2>"$err_file" | append_limited_stdin "$label"
  set -o pipefail

  if [[ -s "$err_file" ]]; then
    local unexpected
    unexpected=$(grep -v -F 'Broken pipe (os error 32)' "$err_file" || true)
    if [[ -n "$unexpected" ]]; then
      printf '%s\n' "$unexpected" >&2
      return 1
    fi
  fi
}

append_policy_expansions() {
  local label="$1"
  local source="$2"
  local remaining
  remaining=$(remaining_bytes)
  if (( remaining <= 0 )); then
    return 0
  fi

  set +o pipefail
  head -n "$POLICY_SOURCE_LINES" "$source" \
    | awk '
      BEGIN {
        split("1 12 123 1234 12345 ! !! @ # 2024 2025 2026", suffixes, " ")
      }
      length($0) >= 4 && length($0) <= 12 {
        for (i = 1; i <= length(suffixes); i++) {
          print $0 suffixes[i]
          print suffixes[i] $0
        }
      }
    ' \
    | append_limited_stdin "$label"
  set -o pipefail
}

if [[ ! -s "$PRIORITY" ]]; then
  PRIORITY="$WORK_DIR/human-keyboard-walks-priority.txt"
  "$EXAMPLES_DIR/generators/build_human_keyboard_wordlist.sh" "$SPEC_ROOT" "$PRIORITY" >/dev/null
fi

log_status "start"

# Highest-value material first: compact curated + broad T1-T3.5 list.
append_file "priority wordlist" "$PRIORITY"

# Common human policy edits. Kept bounded and sourced from the best early rows.
append_policy_expansions "policy suffix/prefix expansions" "$PRIORITY"

# More complex, still human-shaped walks.
append_kwp "T4 left cardinal" "-B 1" "$BD/left_basic.base" "$RD/tier4_three_turns.route"
append_kwp "T4 left diagonals" "-B 1 -1 1 -3 1 -7 1 -9 1" "$BD/left_basic.base" "$RD/tier4_three_turns.route"
append_kwp "T4 full cardinal" "-B 1" "$BD/full_basic.base" "$RD/tier4_three_turns.route"

# Large but still keyboard-walk-native layers.
append_kwp "T5 left cardinal" "-B 1" "$BD/left_basic.base" "$RD/tier5_complex.route"
append_kwp "T5 full cardinal" "-B 1" "$BD/full_basic.base" "$RD/tier5_complex.route"

# Tail filler: broad left-hand diagonal walks. This is capped by TARGET_SIZE.
append_kwp "T5 left diagonals capped" "-B 1 -1 1 -3 1 -7 1 -9 1" "$BD/left_basic.base" "$RD/tier5_complex.route"

wc -l "$OUT"
du -h "$OUT"
sha256sum "$OUT"

if [[ "$COMPRESS" == "1" ]]; then
  zstd -T0 -19 -f "$OUT" -o "$OUT.zst"
  ls -lh "$OUT" "$OUT.zst"
fi
