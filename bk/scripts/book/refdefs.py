#!/usr/bin/env python3
# Purpose: Merge or check dependency references in the curated crate reference file.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026.

"""Merge or check dependency references in the curated crate reference file."""

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
import tomllib

MD_BOOK_UTILS_VERSION = "0.1.4"
MD_BOOK_UTILS_REPO = "https://github.com/john-cd/mdbook-utils"


def _book_root_default() -> Path:
    return Path(__file__).resolve().parents[2]


def _pinned_revision() -> str:
    """Read the same tool revision used by the container and host installers."""
    config = _book_root_default().parent / ".devcontainer" / ".env"
    if config.is_file():
        match = re.search(
            r'^MDBOOK_UTILS_REV="([0-9a-f]{40})"$',
            config.read_text(encoding="utf-8"),
            re.MULTILINE,
        )
        revision = match[1] if match else ""
    else:
        # The container embeds the build argument and omits dotenv files.
        revision = os.environ.get("MDBOOK_UTILS_REV", "")
    if not re.fullmatch(r"[0-9a-f]{40}", revision):
        raise RuntimeError(
            f"Missing pinned MDBOOK_UTILS_REV in {config} or the build environment"
        )
    return revision


def _install_command(revision: str) -> list[str]:
    """Build Cargo's pinned installation command."""
    return [
        "cargo",
        "install",
        "--force",
        "--git",
        MD_BOOK_UTILS_REPO,
        "--rev",
        revision,
        "--locked",
        "mdbook-utils",
    ]


def _resolve_tool() -> str:
    tool = os.environ.get("MDBOOK_UTILS") or shutil.which("mdbook-utils")
    revision = _pinned_revision()
    install = " ".join(_install_command(revision))
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
            f"{revision}; found {version_output.strip() or 'no version output'}.\n"
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
        generated = temporary_path / "crate-refs.md"
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
            raise RuntimeError("mdbook-utils completed without creating crate-refs.md")
        return generated.read_bytes().rstrip(b"\r\n") + b"\n"


def _dependency_names(book_root: Path, current: str) -> dict[str, str]:
    """Map Rust library identifiers to canonical Cargo package names."""
    names = {}
    for manifest in sorted((book_root / "crates").rglob("Cargo.toml")):
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        package = data.get("package", {}).get("name")
        if package:
            names[data.get("lib", {}).get("name", package.replace("-", "_"))] = package
        tables = [data, data.get("workspace", {})]
        tables.extend(data.get("target", {}).values())
        for table in tables:
            for kind in ("dependencies", "dev-dependencies", "build-dependencies"):
                for alias, spec in table.get(kind, {}).items():
                    package = (
                        spec.get("package", alias) if isinstance(spec, dict) else alias
                    )
                    names.setdefault(package.replace("-", "_"), package)
    for package in re.findall(r"^\[c~([^~]+)~crates\.io\]:", current, re.MULTILINE):
        names[package.replace("-", "_")] = package
    return names


def _normalized_refs(
    book_root: Path, current: str, generated: str
) -> tuple[dict[str, str], dict[str, str]]:
    """Convert legacy dependency labels and URLs to the book's crate style."""
    names = _dependency_names(book_root, current)
    definitions = {}
    labels = {}
    for old, url in re.findall(r"^\[([^\]]+)\]:\s+(\S+)", generated, re.MULTILINE):
        name, suffix = old, "docs"
        for ending, kind in (("-crate", "crates.io"), ("-github", "repo")):
            if old.endswith(ending):
                name, suffix = old[: -len(ending)], kind
                break
        if not name:
            continue
        package = names.get(name, name)
        label = f"c~{package}~{suffix}"
        if suffix == "docs":
            url = f"https://docs.rs/{package}"
        elif suffix == "crates.io":
            url = f"https://crates.io/crates/{package}"
        else:
            url = url.rstrip("/").removesuffix(".git")
        definitions[label] = url
        labels[old] = label
    return definitions, labels


def _merge_refs(book_root: Path, current: bytes, generated: bytes) -> bytes:
    """Add missing dependency definitions while retaining curated destinations."""
    text = current.decode("utf-8")
    definitions, _ = _normalized_refs(book_root, text, generated.decode("utf-8"))
    existing = dict(re.findall(r"^\[([^\]]+)\]:\s+(.+)$", text, re.MULTILINE))
    missing = {
        label: url for label, url in definitions.items() if label not in existing
    }
    if not missing:
        return current
    existing.update(missing)
    header = "\n".join(
        line for line in text.splitlines() if not re.match(r"^\[[^\]]+\]:", line)
    ).strip()
    body = "\n".join(
        f"[{label}]: {url.rstrip()}" for label, url in sorted(existing.items())
    )
    return ((header + "\n\n" if header else "") + body + "\n").encode("utf-8")


def _show_diff(current: bytes, generated: bytes, fragment: Path) -> None:
    current_lines = current.decode("utf-8", errors="replace").splitlines(keepends=True)
    generated_lines = generated.decode("utf-8", errors="replace").splitlines(
        keepends=True
    )
    diff = difflib.unified_diff(
        current_lines,
        generated_lines,
        fromfile=str(fragment),
        tofile="merged crate-refs.md",
    )
    sys.stderr.writelines(diff)


def process(book_root: Path, action: str, tool: str) -> int:
    book_root = book_root.resolve()
    fragment = book_root / "src" / "refs" / "crate-refs.md"
    current = fragment.read_bytes() if fragment.exists() else b""
    generated = _merge_refs(book_root, current, _generated_bytes(book_root, tool))
    if current == generated:
        print(f"{fragment} is up to date")
        return 0

    if action == "check":
        print(
            f"{fragment} is missing dependency references; run `just book refdefs-generate` to merge them.",
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
            prefix="crate-refs.",
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
    parser.add_argument(
        "action", choices=("check", "generate", "install", "verify-pin")
    )
    parser.add_argument("--book-root", type=Path, default=_book_root_default())
    args = parser.parse_args(argv)
    try:
        if args.action == "verify-pin":
            entry = subprocess.check_output(
                ["git", "ls-tree", "HEAD", "--", "mdbook-utils"],
                cwd=_book_root_default().parent,
                text=True,
            )
            if not entry.startswith(f"160000 commit {_pinned_revision()}\t"):
                raise RuntimeError(
                    "MDBOOK_UTILS_REV must match the mdbook-utils gitlink"
                )
            return 0
        if args.action == "install":
            subprocess.run(_install_command(_pinned_revision()), check=True)
            return 0
        return process(args.book_root, args.action, _resolve_tool())
    except (OSError, RuntimeError, subprocess.SubprocessError) as error:
        print(error, file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
