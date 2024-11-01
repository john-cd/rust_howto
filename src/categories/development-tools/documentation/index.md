# Documentation

{{#include index.incl.md}}

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

[The rustdoc book][book-rustdoc]{{hi:rustdoc}}⮳

[`docs.rs`][docs-rs]{{hi:docs.rs}}⮳: open-source documentation{{hi:Documentation}} host for Rust crates.

## Documenting your code

- Add documentation comments{{hi:Documentation comments}} to your code.

```rust
{{#include ../../../../deps/tests/cats/development_tools/rustdoc.rs:example}}
```

[`rustdoc`][book-rustdoc]{{hi:rustdoc}}⮳ uses the CommonMark Markdown specification.

```rust
{{#include ../../../../deps/tests/cats/development_tools/rustdoc2.rs:example}}
```

Any item annotated with `#[doc(hidden)]` will not appear in the documentation.

Run `rustdoc src/lib.rs --crate-name <name>` or `cargo doc --open` to create a new directory, `doc` (or `target/doc` when using cargo), with a website inside.

## Module or crate-level documentation

Use `//!` at the top of the file (instead of `///`) for module-level documentation.

The first lines within `lib.rs`{{hi:lib.rs}} will compose the crate-level documentation front-page.

```rust
{{#include ../../../../deps/tests/cats/development_tools/rustdoc3.rs:example}}
```

To add a "run" button on your documentation (allowing its execution in the rust playground{{hi:Rust playground}}), use the following attribute:

```rust
{{#include ../../../../deps/tests/cats/development_tools/rustdoc4.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
</div>
