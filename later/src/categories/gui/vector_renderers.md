<!-- TODO Reviewed ??-??-2026. -->

# Vector Renderers for GUIs

Choose a renderer based on where its frame will go. Femtovg draws to an OpenGL context, Skia can draw to a CPU image, and Vello can render on a compatible `wgpu` adapter. For paths and tessellation, start with the [Graphics chapter](https://github.com/john-cd/rust_howto/blob/ceaf4b050e21e3981d121bb1414fc26b01e16a4b/bk/src/categories/graphics/2d_vector_graphics.md).

## `femtovg` {#femtovg}

{{#badges femtovg graphics=Graphics}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}}

[`femtovg`][c~femtovg~docs]↗{{hi:femtovg}} provides a canvas-style API for antialiased 2D paths, images, and text. Its OpenGL renderer needs a compatible graphics context supplied by the application. Use it when you want a higher-level drawing canvas and already have, or can create, the required window and context.

The companion example creates a window and OpenGL context with `winit` and `glutin`, then draws a path. Run it from `later/crates` with `cargo run --locked -p gui --example femtovg --features femtovg`. It needs a desktop display and a working OpenGL driver; automated headless tests compile it but skip opening the window. On a machine without a display or GPU, run it under a virtual X server with software OpenGL: on Debian or Ubuntu, install `xvfb libgl1-mesa-dri libxkbcommon-x11-0` and use `xvfb-run -a cargo run --locked -p gui --example femtovg --features femtovg`; the window stays open until you close it, so stop it with a timeout.

```rust,ignore,noplayground
{{#include ../../../crates/cats/gui/examples/2d_renderers/femtovg.rs:example}}
```

## `skia-safe` {#skia-safe}

{{#badges skia-safe api-bindings="API bindings" graphics=Graphics multimedia::images=Images rendering::graphics-api="Graphics APIs" visualization=Visualization}}{{hi:OpenGL}}{{hi:Pdf}}{{hi:Rust-bindings}}{{hi:Skia}}{{hi:Vulkan}}

[`skia-safe`][c~skia-safe~docs]↗{{hi:skia-safe}} exposes the Skia drawing library to Rust.

The build script of `skia-safe` downloads prebuilt Skia binaries for common targets, or compiles Skia from source (which needs Python, a C++ toolchain and `ninja`) when none is available. Plan for this in CI and offline builds, and pin the crate version to match other crates that embed Skia. The example draws into a CPU image. From `later/crates`, run it with `cargo run --locked -p gui --example 2d_renderers --features skia,femtovg -- skia_safe`.

```rust,ignore,noplayground
{{#include ../../../crates/cats/gui/examples/2d_renderers/skia_safe.rs:example}}
```

## `vello` {#vello}

{{#badges vello graphics=Graphics rendering=Rendering}}{{hi:2d}}{{hi:Vector-graphics}}

[`vello`][c~vello~docs]↗{{hi:vello}} is a GPU compute-centric 2D renderer built on `wgpu`. Build and test its scene independently of a device, then render it when a compatible GPU adapter is available. From `later/crates`, run the off-screen example with `cargo run --locked -p gui --example vello_renderer --features vello`; it requires a compatible GPU adapter.

```rust,ignore,noplayground
{{#include ../../../crates/cats/gui/examples/2d_renderers/vello.rs:example}}
```

[c~femtovg~docs]: https://docs.rs/femtovg
[c~skia-safe~docs]: https://docs.rs/skia-safe
[c~vello~docs]: https://docs.rs/vello

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
