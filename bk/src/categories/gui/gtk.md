# GTK

{{#include gtk.incl.md}}

## `gtk4` {#gtk4}

[![gtk4-website][c-gtk4-website-badge]][c-gtk4-website] [![gtk4][c-gtk4-badge]][c-gtk4] [![gtk4-crates.io][c-gtk4-crates.io-badge]][c-gtk4-crates.io] [![gtk4-github][c-gtk4-github-badge]][c-gtk4-github] [![gtk4-lib.rs][c-gtk4-lib.rs-badge]][c-gtk4-lib.rs]{{hi:gtk4}}{{hi:Gnome}}{{hi:Gtk}}{{hi:Gtk-rs}}{{hi:gtk4}}{{hi:Gui}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

Rust bindings of the GTK 4 library. These are quite well supported, although you'll often need to use the C documentation.

```rust,editable
{{#include ../../../crates/cats/gui/examples/gtk/gtk4.rs:example}}
```

## `relm4` {#relm4}

[![relm4-website][c-relm4-website-badge]][c-relm4-website] [![relm4][c-relm4-badge]][c-relm4] [![relm4-crates.io][c-relm4-crates.io-badge]][c-relm4-crates.io] [![relm4-github][c-relm4-github-badge]][c-relm4-github] [![relm4-lib.rs][c-relm4-lib.rs-badge]][c-relm4-lib.rs]{{hi:relm4}}{{hi:Elm}}{{hi:Gtk}}{{hi:Gtk4}}{{hi:Gui}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

A higher-level library that sits on top of gtk4-rs. An idiomatic GUI library inspired by Elm.

```rust,editable
{{#include ../../../crates/cats/gui/examples/gtk/relm4.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[gtk: write (P2)](https://github.com/john-cd/rust_howto/issues/383)

</div>
