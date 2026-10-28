<!-- TODO Reviewed ??-??-2026 -->

# Procedural Macros

This crate implements procedural macro helpers (derive macros, attribute-like macros, and function-like macros) using `syn` and `quote` for use across book code examples.

## Key Macro Types

- **Custom Derive Macros**: Automatically implementing traits for custom structs and enums.
- **Attribute Macros**: Transforming struct or function items during compilation.
- **Function-like Macros**: Custom macro syntax invoked with bang notation (`custom_macro!`).

## Usage

This crate is a procedural macro library (`proc-macro = true`) depended upon by examples in other crates.

```console
cargo check --package proc_macros
```

Code examples correspond to procedural macro topics in `bk/src/language/` and `bk/src/categories/development_tools_procedural_macro_helpers`.
