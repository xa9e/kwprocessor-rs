#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ORIGINAL = Path(os.environ.get("KWP_ORIGINAL", ROOT.parent / "kwprocessor"))
RUST_BIN = ROOT / "target" / "release" / "kwp-rs"
C_BIN = ORIGINAL / "kwp"


def run(cmd: list[str], cwd: Path | None = None) -> bytes:
    proc = subprocess.run(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode not in (0, 255):
        sys.stderr.write(proc.stderr.decode("utf-8", "replace"))
        raise SystemExit(f"command failed: {' '.join(cmd)} -> {proc.returncode}")
    return proc.stdout


def ensure_bins() -> None:
    if not ORIGINAL.exists():
        raise SystemExit(
            f"original kwprocessor tree not found: {ORIGINAL}\n"
            "Set KWP_ORIGINAL=/path/to/kwprocessor to run compatibility checks."
        )

    if not C_BIN.exists():
        run(["make"], cwd=ORIGINAL)

    if shutil.which("cargo") is None:
        raise SystemExit("cargo is not installed; install Rust and rerun this test")

    run(["cargo", "build", "--release"], cwd=ROOT)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def compare_case(name: str, args: list[str]) -> None:
    c_out = run([str(C_BIN), *args], cwd=ROOT)
    rs_out = run([str(RUST_BIN), "--compat-order", *args], cwd=ROOT)

    if c_out != rs_out:
        c_path = ROOT / f"target/{name}.c.out"
        rs_path = ROOT / f"target/{name}.rs.out"
        c_path.parent.mkdir(parents=True, exist_ok=True)
        c_path.write_bytes(c_out)
        rs_path.write_bytes(rs_out)
        raise AssertionError(
            f"{name}: output mismatch\n"
            f"  c:  {len(c_out)} bytes sha256={digest(c_out)} -> {c_path}\n"
            f"  rs: {len(rs_out)} bytes sha256={digest(rs_out)} -> {rs_path}"
        )

    print(f"[ok] {name}: {len(c_out)} bytes sha256={digest(c_out)}")


def compare_all_keymaps(basechars: Path, route: Path) -> None:
    for keymap in sorted((ROOT / "keymaps").glob("*.keymap")):
        compare_case(
            f"keymap-{keymap.stem}",
            [
                "--keyboard-all",
                "--keywalk-all",
                str(basechars),
                str(keymap.relative_to(ROOT)),
                str(route),
            ],
        )


def main() -> None:
    ensure_bins()

    with tempfile.TemporaryDirectory(prefix="kwp-rs-test-") as td:
        tmp = Path(td)

        base_r = tmp / "base-r.txt"
        base_r.write_text("r\n", encoding="utf-8")

        keymap_readme = tmp / "keymap-readme.txt"
        keymap_readme.write_text(
            "rty\n"
            "fgh\n"
            "vbn\n"
            "\n"
            "\n"
            "\n"
            "\n"
            "\n"
            "\n"
            "\n"
            "\n"
            "\n",
            encoding="utf-8",
        )

        route_2221 = tmp / "route-2221.txt"
        route_2221.write_text("2221\n", encoding="utf-8")

        base_small = tmp / "base-small.txt"
        base_small.write_text("1qazQ!`\n", encoding="utf-8")

        route_small = tmp / "route-small.txt"
        route_small.write_text("1\n12\n313\n", encoding="utf-8")

        compare_case("readme-example", [str(base_r), str(keymap_readme), str(route_2221)])
        compare_case(
            "default-small",
            [str(base_small), "keymaps/en-us.keymap", str(route_small)],
        )
        compare_case(
            "all-mods-all-dirs-distance2",
            [
                "--keyboard-all",
                "--keywalk-all",
                "-n",
                "1",
                "-x",
                "2",
                str(base_small),
                "keymaps/en-us.keymap",
                str(route_small),
            ],
        )
        compare_case(
            "continuous-shortcut",
            [
                "--keywalk-cont",
                str(base_small),
                "keymaps/en-us.keymap",
                str(route_small),
            ],
        )
        compare_case(
            "italian-no-final-newline",
            [
                "--keyboard-all",
                "--keywalk-all",
                "basechars/it.base",
                "keymaps/it.keymap",
                str(route_2221),
            ],
        )
        compare_all_keymaps(base_small, ROOT / "routes/2-to-4-exhaustive-prince.route")


if __name__ == "__main__":
    main()
