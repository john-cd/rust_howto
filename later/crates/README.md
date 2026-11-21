<!-- TODO Reviewed ??-??-2026. -->

<!-- Reviewed 10-26-2026 -->

# Staging Workspace for Future Code Examples

This folder contains additional code examples, category crates, and prototype implementations currently in development or staged for future integration into the book.

## Checking Preview Links

From the repository root, render a fresh preview and verify its chapter links:

```console
just --justfile bk/justfile book future
```

The recipe checks that every chapter listed in the staged summary rendered and
checks internal destinations, fragments, and unresolved reference labels in the
publication scope. To include deferred Aerospace and Cryptocurrencies pages when
rerunning the audit, use `just --justfile bk/scripts/book/mod.just audit-preview-links true`.

## Workspace Organization

- **`cats/`**: Category crates matching `crates.io` taxonomy (e.g. `gui`, `embedded`, `game_engines`, `wasm`, `science`, `rendering`, etc.).
- **`other/`**: Miscellaneous examples and data processing modules (such as Apache Arrow).
- **Embedded / Standalone Crates**: Standalone subcrates like `cats/embedded` or `cats/gui/vizia` configured as separate workspace units or targets.

## Running Examples

Execute cargo commands targeting specific subcrates:

```console
cargo check -p gui --examples
cargo check -p embedded --examples
```
