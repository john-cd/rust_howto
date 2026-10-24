<!-- TODO Reviewed ??-??-2026 -->

# Leptos Web Framework

{{#include leptos.incl.md}}

[![leptos][c~leptos~docs~badge]][c~leptos~docs]{{hi:leptos}}
[![leptos~crates.io][c~leptos~crates.io~badge]][c~leptos~crates.io]
[![leptos~repo][c~leptos~repo~badge]][c~leptos~repo]
[![leptos~lib.rs][c~leptos~lib.rs~badge]][c~leptos~lib.rs]
[![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[Leptos][c~leptos~website]↗{{hi:Leptos}} is a reactive Rust framework for web interfaces. A Leptos app can render in the browser from WebAssembly, or render HTML on a server and hydrate it in the browser. The rendering mode determines the build tools, app structure, and hosting requirements.

## `leptos` {#leptos}

### Client-side rendering with WebAssembly {#client-side-rendering}

Choose client-side rendering (CSR) when the front end can be served as static files or when it talks to an existing API. The browser loads the JavaScript bindings and WebAssembly module before Leptos mounts the interface, so the initial view depends on the client loading those files.

A CSR project uses the `csr` feature of `leptos` and the `wasm32-unknown-unknown` target. The [Leptos getting started guide][leptos~getting-started~book]↗ and [Trunk][leptos~trunk~website]↗ cover project setup. From a project directory containing `index.html`, install the tools and target, then serve or build the app:

```sh
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
cargo add leptos --features csr
trunk serve --open
trunk build --release
```

`trunk serve` provides a local development server. `trunk build --release` writes the deployable site to `dist/`, which can be hosted as static files. The Leptos guide describes the tradeoffs and [CSR deployment][leptos~csr-deployment~book]↗.

The example uses a signal for the counter state and `view!` to declare the interface. Event handlers update the signal; reactive expressions read it to update the displayed count and whether it is even or odd.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/leptos.rs:example}}
```

### Server-side rendering and hydration {#server-side-rendering}

Choose server-side rendering (SSR) when the server should send rendered HTML before the browser runs the WebAssembly client. Hydration then attaches the client-side behavior to that HTML. This path can provide useful initial content while keeping the interface interactive, but it needs a running server and a coordinated server/client build.

[`cargo-leptos`][leptos~cargo-leptos~repo]↗ builds and runs both sides of a full-stack Leptos app. Start with one of the official templates, such as the [Axum starter][leptos~start-axum~repo]↗, then use `cargo leptos watch` during development and `cargo leptos build --release` for a release build. The generated setup compiles the server with the `ssr` feature and the browser client with the `hydrate` feature; follow the template's feature definitions and deployment instructions.

Plan to host the server executable and its site assets together. A CSR app can be deployed as static files, while an SSR app also needs a server environment. Test both the initial HTML response and the interactive page after hydration in the target deployment.

### Choose a rendering mode {#choose-a-rendering-mode}

|Need|Starting point|
|------|----------------|
|Static hosting or an existing backend|CSR with Trunk|
|Server-rendered HTML and a Rust server|SSR with `cargo-leptos`|
|Browser APIs outside Leptos's view and event model|Use the bindings described in [[interfacing_with_javascript]]|

## Related topics {#related-topics .skip}

- [[wasm_basics | WASM Basics]]
- [[wasm_development | WASM Development]]
- [[interfacing_with_javascript]]
- [[web-programming_http-server | Web Programming: HTTP Server]]
- [[web-programming | Web Programming]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1220)
</div>
