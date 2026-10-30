<!-- TODO Reviewed ??-??-2026 -->

# 2D Renderers

A 2D renderer turns paths, shapes, text, or a pixel buffer into an image. Use this overview to choose a boundary; the [raster](2d_raster_graphics.md) and [vector](2d_vector_graphics.md) chapters own the implementation details and examples.

## Compare Rendering Approaches {#choosing-the-right-library}

| Task                             | Crates                 | Integration boundary                                                     |
| -------------------------------- | ---------------------- | ------------------------------------------------------------------------ |
| Draw paths into a CPU image      | `tiny-skia`, `raqote`  | An in-memory pixel buffer; no GPU or window is needed.                   |
| Present an existing pixel buffer | `pixels`, `minifb`     | Supply pixels and connect a window or graphics backend.                  |
| Tessellate vector geometry       | `lyon`                 | Produces vertices and indices; the application chooses a renderer.       |
| Draw through a canvas API        | `femtovg`, `skia-safe` | Manage the context, target surface, and native-library prerequisites.    |
| Render a vector scene on a GPU   | `vello`, `vger`        | Provide a compatible GPU backend and render target.                      |
| Integrate a browser renderer     | `webrender`            | Browser-engine infrastructure, rather than a general application recipe. |

A crate's GPU backend must be compatible with the versions used elsewhere in an application. In particular, [Vger](2d_vector_graphics.md#vger) expects a WGPU integration different from this workspace; it remains documented without a runnable book recipe. [WebRender](2d_vector_graphics.md#webrender) is background coverage rather than a recommended standalone example. These dispositions avoid presenting unfinished adapters as working recipes.

## Tessellate, Then Rasterize {#tessellate-then-rasterize}

`lyon` produces vertices and indices, and the application chooses what draws them. This example feeds the mesh to `tiny-skia`'s CPU rasterizer; a GPU renderer would upload the same buffers. It needs no window or GPU.

```rust,editable
{{#include ../../../crates/cats/rendering/examples/2d_renderers/main.rs:example}}
```

## CPU Pixel Buffers {#pixel-based-2d-rendering}

See [2D Raster Graphics](2d_raster_graphics.md) for CPU drawing and pixel presentation. Raster images have a fixed resolution; account for scaling, color representation, and alpha when combining buffers.

## Paths and GPU Scenes {#vector-based-2d-rendering}

See [2D Vector Graphics](2d_vector_graphics.md) for paths, tessellation, canvas APIs, and GPU vector renderers. Vector data still needs rasterization before display. Antialiasing smooths shape boundaries; it does not remove the need to choose an appropriate output resolution.

## Compose a Rendering Pipeline {#hybrid}

Applications can combine CPU images, tessellated paths, and GPU scenes. Keep ownership of the window and render loop clear, choose one component to present the final frame, and make device-pixel scaling and alpha conventions explicit. For custom rendering beyond these libraries, see [GPU Abstraction Layers](../rendering_graphics-api/gpu_abstraction_layers.md).

## Related Topics {#related-topics .skip}

- [Image processing](../multimedia_images/images.md).
- [SVG rendering](svg_rendering.md).
- [Text rendering](text_rendering.md).
- [GUI frameworks](../gui/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
