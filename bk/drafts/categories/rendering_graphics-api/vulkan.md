# Low-Level Graphics (Vulkan)

{{#include vulkan.incl.md}}

Consider:

- `ash`
- `erupt`
- `vulkano`

## `vulkano` {#vulkano}

[`vulkano`][c-vulkano]⮳{{hi:vulkano}} provides a safer, higher-level abstraction. These are closer to the metal than many other options.

[`vulkano`][c-vulkano]⮳{{hi:vulkano}} provides Rust bindings for the Vulkan API. Very powerful, but also very complex. Only use this if you absolutely need the fine-grained control that Vulkan provides.

## `ash` {#ash}

[`ash`][c-ash]⮳{{hi:ash}} is a lower-level, more direct binding to Vulkan.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
