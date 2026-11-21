<!-- TODO Reviewed ??-??-2026. -->

# recipe-tables

Updates book recipe table fragments from local `ex~` reference definitions and
headings. It synchronizes recipe titles, crate and category badges, and missing
rows while preserving content outside each table.

From the repository root, run:

```sh
cargo run --manifest-path tools/Cargo.toml --locked -p recipe-tables -- bk
```

The `just --justfile bk/justfile recipes fix_recipe_tables` recipe invokes
this command for `bk/`.
Re-running it makes no changes when the tables already match their source.
