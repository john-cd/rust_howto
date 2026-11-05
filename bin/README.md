<!-- TODO Reviewed ??-??-2026 -->

# Tool Binaries (`bin/`)

This folder stores the tool executables used to generate parts of the book or used by the book building and testing processes.
Generate these executables using `just tools release` (or `just release` inside `tools/`).

## Available Executables

### `clean`
A utility to clean up temporary directories and build artifacts during development and testing.
- **Usage**:
  ```sh
  bin/clean <path_to_directory>
  ```
- **Description**: Recursively cleans `temp/` folders created by test runs across book crates.

### `crate-indices`
Generates the crate index pages of the book and updates crate reference definitions.
- **Usage**:
  ```sh
  bin/crate-indices --help
  bin/crate-indices category_page <CRATE_NAME>...
  bin/crate-indices category_page_from_metadata --metadata <FILE> --book-root <DIR_PATH> <CRATE_NAME>...
  bin/crate-indices alphabetical_page --book-root <DIR_PATH> <CRATE_NAME>...
  bin/crate-indices list_crates -d <DIR_PATH>
  bin/crate-indices update_refdefs -f <FILE> <CRATE_NAME>...
  bin/crate-indices expand_crate_blocks -d <DIR_PATH> -f <FILE>
  ```
- **Description**: Scans crate usage in book examples, organizes crates by crates.io category and alphabetical order, and synchronizes Markdown reference definitions.

### `example-integrity`
Inventories and validates all embedded code examples, includes, chapters, and Cargo targets.
- **Usage**:
  ```sh
  bin/example-integrity [OPTIONS]
  bin/example-integrity --fail
  ```
- **Description**: Traverses the book source tree, verifies anchors and code snippets against actual Cargo targets, and reports integrity status as JSON. Exits with non-zero status when `--fail` is provided and errors are detected.

### `mdbook-private`
Repository-local mdBook preprocessor compatible with mdBook 0.5.
- **Usage**: Invoked directly by `mdbook` as configured in `bk/book.toml`:
  ```toml
  [preprocessor.private]
  command = "../bin/mdbook-private"
  ```
- **Description**: Detects and strips private sections or draft chapters during the book rendering process.

### `mdbook-scrub`
mdBook preprocessor providing hidden section removal and custom directive expansion.
- **Usage**: Invoked directly by `mdbook` as configured in `bk/book.toml`:
  ```toml
  [preprocessor.scrub]
  command = "../bin/mdbook-scrub"

  [preprocessor.directives]
  command = "../bin/mdbook-scrub --stage directives"
  ```
- **Description**: Strips hidden content before mdBook link expansion and expands custom directives (e.g. `{{#crate ...}}`, `{{#categories ...}}`, `{{#example ...}}`) in memory without modifying source files.

### `mdbook-linkcheck-compat`
Adapter and preprocessor bridging mdBook 0.5 books with `mdbook-linkcheck`.
- **Usage**: Invoked by the linkcheck runner in `bk/book.toml`:
  ```toml
  [output.linkcheck]
  command = "../bin/mdbook-linkcheck-compat"
  ```
- **Description**: Translates mdBook 0.5 configuration and book structures to the format expected by `mdbook-linkcheck`, and resolves Markdown headings and HTML anchor fragments.

### `templ`
Generates Markdown badges and reference definitions for crates and categories.
- **Usage**:
  ```sh
  bin/templ --help
  bin/templ badge <CRATE_NAME>
  bin/templ category_badge <CATEGORY_SLUG>
  bin/templ categories_badges_for_crate <CRATE_NAME>
  bin/templ rbe <CHAPTER_NAME>
  bin/templ info <CRATE_NAME>
  ```
- **Description**: Formats consistent shields.io badges, docs.rs links, and reference definitions for documentation.
