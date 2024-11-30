# Retained Mode Native GUI

{{#include retained_mode_gui.incl.md}}

## iced {#iced}

[![iced][c-iced-badge]][c-iced]{{hi:iced}} [![iced-github][c-iced-github-badge]][c-iced-github] is a cross-platform GUI library for Rust, inspired by Elm.

[![iced][c-iced-badge]][c-iced]{{hi:iced}}
[![iced-crates.io][c-iced-crates.io-badge]][c-iced-crates.io]
[![iced-github][c-iced-github-badge]][c-iced-github]
[![iced-lib.rs][c-iced-lib.rs-badge]][c-iced-lib.rs]

Retained mode UI with a nice API. It's useable for basic apps, but has a number of missing features including multiple windows, layers, and proper text rendering.

```rust,editable
{{#include ../../../deps/tests/categories/gui/iced.rs:example}}
```

## floem {#floem}

[![floem][c-floem-badge]][c-floem]{{hi:floem}}
[![floem-crates.io][c-floem-crates.io-badge]][c-floem-crates.io]
[![floem-github][c-floem-github-badge]][c-floem-github]
[![floem-lib.rs][c-floem-lib.rs-badge]][c-floem-lib.rs]

- A native Rust UI library with fine-grained reactivity. Inspired by [Xilem][c-xilem-github]{{hi:xilem}}⮳, [Leptos][c-leptos-github]{{hi:leptos}}⮳ and [rui][c-rui-github]{{hi:rui}}⮳. Floem aims to be a high performance declarative UI library requiring minimal user effort.
- [Floem (github)][c-floem-github]⮳
- [Floem (docs)][c-floem]{{hi:floem}}⮳

Inspired by Xilem, Leptos and rui, floem is currently more complete than any of them for native UI. Used by the `lapce` text editor.

```rust,editable
{{#include ../../../deps/tests/categories/gui/floem.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 organize
</div>
