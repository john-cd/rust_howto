<!-- TODO Reviewed ??-??-2026. -->

# GUI

Comprehensive examples of Rust GUI frameworks including Slint, Egui, Iced, and Tauri. It demonstrates integration with key ecosystem crates including `anyhow`, `arboard`, `baseview`, `clap`, `cosmic-text`, `dioxus`.

## Examples Covered

- **2d Renderers** (`examples/2d_renderers/`): Demonstrates concepts like `femtovg`, `minifb`, `skia_safe`, `vello`.
- Vello is a standalone `vello_renderer` example behind the `vello` feature. It builds on Windows; running it requires a compatible GPU adapter.
- **Clipboard** (`examples/clipboard/`): Demonstrates concepts like `arboard`.
- **File Dialogs** (`examples/file_dialogs/`): Demonstrates concepts like `rfd`.
- **Gtk** (`examples/gtk/`): Demonstrates concepts like `gtk4`, `relm4`.
- **Immediate Mode GUI** (`examples/immediate_mode_gui/`): Demonstrates concepts like `egui`, `egui_headless`, `egui_kittest`, `egui_log_viewer`, `egui_plot`, `egui_table`, `egui_tiles`, `egui_toasts_icons`.
- **Retained Mode GUI** (`examples/retained_mode_gui/`): Demonstrates concepts like `floem`, `iced`, `slint`, `xilem`.
- **Text Layout** (`examples/text_layout/`): Demonstrates concepts like `cosmic_text`, `parley`.
- **Ui Layout** (`examples/ui_layout/`): Demonstrates concepts like `morphorm`, `taffy`.
- **Web** (`examples/web/`): Demonstrates concepts like `dioxus`, `wry_window`.
- **Window Creation** (`examples/window_creation/`): Demonstrates concepts like `baseview`, `tao`, `winit`.

## Usage

To check or run examples in this crate:

```console
cargo check --package gui --examples
cargo test --package gui
```

Code examples correspond to chapters in `later/src/categories/gui` or related book sections.

## Optional GUI paths

From the repository root, run the Iced counter independently of the other
retained-mode frameworks:

```sh
just cargo run --manifest-path bk/platform/Cargo.toml --locked -p gui --example iced --features iced
```

For [#2423](https://github.com/john-cd/rust_howto/issues/2423), the standalone
target in this change was built from base revision `f18b401f9` with only the
`iced` feature on native Windows (build 26300). The counter rendered `0`,
clicking **+** and **−** displayed `1` and `0`, and Alt+F4 exited with code 0.
The counter's headless state test also passed.

| Path | Disposition |
| --- | --- |
| Floem | Keep the opt-in `floem` subcommand of `retained_mode_gui` (features `slint,floem`). Native execution was deferred. |
| Xilem | Keep the existing experimental standalone `xilem` target behind the `xilem` feature. Native execution was deferred. |
| Tauri | Keep the plain Rust greeting in `web` (features `dioxus,tauri`). A native Tauri run requires the separately scaffolded frontend, build script, and system webview described in the [book](../../../src/categories/gui/web_based_gui.md#tauri). |
| Tao and Wry | Keep the sample sources while the GTK 3 / `glib 0.18` dependency chain blocks the workspace integration. Follow [#2213](https://github.com/john-cd/rust_howto/issues/2213) for Wry IPC validation. |

## Native Slint counter verification

From the repository root on a Windows desktop, run:

```powershell
$env:SLINT_BACKEND = 'winit-software'
just cargo run --manifest-path bk/platform/Cargo.toml --locked -p gui --example retained_mode_gui --features slint -- slint
Remove-Item Env:SLINT_BACKEND
```

For [#2427](https://github.com/john-cd/rust_howto/issues/2427), the corrected
recipe was built from base revision `f18b401f9` with this change and exercised on
Windows build 26300 with Slint 1.18.1. The original recipe showed literal
`{root.counter}` text. After fixing the binding, the window displayed
`Count: 0`. Tab focused **Increment**, Space changed the label to `Count: 1`,
and a pointer click changed it to `Count: 2`. The layout redrew at both the
maximized and restored sizes without clipping the button or count. In a fresh
run, Win+Up and Win+Down maximized and restored the window with the count
still at zero. Alt+F4 closed the window with exit code 0.

The headless baseline can run without an interactive desktop:

```sh
just cargo test --manifest-path bk/platform/Cargo.toml --locked -p gui --example retained_mode_gui --features slint
```

The headless callback test now asserts both the counter value and label before
and after invoking the callback. It passed along with the four CLI tests.

## Bounded Background Work

The `background_work` example demonstrates revision-tagged replies, errors and shutdown:

```sh
just cargo test --manifest-path bk/platform/Cargo.toml --locked -p gui --example background_work
just cargo run --manifest-path bk/platform/Cargo.toml --locked -p gui --example background_work -- --smoke
```

The smoke mode needs a native desktop, completes one calculation and closes the window.
See `later/src/categories/gui/background_work.md`; headless state tests do not certify
screen-reader, keyboard or resize behavior.
