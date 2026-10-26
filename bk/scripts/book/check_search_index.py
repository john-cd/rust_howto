# Purpose: Check the built-in HTML search index for expected and private book content.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
"""Check the built-in HTML search index for expected and private book content."""

from pathlib import Path
import json
import re
import sys


def main() -> int:
    """Validate the built index contains visible terms and excludes hidden text."""
    book_html = Path(__file__).resolve().parents[2] / "book" / "html"
    search_indexes = sorted(book_html.glob("searchindex-*.js"))
    if not search_indexes:
        print(f"No mdBook search index found under {book_html}", file=sys.stderr)
        return 1

    search_index = "\n".join(index.read_text(encoding="utf-8") for index in search_indexes)
    required_terms = ("serde", "tokio")
    missing_terms = [term for term in required_terms if term not in search_index]
    if missing_terms:
        print(f"Search index is missing visible terms: {', '.join(missing_terms)}", file=sys.stderr)
        return 1

    hidden_only_term = "expecting"
    if hidden_only_term in search_index:
        print(f"Hidden content leaked into the search index: {hidden_only_term}", file=sys.stderr)
        return 1

    doc_urls_match = re.search(r'"doc_urls"\s*:\s*(\[[^\]]*\])', search_index)
    if doc_urls_match is None:
        print("Search index does not contain a document URL list", file=sys.stderr)
        return 1

    doc_urls = json.loads(doc_urls_match.group(1))
    excluded_urls = [
        url
        for url in doc_urls
        if url.split("#", maxsplit=1)[0].startswith(("drafts/", "later/"))
    ]
    if excluded_urls:
        print("Draft or staging chapters leaked into the search index", file=sys.stderr)
        return 1

    print(f"Checked {len(search_indexes)} mdBook search index file(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
