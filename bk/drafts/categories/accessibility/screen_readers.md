# Screen Readers

{{#include screen_readers.incl.md}}

## Make a user interface accessible to screen readers {#accesskit}

[![accesskit][c-accesskit-badge]][c-accesskit]{{hi:accesskit}}
[![accesskit-crates.io][c-accesskit-crates.io-badge]][c-accesskit-crates.io]
[![accesskit-github][c-accesskit-github-badge]][c-accesskit-github]
[![accesskit-lib.rs][c-accesskit-lib.rs-badge]][c-accesskit-lib.rs]
[![cat-accessibility][cat-accessibility-badge]][cat-accessibility]{{hi:Accessibility}}
[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[`accesskit`][c-accesskit]⮳{{hi:accesskit}} is a Rust crate that simplifies UI [accessibility][p-accessibility] by providing a [cross-platform][p-cross-platform] way to represent UI structure and content to assistive technologies. It allows you to export a semantic tree representing your UI to make accessible to screen readers and other assistive technologies.

```rust,editable
{{#include ../../../crates/cats/accessibility/tests/accesskit.rs:example}}
```

## See also

- [[gui | GUI]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[screen_readers: write (P2)](https://github.com/john-cd/rust_howto/issues/187)
</div>
