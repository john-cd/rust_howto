<!-- TODO Reviewed ??-??-2026 -->

# File Dialogs

{{#include file_dialogs.incl.md}}

Use [`rfd`][c~rfd~docs]↗{{hi:rfd}} for native file and folder pickers, save dialogs, and simple message boxes. It provides synchronous and asynchronous APIs across desktop platforms, with asynchronous file picking available in browsers.

## `rfd` {#rfd}

[![rfd][c~rfd~docs~badge]][c~rfd~docs] [![rfd~crates.io][c~rfd~crates.io~badge]][c~rfd~crates.io] [![rfd~repo][c~rfd~repo~badge]][c~rfd~repo] [![rfd~lib.rs][c~rfd~lib.rs~badge]][c~rfd~lib.rs]{{hi:rfd}}{{hi:Dialog}}{{hi:File}}{{hi:Ui}}

[`rfd`][c~rfd~docs]↗{{hi:rfd}} (Rusty File Dialogs) exposes a consistent API for common native dialog tasks. Use `FileDialog` when blocking the current thread is acceptable, or `AsyncFileDialog` when the dialog should fit an async application.

```rust,editable
{{#include ../../../crates/cats/gui/examples/file_dialogs/rfd.rs:example}}
```

## Open, select, and save {#open-select-and-save}

Use `FileDialog::pick_file()` or `pick_files()` to open one or more files, `pick_folder()` to choose a directory, and `save_file()` to choose an output path. Configure a title, extension filters, or a starting location before showing the dialog. These methods return `Option`; `None` means the user canceled, so handle cancellation as a normal result.

`AsyncFileDialog` provides futures for async applications. It is also the file-picker API available on WebAssembly; browser-specific behavior is described below.

## Message boxes {#message-boxes}

`rfd::MessageDialog` displays a simple native alert or confirmation with a title, description, severity, and button set. Call `show()` to receive the user's response. Use `AsyncMessageDialog` when the surrounding application needs an asynchronous result. On Linux, message dialogs also rely on the `zenity` utility.

## GUI Framework Integration {#gui-framework-integration}

Some [GUI][p~gui] frameworks, such as [`iced`][c~iced~docs]↗{{hi:iced}} or [`egui`][c~egui~docs]↗{{hi:egui}}, provide their own file-dialog integrations. These can wrap `rfd` or use an in-app picker when a dialog should match the framework's visual style.

## Linux backends {#linux-backends}

On Linux and BSD, `rfd` enables the `xdg-portal` backend by default. It delegates to the desktop's GTK or KDE portal; the application needs a supported portal service and D-Bus. If that path is unavailable, `rfd` can fall back to `zenity`, which is also used for message dialogs. Projects that choose the GTK backend instead can disable default features and enable `gtk3`; building that configuration requires GTK 3 development libraries.

## WebAssembly {#webassembly-wasm-considerations}

Browsers support asynchronous file picking, but not folder selection or a native save-path dialog. The browser controls the picker and grants access only to files the user selects; it does not give an app an arbitrary local path. Read the returned `FileHandle` with `.read().await`. Writing a `FileHandle` prompts a browser download instead of returning a filesystem path.

## Alternatives {#alternatives}

[`native-dialog`][c~native-dialog~docs]↗{{hi:native-dialog}} offers a separate builder-style API for file choosers and message boxes and has an independently maintained [repository][c~native-dialog~repo]. Its Linux and BSD implementation requires `zenity`, `kdialog`, or `yad` to be installed.

[`tinyfiledialogs`][c~tinyfiledialogs~docs]↗{{hi:tinyfiledialogs}} is a small Rust binding to a bundled C library. The [Rust wrapper][c~tinyfiledialogs~repo] describes updates as sporadic; the [C upstream][tinyfiledialogs~upstream~website] continues to release separately, so check the wrapper and library versions before adopting it.

## Related Topics {#related-topics .skip}

- [GUI frameworks][p~gui].
- [WebAssembly][p~wasm].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [file_dialogs: write](https://github.com/john-cd/rust_howto/issues/381)
  review in depth.
</div>
