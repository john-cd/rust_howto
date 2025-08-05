# Images

[![cat~multimedia::images][cat~multimedia::images~badge]][cat~multimedia::images]{{hi:Images}}

Process or build images.

For most common image tasks, the [`image`][c~image~docs]↗{{hi:image}} crate will be sufficient. If you need very low-level control or are working with GPU-accelerated [[graphics | graphics]] , [`pixels`][c~pixels~docs]↗{{hi:pixels}} or [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} might be relevant. For more specialized image processing algorithms, explore [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}}. And for color management, use [`palette`][c~palette~docs]↗{{hi:palette}}.

## Image Loading and Saving, Manipulation, and Image Processing

[`image`][c~image~docs]↗{{hi:image}} is the most popular and comprehensive crate for working with images. It supports a wide range of formats (PNG, JPEG, GIF, TIFF, WebP, BMP, etc.) and provides functionality for loading, saving, and manipulating images. It provides functions for resizing, cropping, rotating, filtering, and other image processing operations.

[`imageproc`][c~imageproc~docs]↗{{hi:imageproc}} provides more advanced image processing algorithms.

Specialized crates exist for specific formats like PNG, JPEG, GIF, WebP: [`lodepng`][c~lodepng~docs]↗{{hi:lodepng}}, [`jpeg-decoder`][c~jpeg_decoder~docs]↗{{hi:jpeg-decoder}}, [`gif`][c~gif~docs]↗{{hi:gif}}, [`webp`][c~webp~docs]↗{{hi:webp}}.

{{#include images.incl.md}}

## Low-level Pixel Manipulation - Pixel Buffers and Data Structures

[`pixels`][c~pixels~docs]↗{{hi:pixels}} is a crate for working with pixel buffers directly. Useful for low-level image manipulation or when integrating with graphics APIs.

{{#include pixel_buffers.incl.md}}

## Color Handling

[`palette`][c~palette~docs]↗{{hi:palette}} works with colors, color spaces, and color conversions.

{{#include color_handling.incl.md}}

## Image Rendering

See

- [[rendering | Rendering]].
- [[rendering_data-formats | Rendering Data Formats]].
- [[rendering_engine | Rendering Engine]].
- [[rendering_graphics-api | Rendering: Graphics API]].

## GPU-accelerated Image Processing

The [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`gfx-hal`][c~gfx_hal~docs]↗{{hi:gfx-hal}} low-level graphics APIs are often used for displaying images or performing GPU-accelerated image processing. See [[gpu_abstraction_layers | GPU Abstraction Layers]], [[native_graphics_apis | Native Graphics APIs]], and [[shaders | Shaders]].

## Related Topics

- [[graphics | Graphics]].
- [[multimedia | Multimedia]].
- [[multimedia_audio | Multimedia: Audio]].
- [[multimedia_encoding | Multimedia: Encoding]].
- [[multimedia_video | Multimedia: Video]].

Refer as well to [image-rs (GitHub)](https://github.com/image-rs).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/942)
</div>
