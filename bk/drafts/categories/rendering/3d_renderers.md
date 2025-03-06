# 3D Renderers

{{#include 3d_renderers.incl.md}}

Graphics APIs, wrappers for and backends to Vulkan and OpenGL.

## `glow` {#glow}

GL on Whatever: a set of bindings to run GL (Open GL, OpenGL ES, and WebGL) anywhere, and avoid target-specific code.

## `glutin` {#glutin}

Cross-platform OpenGL context provider.

## `ash` {#ash}

Vulkan bindings for Rust.

## `wgpu` {#wgpu}

Cross-platform, safe, pure-rust graphics API

## `glium` {#glium}

Elegant and safe OpenGL wrapper. Glium is an intermediate layer between OpenGL and your application. You still need to manually handle the graphics pipeline, but without having to use OpenGL's old and error-prone API. Its objectives: - Be safe to use. Many aspects of OpenGL that can trigger a crash if misused are automatically handled by glium. - Provide an API that enforces good pratices such as RAII or stateless function calls. - Be compatible with all OpenGL versions that support shaders, providing unified API when things diverge. - Avoid all OpenGL errors beforehand. - Produce optimized OpenGL function calls, and allow the user to easily use modern OpenGL techniques.

## `gl` {#gl}

OpenGL bindings.

## `vulkano` {#vulkano}

Safe wrapper for the Vulkan graphics API.

## Related Topics

- [[2d_raster_graphics | 2D Raster Graphics]].
- [[2d_vector_graphics | 2D Vector Graphics]].
- [[game-development | Game Development]].
- [[game_engines | Game Engines]].

## See also

- [Are we Game yet? 3D Rendering](https://arewegameyet.rs/ecosystem/3drendering).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write. decide what to cover

3D Rendering

- wgpu: Safe and portable GPU abstraction in Rust, based on WebGPU.
- gfx-rs: A low-level, cross-platform graphics API for Rust.
- vulkano: Safe and rich Rust wrapper around the Vulkan API.

</div>
