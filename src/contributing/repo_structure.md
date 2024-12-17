## Repo structure {#repo-structure}

{{#include repo_structure.incl.md}}

- The repo contains a book, which markdown sources are in the `src` folder.
- After the book is built using [`mdbook`][c-mdbook]{{hi:mdbook}}⮳, the resulting HTML and Javascript are found in `book/html`.
- The intermediate (processed) Markdown is in `book/markdown`. The [`mdbook`][c-mdbook]{{hi:mdbook}}⮳ configuration is in `book.toml`; the templates and assets are in `theme` and `static` respectively.
- The Rust code is organized as a [`cargo`][c-cargo]{{hi:cargo}}⮳ workspace:
  - Examples that are embedded in the book are found in `deps/tests`. These are mostly single, short `.rs` files. The `deps/Cargo.toml` list all dependencies used by the embedded examples. Use `cargo add <crate> -F <feature>` while in the `deps` folder to add more as required. `deps/build.rs` creates the Skpetic tests that validate all embedded examples.
  - Additional examples that are too long or complex to be inserted in the book itself will be added under `xmpl`.
  - `tools` contains utilities that e.g. generate the sitemap file and organize links.
- The Dev Container and Docker (Compose) configuration files are found in `.devcontainer`.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[repo_structure: edit (P1)](https://github.com/john-cd/rust_howto/issues/533)


## All examples are fully and continuously tested {#examples-fully-tested}

In order to make sure that all examples work, they are backed by tests, similar to the following:

```rust,editable,noplayground
# [test]
fn test() {
    main();
}
```

For the sake of readability, that boilerplate is hidden by default. In order to read the full contents, click on the "expand" (<i class="fa fa-expand"></i>) button located in the top right corner of the code snippets.
</div>
