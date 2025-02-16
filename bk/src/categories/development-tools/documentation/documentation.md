# Documentation

{{#include documentation.incl.md}}

## Document your code {#documenting-your-code}

- Add documentation comments{{hi:Documentation comments}} to your code.

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/documentation/rustdoc.rs:example}}
```

[`rustdoc`][book-rustdoc]{{hi:rustdoc}}⮳ uses the CommonMark Markdown specification.

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/documentation/rustdoc2.rs:example}}
```

Any item annotated with `#[doc(hidden)]` will not appear in the documentation.

Run `rustdoc src/lib.rs --crate-name <name>` or `cargo doc --open` to create a new directory, `doc` (or `target/doc` when using [cargo][p-cargo]), with a website inside.

## Create module- or crate-level documentation {#module-or-crate-level-documentation}

Use `//!` at the top of the file (instead of `///`) for module-level documentation.

The first lines within `lib.rs` will compose the crate-level documentation front-page.

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/documentation/rustdoc3.rs:example}}
```

To add a "run" button on your documentation (allowing its execution in the rust playground{{hi:Rust playground}}), use the following attribute:

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/rustdoc4.rs:example}}
```

## See also

[The rustdoc book][book-rustdoc]{{hi:rustdoc}}⮳

[`docs.rs`][docs-rs]{{hi:docs.rs}}⮳: open-source documentation{{hi:Documentation}} host for Rust crates.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[documentation: add (P2)](https://github.com/john-cd/rust_howto/issues/297)

### Add documentation to function arguments in Rust {#roxygen}

[![roxygen][c-roxygen-badge]][c-roxygen] [![roxygen-crates.io][c-roxygen-crates.io-badge]][c-roxygen-crates.io] [![roxygen-github][c-roxygen-github-badge]][c-roxygen-github] [![roxygen-lib.rs][c-roxygen-lib.rs-badge]][c-roxygen-lib.rs]{{hi:roxygen}}{{hi:Arguments}}{{hi:Document}}{{hi:Doxygen}}{{hi:Function}}{{hi:Parameters}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[`roxygen`][c-roxygen]⮳{{hi:roxygen}} helps seamlessly document function parameters with [`rustdoc`][book-rustdoc]⮳{{hi:rustdoc}}.
</div>
