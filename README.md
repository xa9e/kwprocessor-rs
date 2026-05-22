# kwp-rs

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust rewrite of [kwprocessor](https://github.com/hashcat/kwprocessor), a keyboard-walk wordlist generator.

The default generator is optimized for practical use: it walks valid keyboard transitions first and avoids enumerating large invalid keyspaces. If you need byte-for-byte output order compatibility with the original C implementation, use `--compat-order`.

## Features

- Zero runtime crate dependencies
- UTF-8 output with bundled keymaps, basechars, and routes
- Fast default DFS generator for sparse keyboard walks
- Optional C-compatible output order via `--compat-order`
- Reproducible example wordlists and bundled example specs

## Build

```bash
cargo build --release
```

The binary is written as `target/release/kwp-rs`.

## Usage

```bash
./target/release/kwp-rs basechars/full.base keymaps/en-us.keymap routes/2-to-10-max-3-direction-changes.route > target/kwp.txt
```

Append to a file:

```bash
./target/release/kwp-rs -o target/kwp.txt basechars/full.base keymaps/en-us.keymap routes/2-to-10-max-3-direction-changes.route
```

Preserve original C output order:

```bash
./target/release/kwp-rs --compat-order basechars/full.base keymaps/en-us.keymap routes/2-to-10-max-3-direction-changes.route > target/kwp-c-order.txt
```

## Examples

Generate the bundled human-keyboard quick workload:

```bash
./bench_v2_quick.sh --out target/kwp-v2-rust-quick.out
```

Generate the compact curated wordlist:

```bash
examples/generators/gen_human_keyboard_walks.py
```

Generate the broader priority wordlist:

```bash
examples/generators/build_human_keyboard_wordlist.sh
```

Generate a larger capped wordlist:

```bash
examples/generators/build_human_keyboard_wordlist_large.sh examples/specs/human-keyboard-v2 target/wordlists/human-keyboard-walks-large-5g.txt 5G
```

See [examples/README.md](examples/README.md) for details.

## Benchmarks

Quick benchmark:

```bash
./bench_quick.sh
```

Override inputs:

```bash
./bench_quick.sh basechars/full.base keymaps/en-us.keymap routes/2-to-16-max-3-direction-changes.route target/kwp-rs.txt
```

## Compatibility Checks

The compatibility test compares Rust output with an original C `kwp` checkout. It is optional and requires `KWP_ORIGINAL`:

```bash
KWP_ORIGINAL=/path/to/kwprocessor ./tests/compare_with_c.py
```

## Design Notes

The original C version precomputes a large `wchar_t -> transition map` table. This Rust version builds a compact symbol table only for characters that appear in selected keymaps and filtered base characters, then uses dense symbol indexes for fast transition lookup.

Default hot path properties:

- no `HashMap` lookups while generating candidates;
- no per-candidate heap allocation;
- valid-transition DFS avoids enumerating invalid sparse-walk keyspace;
- compact `selection -> next symbol index` transition table;
- direct UTF-8 byte emission into a fixed stack candidate buffer;
- output batching to reduce write calls.

Compatibility quirks intentionally preserved in `--compat-order` mode:

- route repeat parsing uses the original `hex_convert` arithmetic;
- adjacent route segments reject identical compact `distance+modifier+direction` selections, not merely identical geographic directions;
- base-character modifier filtering preserves the original flag behavior where a layer flag means "this layer lookup was attempted".

## Authors

- **xa9e** - Rust rewrite (2026)

## License

MIT. See [LICENSE](LICENSE) for full text including original author attribution.
