# File Dialogs

{{#include file_dialogs.incl.md}}

## `rfd` {#rfd}

[![rfd][c-rfd-badge]][c-rfd] [![rfd-crates.io][c-rfd-crates.io-badge]][c-rfd-crates.io] [![rfd-github][c-rfd-github-badge]][c-rfd-github] [![rfd-lib.rs][c-rfd-lib.rs-badge]][c-rfd-lib.rs]{{hi:rfd}}{{hi:Dialog}}{{hi:File}}{{hi:Ui}}

`rfd` stands for Rusty File Dialog. "Platform-native open/save file dialogs. Can be used in conjunction with other UI libraries."

```rust,editable
{{#include ../../../crates/cats/gui/examples/file_dialogs/rfd.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[file_dialogs: write (P2)](https://github.com/john-cd/rust_howto/issues/381)

For most common file dialog needs, `rfd` is the recommended crate. It's cross-platform, easy to use, and well-maintained. If you're in a Wasm context, you'll need to use JavaScript interop.

## Cross-Platform File Dialogs

- `rfd`: This is the most popular and generally recommended crate for creating cross-platform file dialogs (open file, save file, open directory). It provides a simple and consistent API across different operating systems. It's actively maintained and a good choice for most projects.

### GUI Framework Integration

Some GUI frameworks, like `iced` or `egui`, might have their own file dialog integration, but often they use the underlying `rfd` crate or similar.

### Platform-Specific Considerations

The `rfd` crate handles platform differences for you, so you generally don't need to worry about the specifics of each operating system's file dialog API.

### WebAssembly (WASM) Considerations

File dialogs in WASM are restricted due to security reasons. You'll typically need to use JavaScript APIs to interact with file systems in a Wasm context. Crates like `wasm-bindgen` would be used for this JavaScript interop.

### Asynchronous Operations

`rfd` supports asynchronous file dialogs.

</div>
