# Purpose: Generate book indexes and a sitemap with mdbook-utils.
# Usage: Run with bash bk/crates/cats/development_tools_documentation/examples/mdbook_utils/indexes_and_sitemap.sh from the example workspace, as described in the book.
#
mdbook-utils markdown generate-crates --markdown-dir ./src --output ./crates-index.md
mdbook-utils markdown generate-categories --markdown-dir ./src --output ./categories-index.md
mdbook-utils sitemap --markdown-dir ./src --base-url https://example.org/book/ --output ./book/html/sitemap.xml
mdbook-utils debug --markdown-dir ./src --output ./markdown-events.txt
