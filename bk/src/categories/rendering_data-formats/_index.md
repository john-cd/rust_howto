# Rendering - Data formats

[![cat-rendering::data-formats][cat-rendering::data-formats-badge]][cat-rendering::data-formats]{{hi:Rendering data formats}}

Loading and parsing of data formats related to 2D or 3D rendering, like 3D models or animation sheets.

{{#include data_formats.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/954)

## Loading and Parsing 2D/3D Rendering Data Formats

| Data Format | Rust Crates | Notes |
|---|---|---|
| Image Formats (2D Textures/Sprites) | `image`, `imageproc`, `lodepng`, `jpeg-decoder`, `gif`, `webp` | `image` is a general image processing library supporting many formats. Specialized crates exist for specific formats like PNG, JPEG, GIF, WebP. Used for textures, sprites, etc. |
| 2D Sprite Sheets/Animation Data | (Often custom formats or JSON/YAML) | Many games use custom formats for sprite sheets and animation data. JSON or YAML are often used to describe animation sequences and frame timings. You'll likely use `serde` for parsing. |
| Font Formats (Text Rendering) | `fontdue`, `ttf-parser`, `opentype`, `ab_glyph` | `fontdue` is a fast font rasterizer. `ttf-parser` and `opentype` provide lower-level font parsing capabilities. `ab_glyph` is another font rendering option. |
| Vector Graphics (SVG) | `resvg`, `usvg` | `resvg` is an SVG rendering library. `usvg` is another option. Used for scalable vector graphics. |
| 3D Model Formats | `serde_json` (for glTF), `obj`, `stl`, `assimp` (bindings), `fbx` (sometimes) | glTF often uses JSON for metadata and binary data for geometry. `obj` and `stl` are common 3D model formats. `assimp` is a powerful library (with Rust bindings) that supports many formats. FBX is a complex format; crates are less common and bindings to Assimp might be the most practical approach. |
| Scene Description Languages (SDF) | (Often custom or using parser generators) | SDFs describe 3D scenes mathematically. Parsing often involves custom code or parser generators like `lalrpop` due to the specific syntax. |
| Animation Formats (Keyframe, Skeletal) | (Often custom or using parser generators, or glTF extensions) | Animation data can be stored in various ways. Keyframe animations might be simple data structures. Skeletal animation is more complex. glTF supports animation, but custom formats are also common. You might use `serde` and custom structs. |
| Shader Code (GLSL, HLSL, SPIR-V) | `glsl-to-spirv`, `shaderc`, `naga` | `glsl-to-spirv` compiles GLSL to SPIR-V. `shaderc` is another shader compiler. `naga` is a general-purpose shader translation and analysis tool. Parsing is often part of the compilation process. |
| Materials/Shaders (Custom Formats) | (Often custom or using JSON/YAML) | Materials and shaders are often described using custom formats or JSON/YAML. You'll likely use `serde` for parsing. |
| Level Data (Game Engines) | (Often custom or using JSON/YAML) | Game engines often use their own level formats. JSON or YAML are popular choices for storing level data, but binary formats are also common for performance reasons. You'll often need custom parsing code. |

Key Considerations:

* Performance: For large models or complex animations, efficient loading and parsing are crucial. Consider crates that are optimized for performance.
* Flexibility: If you need to support a wide range of formats, using a library like `assimp` (with bindings) might be a good option.
* Ease of Use: For simpler formats or custom data, writing your own parser using crates like `nom` or `chumsky` might be more straightforward.
* Serialization/Deserialization: `serde` is your friend for easily converting parsed data into Rust structs and vice-versa. This is essential for working with the data after it's loaded.
* Custom Formats: Game development often involves custom formats. Be prepared to write parsers or use parser generators like `lalrpop` if needed.

</div>
