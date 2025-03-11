# Low-Level Graphics (Vulkan)

{{#include vulkan.incl.md}}

`Vulkan` is a low-level, low-overhead cross-platform API and open standard for 3D graphics and computing. It was intended to address the shortcomings of `OpenGL`, and allow developers more control over the GPU.

[`ash`][c-ash]⮳{{hi:ash}} is a lower-level, direct binding to Vulkan. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} provides a safer, higher-level abstraction. These are closer to the metal than many other graphics API options.

## `ash` {#ash}

[`ash`][c-ash]⮳{{hi:ash}} is a lower-level, direct binding to Vulkan.

## `vulkano` {#vulkano}

Safe wrapper for the Vulkan graphics API.

[`vulkano`][c-vulkano]⮳{{hi:vulkano}} provides Rust bindings for the Vulkan API. Very powerful, but also very complex. Only use this if you absolutely need the fine-grained control that Vulkan provides. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} provides a safer, higher-level abstraction than [`ash`][c-ash]⮳{{hi:ash}}. Both are lower-level and require more detailed graphics programming knowledge.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO decide what to cover / write
</div>
