<!-- TODO Reviewed ??-??-2026. -->

# Rendering Engines (Built on Low-Level APIs)

[![cat~rendering::engine][cat~rendering::engine~badge]][cat~rendering::engine]{{hi:Rendering engine}}

Rendering engines coordinate the scene, camera, materials, GPU resources, and passes that produce an image. The output can be a window surface or an offscreen texture.

Rendering engines sit above low-level graphics APIs such as [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} and provide scenes, materials, and cameras. Start with the [engine-selection and offscreen rendering workflow](rendering_engines.md), then see the [3D renderers chapter](../rendering/3d_renderers.md) in the [rendering category](../rendering/index.md) for descriptions of `rend3`, `three-d`, `kiss3d`, and `bevy_render`.

{{#include rendering_engines.incl.md}}

## Related Topics

- [Graphics](https://github.com/john-cd/rust_howto/blob/86dcf0197f8d6810877a6021ffedcf84aa9230c8/bk/src/categories/graphics/index.md).
- [Rendering](../rendering/index.md).
- [Rendering Data Formats](../rendering_data-formats/index.md).
- [Rendering Graphics APIs](../rendering_graphics-api/index.md) - 2D/3D Lower Level Graphics.

### Applications

- [Game Development](../game-development/index.md).
- [Game Engines](../game-engines/index.md).
- [GUI](../gui/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
