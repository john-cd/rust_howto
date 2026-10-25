<!-- TODO Reviewed ??-??-2026 -->

# Clipboard

{{#include clipboard.incl.md}}

Choose a clipboard API for the target environment and formats. [`arboard`][c~arboard~docs]↗{{hi:arboard}} covers common text and image operations across desktop platforms. Use a platform API such as [`clipboard-win`][c~clipboard-win~docs]↗{{hi:clipboard-win}} when Windows-specific clipboard formats are needed. A browser-based [WASM][p~wasm] application must use the browser Clipboard API through JavaScript bindings and follow browser permissions and activation rules.

## `arboard` {#arboard}

[![arboard][c~arboard~docs~badge]][c~arboard~docs] [![arboard~crates.io][c~arboard~crates.io~badge]][c~arboard~crates.io] [![arboard~repo][c~arboard~repo~badge]][c~arboard~repo] [![arboard~lib.rs][c~arboard~lib.rs~badge]][c~arboard~lib.rs]{{hi:arboard}}{{hi:Clipboard}}{{hi:Image}}

[`arboard`][c~arboard~docs]↗{{hi:arboard}} helps with image and text handling for the OS clipboard. Supports copy and pasting of both text and images on Linux (X11/Wayland), MacOS and Windows.

This example uses the builder API in `arboard` 3.7.0: `set().text(...)` writes text and `get().text()` reads it back. Clipboard operations can fail when a desktop clipboard service is unavailable or the clipboard is busy, so propagate their errors instead of assuming access will succeed.

```rust,editable
{{#include ../../../crates/cats/gui/examples/clipboard/arboard.rs:example}}
```

On Linux, X11 and Wayland clipboard contents are served by the application that set them. After that application drops its last `Clipboard` or exits, the contents may become unavailable unless another app or clipboard manager has requested or retained them. A long-running GUI should keep its `Clipboard` instance alive as appropriate; a short-lived command should account for this ownership behavior rather than assume the clipboard text will persist after exit.

## Windows Clipboard Formats {#windows-clipboard-formats}

[`clipboard-win`][c~clipboard-win~docs]↗{{hi:clipboard-win}} exposes Windows clipboard formats including Unicode text, HTML, bitmaps, file lists, and raw registered formats. Windows applications must open the clipboard before accessing it; keep the access scope short and handle the case where another application temporarily has it open. Use the standard Windows format when interoperating with other applications, and define a registered private format only when both sides agree on its representation.

## GUI Framework Integration {#gui-framework-integration}

Some [GUI][p~gui] frameworks, like [`iced`][c~iced~docs]↗{{hi:iced}} or [`egui`][c~egui~docs]↗{{hi:egui}}, have their own clipboard integration.

## WebAssembly (WASM) Considerations {#webassembly-wasm-considerations}

Use [`web-sys`][c~web-sys~docs]↗{{hi:web-sys}} to access `navigator.clipboard` from Rust/WASM. The browser API is asynchronous: text uses `read_text` and `write_text`, while rich clipboard items use `read` and `write` when supported. [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} provides the JavaScript interop.

Clipboard access is available only in secure contexts such as HTTPS, and browser policy may require permission or a recent user gesture. Reading is more restricted than writing, and support varies by browser and data format. Request access in response to a clear user action, propagate rejected promises, and provide a paste or copy event fallback where appropriate. Test the deployed origin in the browsers you support; native desktop clipboard access does not bypass browser security rules.

## Related Topics {#related-topics .skip}

- [[web_based_gui | Web-Based GUIs]].
- [[file_dialogs | File Dialogs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
