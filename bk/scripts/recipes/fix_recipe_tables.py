#!/usr/bin/env python3
# Purpose: Update recipe rows from local references and keep their titles in sync with headings.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026

"""Update recipe rows from local references and keep their titles in sync with headings."""

from __future__ import annotations

import argparse
import os
import re
import stat
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from urllib.parse import unquote, urlsplit


REFERENCE_DEFINITION = re.compile(
    r"^\s*\[ex~((?:\\.|[^\]])+)\]:\s*(<([^>]+)>|(\S+))(?:[ \t]+.*)?$"
)
HEADING = re.compile(r"^ {0,3}(#{1,6})[ \t]+(.+?)[ \t]+\{#([^}]+)\}[ \t]*$")
FENCE = re.compile(r"^ {0,3}(`{3,}|~{3,})")
HIDDEN_DIV = re.compile(r"<div\s+class\s*=\s*(['\"])hidden\1", re.IGNORECASE)
BADGE_LINK = re.compile(r"\[!\[[^\]]*\]\[[^\]]+\]\]\[(?P<label>[^\]]+)\]")
RECIPE_LINK = re.compile(
    r"\[(?P<title>(?:\\.|[^\]])*)\]\[ex~(?P<label>(?:\\.|[^\]])+)\]"
)
SEPARATOR_CELL = re.compile(r":?-{3,}:?")


@dataclass(frozen=True)
class RecipeReference:
    label: str
    anchor: str


@dataclass(frozen=True)
class Heading:
    title: str
    start: int


def _line_body(line: str) -> str:
    if line.endswith("\r\n"):
        return line[:-2]
    if line.endswith(("\n", "\r")):
        return line[:-1]
    return line


def _line_ending(line: str) -> str:
    if line.endswith("\r\n"):
        return "\r\n"
    if line.endswith("\n"):
        return "\n"
    if line.endswith("\r"):
        return "\r"
    return ""


def _preferred_ending(lines: list[str]) -> str:
    return next((ending for line in lines if (ending := _line_ending(line))), "\n")


def _normalize_label(label: str) -> str:
    unescaped: list[str] = []
    index = 0
    punctuation = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
    while index < len(label):
        if (
            label[index] == "\\"
            and index + 1 < len(label)
            and label[index + 1] in punctuation
        ):
            index += 1
        unescaped.append(label[index])
        index += 1
    return " ".join("".join(unescaped).split()).casefold()


def _escape_table_pipes(text: str) -> str:
    result: list[str] = []
    backslashes = 0
    for char in text:
        if char == "|" and backslashes % 2 == 0:
            result.append("\\")
        result.append(char)
        if char == "\\":
            backslashes += 1
        else:
            backslashes = 0
    return "".join(result)


def _split_table_cells(line: str) -> list[str] | None:
    body = _line_body(line).lstrip("\ufeff")
    separators: list[int] = []
    backslashes = 0
    for index, char in enumerate(body):
        if char == "|" and backslashes % 2 == 0:
            separators.append(index)
        if char == "\\":
            backslashes += 1
        else:
            backslashes = 0

    if not separators:
        return None

    cells: list[str] = []
    start = 0
    for separator in separators:
        cells.append(body[start:separator].strip())
        start = separator + 1
    cells.append(body[start:].strip())

    if cells and not cells[0]:
        cells.pop(0)
    if cells and not cells[-1]:
        cells.pop()
    return cells


def _is_separator_row(line: str) -> bool:
    cells = _split_table_cells(line)
    return bool(cells) and all(
        SEPARATOR_CELL.fullmatch(cell.replace(" ", "")) for cell in cells
    )


def _find_recipe_header(lines: list[str]) -> tuple[int, list[str]] | None:
    for index, line in enumerate(lines):
        cells = _split_table_cells(line)
        if cells and cells[0].lstrip("\ufeff").casefold() == "recipe":
            return index, cells
    return None


def _parse_references(refs_path: Path, chapter_path: Path) -> list[RecipeReference]:
    references: list[RecipeReference] = []
    chapter_resolved = chapter_path.resolve()
    seen: set[str] = set()

    for line in refs_path.read_text(encoding="utf-8").splitlines():
        match = REFERENCE_DEFINITION.match(line)
        if not match:
            continue
        label = match.group(1)
        destination = match.group(3) or match.group(4)
        parsed = urlsplit(destination)
        if parsed.scheme or parsed.netloc:
            continue

        target = unquote(parsed.path).replace("\\", "/")
        target_path = (
            (refs_path.parent / target).resolve()
            if target
            else refs_path.parent.resolve()
        )
        if target_path != chapter_resolved:
            continue

        anchor = unquote(parsed.fragment)
        normalized = _normalize_label(label)
        if normalized in seen:
            continue
        seen.add(normalized)
        references.append(RecipeReference(label=label, anchor=anchor))

    return references


def _heading_blocks(chapter_text: str) -> tuple[dict[str, str], dict[str, list[str]]]:
    lines = chapter_text.splitlines()
    headings: list[tuple[int, str, str]] = []
    fence_char: str | None = None
    fence_len = 0

    for index, line in enumerate(lines):
        fence_match = FENCE.match(line)
        if fence_match:
            marker = fence_match.group(1)
            if fence_char is None:
                fence_char, fence_len = marker[0], len(marker)
            elif marker[0] == fence_char and len(marker) >= fence_len:
                fence_char, fence_len = None, 0
            continue
        if fence_char is not None:
            continue
        match = HEADING.match(line)
        if match:
            headings.append((index, match.group(2).strip(), match.group(3).strip()))

    titles: dict[str, str] = {}
    blocks: dict[str, list[str]] = {}
    for position, (start, title, anchor) in enumerate(headings):
        end = headings[position + 1][0] if position + 1 < len(headings) else len(lines)
        titles[anchor] = title
        blocks[anchor] = lines[start:end]
    return titles, blocks


def _badges(block: list[str], *, categories: bool) -> str:
    badges: list[str] = []
    seen: set[str] = set()
    for line in block:
        for match in BADGE_LINK.finditer(line):
            label = match.group("label")
            if categories:
                include = label.startswith("cat~")
            else:
                include = label.startswith("c~") and label.endswith(
                    ("~docs", "~website", "~repo")
                )
            if include and label.casefold() not in seen:
                seen.add(label.casefold())
                badges.append(match.group(0))
    return " ".join(badges)


def _table_end(lines: list[str], header_index: int) -> int:
    index = header_index + 1
    if index < len(lines) and _is_separator_row(lines[index]):
        index += 1
    while index < len(lines):
        body = _line_body(lines[index])
        if not body.strip() or HIDDEN_DIV.search(body):
            break
        if _split_table_cells(lines[index]) is None:
            break
        index += 1
    return index


def _existing_labels(lines: list[str], header_index: int, end_index: int) -> set[str]:
    labels: set[str] = set()
    for line in lines[header_index + 1 : end_index]:
        if _is_separator_row(line):
            continue
        cells = _split_table_cells(line)
        if not cells:
            continue
        for match in RECIPE_LINK.finditer(cells[0]):
            labels.add(_normalize_label(match.group("label")))
    return labels


def _recipe_row(
    reference: RecipeReference,
    heading: Heading | None,
    columns: list[str],
    blocks: dict[str, list[str]],
) -> str:
    title = (
        heading.title
        if heading is not None
        else reference.label.rsplit("~", 1)[-1].replace("-", " ")
    )
    cells = [f"[{title}][ex~{reference.label}]"] + [""] * (len(columns) - 1)
    block = blocks.get(reference.anchor, [])
    for index, column in enumerate(columns[1:], start=1):
        normalized = column.casefold()
        if "crate" in normalized:
            cells[index] = _badges(block, categories=False)
        elif "categor" in normalized:
            cells[index] = _badges(block, categories=True)
    return "| " + " | ".join(_escape_table_pipes(cell) for cell in cells) + " |"


def _update_existing_titles(
    lines: list[str],
    header_index: int,
    end_index: int,
    references: dict[str, RecipeReference],
    titles: dict[str, str],
) -> bool:
    by_label = {
        _normalize_label(label): reference for label, reference in references.items()
    }
    changed = False
    for index in range(header_index + 1, end_index):
        if _is_separator_row(lines[index]):
            continue
        body = _line_body(lines[index])
        match = RECIPE_LINK.search(body)
        if not match:
            continue
        reference = by_label.get(_normalize_label(match.group("label")))
        title = titles.get(reference.anchor) if reference is not None else None
        if title is None:
            continue
        replacement = _escape_table_pipes(f"[{title}][ex~{match.group('label')}]")
        updated = body[: match.start()] + replacement + body[match.end() :]
        if updated != body:
            lines[index] = updated + _line_ending(lines[index])
            changed = True
    return changed


def _wrapper_includes(chapter_path: Path, include_path: Path) -> bool:
    if not chapter_path.exists():
        return False
    text = chapter_path.read_text(encoding="utf-8")
    directive = re.compile(
        r"\{\{\s*#include\s+[^}]*?" + re.escape(include_path.name) + r"\s*\}\}"
    )
    return bool(directive.search(text))


def _write_atomically(path: Path, text: str) -> None:
    mode = stat.S_IMODE(path.stat().st_mode) if path.exists() else None
    with tempfile.NamedTemporaryFile(
        mode="w",
        encoding="utf-8",
        newline="",
        dir=path.parent,
        prefix=f".{path.name}.",
        suffix=".tmp",
        delete=False,
    ) as temporary:
        temporary.write(text)
        temporary_path = Path(temporary.name)
    try:
        if mode is not None:
            os.chmod(temporary_path, mode)
        os.replace(temporary_path, path)
    finally:
        if temporary_path.exists():
            temporary_path.unlink()


def update_chapter(chapter_path: Path) -> bool:
    refs_path = chapter_path.parent / "refs.incl.md"
    if not refs_path.is_file():
        return False
    references = _parse_references(refs_path, chapter_path)
    if not references:
        return False

    include_path = chapter_path.with_suffix(".incl.md")
    is_new_include = not include_path.exists()
    if is_new_include and not _wrapper_includes(chapter_path, include_path):
        print(
            f"Skipping {include_path}: wrapper does not include this fragment",
            file=sys.stderr,
        )
        return False

    old_text = include_path.read_text(encoding="utf-8") if not is_new_include else ""
    lines = old_text.splitlines(keepends=True)
    ending = _preferred_ending(lines)
    if is_new_include or not any(_line_body(line).strip() for line in lines):
        lines = [
            f"| Recipe | Crates | Categories |{ending}",
            f"|--------|--------|------------|{ending}",
        ]
        is_new_include = True

    header = _find_recipe_header(lines)
    if header is None:
        print(f"Skipping {include_path}: no recipe table header found", file=sys.stderr)
        return False
    header_index, columns = header
    table_end = _table_end(lines, header_index)
    chapter_text = chapter_path.read_text(encoding="utf-8")
    titles, blocks = _heading_blocks(chapter_text)
    reference_map = {
        _normalize_label(reference.label): reference for reference in references
    }

    changed = _update_existing_titles(
        lines, header_index, table_end, reference_map, titles
    )
    present = _existing_labels(lines, header_index, table_end)
    new_rows: list[str] = []
    for reference in references:
        if _normalize_label(reference.label) in present:
            continue
        heading = (
            Heading(titles[reference.anchor], 0) if reference.anchor in titles else None
        )
        new_rows.append(_recipe_row(reference, heading, columns, blocks))
        present.add(_normalize_label(reference.label))

    if new_rows:
        insertion_index = _table_end(lines, header_index)
        insertion_ending = _preferred_ending(lines)
        at_end = insertion_index == len(lines)
        had_final_newline = bool(lines and _line_ending(lines[-1]))
        if insertion_index > 0 and not _line_ending(lines[insertion_index - 1]):
            lines[insertion_index - 1] += insertion_ending
        inserted = [row + insertion_ending for row in new_rows]
        if at_end and not had_final_newline:
            inserted[-1] = _line_body(inserted[-1])
        lines[insertion_index:insertion_index] = inserted
        changed = True

    if changed or is_new_include:
        _write_atomically(include_path, "".join(lines))
        return True
    return False


def _is_chapter(path: Path) -> bool:
    name = path.name.casefold()
    return path.suffix.casefold() == ".md" and not (
        name.endswith(".incl.md")
        or name.endswith("index.md")
        or name.endswith("refs.md")
        or name == "toreview.md"
    )


def update_book(root: Path) -> list[Path]:
    changed: list[Path] = []
    for content_dir in (root / "src", root / "drafts"):
        if not content_dir.is_dir():
            continue
        for chapter in sorted(
            path for path in content_dir.rglob("*.md") if _is_chapter(path)
        ):
            if update_chapter(chapter):
                changed.append(chapter.with_suffix(".incl.md"))
    return changed


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "root", type=Path, help="mdBook root containing src/ and drafts/"
    )
    args = parser.parse_args()
    root = args.root.resolve()
    if not root.is_dir():
        parser.error(f"book root is not a directory: {root}")

    changed = update_book(root)
    for path in changed:
        print(f"Updated {path}")
    print(f"Updated {len(changed)} recipe table fragment(s).")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except OSError as error:
        print(f"fix_recipe_tables: {error}", file=sys.stderr)
        raise SystemExit(1) from error
