# File Dialogs

{{#include file_dialogs.incl.md}}

For most common file dialog needs, [`rfd`][c~rfd~docs]↗{{hi:rfd}} is the recommended crate. If you're in a WASM context, you'll need to use JavaScript interop.

## `rfd` {#rfd}

[![rfd][c~rfd~docs~badge]][c~rfd~docs] [![rfd~crates.io][c~rfd~crates.io~badge]][c~rfd~crates.io] [![rfd~github][c~rfd~github~badge]][c~rfd~github] [![rfd~lib.rs][c~rfd~lib.rs~badge]][c~rfd~lib.rs]{{hi:rfd}}{{hi:Dialog}}{{hi:File}}{{hi:Ui}}

[`rfd`][c~rfd~docs]↗{{hi:rfd}} stands for Rusty File Dialog. "Platform-native open/save file dialogs. Can be used in conjunction with other UI libraries." ([blessed.rs](https://blessed.rs/crates)).

[`rfd`][c~rfd~docs]↗{{hi:rfd}} is the most popular and generally recommended crate for creating cross-platform file dialogs (open file, save file, open directory). It provides a simple and consistent API across different operating systems. It's actively maintained and a good choice for most projects. [`rfd`][c~rfd~docs]↗{{hi:rfd}} supports [asynchronous][p~asynchronous] file dialogs.

```rust,editable
{{#include ../../../crates/cats/gui/examples/file_dialogs/rfd.rs:example}}
```

## GUI Framework Integration {#skip}

Some [GUI][p~gui] frameworks, like [`iced`][c~iced~docs]↗{{hi:iced}} or [`egui`][c~egui~docs]↗{{hi:egui}}, have their own file dialog integration, but they often use the underlying [`rfd`][c~rfd~docs]↗{{hi:rfd}} crate or similar.

## WebAssembly (WASM) Considerations {#skip}

File dialogs in [WASM][p~wasm] are restricted due to security reasons. You'll typically need to use JavaScript APIs to interact with file systems in a WASM context. Crates like [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} can be used for this purpose.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[file_dialogs: write](https://github.com/john-cd/rust_howto/issues/381)
review in depth
</div>
