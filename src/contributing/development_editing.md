## Development / Editing

Type `just` (a tool similar to `make`) in your favorite shell to lists all commonly used recipes during book editing and example code development.

Use `just serve` to preview the book by serving it locally on <http://localhost:3000/>.

To add or edit the book, simply update or add a `.md` file in the appropriate `src` subfolder, then add a link in `SUMMARY.md`.

- Add Rust code examples under `deps/examples`.
  - Make sure to format your code (`just fmtall` or `cargo fmt --all`), lint it (`just clippyall` or `cargo clippy --examples`) and verify it compiles (`just buildall` or `cargo build --examples`) and runs correctly (`cargo run --example <name>`).
  - Include your code in the Markdown via `{{# include /path/to/file.rs}}` within pairs of triple backticks.
- You may write very short examples directly in the Markdown (but they won't be be formatted / linted automatically).
- Test all examples within the book (embedded from `deps/tests` and `deps/examples` or in code blocks) with `just test`.
- `rust` language code blocks in the Markdown will automatically get a _play_ button, which will execute the code in the [Rust Playground][rust-playground] and display the output just below the code block. `mdbook-runnable` forces the play button to be displayed when `ignore` is set.
- The Rust playground only supports top 100 most downloaded libraries and libraries in the Rust cookbook. `noplayground` removes the play button if a code block does not work on the playground.
- Example projects that are too complex to be inserted in the book itself (e.g. that include multiple modules) shoud be added as separate folders below `xmpl`. Use `cargo new/init` to create new packages as usual. Insert a link to the appropriate GitHub page in the markdown.

Verify the markdown is properly rendered using `just serve` or `mdbook serve --open`. Pushing a commit to the `main` branch on GitHub will trigger a GitHub Action worfklow that checks formatting / linting, builds / tests all examples, then deploys the book to GitHub Pages.

{{#include ../refs/link-refs.md}}
