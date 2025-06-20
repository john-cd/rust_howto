# Repo Structure {#repo-structure}

{{#include repo_structure.incl.md}}

## Folders {#folders}

- The Dev Container and Docker (Compose) configuration files are found in [`.devcontainer`][dev-containers-devcontainer.json]⮳{{hi:.devcontainer}} .
- The `.github` folder contains the GitHub configuration, including the CD/CI workflows.
- The `.vscode` folder contains the VS Code configuration.
- The `bin` folder stores tools used to generate parts of the book or used by the book building process. Generate these executables using `just tools release` or `just scrub release`. These commands build the code and copy the compiled executables into `bin`.
- The `bk` folder contains the book itself.
  - The [`mdbook`][c-mdbook]{{hi:mdbook}}⮳ configuration is in [`bk/book.toml`][c-mdbook-book.toml]⮳{{hi:book.toml}}.
  - The [markdown][p-markdown] sources of the book are in the `bk/src` folder (work-in-progress chapters are in `bk/drafts` and `bk/later`).
  - After the book is built using [`mdbook`][c-mdbook]{{hi:mdbook}}⮳, the resulting HTML and Javascript are found in `bk/book/html`.
  - The templates and assets are in [`theme`][c-mdbook_theme]⮳{{hi:theme}} and [`static`][c-lazy_static]⮳{{hi:static}} respectively.

## Code Organization {#code-organization}

The Rust examples embedded in the book are found in a [`cargo`][c-cargo]{{hi:cargo}}⮳ workspace, which manifest is in `bk/crates/Cargo.toml`:

- The workspace consists of multiple crates below `bk/crates`, each named after sections of the book (e.g., `bk/crates/language`) or, within the `bk/crates/cats` folder, after `crates.io` categories (e.g., `bk/crates/cats/algorithms`).
- Each crate contains a [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}} file, which list the dependencies used by its code examples. Use `cargo add <crate>` or `cargo add <crate> -F <feature>` while in the appropriate crate folder to add more as required.
- The examples themselves are stored in a subfolder named after their chapter, within the crate's `tests` folder, e.g., `bk/crates/<section or cats/some_category>/tests/<chapter_name>/<example_name>.rs`. In a few cases, you may also find examples in the crate's `examples` or `src` folders or within the `build.rs` file.
- Each example are in a separate module (`.rs` Rust file). All examples of a given chapter are compiled together via a `main.rs` file.

Additional examples that are too long or complex to be inserted in the book itself can be added in a folder under `xmpl`.

The `tools` contains utilities to build sections of the book, for example book indices.

`mdbook-scrub` is a custom mdBook preprocessor. 

The `publish` folder

The `playground` folder

Instead of storing the `cargo` cache in a `target` folder for each workspace.

The `target` folder regroups



## All Examples are Fully and Continuously Tested {#examples-fully-tested}

In order to make sure that all examples work, they are backed by tests, similar to the following:

```rust,editable,noplayground
// ANCHOR: example
// The portion between ANCHOR and ANCHOR_END is shown in the book.
fn main() {
    // Actual code example goes here.
}
// ANCHOR_END: example

// This test is executed by `cargo test` or `cargo nextest run`
// every time the code is built.
#[test]
fn test() {
    main();
}
```

For the sake of readability, that boilerplate is hidden by default. In order to read the full contents, click on the "expand" (<i class="fa fa-expand"></i>) button located in the top right corner of the code snippets.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
