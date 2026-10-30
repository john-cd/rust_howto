<!-- TODO Reviewed ??-??-2026 -->

# GTK Development

{{#include gtk.incl.md}}

GTK 4 is a native widget toolkit with Rust bindings in [`gtk4`][c~gtk4~docs]. Keep the Rust binding and installed GTK library versions compatible. GTK 4 supports Linux, Windows, and macOS, but each platform needs native libraries and a suitable build toolchain. On Debian or Ubuntu, install `build-essential`, `libgtk-4-dev`, and `pkg-config`; the repository's `.devcontainer/install_deps.sh` installs these for Linux development. The examples in this workspace currently exclude Windows builds. For other setups, follow the gtk-rs [Linux][gtk-rs~installation-linux] and [Windows][gtk-rs~installation-windows] guides; GTK uses its Quartz backend on macOS [gtk~macos].

## Core GTK Concepts {#key-concepts}

Widgets represent controls such as buttons and labels. Signals deliver user events to application code, while GTK's main loop manages event processing and redraws.

## Build a GTK 4 Window {#gtk4}

[![gtk4~website][c~gtk4~website~badge]][c~gtk4~website] [![gtk4][c~gtk4~docs~badge]][c~gtk4~docs] [![gtk4~crates.io][c~gtk4~crates.io~badge]][c~gtk4~crates.io] [![gtk4~repo][c~gtk4~repo~badge]][c~gtk4~repo] [![gtk4~lib.rs][c~gtk4~lib.rs~badge]][c~gtk4~lib.rs]{{hi:gtk4}}{{hi:Gnome}}{{hi:Gtk}}{{hi:Gtk-rs}}{{hi:GUI}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

Use `gtk4` when you want direct access to GTK widgets and signals. This example builds a small interactive counter window; consult the GTK API documentation alongside the gtk-rs bindings.

```rust,editable
{{#include ../../../crates/cats/gui/examples/gtk/gtk4.rs:example}}
```

## Define a Composite Template {#composite-template}

For a larger custom widget, define the hierarchy in a GTK Builder `.ui` XML file and bind it to a Rust subclass with `#[derive(CompositeTemplate)]`. The template derive exposes declared child widgets to Rust code [gtk-rs~composite-templates].

[Blueprint][blueprint~guide] is a separate declarative language that compiles `.blp` files into GTK UI definitions. Its documentation describes the compiler as experimental, so check its current status before making it part of a production build.

## Use GNOME Widgets {#libadwaita}

[`libadwaita`][c~libadwaita~docs] adds Adwaita styling, adaptive layouts, and GNOME-specific widgets. Choose it for an application designed for GNOME, and keep its supported GTK version aligned with `gtk4` [gtk-rs~libadwaita].

## Build an Elm-Style App with Relm4 {#relm4}

[![relm4~website][c~relm4~website~badge]][c~relm4~website] [![relm4][c~relm4~docs~badge]][c~relm4~docs] [![relm4~crates.io][c~relm4~crates.io~badge]][c~relm4~crates.io] [![relm4~repo][c~relm4~repo~badge]][c~relm4~repo] [![relm4~lib.rs][c~relm4~lib.rs~badge]][c~relm4~lib.rs]{{hi:relm4}}{{hi:Elm}}{{hi:Gtk4}}{{hi:GUI}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`relm4`][c~relm4~docs] is an Elm-inspired component framework built on gtk-rs. Its model, messages, and update function help organize stateful GTK applications. Run the counter example from `later/crates` with `cargo run -p gui --example gtk --features gtk -- relm4`; it requires GTK 4 development libraries and a display.

```rust,editable
{{#include ../../../crates/cats/gui/examples/gtk/relm4.rs:example}}
```

## Related GTK Crates {#related-gtk-crates}

- [`gdk4`][c~gdk~docs] provides GTK's lower-level graphics and windowing bindings.
- [`gio`][c~gio~docs] provides asynchronous operations, file I/O, and other system services used by GTK applications.
- [`glib`][c~glib~docs] provides the base types and event loop used by GTK and GIO.

## Related Topics {#related-topics .skip}

- Cross-platform GUI development.
- Native desktop toolkits.
- Rust bindings for C libraries.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
