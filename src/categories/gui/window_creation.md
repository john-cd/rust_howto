# Window creation

{{#include window_creation.incl.md}}

## `winit` {#winit}

[![winit][c-winit-badge]][c-winit] [![winit-crates.io][c-winit-crates.io-badge]][c-winit-crates.io] [![winit-github][c-winit-github-badge]][c-winit-github] [![winit-lib.rs][c-winit-lib.rs-badge]][c-winit-lib.rs]{{hi:winit}}{{hi:Windowing}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

Cross-platform window creation library. "The defacto standard option. Uses an event loop based architecture. Widely used and should probably be the default choice."

```rust,editable
{{#include ../../../crates/ex/cats/gui/examples/window_creation/winit.rs:example}}
```

## `tao` {#tao}

[![tao][c-tao-badge]][c-tao] [![tao-crates.io][c-tao-crates.io-badge]][c-tao-crates.io] [![tao-github][c-tao-github-badge]][c-tao-github] [![tao-lib.rs][c-tao-lib.rs-badge]][c-tao-lib.rs]{{hi:tao}}{{hi:Windowing}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

Cross-platform window manager library. The TAO of cross-platform windowing. A library in Rust built for Tauri.

"A fork of winit by the Tauri project, which adds support for things like system menus that desktop apps need."

```rust,editable
{{#include ../../../crates/ex/cats/gui/examples/window_creation/tao.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[window_creation: write (P2)](https://github.com/john-cd/rust_howto/issues/396)

- add baseview example

</div>
