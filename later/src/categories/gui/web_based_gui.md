<!-- TODO Reviewed ??-??-2026 -->

# Web-based GUI

{{#include web_based_gui.incl.md}}

## `tauri` {#tauri}

[![tauri~website][c~tauri~website~badge]][c~tauri~website] [![tauri][c~tauri~docs~badge]][c~tauri~docs] [![tauri~crates.io][c~tauri~crates.io~badge]][c~tauri~crates.io] [![tauri~repo][c~tauri~repo~badge]][c~tauri~repo] [![tauri~lib.rs][c~tauri~lib.rs~badge]][c~tauri~lib.rs]{{hi:tauri}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[`tauri`][c~tauri~docs]↗{{hi:tauri}} makes tiny, secure apps for all desktop platforms. [`tauri`][c~tauri~docs]↗ is an app construction toolkit that lets you build software for all major desktop operating systems using web technologies. Electron-like web-based UI, except it uses system webviews rather than shipping chromium, and non-UI code is written in Rust rather than [`node.js`][node.js~website]↗{{hi:node.js}}.

On Windows, Tauri development requires Microsoft C++ Build Tools with the Desktop development with C++ workload and Microsoft Edge WebView2. WebView2 is already installed on Windows 10 version 1803 and later; see the [Tauri Windows prerequisites][tauri~windows-prerequisites~website]↗ for setup details.

```rust,editable
{{#include ../../../crates/cats/gui/examples/web/tauri/mod.rs:example}}
```

## `dioxus` {#dioxus}

[![dioxus~website][c~dioxus~website~badge]][c~dioxus~website] [![dioxus][c~dioxus~docs~badge]][c~dioxus~docs] [![dioxus~crates.io][c~dioxus~crates.io~badge]][c~dioxus~crates.io] [![dioxus~repo][c~dioxus~repo~badge]][c~dioxus~repo] [![dioxus~lib.rs][c~dioxus~lib.rs~badge]][c~dioxus~lib.rs]{{hi:dioxus}}{{hi:Dom}}{{hi:Gui}}{{hi:React}}{{hi:Ui}}{{hi:Wasm}}

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

## `wry` {#wry}

[![wry][c~wry~docs~badge]][c~wry~docs]{{hi:wry}} [![tao][c~tao~docs~badge]][c~tao~docs]{{hi:tao}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}{{hi:Web view}}

[`wry`][c~wry~docs]↗{{hi:wry}} is the cross-platform web view library that [`tauri`][c~tauri~docs]↗{{hi:tauri}} is built on, and [`tao`][c~tao~docs]↗{{hi:tao}} is the window and event loop library that goes with it. Using them directly is the lightweight option when an application needs only one web view, for example to show a local dashboard or report, and no bundler or JavaScript toolchain.

The example below needs a display, so it is not run by the tests. The tests cover the allow-list for the pages that the program accepts: only local `file://` URIs, validated with the [`url`][c~url~docs]↗{{hi:url}} crate before they reach the web view. The function that opens the page is passed as a closure, so the validation is tested without a window. Do the same whenever the address comes from outside.

```rust,no_run,editable
{{#include ../../../crates/cats/gui/examples/web/wry_window.rs:example}}
```

Run it with `cargo run --example wry_window --features wry`.

Points to remember:

- **The web view belongs to the system.** `wry` uses WebView2 on Windows, WKWebView on macOS and WebKitGTK on Linux. WebView2 ships with current versions of Windows 10 and 11. Linux needs the WebKitGTK and GTK 3 development packages to build (`libwebkit2gtk-4.1-dev` and `libgtk-3-dev` on Debian and Ubuntu) and the runtime libraries to run. Rendering and JavaScript support therefore differ slightly between platforms.
- **Rust and JavaScript talk through IPC.** JavaScript calls `window.ipc.postMessage(text)`, which reaches the handler registered with `WebViewBuilder::with_ipc_handler`. Rust calls back with `WebView::evaluate_script`. Treat the messages as untrusted input.
- **Load content deliberately.** `with_url` loads a page and `with_html` an inline document. Do not navigate to arbitrary addresses from an application that exposes IPC.
- **When to choose [`tauri`][c~tauri~docs]↗{{hi:tauri}} instead.** Tauri adds a command system, a permission model, bundling and installers, auto-update and a JavaScript API. Choose it for a full application with a web front end, and `wry` for a window that displays a page.

## Related Topics {#related-topics .skip}

- Web and desktop UI frameworks.
- [WASM](../wasm/index.md) applications.
- Cross-platform application development.
- Native and hybrid UI toolkits.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [web_based_gui: write](https://github.com/john-cd/rust_howto/issues/394)
  where should we put Dioxus?
  [show a local web page in a native window](https://github.com/john-cd/rust_howto/issues/1952)
</div>
