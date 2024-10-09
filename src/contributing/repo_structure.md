## Repo structure

- The repo contains a book, which markdown sources are in the `src` folder.
- After the book is built using [`{{i:mdbook}}`][mdbook-documentation]⮳, the resulting HTML and Javascript are found in `book/html`.
- The intermediate (processed) Markdown is in `book/markdown`. The [`{{i:mdbook}}`][mdbook-documentation]⮳ configuration is in `book.toml`; the templates and assets are in `theme` and `static` respectively.
- The Rust code is organized as a [`{{i:cargo}}`][cargo]⮳ workspace:
  - Examples that are embedded in the book are found in `deps/tests` and `deps/examples`. These are mostly single, short `.rs` files. The `deps/Cargo.toml` list all dependencies used by the embedded examples. Use `cargo add <crate> -F <feature>` while in the `deps` folder to add more as required. `deps/build.rs` creates the Skpetic tests that validate all embedded examples.
  - Additional examples that are too long or complex to be inserted in the book itself will be added under `xmpl`.
  - `tools` contains utilities that e.g. generate the sitemap file and organize links.
- The Dev Container and Docker (Compose) configuration files are found in `.devcontainer`.

{{#include ../refs/link-refs.md}}
