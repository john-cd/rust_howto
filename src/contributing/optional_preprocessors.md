## Optional pre-processors

- {{hi:mdbook_linkcheck}}[`mdbook_linkcheck`][c-mdbook_linkcheck-github]⮳ is a backend for {{hi:mdbook}}[`mdbook`][c-mdbook-documentation]⮳ that will check links. Install with `cargo install mdbook_linkcheck`. Uncomment the related section in `book.toml`.
- {{hi:mdbook_hide}}[`mdbook_hide`][c-mdbook_hide-github]⮳ hides chapters under construction. Install with `cargo install mdbook_hide`. Uncomment the related section in `book.toml`. To mark a chapter as hidden, add the following comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

- {{hi:mdbook_keeper}}[`mdbook_keeper`][c-mdbook_keeper-crates.io]⮳. Install with

```bash
cargo install mdbook_keeper --git <https://github.com/tfpk/mdbook_keeper.git>
```

{{#include ../refs/link-refs.md}}
