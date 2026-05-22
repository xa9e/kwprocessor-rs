#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
SPEC_ROOT="$ROOT/examples/specs/human-keyboard-v2"
OUT=target/kwp-v2-rust-quick.out

while (($#)); do
  case "$1" in
    --spec-root)
      [[ $# -ge 2 ]] || { echo "missing value for --spec-root" >&2; exit 2; }
      SPEC_ROOT=$2
      shift 2
      ;;
    --out)
      [[ $# -ge 2 ]] || { echo "missing value for --out" >&2; exit 2; }
      OUT=$2
      shift 2
      ;;
    *)
      if [[ "$SPEC_ROOT" == "$ROOT/examples/specs/human-keyboard-v2" ]]; then
        SPEC_ROOT=$1
      elif [[ "$OUT" == "target/kwp-v2-rust-quick.out" ]]; then
        OUT=$1
      else
        echo "unexpected argument: $1" >&2
        exit 2
      fi
      shift
      ;;
  esac
done

KWP="$ROOT/target/release/kwp-rs"
KEYMAP="${KEYMAP:-$SPEC_ROOT/keymaps/en.keymap}"
BD="$SPEC_ROOT/basechars"
RD="$SPEC_ROOT/routes"

cargo build --release --manifest-path "$ROOT/Cargo.toml" >/dev/null

run() {
  "$KWP" "$@"
}

time -p {
  # TIER 1: straight walks.
  run -B 1                  "$BD/home_left_basic.base" "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1 -S 1             "$BD/home_left_full.base"  "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1                  "$BD/num_left_basic.base"  "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1 -S 1             "$BD/num_left_full.base"   "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1                  "$BD/left_basic.base"      "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1 -S 1             "$BD/left_full.base"       "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1                  "$BD/right_basic.base"     "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1                  "$BD/full_basic.base"      "$KEYMAP" "$RD/tier1_straight.route"
  run -B 1 -S 1             "$BD/full_all.base"        "$KEYMAP" "$RD/tier1_straight.route"

  # TIER 2: one-turn walks.
  run -B 1                                  "$BD/left_basic.base"  "$KEYMAP" "$RD/tier2_one_turn.route"
  run -B 1 -1 1 -3 1 -7 1 -9 1             "$BD/left_basic.base"  "$KEYMAP" "$RD/tier2_one_turn.route"
  run -B 1 -S 1                             "$BD/left_full.base"   "$KEYMAP" "$RD/tier2_one_turn.route"
  run -B 1 -S 1 -1 1 -3 1 -7 1 -9 1        "$BD/left_full.base"   "$KEYMAP" "$RD/tier2_one_turn.route"
  run -B 1                                  "$BD/right_basic.base" "$KEYMAP" "$RD/tier2_one_turn.route"
  run -B 1 -1 1 -3 1 -7 1 -9 1             "$BD/right_basic.base" "$KEYMAP" "$RD/tier2_one_turn.route"
  run -B 1                                  "$BD/full_basic.base"  "$KEYMAP" "$RD/tier2_one_turn.route"
  run -B 1 -1 1 -3 1 -7 1 -9 1             "$BD/full_basic.base"  "$KEYMAP" "$RD/tier2_one_turn.route"

  # TIER 3: two-turn walks.
  run -B 1                                  "$BD/left_basic.base"  "$KEYMAP" "$RD/tier3_two_turns.route"
  run -B 1 -1 1 -3 1 -7 1 -9 1             "$BD/left_basic.base"  "$KEYMAP" "$RD/tier3_two_turns.route"
  run -B 1 -S 1 -1 1 -3 1 -7 1 -9 1        "$BD/left_full.base"   "$KEYMAP" "$RD/tier3_two_turns.route"
  run -B 1                                  "$BD/right_basic.base" "$KEYMAP" "$RD/tier3_two_turns.route"
  run -B 1 -1 1 -3 1 -7 1 -9 1             "$BD/right_basic.base" "$KEYMAP" "$RD/tier3_two_turns.route"
  run -B 1                                  "$BD/full_basic.base"  "$KEYMAP" "$RD/tier3_two_turns.route"
  run -B 1 -1 1 -3 1 -7 1 -9 1             "$BD/full_basic.base"  "$KEYMAP" "$RD/tier3_two_turns.route"

  # TIER 3.5: sawtooth walks.
  run -B 1                                  "$BD/left_basic.base"     "$KEYMAP" "$RD/special_sawtooth.route"
  run -B 1 -1 1 -3 1 -7 1 -9 1             "$BD/left_basic.base"     "$KEYMAP" "$RD/special_sawtooth.route"
  run -B 1 -S 1                             "$BD/left_full.base"      "$KEYMAP" "$RD/special_sawtooth.route"
  run -B 1                                  "$BD/num_left_basic.base" "$KEYMAP" "$RD/special_sawtooth.route"
  run -B 1 -S 1                             "$BD/num_left_full.base"  "$KEYMAP" "$RD/special_sawtooth.route"
  run -B 1                                  "$BD/full_basic.base"     "$KEYMAP" "$RD/special_sawtooth.route"
} > "$OUT"

wc -l "$OUT"
sha256sum "$OUT"
