# Tools

This workspace contains tools written in Rust, which you may use to manage the book.

- `clean` is a very simple tool to clean `crates/**/temp` before and after testing.
- `crate_indices` creates the `crates by category` and `crates (alphabetical)` pages.
- `example-integrity` inventories chapters, includes, code snippets, Cargo targets, reachable modules, and exceptions, validating integrity across the book.
- `mdbook-private` is a repository-local port of `mdbook-private` compatible with mdBook 0.5 for defining and stripping private sections and draft chapters.
- `mdbook-scrub` provides mdBook preprocessing stages to remove hidden sections/files, expand custom directives, and provide `mdbook-linkcheck-compat` for fragment validation.
- `templ` generates markdown links, badges, and reference definitions. The outputs can be copy-pasted into the book's markdown where needed.
- `tool_lib` is a Rust library shared between all Rust CLI tools.

<!-- TODO Reviewed ??-??-2026 -->
