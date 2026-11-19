# TODO Reviewed ??-??-2026.

"""Check published HTML links under the Pages base path."""

import argparse
import re
import sys
import tomllib
from html import unescape
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urljoin, urlsplit

BOOK_URL = "https://book.invalid/rust_howto/"
PAGES = (
    "index.html",
    "introduction.html",
    "introduction/index.html",
    "categories/development-tools/benchmark_regressions.html",
    "categories/development-tools/profiling.html",
    "appendices/contributing/mdBook_pre-processors.html",
    "appendices/contributing/documentation_generation.html",
)


class Page(HTMLParser):
    """Collect link and redirect targets from emitted HTML."""

    def __init__(self, html: str) -> None:
        super().__init__()
        self.targets: list[str] = []
        self.anchors: set[str] = set()
        self.feed(html)

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)
        if anchor := values.get("id"):
            self.anchors.add(anchor)
        if tag == "a" and (anchor := values.get("name")):
            self.anchors.add(anchor)
        if tag == "a" and (href := values.get("href")):
            self.targets.append(href)
        if tag == "meta" and (values.get("http-equiv") or "").lower() == "refresh":
            content = values.get("content") or ""
            if match := re.search(r";\s*url\s*=\s*(.+)", content, re.IGNORECASE):
                self.targets.append(match[1].strip("'\""))


def check_word_index(root: Path) -> list[str]:
    """Verify each local fragment against the emitted chapter HTML."""
    source = root / "word_index.html"
    if not source.is_file():
        return [f"missing required page: {source.name}"]
    if source.stat().st_size == 0:
        return [f"empty required page: {source.name}"]
    targets = set(Page(source.read_text(encoding="utf-8")).targets)
    chapters: dict[Path, set[str]] = {}
    failures = []
    for href in sorted(targets):
        link = urlsplit(href)
        if link.scheme or link.netloc or not link.fragment:
            continue
        target = urlsplit(urljoin(BOOK_URL + source.name, href)).path
        if not target.startswith("/rust_howto/"):
            failures.append(f"{source.name}: {href} leaves the book")
            continue
        chapter = root / unquote(target.removeprefix("/rust_howto/"))
        if chapter not in chapters:
            chapters[chapter] = (
                Page(chapter.read_text(encoding="utf-8")).anchors
                if chapter.is_file()
                else set()
            )
        if unquote(link.fragment) not in chapters[chapter]:
            failures.append(f"{source.name}: missing anchor {href}")
    return failures


def check_chapters(
    root: Path, sources: list[str], *, check_fragments: bool
) -> list[str]:
    """Resolve authored local links and catch unresolved references outside code."""
    failures = []
    pages: dict[Path, Page] = {}
    for source in sources:
        path = root / source
        if not path.is_file():
            failures.append(f"{source}: missing required page")
            continue
        if source in PAGES and path.stat().st_size == 0:
            failures.append(f"{source}: empty required page")
            continue
        html = path.read_text(encoding="utf-8")
        prose = re.sub(r"<pre\b.*?</pre>|<script\b.*?</script>", "", html, flags=re.S)
        labels = re.findall(r"\]\[([a-z][a-z0-9_-]*~[^\]\s]+)\]", prose)
        for label in sorted(set(labels)):
            failures.append(f"{source}: unresolved reference {label}")
        if path not in pages:
            pages[path] = Page(html)
        for href in sorted(set(pages[path].targets)):
            target = urlsplit(urljoin(BOOK_URL + source, href))
            if target.netloc != urlsplit(BOOK_URL).netloc:
                continue
            if not target.path.startswith("/rust_howto/"):
                failures.append(f"{source}: {href} leaves the book")
                continue
            candidate = root / unquote(target.path.removeprefix("/rust_howto/"))
            if candidate.is_dir():
                candidate /= "index.html"
            if not candidate.is_file():
                failures.append(f"{source}: missing target {href}")
            elif check_fragments and target.fragment and candidate.suffix == ".html":
                if candidate not in pages:
                    pages[candidate] = Page(candidate.read_text(encoding="utf-8"))
                if unquote(target.fragment) not in pages[candidate].anchors:
                    failures.append(f"{source}: missing anchor {href}")
    return failures


def unavailable_playground_imports(
    code: str, dependencies: set[str], available: set[str]
) -> set[str]:
    """Find imports of known Cargo packages that the Playground does not supply."""
    imports = set(
        re.findall(r"(?m)^\s*(?:pub\s+)?use\s+(?:::)?(\w+)\s*(?=::|\bas\b|;)", code)
    )
    imports.update(re.findall(r"(?m)^\s*(?:pub\s+)?extern\s+crate\s+(\w+)", code))
    modules = set(re.findall(r"(?m)^\s*(?:pub\s+)?mod\s+(\w+)\s*[{;]", code))
    return (imports & dependencies) - available - modules


def check_playground(root: Path) -> list[str]:
    """Check rendered runnable blocks against the pinned Playground crate list."""
    book = Path(__file__).resolve().parents[1]
    crates = tomllib.loads((book / "crates" / "Cargo.lock").read_text(encoding="utf-8"))
    dependencies = {package["name"].replace("-", "_") for package in crates["package"]}
    available = set(
        (book / "master" / "playground-crates.txt").read_text(encoding="utf-8").split()
    )
    blocks = re.compile(r'<pre class="playground"><code\b([^>]*)>(.*?)</code>', re.S)

    failures = []
    if not (root / "index.html").is_file():
        return [f"{root}: missing book index"]
    if (root / "index.html").stat().st_size == 0:
        return [f"{root}: empty book index"]
    for page in sorted(root.rglob("*.html")):
        html = page.read_text(encoding="utf-8")
        for number, (attributes, markup) in enumerate(blocks.findall(html), start=1):
            if "no_run" in attributes:
                continue
            code = unescape(re.sub("<[^>]+>", "", markup))
            missing = unavailable_playground_imports(code, dependencies, available)
            if missing:
                crates_label = ", ".join(sorted(missing))
                failures.append(
                    f"{page.relative_to(root)}: runnable block {number} imports {crates_label}"
                )
    return failures


def main() -> int:
    """Report unresolved chapter references and invalid local targets."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("html_dir", type=Path)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--word-index", action="store_true")
    mode.add_argument("--playground", action="store_true")
    mode.add_argument("--all-chapters", action="store_true")
    args = parser.parse_args()
    root = args.html_dir
    if not root.is_dir():
        print(f"HTML directory does not exist: {root}", file=sys.stderr)
        return 1
    if args.playground:
        failures = check_playground(root)
    elif args.word_index:
        failures = check_word_index(root)
    else:
        sources = list(PAGES)
        if args.all_chapters:
            sources = sorted(
                set(PAGES)
                | {
                    path.relative_to(root).as_posix()
                    for path in root.rglob("*.html")
                    if path.relative_to(root).as_posix() != "word_index.html"
                }
            )
        failures = check_chapters(root, sources, check_fragments=args.all_chapters)
    for failure in failures:
        print(failure)
    return int(bool(failures))


if __name__ == "__main__":
    raise SystemExit(main())
