## Optional pre-processors

- [`mdbook_linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook_linkcheck}}⮳ is a backend for [`mdbook`][c-mdbook-documentation]{{hi:mdbook}}⮳ that will check links. Install with `cargo install mdbook_linkcheck`. Uncomment the related section in `book.toml`.
- [`mdbook_hide`][c-mdbook_hide-github]{{hi:mdbook_hide}}⮳ hides chapters under construction. Install with `cargo install mdbook_hide`. Uncomment the related section in `book.toml`. To mark a chapter as hidden, add the following comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

- [`mdbook_keeper`][c-mdbook_keeper-crates.io]{{hi:mdbook_keeper}}⮳. Install with

```bash
cargo install mdbook_keeper --git https://github.com/tfpk/mdbook_keeper.git
```

{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: add
</div>
