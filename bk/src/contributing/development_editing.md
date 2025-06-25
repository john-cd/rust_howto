# Book Editing and Example Code Development {#book-editing}

{{#include development_editing.incl.md}}

Type [`just`][c-just-website]{{hi:just}}⮳ (a tool similar to [`make`][make-website]{{hi:make}}⮳) in your favorite shell to lists all commonly used recipes during book editing and example code development.

Use `just serve` to preview the book by serving it locally on [http://localhost:3000][localhost:3000]⮳.

To add or edit the book, simply update or add a `.md` file in the appropriate [`src`][rust-howto-src-github]{{hi:src}}⮳ sub-folder, then add a link in [`SUMMARY.md`][rust-howto-summary-github]{{hi:SUMMARY.md}}⮳.

- Add Rust code examples{{hi:Rust code examples}} under [`crates/<section or cats/some_category>/examples/<chapter>/`][rust-howto-code-examples-github]⮳.
  - Make sure to format your code (`just fmtall` or `cargo +nightly fmt --all`), check it compiles (`just buildall` or `cargo build --all-targets`), lint it (`just clippyall` or `cargo clippy --all-targets`), and test it (`just testall` or `cargo test --test <name>` for an individual example). You may also `cargo run --example <name>`.
  - Include your code example{{hi:Code example}} in the Markdown via `{{# include /path/to/file.rs}}` within pairs of triple back-ticks.
- You may write very short examples directly in the [Markdown][p-markdown] (but they won't be be formatted / linted / tested automatically).
- `rust` [language][p-language] code blocks in the [Markdown][p-markdown] will automatically get a _play_ button, which will execute the code in the [Rust Playground][rust-playground-website]{{hi:Rust playground}}⮳ and display the output just below the code block. Adding the `mdbook-runnable` attribute forces the _play_ button to be displayed when [`ignore`][book-rust-code-block-attributes]{{hi:ignore}}⮳ is set.
- The Rust playground{{hi:Rust playground}} only supports the top 100 most downloaded libraries and libraries used by the Rust Cookbook. [`noplayground`][book-rust-code-block-attributes]{{hi:noplayground}}⮳ removes the play button if a code block does not work on the playground.
- Example projects that are too complex to be inserted in the book itself (e.g. that include multiple [modules][p-modules]) should be added as separate folders below `crates/xmpl`. Use `cargo new` or `cargo init` to create packages as usual. Insert a link to the appropriate GitHub page in the [markdown][p-markdown].

Verify the [markdown][p-markdown] is properly rendered using `just serve` or `mdbook serve --open`. Pushing a commit to the `main` branch on GitHub will trigger a GitHub Action workflow that checks [formatting][p-formatting] / linting, builds / tests all examples, then deploys the book to GitHub Pages.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[development_editing: review NOW](https://github.com/john-cd/rust_howto/issues/523)
</div>
