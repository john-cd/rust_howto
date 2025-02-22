# Clipboard

{{#include clipboard.incl.md}}

## `arboard` {#arboard}

[![arboard][c-arboard-badge]][c-arboard] [![arboard-crates.io][c-arboard-crates.io-badge]][c-arboard-crates.io] [![arboard-github][c-arboard-github-badge]][c-arboard-github] [![arboard-lib.rs][c-arboard-lib.rs-badge]][c-arboard-lib.rs]{{hi:arboard}}{{hi:Clipboard}}{{hi:Image}}

[`arboard`][c-arboard]⮳{{hi:arboard}} helps with image and text handling for the OS clipboard. A fork of [`rust-clipboard`][rust-clipboard]⮳{{hi:rust-clipboard}} that supports copy and pasting of both text and images on Linux (X11/Wayland), MacOS and Windows.

```rust,editable
{{#include ../../../crates/cats/gui/examples/clipboard/arboard.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write; expand](https://github.com/john-cd/rust_howto/issues/638)?

In most cases, the `clipboard` crate will be all you need for basic clipboard functionality (reading and writing text).If you're in a [Wasm][p-wasm] context, you'll need to use JavaScript interop.For more complex [data formats][p-data-formats], you'll likely need to handle them separately.

## Clipboard Access

`clipboard`: This is the most common and generally recommended crate for interacting with the system clipboard in Rust. It provides a cross-platform API for reading and writing text to the clipboard.

### GUI Framework Integration

Some [GUI][p-gui] frameworks, like `iced` or `egui`, might have their own clipboard integration, but often they use the underlying clipboard crate.

### WebAssembly (WASM) Considerations

Clipboard access in [WASM][p-wasm] is restricted due to security reasons. You'll typically need to use JavaScript [APIs][p-apis] to interact with the clipboard in a WASM context. Crates like `wasm-bindgen` would be used for this JavaScript interop.

### Platform-Specific Considerations

The `clipboard` crate handles platform differences for you, so you generally don't need to worry about the specifics of each operating system's clipboard API.

### Data Formats

The `clipboard` crate primarily focuses on text. For other [data formats][p-data-formats] (images, files, etc.), you might need to explore more specialized crates or platform-specific APIs.

</div>
