# Repo Structure

{{#include repo_structure.incl.md}}

## Folders and Key Files {#folders}

This book's [GitHub repository][rust-howto~github]↗ is structured as follows:

- The Dev Container and Docker (Compose) configuration files are found in [`.devcontainer`][dev-containers-devcontainer.json]↗{{hi:.devcontainer}}. Review the [[dev_container_docker | Dev Container and Docker]] page for usage.
- The [`.github`](https://github.com/john-cd/rust_howto/tree/main/.github)↗ folder contains the GitHub configuration, including the CD/CI workflows that build the code & book and deploy the book to Github Pages.
- The [`.vscode`](https://github.com/john-cd/rust_howto/tree/main/.vscode)↗ folder contains the VS Code configuration (other editors can be used).
- The [`bin`](https://github.com/john-cd/rust_howto/tree/main/bin)↗ folder stores executables used to generate parts of the book or used by the book building process. Generate these tools using `just tools release` or `just scrub release`. These commands build the code and copy the compiled executables into `bin`.
- The [`bk`](https://github.com/john-cd/rust_howto/tree/main/bk)↗ folder contains the book itself:
  - The [`mdbook`][c~mdbook~docs]{{hi:mdbook}}↗ configuration is in [`bk/book.toml`][c~mdbook~book.toml]↗{{hi:book.toml}}.
  - The [markdown][p~markdown] sources of the book are in the `bk/src` folder (work-in-progress chapters are in `bk/drafts`; stub chapters are in `bk/later`), the structure of which is described below.
  - After the book is built using [`mdbook`][c~mdbook~docs]{{hi:mdbook}}↗, the resulting HTML and Javascript are found in `bk/book/html`.
  - The templates and assets are in [`bk/theme`](https://github.com/john-cd/rust_howto/tree/main/bk/theme)↗ and [`bk/static`](https://github.com/john-cd/rust_howto/tree/main/bk/static)↗ respectively.
  - The Rust examples embedded in the book are found below `bk/crates` (see below for details).
  - The [`bk/master`](https://github.com/john-cd/rust_howto/tree/main/bk/master)↗ folder contains the master list of crates used in the book (which is used by a few scripts to generate tables).
  - [`bk/scripts`](https://github.com/john-cd/rust_howto/tree/main/bk/scripts)↗ contains [just][c~just~website]↗ modules (`mod.just` files) and shell scripts (`*.sh` files) for building the code & book and managing references, links, recipe tables, examples, etc.
- Additional code and tools is found in the `playground`, `publish`, `tools` and `xmpl` folders.
  - The [`playground`](https://github.com/john-cd/rust_howto/tree/main/playground)↗ folder contains bits and pieces of code for testing and exploration. Use it to develop new code examples.
  - The [`publish`](https://github.com/john-cd/rust_howto/tree/main/publish)↗ folder contains a placeholder crate that is published to `crates.io`, so that the links to the book could be found there.
  - [`tools`](https://github.com/john-cd/rust_howto/tree/main/tools)↗ contains several command-line utilities to build specific sections of the book, for example book indices.
    - `mdbook-scrub` is a custom `mdbook` preprocessor used by the book.
  - Additional examples that are too long or complex to be inserted in the book itself are in folders under [`xmpl`][rust-howto~xmpl~github]↗.

## Book Organization {#book-organization}

Within the [`bk/src`][rust-howto~src~github]↗ folder,

- The main table of contents (`SUMMARY.md`) points to the book chapters, which are grouped by section (`language`, `links`...) and, below the `categories` directory, by `crates.io` [categories][crates.io~category_slugs]↗.
  - The `code_organization` section describes how to structure your Rust code using modules, crates, packages and workspaces.
  - The `contributing` section contains information on how to contribute to the book or its examples (including this file).
  - The `crate_selection` section lists the crates used in this book and provide recommendations on how to select external Rust crates for your project.
  - The `indices` folder stores the index of examples and the crate indexes.
  - The [`language`](https://github.com/john-cd/rust_howto/tree/main/bk/src/language)↗ section describes Rust language features: types, functions, control flow, etc.
  - The [`links`](https://github.com/john-cd/rust_howto/tree/main/bk/src/links)↗ section contains links to Rust documentation, books, videos, cheatsheets, etc.
  - [`standard_library`](https://github.com/john-cd/rust_howto/tree/main/bk/src/standard_library)↗ describes core Standard Library types, such as [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)↗ and [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)↗.

Each section or category has one or more chapters. Each chapter consists of a main Markdown file (e.g., `chapter_name.md`) and an associated include file (e.g., `chapter_name.incl.md`) for the 'recipe table', a table of contents with links to all recipes (chapter sections) and associated crates and categories. The main file [includes][blog~mdbook~including-files] the recipe table file, as well as local `refs.incl.md` and global `refs/link-refs.md` files, which contain [link reference definitions](https://spec.commonmark.org/0.31.2/#link-reference-definitions)↗ (the URLs for links).

Each recipe in a given chapter contains a short description of the crates used, links thereto, and [includes][blog~mdbook~including-files]↗ one or more code examples (`*.rs` files) in a `bk/crates` subfolder.

Each book section or category has an introductory `index.md` file that lists its recipes by topic / chapter.

In the `bk/src/refs` folder, you will find `category-refs.md` for link references to the `crates.io` [category pages][crates.io-categories~website]↗; `crate-refs.md` for links to [`crates.io`][crates.io~website]↗, [`lib.rs`][lib.rs~website]↗ and GitHub for each crate used in the book; and `other-refs.md` for links to blogs, books, company websites related to Rust.

The [`bk/drafts`][rust-howto~drafts~github]↗ and [`bk/later`](https://github.com/john-cd/rust_howto/tree/main/bk/later)↗ folders follow the same basic organization.

## Code Organization {#code-organization}

The book's [GitHub repository][rust-howto~github]↗ is a "monorepo" with multiple independent projects:

- The Rust examples embedded in the book use a very large number of dependencies, therefore their compile time is quite long. For that reason, they are in an isolated [`cargo`][c~cargo~docs]{{hi:cargo}}↗ workspace, which manifest is in `bk/crates/Cargo.toml`. The workspace consists of multiple crates below [`bk/crates`][rust-howto~code-examples~github]↗, each named after sections of the book (e.g., `bk/crates/language`) or, within the `bk/crates/cats` folder, after `crates.io` categories (e.g., `bk/crates/cats/algorithms`).
  - Each crate in the workspace contains a [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file, which list the dependencies used by its code examples. Use `cargo add <crate>` or `cargo add <crate> -F <feature>` while in the appropriate crate folder, in order to add more as required.
  - The examples themselves are stored within the crate's `tests` folder in a subfolder named after their chapter, e.g., `bk/crates/<section or cats/some_category>/examples/<chapter_name>/<example_name>.rs`. In a few cases, you may also find examples in the crate's `tests` or `src` folders or within its `build.rs` file.
  - Each example are in a separate `.rs` Rust file that is a module in a `main.rs` file. All examples of a given chapter are therefore compiled together.
  - The book's code examples, being in a [`cargo`]( ){{hi: }} [workspace][book~rust~ch14-03-cargo-workspaces]↗, share the same dependency versions (a single `Cargo.lock` file) and one shared `target` directory, avoid unnecessary dependency rebuilding.
- The rest of the code is either in standalone crates (`publish`) or independent workspaces (`playground`,`tools` and `xmpl`).
  - The [`tools`](https://github.com/john-cd/rust_howto/tree/main/tools)↗ workspace builds a few CLI binaries that share a common `tool_lib` library.
  - In the base folder of each project, type [`just`]( ){{hi: }} at the shell prompt for a list of commands you can use to format, check, build, lint and test the code. `just release` builds tools in release mode and copies the binaries into the `bin` folder (in the repo root).

Note that, instead of storing the [`cargo`]( ){{hi: }} cache and compiler outputs in a separate `target` folder for each workspace or standalone crate, there is a common `target` folder in the repository root. For example, the compiled examples for the book are in `target/bk`. This is configured in `.cargo/config.toml` files.

Consult the `README.md` files of each crate for more details.

## All Examples are Fully and Continuously Tested {#examples-fully-tested}

In order to make sure that all `bk/crates` code examples work, they are backed by tests, similar to the following:

```rust,editable,noplayground
// ANCHOR: example
// The portion between ANCHOR and ANCHOR_END is shown in the book.
fn main() {
    // Actual book example goes here.
}
// ANCHOR_END: example

// This test is executed by `cargo test` or `cargo nextest run`
// every time the code is built. It calls `main()`.
#[test]
fn test() {
    main();
}
```

These tests are run by the CD/CI workflows (in the `.github` folder) following a `git push` or pull request.

For the sake of readability, the test boilerplate is hidden in the book.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
