<!-- TODO Reviewed ??-??-2026 -->

# Yew Web Framework

{{#include yew.incl.md}}

## Create Efficient Web Applications with `yew` {#yew}

[![yew~website][c~yew~website~badge]][c~yew~website] [![yew][c~yew~docs~badge]][c~yew~docs] [![yew~crates.io][c~yew~crates.io~badge]][c~yew~crates.io] [![yew~repo][c~yew~repo~badge]][c~yew~repo] [![yew~lib.rs][c~yew~lib.rs~badge]][c~yew~lib.rs]{{hi:yew}}{{hi:JavaScript}}{{hi:WebAssembly}}{{hi:Web}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[Yew][c~yew~docs]↗{{hi:Yew}} is a component-based Rust framework for interactive browser applications. Its function components, properties, hooks, and HTML-like `html!` macro will feel familiar to React developers. Yew compiles to WebAssembly; in the browser, it builds a virtual DOM and updates the page when component state or properties change.

## Start a browser application

[Trunk][trunk~website]↗{{hi:Trunk}} builds the Rust WebAssembly target, bundles the generated JavaScript and WebAssembly with other assets, and serves the result during development. Create a binary crate, add Yew's client-side rendering feature, install Trunk and the browser target, and serve the project:

```bash
cargo new --bin counter
cd counter
cargo add yew@0.23 --features csr
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
trunk serve --open
```

Save this minimal `index.html` next to `Cargo.toml`. Trunk uses its `data-trunk` link to find and build the Rust application.

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Yew counter</title>
    <link data-trunk rel="rust">
  </head>
  <body></body>
</html>
```

The counter below is the complete component example. On `wasm32`, its `main` function mounts the app in the browser. The native test uses the server renderer to check its initial HTML.

```rust,editable
{{#include ../../../crates/cats/wasm/examples/yew/yew.rs:example}}
```

## Components, properties, and callbacks

A component describes a reusable part of the interface and returns `Html`. Function components use `#[component]`; they can own state with hooks such as `use_state`. Define inputs in a `Properties` struct (typically deriving `Properties` and `PartialEq`) and pass them as attributes in `html!`. See the [function component][yew~function-components~website]↗ and [properties][yew~props~website]↗ guides.

Use a `Callback<T>` for events and for communication from a child component back to its parent. A callback can be supplied as a property and emitted by the child. The counter's `onclick` handlers show the shorter local-state pattern; the [callbacks guide][yew~callbacks~website]↗ covers parent-child callbacks.

## Server-side rendering

Client-side rendering is Yew's default: the browser loads the WebAssembly app and renders its components. For server-side rendering, enable Yew's `ssr` feature and render the component tree with `ServerRenderer`; a client can then hydrate the resulting HTML with `Renderer::hydrate`. Browser APIs are unavailable while rendering on the server, so keep them in browser-only effects. Yew currently describes SSR as experimental; check the [SSR guide][yew~ssr~website]↗ before choosing it for a production site.

## Compare web frameworks

| Framework                  | Browser rendering model                                                               | Server-side rendering                                              |
| -------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| [Yew][c~yew~docs]          | Virtual DOM; client-side rendering with WebAssembly by default.                       | `ServerRenderer` and hydration are available; SSR is experimental. |
| [Leptos][c~leptos~website] | Fine-grained reactive updates; supports browser CSR.                                  | Supports SSR with hydration as well as CSR.                        |
| [Dioxus][c~dioxus~website] | Virtual DOM with platform-specific renderers, including the WebAssembly web renderer. | Dioxus Fullstack supports SSR and web hydration.                   |

Choose based on the rendering model, deployment, and platform targets the application needs. The [[web-programming | Web Programming]] page provides more framework options. Dioxus's [desktop and mobile webview targets](../gui/web_based_gui.md#dioxus) are described in the GUI chapter.

### See Also {#see-also .skip}

- [What is Yew?][c~yew~website]↗.
- [Trunk guide][trunk~website]↗.
- [Example "Real World" app built with Rust, Yew, and WebAssembly][c~yew~realworld~example]↗ [![yew~realworld~example][c~yew~realworld~example~badge]][c~yew~realworld~example].

## Related Topics {#related-topics .skip}

- [[wasm | WASM]].
- [[wasm_development | WASM Development]].
- [[interfacing_with_javascript | Interfacing with JavaScript]].
- [[web-programming | Web Programming]].
- [[web-programming_http-server | Web Programming: HTTP Server]].
- [[gui | GUI]].
- [[websocket | WebSockets]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
