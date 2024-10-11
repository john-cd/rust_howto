## Book Editing and Example Code Development

Type {{hi:just}}[`just`][c-just-website]⮳ (a tool similar to {{hi:make}}[`make`][make-website]⮳) in your favorite shell to lists all commonly used recipes during book editing and example code development.

Use `just serve` to preview the book by serving it locally on <http://localhost:3000/>⮳.

To add or edit the book, simply update or add a `.md` file in the appropriate {{hi:src}}[`src`][rust-howto-src-github]⮳ subfolder, then add a link in {{hi:SUMMARY.md}}[`SUMMARY.md`][rust-howto-summary-github]⮳.

- Add {{i:Rust code examples}} under {{hi:deps/tests}}[`deps/tests`][rust-howto-deps-tests-github]⮳.
  - Make sure to format your code (`just fmtall` or `cargo +nightly fmt --all`), check it compiles (`just buildall` or `cargo build --all-targets`), lint it (`just clippyall` or `cargo clippy --all-targets`), and test it (`just testall` or `cargo test --test <name>` for an individual example). You may also `cargo run --example <name>`.
  - Include your {{i:code example}} in the Markdown via `{{# include /path/to/file.rs}}` within pairs of triple backticks.
- You may write very short {{i:examples}} directly in the Markdown (but they won't be be formatted / linted automatically).
- `rust` language code blocks in the Markdown will automatically get a _play_ button, which will execute the code in the [{{i:Rust Playground}}][rust-playground-website]⮳ and display the output just below the code block. Adding the `mdbook-runnable` attribute forces the _play_ button to be displayed when {{hi:ignore}}[`ignore`][book-rust-code-block-attributes]⮳ is set.
- The {{i:Rust playground}} only supports the top 100 most downloaded libraries and libraries used by the Rust Cookbook. {{hi:noplayground}}[`noplayground`][book-rust-code-block-attributes]⮳ removes the play button if a code block does not work on the playground.
- Example projects that are too complex to be inserted in the book itself (e.g. that include multiple modules) should be added as separate folders below `xmpl`. Use `cargo new` or `cargo init` to create packages as usual. Insert a link to the appropriate GitHub page in the markdown.

Verify the markdown is properly rendered using `just serve` or `mdbook serve --open`. Pushing a commit to the `main` branch on GitHub will trigger a GitHub Action workflow that checks formatting / linting, builds / tests all examples, then deploys the book to GitHub Pages.

{{#include ../refs/link-refs.md}}
