# Rust How-To

**A documentation summary and a compendium of snippets and recipes for the Rust language and ecosystem.**
**Everything you need for day-to-day Rust coding, all in one place.**

## Work In Progress

This book is still _heavily edited_.

## Web site

This book is deployed on <https://john-cd.com/rust_howto>

GitHub repo: <https://john-cd.github.io/rust_howto>

## Repo structure

- The repo contains a book, which markdown sources are in the `src` folder.
- After the book is built using `mdbook`, the resulting HTML and Javascript are found in `book`.
- The `mdbook` configuration is in `book.toml` and templates in `theme`.
- The Rust code is organized as a `cargo` workspace:
  - Examples that are embedded in the book are found in `deps/examples`. These are mostly single, short `.rs` files.
  - Additional examples that are too long or complex to be inserted in the book itself will be added under `xmpl`.
  - The `deps` package's `Cargo.toml` list all dependencies used by embedded examples. Use `cargo add <crate> -F <feature>` while in the `deps` folder to add more as required.
- The Dev Container configuration file and `Dockerfile` are found in `.devcontainer`.

## Installation

### Using VS Code

Clone the repo and open the folder in [VS Code]( https://code.visualstudio.com/ ). It should prompt you to open the code in a `docker` container, which installs `mdbook` and rust tooling automatically. Make sure you have previously installed

- [Dev Container extension]( https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers )
- [Docker Desktop]( https://www.docker.com/products/docker-desktop/ ) (or at least the Docker engine).

Note that opening the code folder in VS Code may take a little while the first time around.

### Other

If you are not using VS Code, install the [Dev Container CLI]( https://github.com/devcontainers/cli ) or simply install the required tools on your local machine:

```bash
apt-get update
apt-get install fzf mold clang # or equivalent for other distros
rustup update
rustup component add clippy
rustup component add rustfmt
cargo install mdbook
cargo install just
```

## Development / Editing

Type `just` (a tool similar to `make`) in your favorite shell to lists all commonly used recipes during book editing and example code development.

Use `just serve` to preview the book by serving it locally on <http://localhost:3000/>.

To add or edit the book, simply update or add a `.md` file in the appropriate `src` subfolder, then add a link in `SUMMARY.md`.

- Add Rust code examples under `deps/examples`.
  - Make sure to format your code (`just fmtall` or `cargo fmt --all`), lint it (`just clippyall` or `cargo clippy --examples`) and verify it compiles (`just buildall` or `cargo build --examples`) and runs correctly (`cargo run --example <name>`).
  - Include your code in the Markdown via `{{#include /path/to/file.rs}}` within pairs of triple backticks.
- You may write very short examples directly in the Markdown (but they won't be be formatted / linted automatically).
- Test all examples within the book (embedded from `deps/examples` or in code blocks) with `just test` / `mdbook test`.
  - Note that `mdbook test` may fail when the example code is dependent on libraries outside of `std` (TODO investigate why). Use `ignore` after triple backticks in the markdown to suppress these errors.
- `rust` language code blocks in the Markdown will automatically get a _play_ button, which will execute the code in the [Rust Playground]( https://play.rust-lang.org/ ) and display the output just below the code block. `mdbook-runnable` forces the play button to be displayed when `ignore` is set.
- The Rust playground only supports top 100 most downloaded libraries and libraries in the Rust cookbook. `noplayground` removes the play button if a code block does not work on the playground.
- Example projects that are too complex to be inserted in the book itself (e.g. that include multiple modules) shoud be added as separate folders below `xmpl`. Use `cargo new/init` to create new packages as usual. Insert a link to the appropriate GitHub page in the markdown.

Verify the markdown is properly rendered using `just serve` or `mdbook serve --open`. Pushing a commit to the `main` branch on GitHub will trigger a GitHub Action worfklow that checks formatting / linting, builds / tests all examples, then deploys the book to GitHub Pages.

## Dev Container and Docker

The `development` target of the multi-stage `.devcontainer\Dockerfile` is used by `.devcontainer/devcontainer.json` to installs `mdbook` and rust tooling.

If you don't want to use Dev Container, use the following from the project's root directory to manually build the Docker image and run it.

```bash
docker build --file .devcontainer/Dockerfile --target development --tag rust_howto_dev --build-arg WORK_DIR=/build .
docker run --rm --detach --name rust_howto_dev1 --volume $(pwd):/build  rust_howto_dev
docker exec -it rust_howto_dev1 bash
```

To cache the crate and the target folders, add

```bash
--mount type=volume,src=rust_howto_cargo-crate-cache,dst=/usr/local/cargo/registry/
--mount type=volume,src=rust_howto_cargo-target-cache,dst=/cargo-target-rust_howto/
```

To connect to the (host OS) docker engine from within the container, add

```bash
--mount type=bind,src=/var/run/docker.sock,dst=/var/run/docker-host.sock
```

## Continuous Integration

The continuous integration worflow is found under `.github`.

The `ci` target in the `Dockerfile` is meant to be used by the GitHub Action continuous workflow.

To test the `docker` image manually, use

```bash
docker build --file .devcontainer/Dockerfile --target ci --tag rust_howto_ci .
docker run -it --rm --name rust_howto_ci1 --volume $(pwd)/book:/build/book rust_howto_ci bash
```

**NOTE:** this is WORK IN PROGRESS.

## Optional pre-processors

[mdbook-linkcheck]( https://github.com/Michael-F-Bryan/mdbook-linkcheck ) is a backend for `mdbook` that will check links.

Install with `cargo install mdbook-linkcheck`. Uncomment the related section in `book.toml`.

[mdbook-hide]( https://github.com/ankitrgadiya/mdbook-hide/ )

Install with `cargo install mdbook-hide`. Uncomment the related section in `book.toml`. To mark a chapter as hidden, add this special Comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

[mdbook-keeper]( https://crates.io/crates/mdbook-keeper )

```bash
cargo install mdbook-keeper --git <https://github.com/tfpk/mdbook-keeper.git>
```

## Links

[mdBook]( https://rust-lang.github.io/mdBook/index.html )
