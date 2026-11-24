#!/usr/bin/env python3
"""Build the PDF on Windows without exceeding CreateProcess's argument limit."""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import tomllib
from pathlib import Path
from urllib.parse import unquote

NATIVE_PREFIXES = (
    "[RawBlock",
    "[Header",
    "[Para",
    "[Plain",
    "[BulletList",
    "[OrderedList",
    "[BlockQuote",
    "[Div",
    "[Table",
    "[DefinitionList",
    "[CodeBlock",
    "[HorizontalRule",
    "[Null",
    "[LineBlock",
    "[Figure",
    "[]",
)
SUMMARY_LINK = re.compile(r"\]\(([^)#]+\.md)(?:#[^)]*)?\)")
IMAGE_PATH = re.compile(r'image\("((?:[^"\\]|\\.)*)"')
KNOWN_BROKEN_MARKERS = (
    "{{!crate",
    "{{#badges",
    "{{#{{#include}}",
    "cat~*",
    r"cat\~\*",
    "\ufffd",
    "\uffff",
)


def build_directory(book_root: Path) -> Path:
    """Return the external mdBook output directory for this checkout slot."""
    override = os.environ.get("MDBOOK_BUILD__BUILD_DIR")
    if override:
        return Path(override).resolve()

    checkout = book_root.parent
    slot = checkout.name if checkout.parent.name == ".worktrees" else "main"
    target_root = Path(os.environ.get("CODE_TARGET_ROOT", "D:/target"))
    return (target_root / "rust_howto" / slot / "book").resolve()


def emit(result: subprocess.CompletedProcess[str]) -> None:
    """Write a completed command's captured output without changing its status."""
    if result.stdout:
        print(result.stdout, end="")
    if result.stderr:
        print(result.stderr, end="", file=sys.stderr)


def ordered_chapters(summary: Path, native_root: Path) -> list[Path]:
    """Read chapter paths in summary order and verify every native AST file."""
    chapters: list[Path] = []
    seen: set[Path] = set()
    for match in SUMMARY_LINK.finditer(summary.read_text(encoding="utf-8")):
        relative = Path(unquote(match.group(1).replace("\\", "/")))
        chapter = (native_root / relative).resolve()
        if not chapter.is_relative_to(native_root.resolve()):
            raise RuntimeError(
                f"chapter path escapes the Pandoc input directory: {relative}"
            )
        if chapter in seen:
            continue
        if not chapter.is_file():
            raise RuntimeError(f"missing preprocessed chapter: {relative.as_posix()}")
        contents = chapter.read_text(encoding="utf-8", errors="replace").lstrip()
        if not contents.startswith(NATIVE_PREFIXES):
            raise RuntimeError(
                f"chapter is not a Pandoc native AST: {relative.as_posix()}"
            )
        chapters.append(chapter)
        seen.add(chapter)

    if not chapters:
        raise RuntimeError(f"no chapter links found in {summary}")
    return chapters


def rewrite_local_images(typst_file: Path, native_root: Path) -> None:
    """Make generated absolute Windows image paths relative to the Typst file."""
    source = typst_file.read_text(encoding="utf-8")
    native_root = native_root.resolve()

    def relative_image(match: re.Match[str]) -> str:
        encoded = match.group(1)
        image = Path(encoded.replace("\\\\", "\\"))
        if not image.is_absolute():
            return match.group(0)
        image = image.resolve()
        if not image.is_relative_to(native_root):
            raise RuntimeError(f"PDF image path is outside the book build: {image}")
        if not image.is_file():
            raise RuntimeError(f"PDF image does not exist: {image}")
        relative = Path(os.path.relpath(image, typst_file.parent)).as_posix()
        return f'image("{relative}"'

    source = IMAGE_PATH.sub(relative_image, source)
    for marker in KNOWN_BROKEN_MARKERS:
        if marker in source:
            raise RuntimeError(f"the generated PDF source still contains {marker!r}")
    for required in ("😀", "π"):
        if required not in source:
            raise RuntimeError(
                f"the generated PDF source is missing required Unicode: {required}"
            )
    typst_file.write_text(source, encoding="utf-8", newline="\n")


def fallback(book_root: Path, output_root: Path) -> int:
    """Convert Pandoc's native chapter files through a defaults JSON file."""
    pandoc_root = output_root / "pandoc" / "pdf"
    native_root = pandoc_root / "src"
    chapters = ordered_chapters(native_root / "SUMMARY.md", native_root)
    config = tomllib.loads((book_root / "book.toml").read_text(encoding="utf-8"))
    book = config["book"]
    profile = dict(config["output"]["pandoc"]["profile"]["pdf"])
    typst_file = pandoc_root / "book.typ"
    pdf_file = pandoc_root / "book.pdf"
    defaults_file = pandoc_root / "book-fallback.defaults.json"
    pandoc_root.mkdir(parents=True, exist_ok=True)

    profile["from"] = "native"
    profile["input-files"] = [chapter.as_posix() for chapter in chapters]
    profile["output-file"] = typst_file.as_posix()

    metadata = dict(profile.get("metadata", {}))
    metadata.setdefault("title", book.get("title", "The Rust How-to Book"))
    if book.get("description"):
        metadata.setdefault("description", book["description"])
    if book.get("authors"):
        metadata.setdefault("author", book["authors"])
    profile["metadata"] = metadata

    variables = dict(profile.get("variables", {}))
    if book.get("language"):
        variables.setdefault("lang", book["language"])
    profile["variables"] = variables
    defaults_file.write_text(json.dumps(profile, ensure_ascii=False), encoding="utf-8")

    print(
        f"Retrying Pandoc with a defaults file for {len(chapters)} chapters "
        "to avoid the Windows command-line length limit.",
        file=sys.stderr,
    )
    conversion = subprocess.run(
        ["pandoc", f"--defaults={defaults_file}", "--quiet"],
        cwd=book_root,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )
    emit(conversion)
    if conversion.returncode:
        return conversion.returncode

    rewrite_local_images(typst_file, native_root)
    compile_pdf = subprocess.run(
        [
            "typst",
            "compile",
            "--root",
            str(output_root),
            "--diagnostic-format",
            "short",
            str(typst_file),
            str(pdf_file),
        ],
        cwd=book_root,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )
    emit(compile_pdf)
    return compile_pdf.returncode


def main() -> int:
    """Run mdBook and retry only its known Windows argument-limit failure."""
    if len(sys.argv) != 2:
        print(f"usage: {Path(sys.argv[0]).name} <book-root>", file=sys.stderr)
        return 2

    book_root = Path(sys.argv[1]).resolve()
    output_root = build_directory(book_root)
    output_root.mkdir(parents=True, exist_ok=True)
    os.environ["MDBOOK_BUILD__BUILD_DIR"] = str(output_root)
    os.environ["MDBOOK_OUTPUT__PANDOC__OPTIONAL"] = "false"

    build = subprocess.run(
        ["mdbook", "build", str(book_root)],
        cwd=book_root.parent,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )
    emit(build)
    if build.returncode == 0:
        return 0

    diagnostics = f"{build.stdout}\n{build.stderr}".lower()
    command_line_limit = (
        "os error 206" in diagnostics
        or "filename or extension is too long" in diagnostics
    )
    if os.name != "nt" or not command_line_limit:
        return build.returncode

    return fallback(book_root, output_root)


if __name__ == "__main__":
    raise SystemExit(main())
