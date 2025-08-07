# Web-based GUI

{{#include web_based_gui.incl.md}}

## `tauri` {#tauri}

[![tauri~website][c~tauri~website~badge]][c~tauri~website] [![tauri][c~tauri~docs~badge]][c~tauri~docs] [![tauri~crates.io][c~tauri~crates.io~badge]][c~tauri~crates.io] [![tauri~github][c~tauri~github~badge]][c~tauri~github] [![tauri~lib.rs][c~tauri~lib.rs~badge]][c~tauri~lib.rs]{{hi:tauri}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[`tauri`][c~tauri~docs]↗{{hi:tauri}} makes tiny, secure apps for all desktop platforms. [`tauri`][c~tauri~docs]↗ is an app construction toolkit that lets you build software for all major desktop operating systems using web technologies. Electron-like web-based UI, except it uses system webviews rather than shipping chromium, and non-UI code is written in Rust rather than [`node.js`](https://nodejs.org)↗{{hi:node.js}}.

```rust,editable
{{#include ../../../crates/cats/gui/examples/web/tauri/mod.rs:example}}
```

## `dioxus` {#dioxus}

[![dioxus~website][c~dioxus~website~badge]][c~dioxus~website] [![dioxus][c~dioxus~docs~badge]][c~dioxus~docs] [![dioxus~crates.io][c~dioxus~crates.io~badge]][c~dioxus~crates.io] [![dioxus~github][c~dioxus~github~badge]][c~dioxus~github] [![dioxus~lib.rs][c~dioxus~lib.rs~badge]][c~dioxus~lib.rs]{{hi:dioxus}}{{hi:Dom}}{{hi:Gui}}{{hi:React}}{{hi:Ui}}{{hi:Wasm}}

[`dioxus`][c~dioxus~docs]↗{{hi:dioxus}} is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. With one codebase, you can build apps that run on web, desktop, and mobile. A very nice API layer that has [`tauri`][c~tauri~docs]↗{{hi:tauri}}, Web, and TUI renderers. A native renderer is coming soon.

Dioxus supports:

- Server-side rendering.
- Concurrent [rendering][p~rendering] (with async support).
- Web/Desktop/Mobile support.
- Pre-rendering and hydration.
- Fragments and Suspense.
- Inline-styles.
- Custom event handlers.
- Custom elements.
- Basic fine-grained reactivity (SolidJS / Svelte-like).

```rust,editable
{{#include ../../../crates/cats/gui/examples/web/dioxus.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[web_based_gui: write](https://github.com/john-cd/rust_howto/issues/394)
where should we put Dioxus?
</div>
