# Images

[![cat-multimedia::images][cat-multimedia::images-badge]][cat-multimedia::images]{{hi:Images}}

Process or build images.

For most common image tasks, the [`image`][c-image]⮳{{hi:image}} crate will be sufficient. If you need very low-level control or are working with GPU-accelerated [[graphics | graphics]] , [`pixels`][c-pixels]⮳{{hi:pixels}} or [`wgpu`][c-wgpu]⮳{{hi:wgpu}} might be relevant. For more specialized image processing algorithms, explore [`imageproc`][c-imageproc]⮳{{hi:imageproc}}. And for color management, use [`palette`][c-palette]⮳{{hi:palette}}.

| Topic | Rust crate(s) | Notes |
|---|---|---|
| General image loading, saving, and manipulation | [`image`][c-image]⮳{{hi:image}} | [`image`][c-image]⮳{{hi:image}} is the most popular and comprehensive crate for working with images. It supports a wide range of formats (PNG, JPEG, GIF, TIFF, WebP, BMP, etc.) and provides functionality for loading, saving, and manipulating images. It provides functions for resizing, cropping, rotating, filtering, and other image processing operations. |
| Low-level pixel manipulation | [`pixels`][c-pixels]⮳{{hi:pixels}} | [`pixels`][c-pixels]⮳{{hi:pixels}} is a crate for working with pixel buffers directly. Useful for low-level image manipulation or when integrating with graphics APIs. |
| Advanced image processing | [`imageproc`][c-imageproc]⮳{{hi:imageproc}} | Provides more advanced image processing algorithms. |
| Color handling | [`palette`][c-palette]⮳{{hi:palette}} | For working with colors, color spaces, and color conversions. |
| GPU-accelerated image processing | [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} | [`wgpu`][c-wgpu]⮳{{hi:wgpu}} (but requires more setup) These low-level graphics APIs are often used for displaying images or performing GPU-accelerated image processing. |

## Image Loading and Saving

{{#include images.incl.md}}

## Image Manipulation

## Pixel Buffers and Data Structures

## Color Handling

## Image Processing

## Graphics APIs

See [[graphics | Graphics]].

## Related Topics

- [[multimedia | Multimedia]]
- [[multimedia_audio | Multimedia: Audio]]
- [[multimedia_encoding | Multimedia: Encoding]]
- [[multimedia_video | Multimedia: Video]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/942)
cover loading/saving, manip, pixels, color, etc
- Image Decoding and Encoding
- Image Filters and Transformations
- Image Analysis
- Image Compression
- Image Rendering
- Image processing
</div>
