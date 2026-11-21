<!-- TODO Reviewed ??-??-2026. -->

# Tools

This workspace contains tools written in Rust, which you may use to manage the book.

- `clean` is a very simple tool to clean `crates/**/temp` before and after testing.
- `book-audit` validates rendered book links, preview output, search-index contents, Playground imports, and visible TODOs.
- `crate_indices` creates the `crates by category` and `crates (alphabetical)` pages.
- `example-integrity` inventories chapters, code snippets, and Cargo targets, auditing example integrity across the book.
- `mdbook-private` is a repository-local mdBook 0.5 port of `mdbook-private` that filters or styles private chapters and sections.
- `mdbook-scrub` is a custom mdBook preprocessor that removes hidden sections/chapters and expands custom directives.
- `templ` generates markdown links and reference definitions. The outputs can be copy-pasted into the book's markdown where needed.
- `tool_lib` is a Rust library shared between all Rust CLI tools.

## Tools Overview and Usage

### `clean`

A simple utility to empty `temp` directories before and after running tests, preserving hidden files like `.gitkeep`.

```sh
clean <directory>
```

### `crate_indices`

Generates crate index pages by category and alphabetically, and updates the master list of crates and reference definitions.

```sh
# Generate category and alphabetical indexes from bk/
just book _generate-index-category
just book _generate-index-alphabetical

# Update master reference definitions
cat master/crates.txt | just indices crate_indices update_refdefs -f src/refs/crate-refs.md
```

### `example-integrity`

Inventories the book's Markdown chapters, recursive includes, Rust code snippets, Cargo targets, reachable Rust modules, and documented exceptions. Emits stable JSON to stdout and can fail builds on integrity errors.

Usage:

```sh
# Run audit from tools/ using just
just audit-examples

# Or run directly via cargo
cargo run --locked -p example-integrity -- [--repo-root <PATH>] [--fail]

# Survey example test conventions or list unreferenced book sources
cargo run --locked -p example-integrity -- tests [--summary] [--csv] [ROOT ...]
cargo run --locked -p example-integrity -- unused-examples [--book-root <PATH>] [--summary]
```

- `--fail`: Exit with a failure status if any integrity errors are found.
- `--repo-root <PATH>`: Specify the repository root directory to scan (defaults to the containing repository).
- `tests`: Report example test-convention coverage. Informational by default; add `--fail` to gate on violations.
- `unused-examples`: List `.rs` files under the selected book's `crates/` directory that native includes in `bk/src` and `later/src` do not reference.

### `book-audit`

Checks generated HTML, staged previews, the mdBook search index, runnable Playground imports, and visible TODO placeholders. The `just book buildbook` recipe runs the published-book checks; `just book future` builds the future-book preview and checks its links.

```sh
cargo run --locked -p book-audit -- --repo-root <REPO> rendered-links bk/book/html --all-chapters
cargo run --locked -p book-audit -- --repo-root <REPO> word-index bk/book/html
cargo run --locked -p book-audit -- --repo-root <REPO> playground bk/book/html --book-root bk
cargo run --locked -p book-audit -- --repo-root <REPO> search-index --book-root bk
cargo run --locked -p book-audit -- --repo-root <REPO> preview-links bk/book/future [--include-deferred]
cargo run --locked -p book-audit -- --repo-root <REPO> visible-todos
```

Rendered-link and preview findings include the rendered file and line when available. All checks return a failing exit status when they find a gap.

### `mdbook-private`

A repository-local port of `mdbook-private` compatible with mdBook 0.5. It hides or styles private sections and chapters according to configuration in `book.toml`.

Usage:

Build the release binary to `bin/` with:

```sh
just release
```

Configured as a preprocessor in `book.toml`:

```toml
[preprocessor.private]
command = "../bin/mdbook-private"
```

CLI invocation (typically invoked by mdBook):

```sh
mdbook-private supports <renderer>
mdbook-private
```

### `mdbook-scrub`

A custom mdBook preprocessor providing two stages:

1. `scrub` (default): Strips Markdown inside HTML elements with class `hidden` and strips includes of hidden chapters (prefixed with `_`).
2. `directives`: Expands custom directives (`{{#crate ...}}`, `{{#categories ...}}`, `{{#example ...}}`, and wikilinks) into badges, links, and code blocks.

Usage:

Build the release binary to `bin/` with:

```sh
just release
```

Configure in `book.toml`:

```toml
[preprocessor.scrub]
command = "../bin/mdbook-scrub"
remove-hidden-sections = true
do-not-include-hidden-chapters = true
hidden-chapter-prefix = "_"
after = ["private"]
before = ["links"]

[preprocessor.directives]
command = "../bin/mdbook-scrub --stage directives"
after = ["links"]
```

CLI invocation:

```sh
# Run preprocessor for a specific stage
mdbook-scrub [--stage <scrub|directives>]

# Check renderer support
mdbook-scrub supports <renderer>
```

### `templ`

Generates markdown badges, reference links, and RBE (Rust By Example) badges for crates and categories.

Usage:

```sh
# Run via just from bk/ or tools/
just templ badge <CRATE_NAME>...
just templ category_badge <CRATE_NAME>...
just templ rbe <CONCEPT>...
just templ info <CRATE_NAME>...
```

### `tool_lib`

A shared Rust library crate used internally across the workspace tools (`crate_indices`, `templ`, `example-integrity`, `mdbook-scrub`).
