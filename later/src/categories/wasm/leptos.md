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

## Build a Client-side App {#client-side-app}

A client-side Leptos dashboard needs routing, HTTP calls from WebAssembly, browser storage, and, when the back end speaks gRPC, a gRPC-Web client. The examples below live in one crate whose tests run natively. Code that touches browser APIs is compiled only for `wasm32-unknown-unknown` (`#[cfg(target_arch = "wasm32")]`), and the logic around it is kept in plain functions that the tests cover.

### Route with `leptos_router` {#leptos-router}

[![leptos_router][c~leptos_router~docs~badge]][c~leptos_router~docs]{{hi:leptos_router}} [![leptos][c~leptos~docs~badge]][c~leptos~docs]{{hi:leptos}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`leptos_router`][c~leptos_router~docs]↗{{hi:leptos_router}} maps URLs to components. `<Router>` and `<Routes>` wrap the route tree; `<ParentRoute>` renders a shared layout with an `<Outlet/>` for its children; `<Route path=path!("users/:id") .../>` declares a route with a parameter, which a component reads with `use_params_map`; and `fallback` renders the not-found page. Declare paths with the `path!` macro.

The test renders the application to HTML for several URLs as a server would, by providing a `RequestUrl`, so no browser is needed.

```rust,editable,noplayground
{{#include ../../../crates/cats/wasm/examples/leptos_client/leptos_router.rs:example}}
```

### Call an API with `gloo-net` {#gloo-net}

[![gloo-net][c~gloo-net~docs~badge]][c~gloo-net~docs]{{hi:gloo-net}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`gloo-net`][c~gloo-net~docs]↗{{hi:gloo-net}} wraps the browser's `fetch` API. `Request::get(url).header("Authorization", ..).send().await?.json::<T>().await` returns typed data. The function that builds the URL and the bearer header is target-independent and unit tested; only `fetch_json` is compiled for the browser.

```rust,editable,noplayground
{{#include ../../../crates/cats/wasm/examples/leptos_client/gloo_fetch.rs:example}}
```

Keep the token out of `localStorage` where possible, since any script on the page can read it; an `HttpOnly` session cookie is safer.

### Persist Settings with `gloo-storage` {#gloo-storage}

[![gloo-storage][c~gloo-storage~docs~badge]][c~gloo-storage~docs]{{hi:gloo-storage}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`gloo-storage`][c~gloo-storage~docs]↗{{hi:gloo-storage}} reads and writes typed values in `localStorage` (or `sessionStorage`) as JSON. Give every field of the settings struct a default, so that a missing key, an invalid value, or a value saved by an older version of the application falls back to the defaults instead of failing.

```rust,editable,noplayground
{{#include ../../../crates/cats/wasm/examples/leptos_client/gloo_storage.rs:example}}
```

### Call a gRPC Service with `tonic-web-wasm-client` {#grpc-web-client}

[![tonic-web-wasm-client][c~tonic-web-wasm-client~docs~badge]][c~tonic-web-wasm-client~docs]{{hi:tonic-web-wasm-client}} [![tonic][c~tonic~docs~badge]][c~tonic~docs]{{hi:tonic}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`tonic-web-wasm-client`][c~tonic-web-wasm-client~docs]↗{{hi:tonic-web-wasm-client}} lets a `tonic` client speak gRPC-Web from the browser. The same call runs over `tonic-web-wasm-client::Client` on `wasm32` and over a regular `Channel` on native targets; the server side is described in [[grpc | gRPC]]. The example writes the messages and the call by hand to stay self-contained; in a real project `tonic-prost-build` generates them from a `.proto` file.

```rust,editable,noplayground
{{#include ../../../crates/cats/wasm/examples/leptos_client/grpc_web_client.rs:example}}
```

## Related topics {#related-topics .skip}

- [[wasm_basics | WASM Basics]]
- [[wasm_development | WASM Development]]
- [[interfacing_with_javascript]]
- [[web-programming_http-server | Web Programming: HTTP Server]]
- [[web-programming | Web Programming]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[client-side app written for issue #1944](https://github.com/john-cd/rust_howto/issues/1944)

[write](https://github.com/john-cd/rust_howto/issues/1220)
</div>
