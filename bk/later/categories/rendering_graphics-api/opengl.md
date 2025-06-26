# OpenGL

OpenGL (Open Graphics Library) is a cross-language, cross-platform application programming interface (API) for rendering 2D and 3D vector graphics. The API is typically used to interact with a graphics processing unit (GPU), to achieve hardware-accelerated rendering.

## `gl` {#gl}

OpenGL bindings.

## `glow` {#glow}

GL on Whatever: a set of bindings to run GL (Open GL, OpenGL ES, and WebGL) anywhere, and avoid target-specific code.

## `glium` {#glium}

[`glium`][c~glium~docs]⮳{{hi:glium}} is an older, but simpler OpenGL wrapper.

Elegant and safe OpenGL wrapper. Glium is an intermediate layer between OpenGL and your application. You still need to manually handle the graphics pipeline, but without having to use OpenGL's old and error-prone API. Its objectives:

- Be safe to use. Many aspects of OpenGL that can trigger a crash if misused are automatically handled by glium.
- Provide an API that enforces good practices, such as RAII or stateless function calls.
- Be compatible with all OpenGL versions that support [shaders][p~shaders], providing unified API when things diverge.
- Avoid all OpenGL errors beforehand.
- Produce optimized OpenGL function calls, and allow the user to easily use modern OpenGL techniques.

## `glutin` {#glutin}

Cross-platform OpenGL context provider.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1229)
</div>
