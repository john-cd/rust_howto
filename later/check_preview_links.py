# TODO Reviewed ??-??-2026.

"""Check rendered targets and fragments in the staged book's publication scope."""

import argparse
import re
import sys
from pathlib import Path
from urllib.parse import unquote

LATER = Path(__file__).resolve().parent
sys.path.insert(0, str(LATER.parent / "bk" / "scripts"))

from check_rendered_entry_links import check_chapters  # noqa: E402


def main() -> int:
    """Require the staged entry page and all chapter outputs before checking links."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("html_dir", type=Path)
    parser.add_argument("--include-deferred", action="store_true")
    args = parser.parse_args()
    summary = (LATER / "src" / "SUMMARY.md").read_text(encoding="utf-8")
    sources = sorted(
        {
            Path(unquote(target)).with_suffix(".html").as_posix()
            for target in re.findall(r"^\s*-\s+\[.*?\]\(([^)]+\.md)\)", summary, re.M)
        }
    )
    if not sources:
        parser.error("the staged summary lists no chapters")
    missing = [
        source
        for source in ["index.html", *sources]
        if not (args.html_dir / source).is_file()
        or not (args.html_dir / source).stat().st_size
    ]
    if missing:
        parser.error(f"preview output missing or empty: {', '.join(missing)}")
    if not args.include_deferred:
        sources = [
            source
            for source in sources
            if not source.startswith(
                ("categories/aerospace", "categories/cryptography_cryptocurrencies/")
            )
        ]
    failures = check_chapters(args.html_dir, sources, check_fragments=True)
    for failure in failures:
        print(failure)
    print(f"Checked {len(sources)} chapters, found {len(failures)} link findings.")
    return int(bool(failures))


if __name__ == "__main__":
    raise SystemExit(main())
