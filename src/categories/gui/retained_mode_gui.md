# Retained Mode Native GUI

{{#include retained_mode_gui.incl.md}}

## `iced` {#iced}

[![iced-website][c-iced-website-badge]][c-iced-website] [![iced][c-iced-badge]][c-iced] [![iced-crates.io][c-iced-crates.io-badge]][c-iced-crates.io] [![iced-github][c-iced-github-badge]][c-iced-github] [![iced-lib.rs][c-iced-lib.rs-badge]][c-iced-lib.rs]{{hi:iced}}{{hi:Graphics}}{{hi:Gui}}{{hi:Interface}}{{hi:Ui}}{{hi:Widgets}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[`iced`][c-iced]⮳ is a cross-platform GUI library for Rust, inspired by Elm. Retained mode UI with a nice API. It's useable for basic apps, but has a number of missing features including multiple windows, layers, and proper text rendering.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/gui/iced.rs:example}}
```

## `floem` {#floem}

[![floem][c-floem-badge]][c-floem] [![floem-crates.io][c-floem-crates.io-badge]][c-floem-crates.io] [![floem-github][c-floem-github-badge]][c-floem-github] [![floem-lib.rs][c-floem-lib.rs-badge]][c-floem-lib.rs]{{hi:floem}}

`floem` is a native Rust UI library with fine-grained reactivity, and inspired by [`xilem`][c-xilem-github]{{hi:xilem}}⮳, [`leptos`][c-leptos-github]{{hi:leptos}}⮳ and [`rui`][c-rui-github]{{hi:rui}}⮳. `floem` aims to be a high performance declarative UI library requiring minimal user effort.

`floem` is currently more complete than any of them for native UI. Used by the `lapce` text editor.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/gui/floem.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[retained_mode_gui: organize (P2)](https://github.com/john-cd/rust_howto/issues/389)

</div>
