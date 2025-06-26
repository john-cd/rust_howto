# GTK Development

{{#include gtk.incl.md}}

For GTK development in Rust, use the `gtk` crate. You'll use it to create widgets, handle events, manage layouts, and interact with the GTK library. The [`gdk`][c~gdk~docs]⮳{{hi:gdk}}, [`gio`][c~gio~docs]⮳{{hi:gio}}, and [`glib`][c~glib~docs]⮳{{hi:glib}} crates are used indirectly through `gtk`. You will need to understand GTK concepts and consult the official GTK documentation for effective development.

## Key Concepts {#skip}

- Widgets: The basic building blocks of a GTK application (buttons, labels, windows, etc.).
- Signals: How GTK widgets communicate with each other and your application code (e.g., button clicks).
- Layouts: How widgets are arranged within windows.
- Events: User input (mouse clicks, keyboard presses, etc.).
- GTK Main Loop: The event loop that processes GTK events.

## `gtk4` {#gtk4}

[![gtk4~website][c~gtk4~website~badge]][c~gtk4~website] [![gtk4][c~gtk4~docs~badge]][c~gtk4~docs] [![gtk4~crates.io][c~gtk4~crates.io~badge]][c~gtk4~crates.io] [![gtk4~github][c~gtk4~github~badge]][c~gtk4~github] [![gtk4~lib.rs][c~gtk4~lib.rs~badge]][c~gtk4~lib.rs]{{hi:gtk4}}{{hi:Gnome}}{{hi:Gtk}}{{hi:Gtk-rs}}{{hi:gtk4}}{{hi:Gui}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`gtk4`][c~gtk4~docs]⮳{{hi:gtk4}} offers Rust bindings of the GTK 4 library. These are quite well supported, although you'll often need to use the C documentation.

Make sure the `gtk` crate version you're using is compatible with the GTK version installed on your system.

```rust,editable
{{#include ../../../crates/cats/gui/examples/gtk/gtk4.rs:example}}
```

## `relm4` {#relm4}

[![relm4~website][c~relm4~website~badge]][c~relm4~website] [![relm4][c~relm4~docs~badge]][c~relm4~docs] [![relm4~crates.io][c~relm4~crates.io~badge]][c~relm4~crates.io] [![relm4~github][c~relm4~github~badge]][c~relm4~github] [![relm4~lib.rs][c~relm4~lib.rs~badge]][c~relm4~lib.rs]{{hi:relm4}}{{hi:Elm}}{{hi:Gtk}}{{hi:Gtk4}}{{hi:Gui}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`relm4`][c~relm4~docs]⮳{{hi:relm4}} is a higher-level library that sits on top of [`gtk4-rs`][c~gtk4~docs]⮳{{hi:gtk4-rs}}. An idiomatic GUI library inspired by `Elm`.

```rust,editable
{{#include ../../../crates/cats/gui/examples/gtk/relm4.rs:example}}
```

## Related GTK Crates {#skip}

- [`gdk`][c~gdk~docs]⮳{{hi:gdk}} provides bindings to the GDK (Graphics Device Kit) library, which is used by GTK for low-level [graphics][p~graphics] and windowing. You'll often use this indirectly through `gtk`.
- [`gio`][c~gio~docs]⮳{{hi:gio}} offers bindings to the GIO (GNOME Input/Output) library, used for [asynchronous][p~asynchronous] operations, file I/O, and other system-related tasks in GTK applications.
- [`glib`][c~glib~docs]⮳{{hi:glib}} provides bindings to the GLib library, which is the foundation for GTK and GIO. You'll often use this indirectly.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[gtk: write](https://github.com/john-cd/rust_howto/issues/383)
review in depth
</div>
