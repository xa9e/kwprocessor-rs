#!/usr/bin/env bash
set -euo pipefail

cargo build --release

BASE=${1:-basechars/full.base}
KEYMAP=${2:-keymaps/en-us.keymap}
ROUTES=${3:-routes/2-to-10-max-3-direction-changes.route}
OUT=${4:-target/kwp-rs.out}

time -p ./target/release/kwp-rs "$BASE" "$KEYMAP" "$ROUTES" > "$OUT"
wc -l "$OUT"
sha256sum "$OUT"
