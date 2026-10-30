<!-- TODO Reviewed ??-??-2026 -->

# Window Creation

{{#include window_creation.incl.md}}

Window libraries create native windows and deliver input and lifecycle events. They do not draw your interface. Pair them with `wgpu`, `softbuffer`, `pixels`, or an OpenGL context from `glutin`, or choose a GUI toolkit that manages both jobs.

## `winit` {#winit}

[![winit][c~winit~docs~badge]][c~winit~docs] [![winit~crates.io][c~winit~crates.io~badge]][c~winit~crates.io] [![winit~repo][c~winit~repo~badge]][c~winit~repo] [![winit~lib.rs][c~winit~lib.rs~badge]][c~winit~lib.rs]{{hi:winit}}{{hi:Windowing}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`winit`][c~winit~docs]↗{{hi:winit}} is the usual starting point for a cross-platform native window and input event loop. Its current application model uses `ApplicationHandler`; pair it with a renderer or pixel presentation layer because Winit does not draw the interface.

```rust,editable
{{#include ../../../crates/cats/gui/examples/window_creation/winit.rs:example}}
```

## `tao` {#tao}

[![tao][c~tao~docs~badge]][c~tao~docs] [![tao~crates.io][c~tao~crates.io~badge]][c~tao~crates.io] [![tao~repo][c~tao~repo~badge]][c~tao~repo] [![tao~lib.rs][c~tao~lib.rs~badge]][c~tao~lib.rs]{{hi:tao}}{{hi:Windowing}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`tao`][c~tao~docs]↗{{hi:tao}} is Tauri's cross-platform fork of Winit. Its supported platforms include Windows, macOS, Linux, iOS, and Android; on Linux it uses GTK 3. Tao does not itself provide Tauri's system menus or tray icons: use the separate [`muda`][muda~repo] and [`tray-icon`][tray-icon~repo] crates when needed. This workspace disables its Tao example because the current GTK 3 / `glib 0.18` dependency chain conflicts with its GTK 4 dependencies.

```rust,ignore
{{#include ../../../crates/cats/gui/examples/window_creation/tao.rs:example}}
```

## `baseview` {#baseview}

[![baseview~repo][baseview~repo~badge]][baseview~repo]

`baseview` is a low-level window system interface for [audio][p~audio] plugin UIs. Pick it over `winit` when your window must be embedded in, or parented to, a window owned by a plugin host (a DAW), which `winit` does not support. For an ordinary application window, use `winit`. `baseview` is not published on crates.io, so the workspace depends on a pinned Git revision behind the optional `baseview` feature (`cargo test -p gui --features baseview --example window_creation`). On Linux, install `libx11-dev libxcb1-dev libx11-xcb-dev libgl1-mesa-dev` first; the window-opening test is ignored because it needs a display.

The `winit` and `baseview` examples are compiled on Windows as well as other supported desktop platforms. `baseview` selects its Windows backend for Windows builds. The Tao source remains in this chapter, but its Cargo dependency and command are disabled because the current GTK 3 / `glib 0.18` dependency chain is incompatible with the workspace.

```rust,noplayground
{{#include ../../../crates/cats/gui/examples/window_creation/baseview.rs:example}}
```

## Draw Pixels with `softbuffer`

[![softbuffer][c~softbuffer~docs~badge]][c~softbuffer~docs]

`softbuffer` presents CPU-written pixels without requiring a GPU renderer. Create the Winit window after the event loop resumes, retain its surface, resize that surface for the current physical window dimensions, and present a buffer on `RedrawRequested`. Skip zero-sized windows while minimized. Pixels use `0x00RRGGBB`, with an unused high byte.

```rust,noplayground
{{#include ../../../crates/cats/gui/examples/window_creation/software_pixels.rs:example}}
```

From `later/crates`, run `cargo run -p gui --example window_creation --features softbuffer -- software-pixels`. This opens a solid blue window; close it to finish. The interactive test is ignored by default, while the example is compiled and linted with the feature enabled.

## SDL2 and SDL3 {#sdl}

[`sdl2`][c~sdl2~docs] and [`sdl3`][c~sdl3~docs] are bindings to the SDL C library. Consider them when a project already depends on SDL or needs its audio, input, and windowing features; follow the binding's instructions for installing the native library or building it from source.

## See Also {#see-also .skip}

- [`glfw`][c~glfw~docs]↗{{hi:glfw}} provides bindings to the GLFW library.

## Related Topics {#related-topics .skip}

- Window management and event handling.
- Cross-platform GUI integration.
- Native application toolkits.
- [File Dialogs](./file_dialogs.md) and [Clipboard](./clipboard.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
