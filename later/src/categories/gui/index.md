<!-- TODO Reviewed ??-??-2026 -->

# GUI

[![cat~gui][cat~gui~badge]][cat~gui]{{hi::GUI}}

Libraries and bindings for building graphical user interfaces in desktop and browser applications. This index groups them by application target and interaction model; the project pages provide the examples.

## Choosing the Right Framework

Framework maturity, maintenance, accessibility, styling, and platform support vary. Check the project's current documentation and native prerequisites against your deployment needs.

| Application need                                         | Examples                                                                                                   | Where to look                                                         |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| Native application using an existing GUI toolkit         | [`gtk4`][c~gtk4~docs]↗{{hi:GTK}}, [`qt`][c~qt~docs]↗{{hi:Qt}}, [`fltk-rs`][c~fltk~docs]↗{{hi:FLTK}}        | [Bindings to native GUI toolkits](#bindings-to-native-gui-toolkits)   |
| Immediate-mode UI for tools and interactive applications | [`egui`][c~egui~docs]↗{{hi:egui}}                                                                          | [Immediate-mode GUIs](#immediate-mode-guis)                           |
| Retained-mode or declarative UI                          | [`iced`][c~iced~docs]↗{{hi:iced}}, [`slint`][c~slint~docs]↗{{hi:slint}}                                    | [Retained-mode GUIs](#retained-mode-guis)                             |
| Desktop application using web technologies               | [`tauri`][c~tauri~docs]↗{{hi:tauri}}                                                                       | [Web-based desktop GUIs](#web-based-guis)                             |
| GUI that runs in a web browser                           | [`yew`][c~yew~docs]↗{{hi:yew}}, [`seed`][c~seed~docs]↗{{hi:seed}}, [`dioxus`][c~dioxus~docs]↗{{hi:dioxus}} | [Web Frameworks](#web-frameworks)                                     |
| Custom window and renderer                               | [`winit`][c~winit~docs]↗{{hi:winit}}, [`minifb`][c~minifb~docs]↗{{hi:minifb}}                              | [Window creation](#window-creation) and [2D renderers](#2d-renderers) |

GTK and Tauri are the options that are most production-ready without caveats; the Rust-native toolkits are usable for simple projects but are still maturing. For web front ends that also generate static pages, see [`perseus`][c~perseus~docs]↗{{hi:perseus}}.

For broader catalogs and surveys, see [Are We GUI Yet?][are-we-gui-yet?~website]↗, the GUI section of [Blessed.rs][blessed-rs~website]↗, and [The State of Rust GUI libraries][blog~state-rust-gui-libraries]↗.

## GUI Frameworks

### Bindings to Native GUI Toolkits

{{#include gtk.incl.md}}

Alternatively, [`fltk-rs`][c~fltk~docs]↗{{hi:fltk-rs}} provides Rust bindings for the [`FLTK`][fltk~website]↗ graphical user interface library.

### Immediate-mode GUIs

Immediate-mode GUIs describe the interface as application state changes, while retained-mode GUIs keep a persistent UI tree. This is a broad distinction; each toolkit defines its own update and rendering model.

{{#include immediate_mode_gui.incl.md}}

### Retained-mode GUIs

{{#include retained_mode_gui.incl.md}}

### Web-based GUIs

[`tauri`][c~tauri~docs]↗{{hi:tauri}} builds desktop applications with web technologies (HTML, CSS, and JavaScript). A webview-based desktop application is distinct from a site or browser-only application.

{{#include web_based_gui.incl.md}}

## Web Frameworks

For browser-based user interfaces, see [[web-programming | Web Programming]] and [[web-programming_http-server | Web Programming: HTTP Server]].

## GUI Components

### 2D Renderers

See [[graphics | Graphics]] and [[rendering | Rendering]].

For a minimal CPU-rendered desktop window, [`minifb`][c~minifb~docs]↗{{hi:minifb}} presents a pixel buffer directly. From `later/crates`, run it with `cargo run -p gui --example minifb_renderer --features minifb`.

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/minifb.rs:example}}
```

The optional Vello example renders vector shapes into a WGPU texture. From `later/crates`, run it with `cargo run -p gui --example vello_renderer --features vello`. It requires a compatible GPU adapter.

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/vello.rs:example}}
```

### UI Layout

{{#include ui_layout.incl.md}}

### Text Layout

{{#include text_layout.incl.md}}

### Window Creation

While primarily for window creation, [`winit`][c~winit~docs]↗{{hi:winit}} is often used as a foundation for building custom GUIs or integrating with other GUI libraries. It handles window events and input.

{{#include window_creation.incl.md}}

### File Dialogs

{{#include file_dialogs.incl.md}}

### Clipboard

{{#include clipboard.incl.md}}

## References

- Turn any webpage into a desktop app with Rust [Pake][pake~repo]↗.
- [Logan Keenan - Client-Side Server with Rust- A New Approach to UI Development][blog~client-side-server-with-rust]↗.

## Related Topics

- [[accessibility | Accessibility]].
- [[command-line-interface | Command Line Interface]].
- [[game_development | Game Development]].
- [[game-engines | Game Engines]].
- [[graphics | Graphics]].
- [[internationalization | Internationalization]].
- [[multimedia | Multimedia]].
- [[rendering | Rendering]].
- [[template-engine | Template Engine]].
- [[wasm | WASM]].
- [[web-programming | Web Programming]].
- [[web-programming_http-server | Web Programming: HTTP Server]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize](https://github.com/john-cd/rust_howto/issues/397)
[Are we (I)DE yet?][are-we-ide-yet?~website]↗.
[Porting a cross-platform GUI application to Rust][blog~porting-a-cross-platform-gui-application-to-rust]↗.
cover zola?
</div>
