# Clipboard

{{#include clipboard.incl.md}}

In most cases, the [`arboard`][c-arboard]⮳ crate will be all you need for basic clipboard functionality (reading and writing text). If you're in a [WASM][p-wasm] context, you'll need to use JavaScript interop. For more complex [data formats][p-data-formats], you'll likely need to handle them separately.

## `arboard` {#arboard}

[![arboard][c-arboard-badge]][c-arboard] [![arboard-crates.io][c-arboard-crates.io-badge]][c-arboard-crates.io] [![arboard-github][c-arboard-github-badge]][c-arboard-github] [![arboard-lib.rs][c-arboard-lib.rs-badge]][c-arboard-lib.rs]{{hi:arboard}}{{hi:Clipboard}}{{hi:Image}}

[`arboard`][c-arboard]⮳{{hi:arboard}} helps with image and text handling for the OS clipboard. A fork of [`rust-clipboard`][rust-clipboard]⮳{{hi:rust-clipboard}} that supports copy and pasting of both text and images on Linux (X11/Wayland), MacOS and Windows.

```rust,editable
{{#include ../../../crates/cats/gui/examples/clipboard/arboard.rs:example}}
```

## GUI Framework Integration {#skip}

Some [GUI][p-gui] frameworks, like [`iced`][c-iced]⮳{{hi:iced}} or [`egui`][c-egui]⮳{{hi:egui}}, have their own clipboard integration.

## WebAssembly (WASM) Considerations {#skip}

Clipboard access in [WASM][p-wasm] is restricted due to security reasons. You'll typically need to use JavaScript APIs to interact with the clipboard in a WASM context. Crates like [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} would be used for this JavaScript interop.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; expand](https://github.com/john-cd/rust_howto/issues/638)?
Cover `clipboard-win`
Details of [WASM][p-wasm] integration
</div>
