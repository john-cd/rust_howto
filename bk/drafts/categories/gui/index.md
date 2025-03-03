# GUI

[![cat-gui][cat-gui-badge]][cat-gui]{{hi::GUI}}

Tooling to help you create a graphical user interface.

## Status

[Are we GUI yet?][are-we-gui-yet?-website]⮳

GTK and Tauri are probably the only options which can be described as production-ready without caveats. The Rust native options are usable for simple projects but are all still quite incomplete.

See also the relevant section in [blessed.rs][blessed-rs-website]⮳.

## GUIs

### GTK

{{#include gtk.incl.md}}

## Web-based GUIs

{{#include web_based_gui.incl.md}}

### Immediate-mode GUIs

{{#include immediate_mode_gui.incl.md}}

### Retained-mode GUIs

{{#include retained_mode_gui.incl.md}}

## Other GUIs

{{#include other_gui.incl.md}}

### 2D Renderers

{{#include 2d_renderers.incl.md}}

### UI Layout

{{#include ui_layout.incl.md}}

### Text Layout

{{#include text_layout.incl.md}}

### Window creation

{{#include window_creation.incl.md}}

### File dialogs

{{#include file_dialogs.incl.md}}

### Clipboard

{{#include clipboard.incl.md}}

## See also

Turn any webpage into a desktop app with Rust [Pake][pake-github]⮳

[Logan Keenan - Client-Side Server with Rust- A New Approach to UI Development][client-side-server-with-rust]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize](https://github.com/john-cd/rust_howto/issues/397)

## Key Concepts

- Immediate Mode vs. Retained Mode: Immediate mode GUIs redraw everything every frame, while retained mode GUIs maintain a data structure representing the UI state.
- Event Handling: How the GUI responds to user input.
- Layout: How UI elements are arranged.
- Styling: How the UI looks.
- Cross-Platform: Whether the GUI can run on multiple operating systems.

## Immediate Mode GUI

- [`egui`][c-egui]⮳{{hi:egui}}: A very popular and easy-to-use immediate mode GUI library. Great for tools, games, and quick prototyping.

## Retained Mode GUI

- [`iced`][c-iced]⮳{{hi:iced}}: A cross-platform GUI library focused on simplicity, type safety, and a declarative style (using Elm architecture).
- [`tauri`][c-tauri]⮳{{hi:tauri}}: Framework for building desktop applications with web technologies (HTML, CSS, JavaScript). While not strictly a -GUI- crate itself, it allows you to build GUIs using web technologies.
- [`slint`][c-slint]⮳{{hi:slint}}: A declarative UI toolkit for embedded and desktop applications.

## GUI Frameworks (Higher Level)

- [`dioxus`][c-dioxus]⮳{{hi:dioxus}}: A library for building fast, portable, and ergonomic user interfaces with Rust. Similar in some ways to React.

## Bindings to Native GUI Toolkits

- `gtk-rs`: Bindings to GTK.
- [`qt-rs`][c-qt]⮳{{hi:qt-rs}}: Bindings to Qt.
- [`winit`][c-winit]⮳{{hi:winit}}: While primarily for window creation, [`winit`][c-winit]⮳{{hi:winit}} is often used as a foundation for building custom GUIs or integrating with other GUI libraries. It handles window events and input.

## Web-Based GUIs (Using Web Technologies)

- [`yew`][c-yew]⮳{{hi:yew}}: A modern Rust framework inspired by React for creating single-page web applications. Can be used to build web-based GUIs.
- [`seed`][c-seed]⮳{{hi:seed}}: Another web framework for building web apps.

## Other GUI-Related Crates

- [`druid`][c-druid]⮳{{hi:druid}}: A data-first Rust UI toolkit.
- [`conrod`][c-conrod]⮳{{hi:conrod}}: An older, but still usable, immediate mode GUI library.

## Choosing the right crate

- Simple GUIs, Tools, Games: [`egui`][c-egui]⮳{{hi:egui}} is an excellent choice.
- More Complex Applications, Declarative UI [`iced`][c-iced]⮳{{hi:iced}} or [`slint`][c-slint]⮳{{hi:slint}}.
- Web-Based GUIs [`yew`][c-yew]⮳{{hi:yew}} or [`seed`][c-seed]⮳{{hi:seed}}.
- Native Look and Feel `gtk-rs` or [`qt-rs`][c-qt]⮳{{hi:qt-rs}} (but require more setup and native dependencies).
- Web Technologies for Desktop [`tauri`][c-tauri]⮳{{hi:tauri}}.

For most common GUI needs, [`egui`][c-egui]⮳{{hi:egui}} and [`iced`][c-iced]⮳{{hi:iced}} are excellent starting points. If you're targeting the web, [`yew`][c-yew]⮳{{hi:yew}} or [`seed`][c-seed]⮳{{hi:seed}} are good choices. If you need a native look and feel or have existing experience with GTK or Qt, then their respective bindings might be appropriate. [`tauri`][c-tauri]⮳{{hi:tauri}} is a good option if you're comfortable with web technologies and want to build cross-platform desktop apps.

</div>
