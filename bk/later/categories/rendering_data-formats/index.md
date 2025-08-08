# Rendering - Data Formats

[![cat~rendering::data-formats][cat~rendering::data-formats~badge]][cat~rendering::data-formats]{{hi:Rendering data formats}}

Loading and parsing of data formats related to 2D or 3D rendering, like 3D models or animation sheets.

| Data Format | Rust Crates | Notes |
|---|---|---|
| Image Formats (2D Textures/Sprites) | [`image`][c~image~docs]↗{{hi:image}}, [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}}, [`lodepng`][c~lodepng~docs]↗{{hi:lodepng}}, [`jpeg-decoder`][c~jpeg-decoder~docs]↗{{hi:jpeg-decoder}}, [`gif`][c~gif~docs]↗{{hi:gif}}, [`webp`][c~webp~docs]↗{{hi:webp}} | See [[multimedia_images | Multimedia: Images]]. |
| 2D Sprite Sheets / Animation Data | Often custom formats or [[json | JSON]], [[yaml | YAML]] | Many games use custom formats for sprite sheets and animation data. JSON or YAML are often used to describe animation sequences and frame timings. You'll likely use [`serde`][c~serde~docs]↗{{hi:serde}} for custom parsing. See [[serde | Serde]]. |
| Font Formats (Text Rendering) | [`fontdue`][c~fontdue~docs]↗{{hi:fontdue}}, [`ttf-parser`][c~ttf-parser~docs]↗{{hi:ttf-parser}}, [`opentype`][c~opentype~docs]↗{{hi:opentype}}, [`ab_glyph`][c~ab_glyph~docs]↗{{hi:ab_glyph}} | See [[text_rendering | Text Rendering]]. |
| Vector Graphics (SVG) | [`resvg`][c~resvg~docs]↗{{hi:resvg}}, [`usvg`][c~usvg~docs]↗{{hi:usvg}} | See [[svg_rendering | Svg Rendering]]. |
| 3D Model Formats | [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} (for glTF), [`assimp`][c~assimp~docs]↗{{hi:assimp}} (bindings) | glTF often uses JSON for metadata and binary data for geometry. [`assimp`][c~assimp~docs]↗{{hi:assimp}}, the Open Asset Import Library, is a powerful library (with Rust bindings) that supports many formats. For 'stl', common 3D model format, and FBX, a complex format, crates are less common and bindings to Assimp might be the most practical approach. |
| Scene Description Languages (SDF) | Often custom or using parser generators | SDFs describe 3D scenes mathematically. Parsing often involves custom code or parser generators like [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}} due to the specific syntax. |
| Animation Formats (Keyframe, Skeletal) | Often custom or using parser generators, or glTF extensions | Animation data can be stored in various ways. Keyframe animations might be simple data structures. Skeletal animation is more complex. glTF supports animation, but custom formats are also common. You might use [`serde`][c~serde~docs]↗{{hi:serde}} and custom structs. |
| Level Data (Game Engines) | Often custom or using JSON/YAML. | Game engines often use their own level formats. JSON or YAML are popular choices for storing level data, but binary formats are also common for performance reasons. You'll often need custom parsing code. |
| Materials/Shaders (Custom Formats) | Often custom or using JSON/YAML. | Materials and shaders are often described using custom formats or JSON/YAML. You'll likely use [`serde`][c~serde~docs]↗{{hi:serde}} for parsing. |

- If you need to support a wide range of formats, using a library like [`assimp`][c~assimp~docs]↗{{hi:assimp}} (through bindings) might be a good option.
- For simpler formats or custom data, writing your own parser using crates like [`nom`][c~nom~docs]↗{{hi:nom}} or [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}} might be more straightforward. See [[parsing | Parsing]].
- [`serde`][c~serde~docs]↗{{hi:serde}} is your friend for easily converting parsed data into Rust structs and vice-versa. See [[serde | Serde]].
- Game development often involves custom formats. Be prepared to write parsers or use parser generators like [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}} if needed.

## Code Examples

{{#include data_formats.incl.md}}

## Related Topics

- [[rendering | Rendering]].
- [[rendering_engine | Rendering Engine]].
- [[2d_renderers | 2D Renderers]].
  - [[2d_raster_graphics | 2D Raster Graphics]].
  - [[2d_vector_graphics | 2D Vector Graphics]].
- [[3d_renderers | 3d Renderers]].
- [[svg_rendering | SVG Rendering]].
- Low-level Graphics API: [[rendering_graphics-api | Rendering: Graphics API]].
  - [[gpu_abstraction_layers | Gpu Abstraction Layers]].
  - [[opengl | OpenGL]].
  - [[native_graphics_apis | Native Graphics APIs]].
  - [[vulkan | Vulkan]].
  - [[shaders | Shaders]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/954)
review in depth
decide what crates to cover
split into multiple pages
</div>
