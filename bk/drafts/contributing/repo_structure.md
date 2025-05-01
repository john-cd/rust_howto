# Repo Structure {#repo-structure}

{{#include repo_structure.incl.md}}

- The repo contains a book, which [markdown][p-markdown] sources are in the `src` folder.
- After the book is built using [`mdbook`][c-mdbook]{{hi:mdbook}}⮳, the resulting HTML and Javascript are found in `book/html`.
- The intermediate (processed) [Markdown][p-markdown] is in `book/markdown`. The [`mdbook`][c-mdbook]{{hi:mdbook}}⮳ configuration is in [`book.toml`][c-mdbook-book.toml]⮳{{hi:book.toml}} ; the templates and assets are in [`theme`][c-mdbook_theme]⮳{{hi:theme}} and [`static`][c-lazy_static]⮳{{hi:static}} respectively.
- The Rust code is organized as a [`cargo`][c-cargo]{{hi:cargo}}⮳ workspace:
  - Examples that are embedded in the book are found in crates below `bk/crates`, named after sections of the book or grouping multiple (crates.io) categories of examples. Each example is in a single, short `.rs` file. The [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}} within these crates list the dependencies used by the embedded examples. Use `cargo add <crate> -F <feature>` while in the appropriate crate folder to add more as required.
  - Additional examples that are too long or complex to be inserted in the book itself can be added under `crates/xmpl`.
  - `crates/tools` contains utilities to build sections of the book, for example some indices.
- The Dev Container and Docker (Compose) configuration files are found in [`.devcontainer`][dev-containers-devcontainer.json]⮳{{hi:.devcontainer}} .

## All Examples are Fully and Continuously Tested {#examples-fully-tested}

In order to make sure that all examples work, they are backed by tests, similar to the following:

```rust,editable,noplayground
#[test]
fn test() {
    main();
}
```

For the sake of readability, that boilerplate is hidden by default. In order to read the full contents, click on the "expand" (<i class="fa fa-expand"></i>) button located in the top right corner of the code snippets.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[repo_structure: edit NOW](https://github.com/john-cd/rust_howto/issues/533)
</div>
