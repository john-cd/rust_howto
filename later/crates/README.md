<!-- Reviewed 10-26-2026 -->

# Staging Workspace for Future Code Examples

This folder contains additional code examples, category crates, and prototype implementations currently in development or staged for future integration into the book.

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
