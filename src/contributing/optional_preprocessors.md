## Optional pre-processors

- [`{{i:mdbook-linkcheck}}`][c-mdbook-linkcheck-github]⮳ is a backend for [`{{i:mdbook}}`][c-mdbook-documentation]⮳ that will check links. Install with `cargo install mdbook-linkcheck`. Uncomment the related section in `book.toml`.
- [`{{i:mdbook-hide}}`][c-mdbook-hide-github]⮳ hides chapters under construction. Install with `cargo install mdbook-hide`. Uncomment the related section in `book.toml`. To mark a chapter as hidden, add the following comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

- [`{{i:mdbook-keeper}}`][c-mdbook-keeper-crates.io]⮳. Install with

```bash
cargo install mdbook-keeper --git <https://github.com/tfpk/mdbook-keeper.git>
```

{{#include ../refs/link-refs.md}}
