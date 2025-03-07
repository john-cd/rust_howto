# Retained Mode Native GUI

{{#include retained_mode_gui.incl.md}}

## `iced` {#iced}

[![iced-website][c-iced-website-badge]][c-iced-website] [![iced][c-iced-badge]][c-iced] [![iced-crates.io][c-iced-crates.io-badge]][c-iced-crates.io] [![iced-github][c-iced-github-badge]][c-iced-github] [![iced-lib.rs][c-iced-lib.rs-badge]][c-iced-lib.rs]{{hi:iced}}{{hi:Graphics}}{{hi:Gui}}{{hi:Interface}}{{hi:Ui}}{{hi:Widgets}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[`iced`][c-iced]⮳ is a cross-platform GUI library for Rust, inspired by [`Elm`](https://elm-lang.org). Retained mode UI with a nice API. It's usable for basic apps, but has a number of missing features including multiple [windows][p-windows], layers, and proper text rendering.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/iced.rs:example}}
```

## `floem` {#floem}

[![floem][c-floem-badge]][c-floem] [![floem-crates.io][c-floem-crates.io-badge]][c-floem-crates.io] [![floem-github][c-floem-github-badge]][c-floem-github] [![floem-lib.rs][c-floem-lib.rs-badge]][c-floem-lib.rs]{{hi:floem}}

[`floem`][c-floem]⮳{{hi:floem}} is a native Rust UI library with fine-grained reactivity, and inspired by [`xilem`][c-xilem-github]{{hi:xilem}}⮳, [`leptos`][c-leptos-github]{{hi:leptos}}⮳ and [`rui`][c-rui-github]{{hi:rui}}⮳. [`floem`][c-floem]⮳{{hi:floem}} aims to be a high performance declarative UI library requiring minimal user effort.

[`floem`][c-floem]⮳{{hi:floem}} is currently more complete than any of them for native UI. Used by the `lapce` text editor.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/floem.rs:example}}
```

## `vizia` {#vizia}

[![vizia][c-vizia-badge]][c-vizia] [![vizia-crates.io][c-vizia-crates.io-badge]][c-vizia-crates.io] [![vizia-github][c-vizia-github-badge]][c-vizia-github] [![vizia-lib.rs][c-vizia-lib.rs-badge]][c-vizia-lib.rs]{{hi:vizia}}

[`vizia`][c-vizia]⮳{{hi:vizia}} is a declarative desktop GUI framework.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/vizia.rs:example}}
```

## Other GUI frameworks

### `slint` {#slint}

[![slint-website][c-slint-website-badge]][c-slint-website] [![slint][c-slint-badge]][c-slint] [![slint-crates.io][c-slint-crates.io-badge]][c-slint-crates.io] [![slint-github][c-slint-github-badge]][c-slint-github] [![slint-lib.rs][c-slint-lib.rs-badge]][c-slint-lib.rs]{{hi:slint}}{{hi:Design}}{{hi:Graphics}}{{hi:Gui}}{{hi:Toolkit}}{{hi:Ui}}[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}[![cat-rendering::engine][cat-rendering::engine-badge]][cat-rendering::engine]{{hi:Rendering engine}}

[`slint`][c-slint]⮳{{hi:slint}} is a GUI toolkit to efficiently develop fluid graphical user interfaces for embedded devices and desktop applications.
Possibly the most complete rust-native UI library. But note that it's dual GPL3/commercial licensed.

- [slint.dev][c-slint-website]⮳.
- [Slint (github)][c-slint-github]⮳.
- [madewithslint.com][c-slint-madewithslint-website]⮳.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/slint.rs:example}}
```

### Azul {#azul}

[![azul-website][c-azul-website-badge]][c-azul-website] [![azul][c-azul-badge]][c-azul] [![azul-crates.io][c-azul-crates.io-badge]][c-azul-crates.io] [![azul-github][c-azul-github-badge]][c-azul-github] [![azul-lib.rs][c-azul-lib.rs-badge]][c-azul-lib.rs]{{hi:azul}}{{hi:User-interface}}{{hi:Gui}}{{hi:Svg}}{{hi:Graphics}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

`Azul` GUI is a free, functional, reactive GUI framework for rapid development of desktop applications written in Rust, C, and C++, using the Mozilla `WebRender` rendering engine.

Azul is a document object model, similar to HTML and CSS. Because Azul leverages `WebRender`, it provides features like gradients, box shadows, border styling, and CSS transforms. Azul is licensed under MPL-2.0. You can build proprietary applications using `azul` without having to publish your code, but you have to publish changes made to the library itself.

{{#example azul}}

### `xilem` {#xilem}

[![xilem-website][c-xilem-website-badge]][c-xilem-website] [![xilem][c-xilem-badge]][c-xilem] [![xilem-crates.io][c-xilem-crates.io-badge]][c-xilem-crates.io] [![xilem-github][c-xilem-github-badge]][c-xilem-github] [![xilem-lib.rs][c-xilem-lib.rs-badge]][c-xilem-lib.rs]{{hi:xilem}}{{hi:Gui}}{{hi:Ui}}{{hi:Native}}{{hi:Gpu}}{{hi:Performance}}[![cat-internationalization][cat-internationalization-badge]][cat-internationalization]{{hi:Internationalization (i18n)}}[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}[![cat-accessibility][cat-accessibility-badge]][cat-accessibility]{{hi:Accessibility}}

[`xilem`][c-xilem]⮳{{hi:xilem}} is the replacement for [`druid`][c-druid]⮳{{hi:druid}}, based on the more interoperable [`vello`][c-vello]⮳{{hi:vello}} and [`glazier`][c-glazier]⮳{{hi:glazier}} crates. However, it's currently not complete enough to be usable.

If you’re a Swift developer, you should find Xilem easy to use because its syntax and concepts resemble SwiftUI's.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/xilem.rs:example}}
```

### GPUI {#gpui}

[![gpui][c-gpui-badge]][c-gpui] [![gpui-crates.io][c-gpui-crates.io-badge]][c-gpui-crates.io] [![gpui-github][c-gpui-github-badge]][c-gpui-github] [![gpui-lib.rs][c-gpui-lib.rs-badge]][c-gpui-lib.rs]{{hi:gpui}}
[![gpui][c-gpui-badge]][c-gpui]{{hi:gpui}}

[gpui.rs][gpui.rs]⮳ is a fast, productive UI framework for Rust from the creators of the [Zed][c-zed-website]⮳ text editor.

### `druid` {#druid}

[![druid][c-druid-badge]][c-druid] [![druid-crates.io][c-druid-crates.io-badge]][c-druid-crates.io] [![druid-github][c-druid-github-badge]][c-druid-github] [![druid-lib.rs][c-druid-lib.rs-badge]][c-druid-lib.rs]{{hi:druid}}{{hi:Toolkit}}{{hi:Gui}}{{hi:Ui}}[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[`druid`][c-druid]⮳{{hi:druid}} is a data-first Rust-native UI design toolkit (experimental). Druid is a relatively mature alternative to Iced / Slint, however it has been discontinued in favor of Xilem, so its use for new projects is discouraged.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[retained_mode_gui: organize/write](https://github.com/john-cd/rust_howto/issues/389)
</div>
