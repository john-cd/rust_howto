# Images

[![cat-multimedia::images][cat-multimedia::images-badge]][cat-multimedia::images]{{hi:Images}}

Process or build images.

{{#include images.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/942)

## Key Concepts

- Image formats: PNG, JPEG, GIF, TIFF, WebP, etc.
- Pixel data: The raw data representing the image pixels.
- Color spaces: RGB, CMYK, etc.
- Image processing: Resizing, cropping, filtering, etc.

## Choosing Crates

- General image loading, saving, and manipulation: `image` is the standard and usually the best choice.
- Low-level pixel manipulation: `pixels`.
- Advanced image processing: `imageproc`.
- Color handling: `palette`.
- GPU-accelerated image processing: `wgpu` (but requires more setup).

For most common image tasks, the `image` crate will be sufficient. If you need very low-level control or are working with GPU-accelerated graphics, `pixels` or `wgpu` might be relevant. For more specialized image processing algorithms, explore `imageproc`. And for color management, use `palette`.

## Image Loading and Saving

- `image`: The most popular and comprehensive crate for working with images. Supports a wide range of formats (PNG, JPEG, GIF, TIFF, WebP, BMP, etc.) and provides functionality for loading, saving, and manipulating images.

## Image Manipulation

- `image`: Also used for manipulation. Provides functions for resizing, cropping, rotating, filtering, and other image processing operations.

## Image Formats (Specific Formats)

- Often, the `image` crate handles most common formats. However, for less common or specialized formats, you might need to find specific crates.

## Pixel Buffers and Data Structures

- `pixels`: A crate for working with pixel buffers directly. Useful for low-level image manipulation or when integrating with graphics APIs.

## Color Handling

- `palette`: For working with colors, color spaces, and color conversions.

## Image Processing Libraries (More Advanced)

- `imageproc`: A crate that provides more advanced image processing algorithms.

## Graphics APIs (Related to Images)

- `wgpu`, `gfx-hal`: These low-level graphics APIs are often used for displaying images or performing GPU-accelerated image processing.

</div>
