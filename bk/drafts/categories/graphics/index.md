<!-- TODO Reviewed ??-??-2026 -->
# Graphics

[![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

Graphics libraries cover different layers of a rendering pipeline, from drawing 2D shapes to submitting GPU work. Choose by the output you need and how much control you want over the renderer, window, and event loop.

## Choose a Graphics Layer

| Goal | Crates | What they provide |
|---|---|---|
| Draw 2D shapes and paths | [`raqote`][c~raqote~docs]↗{{hi:raqote}}, [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} | 2D drawing APIs for paths, fills, strokes, and images. |
| Render a pixel buffer | [`pixels`][c~pixels~docs]↗{{hi:pixels}} | A GPU-backed pixel buffer built on [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}. |
| Turn vector paths into renderable geometry | [`lyon`][c~lyon~docs]↗{{hi:lyon}} | Tessellates 2D fills and strokes into geometry; a renderer still draws the result. |
| Build a GPU renderer or 3D application | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} | A cross-platform graphics API with native and WebAssembly backends. |
| Make sketches and generative artwork | [`nannou`][c~nannou~repo]↗{{hi:nannou}} | A creative-coding framework with drawing, graphics, windowing, and UI APIs. |

These tools are not interchangeable. For example, `lyon` prepares path geometry while a renderer such as `wgpu` submits it to a graphics backend. `pixels` starts from a pixel buffer, while 2D drawing APIs expose higher-level path and paint operations. A GUI framework or game engine may provide window management and rendering as part of a larger application framework.

## Rendering Topics

Graphics work commonly involves:

- Raster drawing, pixel buffers, and image surfaces.
- Vector paths, fills, strokes, SVG, and text rendering.
- 3D meshes, materials, real-time rendering, and scene organization.
- GPU APIs, shaders, and graphics data formats.
- Visualization, creative coding, and generative art.

Image processing, computer vision, simulation, and physics use some of the same image, geometry, and GPU concepts, but are separate application areas.

## Related Topics

Graphics code relies on [linear algebra][p~linear-algebra] for vectors, matrices, and transforms. Higher-level GUI and game frameworks can provide the windowing and rendering layers for complete applications.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
