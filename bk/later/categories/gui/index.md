# GUI

[![cat~gui][cat~gui~badge]][cat~gui]{{hi::GUI}}

Tooling to help you create a graphical user interface.

## Choosing the Right Framework

GTK and Tauri are probably the only options which can be described as production-ready without caveats. The Rust native options are usable for simple projects but are all still quite incomplete.

For most common GUI needs, [`egui`][c~egui~docs]↗{{hi:egui}} and [`iced`][c~iced~docs]↗{{hi:iced}} are excellent starting points. If you're targeting the web, [`yew`][c~yew~docs]↗{{hi:yew}} or [`seed`][c~seed~docs]↗{{hi:seed}} are good choices. If you need a native look and feel or have existing experience with GTK or Qt, then their respective bindings might be appropriate. [`tauri`][c~tauri~docs]↗{{hi:tauri}} is a good option if you're comfortable with web technologies and want to build cross-platform desktop apps.

| GUI type | Framework |
|---|---|
| Simple GUIs, Tools, Games | [`egui`][c~egui~docs]↗{{hi:egui}} is an excellent choice. |
| More Complex Applications, Declarative UI | [`iced`][c~iced~docs]↗{{hi:iced}} or [`slint`][c~slint~docs]↗{{hi:slint}}. |
| Native Look and Feel | `gtk-rs` or [`qt-rs`][c~qt~docs]↗{{hi:qt-rs}} (but require more setup and native dependencies) |
| Web Technologies for Desktop | [`tauri`][c~tauri~docs]↗{{hi:tauri}}. |
| Web-Based GUIs | [`yew`][c~yew~docs]↗{{hi:yew}} or [`seed`][c~seed~docs]↗{{hi:seed}}. See [[web-programming | Web Programming]] and [[web-programming_http-server | Web Programming: HTTP Server]] |

See also

- [Are we GUI yet?][are-we-gui-yet?~website]↗.
- the relevant section in [blessed.rs][blessed-rs~website]↗.
- [The State of Rust GUI libraries](https://blog.logrocket.com/state-rust-gui-libraries/).
- [LibHunt](https://www.libhunt.com/l/rust/topic/gui).
- [simplifycpp.org](https://simplifycpp.org/?id=a0507).

## GUI Frameworks

### Bindings to Native GUI Toolkits

{{#include gtk.incl.md}}

Alternatively, look at [`fltk-rs`][c~fltk~docs]↗{{hi:fltk-rs}}, which offers Rust bindings for the [`FLTK`](https://www.fltk.org) 1.4 Graphical User Interface library.

### Immediate-mode GUIs

Immediate-mode GUIs redraw everything every frame, while retained-mode GUIs maintain a data structure representing the UI state.

{{#include immediate_mode_gui.incl.md}}

### Retained-mode GUIs

{{#include retained_mode_gui.incl.md}}

### Web-based GUIs

[`tauri`][c~tauri~docs]↗{{hi:tauri}} is a cross-platform framework for building desktop applications with web technologies (HTML, CSS, JavaScript).

{{#include web_based_gui.incl.md}}

## Web Frameworks

See [[web-programming | Web Programming]] and [[web-programming_http-server | Web Programming HTTP Server]].

## GUI Components

### 2D Renderers

See [[graphics | Graphics]] and [[rendering | Rendering]].

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

## References

- Turn any webpage into a desktop app with Rust [Pake][pake~github]↗.
- [Logan Keenan - Client-Side Server with Rust- A New Approach to UI Development][blog~client-side-server-with-rust]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize](https://github.com/john-cd/rust_howto/issues/397)
review which GUI framework should be recommended - make a table; review links in Choosing the right framework section again
cover perseus / zola?

- [Are we (I)DE yet?](https://areweideyet.com/)

</div>
