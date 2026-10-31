<!-- TODO Reviewed ??-??-2026 -->

# OpenGL

{{#include opengl.incl.md}}

OpenGL is a cross-platform graphics API with a large installed base, but it is a legacy choice for new Rust applications. Apple deprecates OpenGL on macOS ([platform notice][apple~opengl~deprecation]) and OpenGL ES on iOS ([platform notice][apple~opengles~deprecation]); use Metal for new native Apple rendering. For new cross-platform applications, start with the [wgpu guide][ex~rendering_graphics-api~wgpu]. OpenGL still makes sense for existing applications, older hardware, and WebGL-compatible targets.

## Cross-platform bindings with `glow` {#glow}

[![glow][c~glow~docs~badge]][c~glow~docs] [![glow~crates.io][c~glow~crates.io~badge]][c~glow~crates.io]{{hi:glow}}

[`glow`][c~glow~docs]↗{{hi:glow}} provides bindings for OpenGL, OpenGL ES, and WebGL behind one Rust API. The [`egui_glow` integration][egui_glow~repo] uses it for GL rendering; `wgpu` provides a separate GL/GLES backend on supported targets. The tested example below shows how to load the bindings and clear a framebuffer; a windowing crate must create and make the context current first.

```rust
{{#include ../../../crates/cats/rendering_graphics_api/examples/opengl/main.rs:example}}
```

## Create a window and context with `glutin` and `winit` {#glutin}

[![glutin][c~glutin~docs~badge]][c~glutin~docs] [![winit][c~winit~docs~badge]][c~winit~docs]{{hi:glutin}}{{hi:winit}}

[`winit`][c~winit~docs] handles cross-platform windows and input events. [`glutin`][c~glutin~docs] creates the platform OpenGL context and surface; [`glutin-winit`][c~glutin-winit~docs] provides helpers to connect them.

## Higher-level OpenGL with `glium` {#glium}

[![glium][c~glium~docs~badge]][c~glium~docs] [![glium~crates.io][c~glium~crates.io~badge]][c~glium~crates.io]{{hi:glium}}

[`glium`][c~glium~docs] provides a safe, higher-level wrapper for OpenGL 3+ buffers, textures, and drawing. The [project README][c~glium~repo] notes that the original author no longer actively develops it and that community maintenance continues; check current releases and issues before choosing it for new work.

## Raw OpenGL bindings with `gl` {#gl}

[![gl][c~gl~docs~badge]][c~gl~docs] [![gl~crates.io][c~gl~crates.io~badge]][c~gl~crates.io]{{hi:gl}}

[`gl`][c~gl~docs] provides low-level function-pointer bindings generated for OpenGL APIs. Applications must load the functions from a current context and call them through `unsafe`; use `glow` when you need a shared OpenGL, OpenGL ES, and WebGL interface.

## Related Topics {#related-topics .skip}

- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[gpu | GPU]].
- [[shaders | Shaders]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
