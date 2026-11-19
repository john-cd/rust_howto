<!-- TODO Reviewed ??-??-2026. -->

# GTK Development

{{#include gtk.incl.md}}

GTK 4 is a native widget toolkit with Rust bindings in [`gtk4`][c~gtk4~docs]. Keep the Rust binding and installed GTK library versions compatible. GTK 4 supports Linux, Windows, and macOS, but each platform needs native libraries and a suitable build toolchain. On Debian or Ubuntu, install `build-essential`, `libgtk-4-dev`, and `pkg-config`; the repository's `.devcontainer/install_deps.sh` installs these for Linux development. The examples in this workspace currently exclude Windows builds. For other setups, follow the gtk-rs [Linux][gtk-rs~installation-linux] and [Windows][gtk-rs~installation-windows] guides; GTK uses its Quartz backend on macOS [gtk~macos].

## Core GTK Concepts {#key-concepts}

Widgets represent controls such as buttons and labels. Signals deliver user events to application code, while GTK's main loop manages event processing and redraws.

## Build a GTK 4 Window {#gtk4}

[![gtk4~website][c~gtk4~website~badge]][c~gtk4~website] {{#badges gtk4}} {{hi:Gnome}} {{hi:Gtk}} {{hi:Gtk-rs}} {{hi:GUI}} {{#categories api-bindings="API bindings",gui=GUI}}

Use `gtk4` when you want direct access to GTK widgets and signals. Run the
interactive counter from the repository root:

```console
just cargo run --manifest-path bk/platform/Cargo.toml --locked -p gui --features gtk --example gtk -- gtk4
```

The outer command selects the example with Clap. Give GTK a separate argument
list containing only the application name so GApplication does not interpret
`gtk4` as a file to open.

```rust,noplayground
{{#include ../../../platform/cats/gui/examples/gtk/gtk4.rs:example}}
```

## Define a Composite Template {#composite-template}

For a larger custom widget, define the hierarchy in a GTK Builder `.ui` XML file and bind it to a Rust subclass with `#[derive(CompositeTemplate)]`. The template derive exposes declared child widgets to Rust code [gtk-rs~composite-templates].

[Blueprint][blueprint~guide] is a separate declarative language that compiles `.blp` files into GTK UI definitions. Its documentation describes the compiler as experimental, so check its current status before making it part of a production build.

## Use GNOME Widgets {#libadwaita}

[`libadwaita`][c~libadwaita~docs] adds Adwaita styling, adaptive layouts, and GNOME-specific widgets. Choose it for an application designed for GNOME, and keep its supported GTK version aligned with `gtk4` [gtk-rs~libadwaita].

## Build an Elm-Style App with Relm4 {#relm4}

[![relm4~website][c~relm4~website~badge]][c~relm4~website] {{#badges relm4}} {{hi:Elm}} {{hi:Gtk4}} {{hi:GUI}} {{#categories gui=GUI}}

[`relm4`][c~relm4~docs] is an Elm-inspired component framework built on gtk-rs. Its model, messages, and update function help organize stateful GTK applications. Run it with the same command and replace the final `gtk4` subcommand with
`relm4`. Here `with_args` passes the application name through Relm4 to GTK.

```rust,noplayground
{{#include ../../../platform/cats/gui/examples/gtk/relm4.rs:example}}
```

## Related GTK Crates {#related-gtk-crates}

- [`gdk4`][c~gdk~docs] provides GTK's lower-level graphics and windowing bindings.
- [`gio`][c~gio~docs] provides asynchronous operations, file I/O, and other system services used by GTK applications.
- [`glib`][c~glib~docs] provides the base types and event loop used by GTK and GIO.

## Related Topics {#related-topics .skip}

- [GUI toolkits](index.md).
- [Window creation](window_creation.md).
- [API bindings](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/api-bindings/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
