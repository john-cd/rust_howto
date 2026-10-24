<!-- TODO Reviewed ??-??-2026 -->

# Clipboard

{{#include clipboard.incl.md}}

In most cases, the [`arboard`][c~arboard~docs]↗ crate will be all you need for basic clipboard functionality (reading and writing text). If you're in a [WASM][p~wasm] context, you'll need to use JavaScript interop. For more complex [data formats][p~data-formats], you'll likely need to handle them separately.

## `arboard` {#arboard}

[![arboard][c~arboard~docs~badge]][c~arboard~docs] [![arboard~crates.io][c~arboard~crates.io~badge]][c~arboard~crates.io] [![arboard~repo][c~arboard~repo~badge]][c~arboard~repo] [![arboard~lib.rs][c~arboard~lib.rs~badge]][c~arboard~lib.rs]{{hi:arboard}}{{hi:Clipboard}}{{hi:Image}}

[`arboard`][c~arboard~docs]↗{{hi:arboard}} helps with image and text handling for the OS clipboard. Supports copy and pasting of both text and images on Linux (X11/Wayland), MacOS and Windows.

This example uses the builder API in `arboard` 3.7.0: `set().text(...)` writes text and `get().text()` reads it back. Clipboard operations can fail when a desktop clipboard service is unavailable or the clipboard is busy, so propagate their errors instead of assuming access will succeed.

```rust,editable
{{#include ../../../crates/cats/gui/examples/clipboard/arboard.rs:example}}
```

On Linux, X11 and Wayland clipboard contents are served by the application that set them. After that application drops its last `Clipboard` or exits, the contents may become unavailable unless another app or clipboard manager has requested or retained them. A long-running GUI should keep its `Clipboard` instance alive as appropriate; a short-lived command should account for this ownership behavior rather than assume the clipboard text will persist after exit.

## GUI Framework Integration {#gui-framework-integration}

Some [GUI][p~gui] frameworks, like [`iced`][c~iced~docs]↗{{hi:iced}} or [`egui`][c~egui~docs]↗{{hi:egui}}, have their own clipboard integration.

## WebAssembly (WASM) Considerations {#webassembly-wasm-considerations}

Clipboard access in [WASM][p~wasm] is restricted due to security reasons. You'll typically need to use JavaScript APIs to interact with the clipboard in a WASM context. Crates like [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} would be used for this JavaScript interop.

## Related Topics {#related-topics .skip}

- GUI interoperability and input.
- Cross-platform clipboard APIs.
- WebAssembly clipboard handling.
- File dialogs and drag-and-drop.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
Updated the example to use the current fallible builder API, added an ignored desktop clipboard round-trip test, and documented Linux clipboard ownership and process-exit behavior.

[write; expand](https://github.com/john-cd/rust_howto/issues/638)?
Cover `clipboard-win`.
Details of [WASM][p~wasm] integration.
</div>
