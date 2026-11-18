<!-- TODO Reviewed ??-??-2026. -->

# Rendering Engines (Built on Low-Level APIs)

[![cat~rendering::engine][cat~rendering::engine~badge]][cat~rendering::engine]{{hi:Rendering engine}}

Rendering engines coordinate the scene, camera, materials, GPU resources, and passes that produce an image. The output can be a window surface or an offscreen texture.

Start with the [windowed scene or offscreen frame](rendering_engines.md). The windowed example delegates scene presentation to three-d. The offscreen example shows the resource and frame responsibilities of an application built directly on `wgpu`.

{{#include rendering_engines.incl.md}}

## Related Topics

- [Graphics](../graphics/index.md).
- [Rendering Data Formats](../rendering_data-formats/index.md).
- [Graphics APIs](../rendering_graphics-api/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
