# Low-Level Graphics (Vulkan)

{{#include vulkan.incl.md}}

[`Vulkan`][vulkan~website]↗{{hi:Vulkan}} is a low-level, low-overhead cross-platform API and open standard for 3D graphics and computing. It was intended to address the shortcomings of `OpenGL`, and allow developers more control over the GPU.

[`ash`][c~ash~docs]↗{{hi:ash}} is a lower-level, direct binding to Vulkan. [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} provides a safer, higher-level abstraction. These are closer to the metal than many other graphics API options.

## `ash` {#ash}

[`ash`][c~ash~docs]↗{{hi:ash}} is a lower-level, direct binding to Vulkan.

## `vulkano` {#vulkano}

Safe wrapper for the Vulkan graphics API.

[`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} provides Rust bindings for the Vulkan API. Very powerful, but also very complex. Only use this if you absolutely need the fine-grained control that Vulkan provides. [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} provides a safer, higher-level abstraction than [`ash`][c~ash~docs]↗{{hi:ash}}. Both are lower-level and require more detailed graphics programming knowledge.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[decide what to cover / write](https://github.com/john-cd/rust_howto/issues/1227)
</div>
