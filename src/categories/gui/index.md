# GUI

Tooling to help you create a graphical user interface.

{{#include index.incl.md}}

[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[Are we GUI yet?][are-we-gui-yet?-website]⮳

GTK and Tauri are probably the only options which can be described as production-ready without caveats. The Rust native options are usable for simple projects but are all still quite incomplete.

See the relevant section in [blessed.rs][blessed-rs-website]⮳

## GTK

### gtk4

[![gtk4][c-gtk4-badge]][c-gtk4]{{hi:gtk4}}
[![gtk4-crates.io][c-gtk4-crates.io-badge]][c-gtk4-crates.io]
[![gtk4-github][c-gtk4-github-badge]][c-gtk4-github]
[![gtk4-lib.rs][c-gtk4-lib.rs-badge]][c-gtk4-lib.rs]

Rust bindings to GTK4. These are quite well supported, although you'll often need to use the C documentation.

### relm4

[![relm4][c-relm4-badge]][c-relm4]{{hi:relm4}}
[![relm4-crates.io][c-relm4-crates.io-badge]][c-relm4-crates.io]
[![relm4-github][c-relm4-github-badge]][c-relm4-github]
[![relm4-lib.rs][c-relm4-lib.rs-badge]][c-relm4-lib.rs]

A higher-level library that sits on top of gtk4-rs

## Web-based GUI

### dioxus

[![dioxus][c-dioxus-badge]][c-dioxus]{{hi:dioxus}}
[![dioxus-crates.io][c-dioxus-crates.io-badge]][c-dioxus-crates.io]
[![dioxus-github][c-dioxus-github-badge]][c-dioxus-github]
[![dioxus-lib.rs][c-dioxus-lib.rs-badge]][c-dioxus-lib.rs]

A very nice API layer that has Tauri, Web, and TUI renderers. A native renderer is coming soon.

### Tauri

[![tauri][c-tauri-badge]][c-tauri]{{hi:tauri}}  [![tauri-website][c-tauri-website-badge]][c-tauri-website] [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}
[![tauri-crates.io][c-tauri-crates.io-badge]][c-tauri-crates.io]
[![tauri-github][c-tauri-github-badge]][c-tauri-github]
[![tauri-lib.rs][c-tauri-lib.rs-badge]][c-tauri-lib.rs]

[`tauri`][c-tauri]⮳ is an app construction toolkit that lets you build software for all major desktop operating systems using web technologies. It is similar to Electron.

Electron-like web-based UI. Except it uses system webviews rather than shipping chromium, and non-UI code is written in Rust rather than node.js

## Immediate Mode Native GUI

### egui

[![egui][c-egui-badge]][c-egui]{{hi:egui}}  [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}
[![egui-crates.io][c-egui-crates.io-badge]][c-egui-crates.io]
[![egui-github][c-egui-github-badge]][c-egui-github]
[![egui-lib.rs][c-egui-lib.rs-badge]][c-egui-lib.rs]

[`egui`][c-egui]{{hi:egui}}⮳ is an easy-to-use immediate mode GUI that runs on both web and native. [`egui`][c-egui]{{hi:egui}}⮳ aims to be the best choice when you want a simple way to create a GUI, or you want to add a GUI to a game engine.

Immediate-mode UI. Lots of widgets. The most useable out of the box if your needs are simple and you don't need to customise of the look and feel.

- [egui.rs][c-egui-website]⮳
- [egui_demo_lib][c-egui_demo_lib-github]⮳
- [egui widgets][c-egui-widgets]⮳
- [egui containers][c-egui-containers]⮳
- [egui struct.Ui][c-egui::Ui]⮳

## Retained Mode Native GUI

### iced

[![iced][c-iced-badge]][c-iced]{{hi:iced}}  [![iced-github][c-iced-github-badge]][c-iced-github] is a cross-platform GUI library for Rust, inspired by Elm.

[![iced][c-iced-badge]][c-iced]{{hi:iced}}
[![iced-crates.io][c-iced-crates.io-badge]][c-iced-crates.io]
[![iced-github][c-iced-github-badge]][c-iced-github]
[![iced-lib.rs][c-iced-lib.rs-badge]][c-iced-lib.rs]

Retained mode UI with a nice API. It's useable for basic apps, but has a number of missing features including multiple windows, layers, and proper text rendering.

### floem

[![floem][c-floem-badge]][c-floem]{{hi:floem}}
[![floem-crates.io][c-floem-crates.io-badge]][c-floem-crates.io]
[![floem-github][c-floem-github-badge]][c-floem-github]
[![floem-lib.rs][c-floem-lib.rs-badge]][c-floem-lib.rs]

- A native Rust UI library with fine-grained reactivity. Inspired by [Xilem][c-xilem-github]{{hi:xilem}}⮳, [Leptos][c-leptos-github]{{hi:leptos}}⮳ and [rui][c-rui-github]{{hi:rui}}⮳. Floem aims to be a high performance declarative UI library requiring minimal user effort.
- [Floem (github)][c-floem-github]⮳
- [Floem (docs)][c-floem]{{hi:floem}}⮳

Inspired by Xilem, Leptos and rui, floem is currently more complete than any of them for native UI. Used by the `lapce` text editor.

## Other GUI frameworks

### xilem

[![xilem][c-xilem-badge]][c-xilem]{{hi:xilem}}
[![xilem-crates.io][c-xilem-crates.io-badge]][c-xilem-crates.io]
[![xilem-github][c-xilem-github-badge]][c-xilem-github]
[![xilem-lib.rs][c-xilem-lib.rs-badge]][c-xilem-lib.rs]

The replacement for Druid based on the more interoperable Vello and Glazier crates. However, it's currently not complete enough to be usable.

### slint

[![slint][c-slint-badge]][c-slint]{{hi:slint}}  [![slint-website][c-slint-website-badge]][c-slint-website]  [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[![slint-crates.io][c-slint-crates.io-badge]][c-slint-crates.io]
[![slint-github][c-slint-github-badge]][c-slint-github]
[![slint-lib.rs][c-slint-lib.rs-badge]][c-slint-lib.rs]

Possibly the most complete rust-native UI library. But note that it's dual GPL3/commercial licensed.

- [slint.dev][c-slint-website]⮳
- [Slint (github)][c-slint-github]⮳
- [madewithslint.com][c-slint-madewithslint-website]⮳

### druid

[![druid][c-druid-badge]][c-druid]{{hi:druid}}  [![druid-github][c-druid-github-badge]][c-druid-github] is a data-first Rust-native UI design toolkit (experimental).

Druid is a relatively mature alternative to Iced/Slint, however it has been discontinued in favour of Xilem so it's use for new projects is discouraged.

### GPUI

[![gpui][c-gpui-badge]][c-gpui]{{hi:gpui}}
[![gpui-crates.io][c-gpui-crates.io-badge]][c-gpui-crates.io]
[![gpui-github][c-gpui-github-badge]][c-gpui-github]
[![gpui-lib.rs][c-gpui-lib.rs-badge]][c-gpui-lib.rs]

- [gpui.rs][gpui.rs]{{hi:gpui.rs}}⮳ A fast, productive UI framework for Rust from the creators of [Zed][c-zed-website]⮳

High performance framework used in the Zed text editor. Now available on macOS and linux.

## See also

Turn any webpage into a desktop app with Rust [Pake][pake-github]⮳

[Logan Keenan - Client-Side Server with Rust- A New Approach to UI Development][client-side-server-with-rust]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO organize
</div>
