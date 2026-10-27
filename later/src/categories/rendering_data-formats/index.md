<!-- TODO Reviewed ??-??-2026 -->

# Rendering - Data Formats

[![cat~rendering::data-formats][cat~rendering::data-formats~badge]][cat~rendering::data-formats]{{hi:Rendering data formats}}

Use a format-specific page to choose parsers for 3D models, SVG, and compressed GPU textures. Raster image decoding is covered in [[multimedia_images | Multimedia: Images]].

| Asset family                         | Crates or formats                                              | Guide                                                      |
| ------------------------------------ | -------------------------------------------------------------- | ---------------------------------------------------------- |
| Raster images and sprites            | `image`, `imageproc`, `lodepng`, `jpeg-decoder`, `gif`, `webp` | [[multimedia_images                                     | Multimedia: Images]] |
| 3D scenes and meshes                 | glTF, OBJ, STL, PLY                                            | [3D model formats](3d_model_formats.md)                    |
| Vector graphics                      | SVG, `usvg`, `resvg`                                           | [Vector formats](vector_formats.md)                        |
| GPU texture containers               | KTX2, DDS; Basis Universal transcoding                         | [Texture formats](texture_formats.md)                      |
| Fonts                                | `fontdue`, `ttf-parser`, `opentype`, `ab_glyph`                | [[text_rendering                                        | Text Rendering]] |
| Sprite sheets and animation metadata | Application-specific JSON, YAML, or binary formats             | Use `serde` or a format-specific parser.                   |
| Scene descriptions and levels        | Application-specific text or binary formats                    | Use a parser generator when the format needs a grammar.    |

## Code Examples {#code-examples}

{{#include data_formats.incl.md}}

## Related Topics {#related-topics .skip}

- [[rendering | Rendering]].
- [[rendering_engine | Rendering Engine]].
- [[2d_renderers | 2D Renderers]].
- [[3d_renderers | 3D Renderers]].
- [[svg_rendering | SVG Rendering]].
- [[rendering_graphics-api | Rendering: Graphics API]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
