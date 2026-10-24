#!/usr/bin/env python3
# TODO Reviewed ??-??-2026

"""Generate or check the owned crate-dependency reference fragment."""

from __future__ import annotations

import argparse
import difflib
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

MD_BOOK_UTILS_VERSION = "0.1.4"
MD_BOOK_UTILS_REV = "dedddf3699e461cd418c1a044d82aabd3fec62e0"
MD_BOOK_UTILS_REPO = "https://github.com/john-cd/mdbook-utils"


def _book_root_default() -> Path:
    return Path(__file__).resolve().parents[2]


def _resolve_tool() -> str:
    tool = os.environ.get("MDBOOK_UTILS") or shutil.which("mdbook-utils")
    install = (
        f"cargo install --git {MD_BOOK_UTILS_REPO} --rev {MD_BOOK_UTILS_REV} "
        "--locked --force mdbook-utils"
    )
    if not tool:
        raise RuntimeError(
            f"mdbook-utils is not installed; install the pinned revision with:\n{install}"
        )

    result = subprocess.run(
        [tool, "--version"],
        check=False,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    version_output = result.stdout + result.stderr
    if result.returncode != 0 or not re.search(
        rf"\b{re.escape(MD_BOOK_UTILS_VERSION)}\b", version_output
    ):
        raise RuntimeError(
            f"Expected mdbook-utils {MD_BOOK_UTILS_VERSION} from revision "
            f"{MD_BOOK_UTILS_REV}; found {version_output.strip() or 'no version output'}.\n"
            f"Install the pinned revision with:\n{install}"
        )
    return tool


def _generated_bytes(book_root: Path, tool: str) -> bytes:
    cargo_root = book_root / "crates"
    if not (cargo_root / "Cargo.toml").is_file():
        raise RuntimeError(
            f"Cargo workspace manifest not found: {cargo_root / 'Cargo.toml'}"
        )

    with tempfile.TemporaryDirectory(prefix="rust-howto-refdefs-") as temporary:
        temporary_path = Path(temporary)
        markdown_source = temporary_path / "src"
        markdown_source.mkdir()
        generated = temporary_path / "dependency-refs.md"
        command = [
            tool,
            "refdefs",
            "generate",
            "--from-dependencies",
            "--cargo-toml-dir",
            str(cargo_root.resolve()),
            "--markdown-dir",
            str(markdown_source.resolve()),
            "--output",
            str(generated.resolve()),
        ]
        result = subprocess.run(
            command,
            cwd=book_root,
            check=False,
            capture_output=True,
            text=True,
            encoding="utf-8",
        )
        if result.returncode != 0:
            detail = "\n".join(
                part.strip() for part in (result.stdout, result.stderr) if part.strip()
            )
            raise RuntimeError(f"mdbook-utils refdefs generation failed:\n{detail}")
        if not generated.is_file():
            raise RuntimeError(
                "mdbook-utils completed without creating dependency-refs.md"
            )
        return generated.read_bytes().rstrip(b"\r\n") + b"\n"


def _preserve_review_marker(current: bytes, generated: bytes) -> bytes:
    """Retain a human review marker while regenerating the owned fragment."""
    marker = re.match(rb"(<!-- TODO Reviewed [^\r\n]* -->\r?\n)", current)
    if marker is not None and not generated.startswith(marker.group(1)):
        return marker.group(1) + generated
    return generated


def _show_diff(current: bytes, generated: bytes, fragment: Path) -> None:
    current_lines = current.decode("utf-8", errors="replace").splitlines(keepends=True)
    generated_lines = generated.decode("utf-8", errors="replace").splitlines(
        keepends=True
    )
    diff = difflib.unified_diff(
        current_lines,
        generated_lines,
        fromfile=str(fragment),
        tofile="generated dependency-refs.md",
    )
    sys.stderr.writelines(diff)


def process(book_root: Path, action: str, tool: str) -> int:
    book_root = book_root.resolve()
    fragment = book_root / "src" / "refs" / "dependency-refs.md"
    current = fragment.read_bytes() if fragment.exists() else b""
    generated = _preserve_review_marker(current, _generated_bytes(book_root, tool))
    if current == generated:
        print(f"{fragment} is up to date")
        return 0

    if action == "check":
        print(
            f"{fragment} is stale; run `just book refdefs-generate` to update this generated fragment.",
            file=sys.stderr,
        )
        _show_diff(current, generated, fragment)
        return 1

    fragment.parent.mkdir(parents=True, exist_ok=True)
    temporary_name: str | None = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="wb",
            dir=fragment.parent,
            prefix="dependency-refs.",
            suffix=".tmp",
            delete=False,
        ) as output:
            temporary_name = output.name
            output.write(generated)
            output.flush()
            os.fsync(output.fileno())
        os.replace(temporary_name, fragment)
    finally:
        if temporary_name is not None and os.path.exists(temporary_name):
            os.unlink(temporary_name)
    print(f"Updated generated reference fragment: {fragment}")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("check", "generate"))
    parser.add_argument("--book-root", type=Path, default=_book_root_default())
    args = parser.parse_args(argv)
    try:
        return process(args.book_root, args.action, _resolve_tool())
    except (OSError, RuntimeError, subprocess.SubprocessError) as error:
        print(error, file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
