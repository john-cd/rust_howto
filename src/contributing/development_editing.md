## Book Editing and Example Code Development

Type [`just`][just-website] (a tool similar to `make`) in your favorite shell to lists all commonly used recipes during book editing and example code development.

Use `just serve` to preview the book by serving it locally on <http://localhost:3000/>.

To add or edit the book, simply update or add a `.md` file in the appropriate [`src`][rust-howto-github-src] subfolder, then add a link in [`SUMMARY.md`][rust-howto-github-summary].

- Add Rust code examples under [`deps/tests`][rust-howto-github-deps-tests] (or sometimes under `deps/examples`).
  - Make sure to format your code (`just fmtall` or `cargo +nightly fmt --all`), check it compiles (`just buildall` or `cargo build --all-targets`), lint it (`just clippyall` or `cargo clippy --all-targets`), and test it (`just testall` or `cargo test --test <name>` for an individual example). You may also `cargo run --example <name>`.
  - Include your code example in the Markdown via `{{# include /path/to/file.rs}}` within pairs of triple backticks.
- You may write very short examples directly in the Markdown (but they won't be be formatted / linted automatically).
- `rust` language code blocks in the Markdown will automatically get a _play_ button, which will execute the code in the [Rust Playground][rust-playground] and display the output just below the code block. Adding the `mdbook-runnable` attribute forces the _play_ button to be displayed when `ignore` is set.
- The Rust playground only supports the top 100 most downloaded libraries and libraries used by the Rust Cookbook. `noplayground` removes the play button if a code block does not work on the playground.
- Example projects that are too complex to be inserted in the book itself (e.g. that include multiple modules) should be added as separate folders below `xmpl`. Use `cargo new` or `cargo init` to create packages as usual. Insert a link to the appropriate GitHub page in the markdown.

Verify the markdown is properly rendered using `just serve` or `mdbook serve --open`. Pushing a commit to the `main` branch on GitHub will trigger a GitHub Action workflow that checks formatting / linting, builds / tests all examples, then deploys the book to GitHub Pages.

{{#include ../refs/link-refs.md}}
