<!-- TODO Reviewed ??-??-2026 -->

# Toasty Example Validation

This package validates the schema definition and code generation for Toasty ORM integration examples.

## Purpose

- Demonstrates Toasty schema definition (`#[derive(Statement)]`, `#[toasty::model]`).
- Validates model relationships, queries, and data manipulation code examples for the database section.

## Usage

To check or run tests in this package:

```console
cargo check --package toasty-validation
cargo test --package toasty-validation
```

Code examples correspond to chapters in `bk/src/categories/database`.
