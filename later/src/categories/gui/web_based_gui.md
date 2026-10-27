<!-- TODO Reviewed ??-??-2026 -->

# Web-based GUI

{{#include web_based_gui.incl.md}}

Web-based GUI toolkits use HTML, CSS, and JavaScript, or Rust components that render through web technologies. The choice depends on how much of the application should be native Rust, whether you need a full framework or just a webview, and which desktop or mobile targets you need.

| Option                        | UI code                                                        | Renderer                                                                               | Platforms                               | Bundle-size considerations                                                                             |
| ----------------------------- | -------------------------------------------------------------- | -------------------------------------------------------------------------------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| [`tauri`][c~tauri~docs]↗      | Any HTML/CSS/JavaScript front end with a Rust back end         | System webview through [`wry`][c~wry~docs]↗                                            | Windows, macOS, Linux, Android, and iOS | Does not bundle a browser engine; app assets, plugins, and platform runtime setup affect package size. |
| [`dioxus`][c~dioxus~docs]↗    | Rust components written with `rsx!`, with web assets as needed | Browser DOM on web; system webview on desktop/mobile; experimental Blitz/WGPU renderer | Web, desktop, Android, and iOS          | System webviews keep desktop packages small; assets and selected renderer/features affect output.      |
| [`wry`][c~wry~docs]↗ directly | Web content plus a Rust window and event-loop host             | Platform system webview                                                                | Windows, macOS, Linux, Android, and iOS | Low-level library only; the application chooses and packages its UI assets and other dependencies.     |

## `tauri` {#tauri}

[![tauri~website][c~tauri~website~badge]][c~tauri~website] [![tauri][c~tauri~docs~badge]][c~tauri~docs] [![tauri~crates.io][c~tauri~crates.io~badge]][c~tauri~crates.io] [![tauri~repo][c~tauri~repo~badge]][c~tauri~repo] [![tauri~lib.rs][c~tauri~lib.rs~badge]][c~tauri~lib.rs]{{hi:tauri}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[`tauri`][c~tauri~docs]↗ builds desktop and mobile applications with a Rust back end and a front end made with web technologies. The front end calls registered Rust commands over Tauri's IPC API. Tauri uses the system webview through `wry`, rather than shipping a separate browser engine with every app. The selected frontend framework, app assets, plugins, and platform runtime setup still affect the final package.

Create a starter project with `npm create tauri-app@latest`, then follow the platform prerequisites and run the app with `cargo tauri dev`. The shared GUI crate retains a dependency-free Rust greeting function and its unit test:

```rust,editable
{{#include ../../../crates/cats/gui/examples/web/tauri/mod.rs:example}}
```

This source does not link the Tauri runtime. A full Tauri application also needs a frontend, `tauri.conf.json`, and `build.rs`; on Linux the platform runtime uses GTK 3.

In the Tauri project's Rust back end, expose the function as a command and register it with the application:

```rust,ignore,editable
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to Tauri.")
}

tauri::Builder::default().invoke_handler(tauri::generate_handler![greet]);
```

The JavaScript front end imports and invokes the command:

```javascript
import { invoke } from "@tauri-apps/api/core";

const greeting = await invoke("greet", { name: "Ferris" });
```

On Windows, Tauri development requires Microsoft C++ Build Tools and Microsoft Edge WebView2. Linux builds require the platform's WebKitGTK development packages. See the [Tauri platform prerequisites][tauri~windows-prerequisites~website]↗ and the relevant OS guide before building.

## `dioxus` {#dioxus}

[![dioxus~website][c~dioxus~website~badge]][c~dioxus~website] [![dioxus][c~dioxus~docs~badge]][c~dioxus~docs] [![dioxus~crates.io][c~dioxus~crates.io~badge]][c~dioxus~crates.io] [![dioxus~repo][c~dioxus~repo~badge]][c~dioxus~repo] [![dioxus~lib.rs][c~dioxus~lib.rs~badge]][c~dioxus~lib.rs]{{hi:dioxus}}{{hi:Dom}}{{hi:Gui}}{{hi:React}}{{hi:Ui}}{{hi:Wasm}}

[`dioxus`][c~dioxus~docs]↗ lets you write reusable UI components in Rust with `rsx!`. Its renderer is selected for the target: the web renderer runs in the browser with WebAssembly, while desktop and mobile apps run Rust natively and render through the platform's system webview. Dioxus also has an experimental native renderer based on Blitz and WGPU. The desktop and mobile targets belong here; for the browser target, see the [WASM front-end frameworks section](../wasm/index.md#rust-front-end-frameworks).

Use the [Dioxus desktop guide][dioxus~desktop~website]↗ or [mobile guide][dioxus~mobile~website]↗ for target-specific setup. The shared example demonstrates a component; enable the matching Dioxus platform feature in a project scaffolded with `dx new`, then use `dx serve` to run it:

```rust,editable
{{#include ../../../crates/cats/gui/examples/web/dioxus.rs:example}}
```

## `wry` {#wry}

[![wry][c~wry~docs~badge]][c~wry~docs]{{hi:wry}} [![tao][c~tao~docs~badge]][c~tao~docs]{{hi:tao}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}{{hi:Web view}}

[`wry`][c~wry~docs]↗ is a cross-platform webview library; [`tao`][c~tao~docs]↗ provides the window and event loop. Use them directly when you need a webview in a native window but do not need Tauri's application commands, permission model, bundling, or updater. Your application owns window management, event handling, and packaging.

The Wry/Tao source remains available, but its Cargo registration, feature, and dependencies are commented out until a compatible update removes affected `glib 0.18`:

```rust,ignore
{{#include ../../../crates/cats/gui/examples/web/wry_window.rs:example}}
```

- **The webview comes from the platform.** Wry uses WebView2 on Windows, WKWebView on macOS and iOS, WebKitGTK on Linux, and Android System WebView on Android. Runtime versions and capabilities vary by platform. Linux builds need WebKitGTK and GTK development packages.
- **Rust and JavaScript communicate through IPC.** JavaScript can call `window.ipc.postMessage(text)`, handled by `WebViewBuilder::with_ipc_handler`; Rust can call `WebView::evaluate_script`. Validate messages and restrict navigation before loading untrusted content.
- **Load content deliberately.** `with_url` loads a page and `with_html` loads inline markup. Avoid allowing an application with an IPC bridge to navigate to arbitrary sites.

- **Choose [`tauri`][c~tauri~docs]↗{{hi:tauri}} for a full application.** It adds a command system, a permission model, bundling and installers, auto-update, and a JavaScript API.

## Related Topics {#related-topics .skip}

- [WASM front-end frameworks](../wasm/index.md#rust-front-end-frameworks).
- [Window Creation](./window_creation.md) and [File Dialogs](./file_dialogs.md).
- Native and hybrid UI toolkits.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [web_based_gui: write](https://github.com/john-cd/rust_howto/issues/394)
  where should we put Dioxus?
  [show a local web page in a native window](https://github.com/john-cd/rust_howto/issues/1952)
</div>
