mdbook-utils links write-all --markdown-dir ./src --output ./all-links.md
mdbook-utils links duplicate-links --markdown-dir ./src --output ./duplicate-links.md
mdbook-utils links broken-links --markdown-dir ./src --output ./broken-links.md
mdbook-utils links write-inline --markdown-dir ./src --link-format reference --output ./inline-links.md
mdbook-utils refdefs write --markdown-dir ./src --output ./all-reference-definitions.md
mdbook-utils refdefs generate --badges --markdown-dir ./src --output ./badge-refs.md
mdbook-utils refdefs generate --from-dependencies --markdown-dir ./src --cargo-toml-dir ./crates --output ./dependency-refs.md
