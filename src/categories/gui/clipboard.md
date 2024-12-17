# Clipboard

{{#include clipboard.incl.md}}

## `arboard` {#arboard}

[![arboard][c-arboard-badge]][c-arboard] [![arboard-crates.io][c-arboard-crates.io-badge]][c-arboard-crates.io] [![arboard-github][c-arboard-github-badge]][c-arboard-github] [![arboard-lib.rs][c-arboard-lib.rs-badge]][c-arboard-lib.rs]{{hi:arboard}}{{hi:Clipboard}}{{hi:Image}}

Image and text handling for the OS clipboard. A fork of `rust-clipboard` that supports copy and pasting of both text and images on Linux (X11/Wayland), MacOS and Windows.

```rust,editable
{{#include ../../../deps/tests/categories/gui/arboard.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write; expand](https://github.com/john-cd/rust_howto/issues/638)?
</div>
