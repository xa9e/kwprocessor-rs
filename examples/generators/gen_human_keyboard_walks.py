#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path


BASIC_ROWS = [
    "`1234567890-=",
    " qwertyuiop[]\\",
    " asdfghjkl;'",
    " zxcvbnm,./",
]

SHIFT_ROWS = [
    "~!@#$%^&*()_+",
    " QWERTYUIOP{}|",
    ' ASDFGHJKL:"',
    " ZXCVBNM<>?",
]

KNOWN_SEEDS = [
    "qwerty",
    "qwertyuiop",
    "asdfgh",
    "asdfghjkl",
    "zxcvbnm",
    "123456",
    "123456789",
    "1234567890",
    "qazwsx",
    "1qaz2wsx",
    "1qazxsw2",
    "zaq12wsx",
    "qweasd",
    "qwerasdf",
    "asdfzxcv",
    "qwertasdfg",
    "1q2w3e",
    "1q2w3e4r",
    "q1w2e3",
    "q2w3e4r",
    "!@#$%^",
    "!QAZ@WSX",
    "QWERTY",
    "ASDFGH",
    "ZXCVBNM",
    "poiuyt",
    "lkjhgf",
    "mnbvcxz",
    "0987654321",
    "plokij",
    "olp;/.",
]

COMMON_SUFFIXES = [
    "1",
    "12",
    "123",
    "1234",
    "!",
    "!!",
    "@",
    "#",
    "2024",
    "2025",
    "2026",
]


class Emitter:
    def __init__(self, min_len: int, max_len: int) -> None:
        self.min_len = min_len
        self.max_len = max_len
        self.seen: set[str] = set()
        self.words: list[str] = []

    def add(self, word: str) -> None:
        if not (self.min_len <= len(word) <= self.max_len):
            return
        if " " in word:
            return
        if word in self.seen:
            return
        self.seen.add(word)
        self.words.append(word)


def make_grid(rows: list[str]) -> dict[tuple[int, int], str]:
    grid = {}
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            if ch != " ":
                grid[(x, y)] = ch
    return grid


def row_chars(row: str) -> str:
    return row.replace(" ", "")


def add_known(emitter: Emitter) -> None:
    seeds = list(KNOWN_SEEDS)
    for seed in seeds:
        emitter.add(seed)
        emitter.add(seed[::-1])
        if seed and seed[0].islower():
            emitter.add(seed.capitalize())
        if seed.islower():
            emitter.add(seed.upper())

    for seed in seeds:
        if len(seed) > 10:
            continue
        variants = [seed]
        if seed and seed[0].islower():
            variants.append(seed.capitalize())
        if seed.islower():
            variants.append(seed.upper())
        for variant in variants:
            for suffix in COMMON_SUFFIXES:
                emitter.add(variant + suffix)
                emitter.add(suffix + variant)


def add_row_substrings(emitter: Emitter, rows: list[str]) -> None:
    for row in rows:
        chars = row_chars(row)
        for length in range(emitter.min_len, min(emitter.max_len, len(chars)) + 1):
            for start in range(0, len(chars) - length + 1):
                word = chars[start : start + length]
                emitter.add(word)
                emitter.add(word[::-1])


def add_straight_walks(emitter: Emitter, grid: dict[tuple[int, int], str]) -> None:
    dirs = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
    for start in sorted(grid):
        for dx, dy in dirs:
            chars = []
            x, y = start
            for _ in range(emitter.max_len):
                ch = grid.get((x, y))
                if ch is None:
                    break
                chars.append(ch)
                if len(chars) >= emitter.min_len:
                    emitter.add("".join(chars))
                x += dx
                y += dy


def add_sawtooth(emitter: Emitter, grid: dict[tuple[int, int], str]) -> None:
    max_x = max(x for x, _ in grid)
    for y in range(3):
        for start_x in range(max_x + 1):
            for first_row in (y, y + 1):
                second_row = y + 1 if first_row == y else y
                for skew in (0, 1):
                    chars = []
                    for i in range(emitter.max_len):
                        yy = first_row if i % 2 == 0 else second_row
                        xx = start_x + (i // 2) + (skew if i % 2 else 0)
                        ch = grid.get((xx, yy))
                        if ch is None:
                            break
                        chars.append(ch)
                        if len(chars) >= emitter.min_len:
                            word = "".join(chars)
                            emitter.add(word)
                            emitter.add(word[::-1])


def vertical_column(grid: dict[tuple[int, int], str], x: int) -> str:
    return "".join(grid[(x, y)] for y in range(4) if (x, y) in grid)


def add_columns(emitter: Emitter, grid: dict[tuple[int, int], str]) -> None:
    max_x = max(x for x, _ in grid)
    columns = [vertical_column(grid, x) for x in range(max_x + 1)]

    for col in columns:
        if len(col) < emitter.min_len:
            continue
        for length in range(emitter.min_len, min(emitter.max_len, len(col)) + 1):
            for start in range(0, len(col) - length + 1):
                word = col[start : start + length]
                emitter.add(word)
                emitter.add(word[::-1])

    for start_x in range(max_x + 1):
        down = []
        up = []
        snake = []
        for x in range(start_x, max_x + 1):
            col = columns[x]
            if len(col) < 2:
                break
            down.append(col)
            up.append(col[::-1])
            snake.append(col if (x - start_x) % 2 == 0 else col[::-1])
            for word in ("".join(down), "".join(up), "".join(snake)):
                if len(word) > emitter.max_len:
                    break
                emitter.add(word)
                emitter.add(word[::-1])


def add_boxes(emitter: Emitter, grid: dict[tuple[int, int], str]) -> None:
    max_x = max(x for x, _ in grid)
    max_y = max(y for _, y in grid)
    for height in range(2, 5):
        for width in range(2, 6):
            for y0 in range(0, max_y - height + 2):
                for x0 in range(0, max_x - width + 2):
                    cells = [(x, y) for y in range(y0, y0 + height) for x in range(x0, x0 + width)]
                    if any(cell not in grid for cell in cells):
                        continue

                    rows = []
                    for y in range(y0, y0 + height):
                        row = [grid[(x, y)] for x in range(x0, x0 + width)]
                        rows.extend(row if (y - y0) % 2 == 0 else row[::-1])

                    cols = []
                    for x in range(x0, x0 + width):
                        col = [grid[(x, y)] for y in range(y0, y0 + height)]
                        cols.extend(col if (x - x0) % 2 == 0 else col[::-1])

                    perimeter = []
                    perimeter.extend(grid[(x, y0)] for x in range(x0, x0 + width))
                    perimeter.extend(grid[(x0 + width - 1, y)] for y in range(y0 + 1, y0 + height))
                    perimeter.extend(grid[(x, y0 + height - 1)] for x in range(x0 + width - 2, x0 - 1, -1))
                    perimeter.extend(grid[(x0, y)] for y in range(y0 + height - 2, y0, -1))

                    for seq in (rows, cols, perimeter):
                        word = "".join(seq)
                        emitter.add(word)
                        emitter.add(word[::-1])


def build_words(min_len: int, max_len: int) -> list[str]:
    emitter = Emitter(min_len=min_len, max_len=max_len)
    add_known(emitter)

    for rows in (BASIC_ROWS, SHIFT_ROWS):
        grid = make_grid(rows)
        add_row_substrings(emitter, rows)
        add_straight_walks(emitter, grid)
        add_sawtooth(emitter, grid)
        add_columns(emitter, grid)
        add_boxes(emitter, grid)

    return emitter.words


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate a compact human keyboard-walk wordlist")
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=Path("target/wordlists/human-keyboard-walks-curated.txt"),
    )
    parser.add_argument("--min-len", type=int, default=3)
    parser.add_argument("--max-len", type=int, default=16)
    args = parser.parse_args()

    words = build_words(args.min_len, args.max_len)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text("\n".join(words) + "\n", encoding="utf-8")
    print(f"wrote {len(words)} words to {args.output}")


if __name__ == "__main__":
    main()
