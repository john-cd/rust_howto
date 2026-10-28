<!-- TODO Reviewed ??-??-2026 -->

# Code Organization

This crate contains examples demonstrating Rust's module system, visibility modifiers, `use` declarations, package structures, and crate organization patterns.

## Examples Covered

- **Modules & Path Resolution**: Defining nested modules, file-based module structures (`mod`), and pathing (`self`, `super`, `crate`).
- **Visibility**: Understanding `pub`, `pub(crate)`, `pub(super)`, and restricted visibility.
- **Imports & Re-exports**: Bringing items into scope with `use`, glob imports, and re-exporting (`pub use`).

## Usage

To check or run examples in this crate:

```console
cargo check --package code_organization --examples
cargo test --package code_organization
```

Code examples correspond to chapters in `bk/src/code_organization/`.
