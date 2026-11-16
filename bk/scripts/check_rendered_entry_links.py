# TODO Reviewed ??-??-2026.

"""Check the published landing page and introduction under the Pages base path."""

import argparse
import re
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urljoin, urlsplit

BOOK_URL = "https://book.invalid/rust_howto/"
PAGES = (
    "index.html",
    "introduction.html",
    "introduction/index.html",
    "appendices/contributing/mdBook_pre-processors.html",
    "appendices/contributing/documentation_generation.html",
)


class Page(HTMLParser):
    """Collect link and redirect targets from emitted HTML."""

    def __init__(self, html: str) -> None:
        super().__init__()
        self.targets: list[str] = []
        self.feed(html)

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)
        if tag == "a" and (href := values.get("href")):
            self.targets.append(href)
        if tag == "meta" and (values.get("http-equiv") or "").lower() == "refresh":
            content = values.get("content") or ""
            if match := re.search(r";\s*url\s*=\s*(.+)", content, re.IGNORECASE):
                self.targets.append(match[1].strip("'\""))


def main() -> int:
    """Report unresolved chapter references and invalid local targets."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("html_dir", type=Path)
    root = parser.parse_args().html_dir
    failures = []
    for source in PAGES:
        html = (root / source).read_text(encoding="utf-8")
        for label in sorted(set(re.findall(r"\]\[(p~[^\]\s]+)\]", html))):
            failures.append(f"{source}: unresolved reference {label}")
        for href in Page(html).targets:
            link = urlsplit(href)
            if link.scheme or link.netloc or not link.path:
                continue
            target = urlsplit(urljoin(BOOK_URL + source, href)).path
            relative = unquote(target).removeprefix("/rust_howto/")
            if not target.startswith("/rust_howto/"):
                failures.append(f"{source}: {href} leaves the book")
                continue
            candidate = root / relative
            if candidate.is_dir():
                candidate /= "index.html"
            if not candidate.is_file():
                failures.append(f"{source}: missing target {href}")
    for failure in failures:
        print(failure)
    return int(bool(failures))


if __name__ == "__main__":
    raise SystemExit(main())
