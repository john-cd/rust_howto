mdbook-utils markdown generate-crates --markdown-dir ./src --output ./crates-index.md
mdbook-utils markdown generate-categories --markdown-dir ./src --output ./categories-index.md
mdbook-utils sitemap --markdown-dir ./src --base-url https://example.org/book/ --output ./book/html/sitemap.xml
mdbook-utils debug --markdown-dir ./src --output ./markdown-events.txt
