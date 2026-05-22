# Examples

## Human Keyboard Wordlists

Tracked artifacts:

- `wordlists/human-keyboard-walks-curated.txt` - compact hand-prioritized keyboard patterns.
- `wordlists/human-keyboard-walks-priority.txt.zst` - compressed copy of the broader priority wordlist.

The uncompressed broad wordlist is generated as `target/wordlists/human-keyboard-walks-priority.txt` by default and is intentionally not tracked.

Bundled specs:

- `specs/human-keyboard-v2/basechars/` - practical start-character sets.
- `specs/human-keyboard-v2/routes/` - route tiers used by the builders.
- `specs/human-keyboard-v2/keymaps/en.keymap` - bundled QWERTY keymap for reproducible examples.

Regenerate:

```bash
examples/generators/build_human_keyboard_wordlist.sh
examples/generators/gen_human_keyboard_walks.py -o examples/wordlists/human-keyboard-walks-curated.txt
zstd -T0 -19 -f target/wordlists/human-keyboard-walks-priority.txt -o examples/wordlists/human-keyboard-walks-priority.txt.zst
```

Build a much larger practical list, capped by approximate output size without cutting the final line:

```bash
examples/generators/build_human_keyboard_wordlist_large.sh examples/specs/human-keyboard-v2 target/wordlists/human-keyboard-walks-large-5g.txt 5G
```

The large builder keeps higher-signal material first: saved priority list, common password-policy suffix/prefix edits, T4 walks, T5 cardinal walks, and only then a capped left-hand diagonal T5 tail. Use `10G` as the third argument for a larger run.

It intentionally avoids the noisiest layers such as full-keyboard shift+all-directions+repeat T5. The large output is not globally deduplicated, because external dedupe at 5-10 GiB is usually slower than generating the list and can destroy the intended priority order if done naively.
