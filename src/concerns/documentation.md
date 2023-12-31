# Documentation

[The rustdoc book][rustdoc-book]⮳

[docs.rs][docs-rs]⮳: open-source documentation host for Rust crates.

## Documenting your code

- Add documentation comments to your code.

```rust,editable,ignore
/// This is a doc comment. It is equivalent to the next line.
#[doc = r" This is a doc comment."]
```

`rustdoc` uses the CommonMark Markdown specification.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/rustdoc.rs}}
```

Any item annotated with `#[doc(hidden)]` will not appear in the documentation.

- Run `rustdoc src/lib.rs --crate-name <name>` or `cargo doc --open` to create a new directory, `doc` (or `target/doc` when using cargo), with a website inside.

## Module- or crate-level documentation

Use `//!` at the top of the file (instead of `///`) for module-level documentation.

The first lines within `lib.rs` will compose the crate-level documentation front-page.

```rust,editable,ignore
//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com
```

- To add a "run" button on your documentation (allowing its execution in the rust playground), use the following attribute:

```rust,editable,ignore
#![doc(html_playground_url = "https://playground.example.com/")]
```

## Badges

[Shield.io][shield-io]⮳

{{#include ../link-refs.md}}
