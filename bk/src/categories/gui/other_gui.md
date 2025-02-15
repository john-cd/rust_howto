# Other GUI frameworks

{{#include other_gui.incl.md}}

## `xilem` {#xilem}

[![xilem-website][c-xilem-website-badge]][c-xilem-website] [![xilem][c-xilem-badge]][c-xilem] [![xilem-crates.io][c-xilem-crates.io-badge]][c-xilem-crates.io] [![xilem-github][c-xilem-github-badge]][c-xilem-github] [![xilem-lib.rs][c-xilem-lib.rs-badge]][c-xilem-lib.rs]{{hi:xilem}}{{hi:Gui}}{{hi:Ui}}{{hi:Native}}{{hi:Gpu}}{{hi:Performance}}[![cat-internationalization][cat-internationalization-badge]][cat-internationalization]{{hi:Internationalization (i18n)}}[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}[![cat-accessibility][cat-accessibility-badge]][cat-accessibility]{{hi:Accessibility}}

[`xilem`][c-xilem]⮳{{hi:xilem}} is the replacement for [`druid`][c-druid]⮳{{hi:druid}}, based on the more interoperable [`vello`][c-vello]⮳{{hi:vello}} and [`glazier`][c-glazier]⮳{{hi:glazier}} crates. However, it's currently not complete enough to be usable.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/xilem.rs:example}}
```

## `slint` {#slint}

[![slint-website][c-slint-website-badge]][c-slint-website] [![slint][c-slint-badge]][c-slint] [![slint-crates.io][c-slint-crates.io-badge]][c-slint-crates.io] [![slint-github][c-slint-github-badge]][c-slint-github] [![slint-lib.rs][c-slint-lib.rs-badge]][c-slint-lib.rs]{{hi:slint}}{{hi:Design}}{{hi:Graphics}}{{hi:Gui}}{{hi:Toolkit}}{{hi:Ui}}[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}[![cat-rendering::engine][cat-rendering::engine-badge]][cat-rendering::engine]{{hi:Rendering engine}}

[`slint`][c-slint]⮳{{hi:slint}} is a GUI toolkit to efficiently develop fluid graphical user interfaces for embedded devices and desktop applications.
Possibly the most complete rust-native UI library. But note that it's dual GPL3/commercial licensed.

- [slint.dev][c-slint-website]⮳
- [Slint (github)][c-slint-github]⮳
- [madewithslint.com][c-slint-madewithslint-website]⮳

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/slint.rs:example}}
```

## GPUI {#gpui}

[![gpui][c-gpui-badge]][c-gpui] [![gpui-crates.io][c-gpui-crates.io-badge]][c-gpui-crates.io] [![gpui-github][c-gpui-github-badge]][c-gpui-github] [![gpui-lib.rs][c-gpui-lib.rs-badge]][c-gpui-lib.rs]{{hi:gpui}}
[![gpui][c-gpui-badge]][c-gpui]{{hi:gpui}}

[gpui.rs][gpui.rs]⮳ is a fast, productive UI framework for Rust from the creators of the [Zed][c-zed-website]⮳ text editor.

## `druid` {#druid}

[![druid][c-druid-badge]][c-druid] [![druid-crates.io][c-druid-crates.io-badge]][c-druid-crates.io] [![druid-github][c-druid-github-badge]][c-druid-github] [![druid-lib.rs][c-druid-lib.rs-badge]][c-druid-lib.rs]{{hi:druid}}{{hi:Toolkit}}{{hi:Gui}}{{hi:Ui}}[![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[`druid`][c-druid]⮳{{hi:druid}} is a data-first Rust-native UI design toolkit (experimental). Druid is a relatively mature alternative to Iced / Slint, however it has been discontinued in favor of Xilem, so its use for new projects is discouraged.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[other_gui: organize; titles (P1)](https://github.com/john-cd/rust_howto/issues/387)

</div>
