<!-- TODO Reviewed ??-??-2026 -->

# GUI

Comprehensive examples of Rust GUI frameworks including Slint, Egui, Iced, and Tauri. It demonstrates integration with key ecosystem crates including `anyhow`, `arboard`, `baseview`, `clap`, `cosmic-text`, `dioxus`.

## Examples Covered

- **2d Renderers** (`examples/2d_renderers/`): Demonstrates concepts like `femtovg`, `minifb`, `skia_safe`, `vello`.
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

Code examples correspond to chapters in `bk/src/categories/gui` or related book sections.
