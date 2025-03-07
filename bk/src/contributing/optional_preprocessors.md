# MdBook pre-processors

{{#include optional_preprocessors.incl.md}}

- [`mdbook-linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}⮳ is a backend for [`mdbook`][c-mdbook]{{hi:mdbook}}⮳ that will check links. Install with `cargo install mdbook-linkcheck`. Un-comment the related section in [`book.toml`][c-mdbook-book.toml]⮳{{hi:book.toml}} .

## Alternatives

- [`mdbook-hide`][c-mdbook_hide-github]{{hi:mdbook-hide}}⮳ hides chapters under construction. Install with `cargo install mdbook-hide`. Un-comment the related section in [`book.toml`][c-mdbook-book.toml]⮳{{hi:book.toml}} . To mark a chapter as hidden, add the following comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

- [`mdbook-keeper`][c-mdbook_keeper-crates.io]{{hi:mdbook-keeper}}⮳. Install with

```bash
cargo install mdbook_keeper --git https://github.com/tfpk/mdbook_keeper.git
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[optional_preprocessors: add](https://github.com/john-cd/rust_howto/issues/530)
</div>
