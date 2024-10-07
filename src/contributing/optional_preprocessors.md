## Optional pre-processors

- [`mdbook-linkcheck`][mdbook-linkcheck-github]⮳ is a backend for {{i:`mdbook`}} that will check links. Install with `cargo install mdbook-linkcheck`. Uncomment the related section in `book.toml`.
- [`mdbook-hide`][mdbook-hide-github]⮳ hides chapters under construction. Install with `cargo install mdbook-hide`. Uncomment the related section in `book.toml`. To mark a chapter as hidden, add the following comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

- [`mdbook-keeper`][mdbook-keeper-crate]⮳. Install with

```bash
cargo install mdbook-keeper --git <https://github.com/tfpk/mdbook-keeper.git>
```

{{#include ../refs/link-refs.md}}
