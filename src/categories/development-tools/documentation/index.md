# Documentation

{{#include index.incl.md}}

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[The rustdoc book][book-rustdoc]⮳

{{hi:docs.rs}}[`docs.rs`][docs-rs]⮳: open-source {{i:documentation}} host for Rust crates.

## Documenting your code

- Add {{i:documentation comments}} to your code.

```rust,editable
/// This is a doc comment. It is equivalent to the next line.
#[doc = r" This is a doc comment."]
fn main() {}
```

{{hi:rustdoc}}[`rustdoc`][book-rustdoc]⮳ uses the CommonMark Markdown specification.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/rustdoc.rs}}
```

Any item annotated with `#[doc(hidden)]` will not appear in the documentation.

- Run `rustdoc src/lib.rs --crate-name <name>` or `cargo doc --open` to create a new directory, `doc` (or `target/doc` when using cargo), with a website inside.

## Module or crate-level documentation

Use `//!` at the top of the file (instead of `///`) for module-level documentation.

The first lines within `lib.rs`{{hi:lib.rs}} will compose the crate-level documentation front-page.

```rust,editable
//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue. When the abstraction is used
//! there are these advantages:
//! - Fast
//! - `[Easy]`
//!
//! [Easy]: http://thatwaseasy.example.com

# fn main() {}
```

- To add a "run" button on your documentation (allowing its execution in the {{i:rust playground}}), use the following attribute:

```rust,editable
#![doc(html_playground_url = "https://playground.example.com/")]

# fn main() {}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
