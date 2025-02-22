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
| Image Formats (2D Textures/Sprites) | [`image`][c-image]⮳{{hi:image}}, [`imageproc`][c-imageproc]⮳{{hi:imageproc}}, [`lodepng`][c-lodepng]⮳{{hi:lodepng}}, [`jpeg-decoder`][c-jpeg_decoder]⮳{{hi:jpeg-decoder}}, [`gif`][c-gif]⮳{{hi:gif}}, [`webp`][c-webp]⮳{{hi:webp}} | [`image`][c-image]⮳{{hi:image}} is a general image processing library supporting many formats. Specialized crates exist for specific formats like PNG, JPEG, GIF, WebP. Used for textures, sprites, etc. |
| 2D Sprite Sheets/Animation Data | (Often custom formats or JSON/YAML) | Many games use custom formats for sprite sheets and animation data. JSON or YAML are often used to describe animation sequences and frame timings. You'll likely use [`serde`][c-serde]⮳{{hi:serde}} for parsing. |
| Font Formats (Text Rendering) | [`fontdue`][c-fontdue]⮳{{hi:fontdue}}, [`ttf-parser`][c-ttf_parser]⮳{{hi:ttf-parser}}, [`opentype`][c-opentype]⮳{{hi:opentype}}, [`ab_glyph`][c-ab_glyph]⮳{{hi:ab_glyph}} | [`fontdue`][c-fontdue]⮳{{hi:fontdue}} is a fast font rasterizer. [`ttf-parser`][c-ttf_parser]⮳{{hi:ttf-parser}} and [`opentype`][c-opentype]⮳{{hi:opentype}} provide lower-level font parsing capabilities. [`ab_glyph`][c-ab_glyph]⮳{{hi:ab_glyph}} is another font rendering option. |
| Vector Graphics (SVG) | [`resvg`][c-resvg]⮳{{hi:resvg}}, `usvg` | [`resvg`][c-resvg]⮳{{hi:resvg}} is an SVG rendering library. [`usvg`][c-usvg]⮳{{hi:usvg}} is another option. Used for scalable vector graphics. |
| 3D Model Formats | [`serde_json`][c-serde_json]⮳{{hi:serde_json}} (for glTF), `obj`, `stl`, [`assimp`][c-assimp]⮳{{hi:assimp}} (bindings), [`fbx`][c-fbx]⮳{{hi:fbx}} (sometimes) | glTF often uses JSON for metadata and binary data for geometry. `obj` and [`stl`][c-stl]⮳{{hi:stl}} are common 3D model formats. [`assimp`][c-assimp]⮳{{hi:assimp}} is a powerful library (with Rust bindings) that supports many formats. FBX is a complex format; crates are less common and bindings to Assimp might be the most practical approach. |
| Scene Description Languages (SDF) | (Often custom or using parser generators) | SDFs describe 3D scenes mathematically. Parsing often involves custom code or parser generators like [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}} due to the specific syntax. |
| Animation Formats (Keyframe, Skeletal) | (Often custom or using parser generators, or glTF extensions) | Animation data can be stored in various ways. Keyframe animations might be simple data structures. Skeletal animation is more complex. glTF supports animation, but custom formats are also common. You might use [`serde`][c-serde]⮳{{hi:serde}} and custom structs. |
| Shader Code (GLSL, HLSL, SPIR-V) | [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}}, [`shaderc`][c-shaderc]⮳{{hi:shaderc}}, `naga` | [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}} compiles GLSL to SPIR-V. [`shaderc`][c-shaderc]⮳{{hi:shaderc}} is another shader compiler. [`naga`][c-naga]⮳{{hi:naga}} is a general-purpose shader translation and analysis tool. Parsing is often part of the compilation process. |
| Materials/Shaders (Custom Formats) | (Often custom or using JSON/YAML) | Materials and shaders are often described using custom formats or JSON/YAML. You'll likely use [`serde`][c-serde]⮳{{hi:serde}} for parsing. |
| Level Data (Game Engines) | (Often custom or using JSON/YAML) | Game engines often use their own level formats. JSON or YAML are popular choices for storing level data, but binary formats are also common for performance reasons. You'll often need custom parsing code. |

Key Considerations:

* Performance: For large models or complex animations, efficient loading and parsing are crucial. Consider crates that are optimized for performance.
* Flexibility: If you need to support a wide range of formats, using a library like [`assimp`][c-assimp]⮳{{hi:assimp}} (with bindings) might be a good option.
* Ease of Use: For simpler formats or custom data, writing your own parser using crates like [`nom`][c-nom]⮳{{hi:nom}} or [`chumsky`][c-chumsky]⮳{{hi:chumsky}} might be more straightforward.
* Serialization/Deserialization: [`serde`][c-serde]⮳{{hi:serde}} is your friend for easily converting parsed data into Rust structs and vice-versa. This is essential for working with the data after it's loaded.
* Custom Formats: Game development often involves custom formats. Be prepared to write parsers or use parser generators like [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}} if needed.

</div>
