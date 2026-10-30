<!-- TODO Reviewed ??-??-2026 -->

# Low-Level Graphics (Vulkan)

{{#include vulkan.incl.md}}

[`Vulkan`][vulkan~website]↗{{hi:Vulkan}} is a low-level, low-overhead cross-platform API and open standard for 3D graphics and computing. It was intended to address the shortcomings of `OpenGL`, and allow developers more control over the GPU.

[`ash`][c~ash~docs]↗{{hi:ash}} is a lower-level, direct binding to Vulkan. [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} provides a safer, higher-level abstraction. These are closer to the metal than many other graphics API options.

## `ash` {#ash}

[`ash`][c~ash~docs]↗{{hi:ash}} is a lower-level, direct binding to Vulkan.

`ash` loads the Vulkan library at runtime, so a program can detect a missing driver instead of failing to start. This example reports the instance version when a Vulkan loader is installed and degrades gracefully otherwise. Creating an instance and a device is the next step; it needs a driver and, to present images, a window system.

```rust,editable
{{#include ../../../crates/cats/rendering_graphics_api/examples/vulkan/main.rs:example}}
```

## `vulkano` {#vulkano}

Safe wrapper for the Vulkan graphics API.

[`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} provides Rust bindings for the Vulkan API. Very powerful, but also very complex. Only use this if you absolutely need the fine-grained control that Vulkan provides. [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} provides a safer, higher-level abstraction than [`ash`][c~ash~docs]↗{{hi:ash}}. Both are lower-level and require more detailed graphics programming knowledge.

## Related Topics {#related-topics .skip}

- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[gpu | GPU]].
- [[shaders | Shaders]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
