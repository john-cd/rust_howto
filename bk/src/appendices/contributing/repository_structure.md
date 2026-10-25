<!-- TODO Reviewed ??-??-2026 -->

# Repository Structure

{{#include repository_structure.incl.md}}

## Folders and Key Files {#folders-and-key-files}

This book's [GitHub repository][rust-howto~repo]↗ is structured as follows:

- The Dev Container and Docker (Compose) configuration files are found in [`.devcontainer`][dev-containers-devcontainer.json]↗{{hi:.devcontainer}}. Review the [[dev_container_docker | Dev Container and Docker]] page for usage.
- The [`.github`][rust-howto~.github~repo]↗ folder contains the GitHub configuration, including the CD/CI workflows that build the code & book and deploy the book to Github Pages.
- The [`.vscode`][rust-howto~.vscode~repo]↗ folder contains the VS Code configuration (other editors can be used).
- The [`bin`][rust-howto~bin~repo]↗ folder stores executables used to generate parts of the book or used by the book building process. Generate these tools using `just tools release` or `just scrub release`. These commands build the code and copy the compiled executables into `bin`.
- The [`bk`][rust-howto~bk~repo]↗ folder contains the book itself:
  - The [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} configuration is in [`bk/book.toml`][c~mdbook~book.toml]↗{{hi:book.toml}}.
  - The [[markdown]] sources of the book are in the `bk/src` folder (work-in-progress chapters are in `bk/drafts`; stub chapters are in `later/src`), the structure of which is described below.
  - After the book is built using [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}}, the resulting HTML and Javascript are found in `bk/book/html`.
  - The templates and assets are in [`bk/theme`][rust-howto~bk-theme~repo]↗ and [`bk/static`][rust-howto~bk-static~repo]↗ respectively.
  - The Rust examples embedded in the book are found below `bk/crates` (see below for details).
  - The [`bk/master`][rust-howto~bk-master~repo]↗ folder contains the master list of crates used in the book (which is used by a few scripts to generate tables).
  - [`bk/scripts`][rust-howto~bk-scripts~repo]↗ contains [just][c~just~website]↗ modules (`mod.just` files) and shell scripts (`*.sh` files) for building the code & book and managing references, links, recipe tables, examples, etc.
- Additional code and tools are found in the `playground`, `publish`, `tools`, and `xmpl` folders.
  - The [`playground`][rust-howto~playground~repo]↗ folder contains bits and pieces of code for testing and exploration. Use it to develop new code examples.
  - The [`publish`][rust-howto~publish~repo]↗ folder contains a placeholder crate that is published to `crates.io`, so that the links to the book could be found there.
  - [`tools`][rust-howto~tools~repo]↗ contains several command-line utilities to build specific sections of the book, for example book indices.
    - `mdbook-scrub` is a custom `mdbook` preprocessor used by the book.
  - Additional examples that are too long or complex to be inserted in the book itself are in folders under [`xmpl`][rust-howto~xmpl~repo]↗.

## Book Organization {#book-organization}

Within the [`bk/src`][rust-howto~src~repo]↗ folder,

- The main table of contents (`SUMMARY.md`) points to the book chapters, which are grouped by section (`language`, `links`...) and, below the `categories` directory, by `crates.io` [categories][crates.io~category_slugs]↗.
  - The `code_organization` section describes how to structure your Rust code using modules, crates, packages and workspaces.
  - The `contributing` section contains information on how to contribute to the book or its examples (including this file).
  - The `crate_selection` section lists the crates used in this book and provide recommendations on how to select external Rust crates for your project.
  - The `indices` folder stores the index of examples and the crate indexes.
  - The [`language`][rust-howto~bk-src-language~repo]↗ section describes Rust language features: types, functions, control flow, etc.
  - The [`links`][rust-howto~bk-src-links~repo]↗ section contains links to Rust documentation, books, videos, cheatsheets, etc.
  - [`standard_library`][rust-howto~bk-src-standard_library~repo]↗ describes core Standard Library types, such as [`Option`][c~std::option::Option~docs]↗ and [`Result`][c~std::result::Result~docs]↗.

Each section or category has one or more chapters. Each chapter consists of a main Markdown file (e.g., `chapter_name.md`) and an associated include file (e.g., `chapter_name.incl.md`) for the 'recipe table', a table of contents with links to all recipes (chapter sections) and associated crates and categories. The main file [includes][blog~mdbook~including-files]↗ the recipe table file, as well as local `refs.incl.md` and global `refs/link-refs.md` files, which contain [link reference definitions][commonmark~link-reference-definitions~website]↗ (the URLs for links).

Each recipe in a given chapter contains a short description of the crates used, links thereto, and [includes][blog~mdbook~including-files]↗ one or more code examples (`*.rs` files) in a `bk/crates` subfolder.

Each book section or category has an introductory `index.md` file that lists its recipes by topic / chapter.

In the `bk/src/refs` folder, you will find `category-refs.md` for link references to the `crates.io` [category pages][crates.io-categories~website]↗; `crate-refs.md` for curated and dependency-derived links to [`crates.io`][crates.io~website], [`lib.rs`][lib.rs~website] and GitHub; and `other-refs.md` for links to blogs, books, company websites related to Rust. Run `just book refdefs-generate` to merge missing dependency references into `crate-refs.md`; existing curated destinations are preserved, and normal book builds check for missing definitions.

The [`bk/drafts`][rust-howto~drafts~repo]↗ and [`later/src`][rust-howto~bk-later~repo]↗ folders follow the same basic organization.

## Code Organization {#code-organization}

The book's [GitHub repository][rust-howto~repo]↗ is a "monorepo" with multiple independent projects:

- The Rust examples embedded in the book use many dependencies, so compiling them can take time. They live in an isolated [`cargo`][c~cargo~docs]↗{{hi:cargo}} workspace whose manifest is `bk/crates/Cargo.toml`. Its member crates are grouped by book section (for example, `bk/crates/language`) or, under `bk/crates/cats`, by `crates.io` category (for example, `bk/crates/cats/algorithms`).
  - Each workspace crate contains a [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file that declares the dependencies used by its examples. Use `cargo add <crate>` or `cargo add <crate> -F <feature>` from that crate's directory to add a dependency.
  - Most examples live under `bk/crates/<section-or-category>/examples/<chapter_name>/<example_name>.rs`. A few supporting files live in `src`, `tests`, or `build.rs`.
  - Each example is a separate `.rs` module included by the chapter's `main.rs`, so the examples for a chapter compile together.
  - The book examples share one `Cargo.lock` and workspace output directory. Workspace members can inherit dependency versions from `[workspace.dependencies]`, but each crate declares the dependencies it uses.
- The rest of the code is either in standalone crates (`publish`) or independent workspaces (`playground`,`tools` and `xmpl`).
  - The [`tools`][rust-howto~tools~repo]↗ workspace builds a few CLI binaries that share a common `tool_lib` library.
  - In the base folder of each project, type [`just`][c~just~docs]↗{{hi:just}} at the shell prompt for a list of commands you can use to format, check, build, lint and test the code. `just release` builds tools in release mode and copies the binaries into the `bin` folder (in the repo root).

Cargo's `target` and intermediate `build` directories are linked from the repository root to persistent, per-checkout cache directories by `just build-setup`. On Windows these are `D:\target\rust_howto\<slot>` and `C:\code_build\rust_howto\<slot>`; the Dev Container uses `/root/.cache/rust_howto/target/<slot>` and `/root/.cache/rust_howto/build/<slot>`. The root `.cargo/config.toml` and shared `scripts/build.just` configure these paths for the independent workspaces. Run `just build-setup` once in a checkout before direct Cargo commands; `just worktree` sets up new slots automatically.

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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
```

These tests are run by the CD/CI workflows (in the `.github` folder) following a `git push` or pull request.

For the sake of readability, the test boilerplate is hidden in the book.

## Related Topics {#related-topics .skip}

- [[book_editing_and_example_code_development | Book Editing and Example Code Development]].
- [[modules_and_paths | Modules and Paths]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [[cargo | Cargo]].

[accuracy review: contributing guide](https://github.com/john-cd/rust_howto/issues/1896)

</div>
