<!-- TODO Reviewed ??-??-2026. -->

# Pixel Buffers and Data Structures

{{#include pixel_buffers.incl.md}}

## Inspect an RGBA Pixel Buffer {#pixel_buffers}

{{#badges image multimedia::images=Images graphics=Graphics}}{{hi:2d}}{{hi:Framebuffer}}{{hi:Gpu}}{{hi:Pixels}}

[`image`][c~image~docs] stores an `RgbaImage` as a CPU-side, row-major buffer.
The example below inspects its byte order without opening a window. For
GPU-backed presentation, [`pixels`][c~pixels~docs] exposes a mutable RGBA8 frame
through `frame_mut()`. The surface has its own format and lifetime requirements.

## Layout and Stride

For a tightly packed image, the byte offset of pixel `(x, y)` is:

```text
offset = (y * width + x) * bytes_per_pixel
```

This formula assumes rows have no padding. General image and GPU APIs may use a stride, or row pitch, larger than `width * bytes_per_pixel`; calculate the row start as `y * stride` and then add the pixel's byte offset. Some GPU transfer APIs also require row-pitch alignment, so consult the API's rules instead of assuming a tightly packed layout can be copied directly.

Keep the pixel format alongside the buffer. Four bytes might mean RGBA, BGRA, or another channel order, and a 16-bit or floating-point format has different component sizes. Likewise, an alpha channel can be straight or premultiplied; blending code must match the convention expected by the renderer.

The tested buffer example below uses two tightly packed RGBA pixels and verifies channel order and alpha:

```rust
{{#include ../../../crates/cats/multimedia_images/examples/pixel_buffers/rgba_pixel_buffer.rs:example}}
```

## Update a Frame

With a configured [`pixels::Pixels`][c~pixels~docs] value named `pixels`, fill
the RGBA8 frame as a byte slice:

```rust,noplayground
let frame = pixels.frame_mut();

for pixel in frame.chunks_exact_mut(4) {
    pixel.copy_from_slice(&[0x20, 0x70, 0xc0, 0xff]);
}
```

This paints every pixel the same opaque blue. For a gradient or image, compute each pixel's coordinates, convert the chosen color into the target channel format, and write only within the frame's bounds. `frame_mut()` retains the previous contents, so clear or update every region that should change before rendering the next frame.

## Check Size and Ownership

Before allocating a buffer, compute `width * height * bytes_per_pixel` with checked arithmetic and compare it with the application's memory budget. Validate external dimensions before using them to allocate or index. Reuse a frame buffer between frames when its dimensions are unchanged, and avoid copying full images when a row-wise conversion or borrowed slice is sufficient.

Do not cast arbitrary bytes to a pixel struct unless its layout, alignment, and validity are guaranteed. Use explicit channel conversion or a crate's documented pixel type. When handing a buffer to a GPU or another API, check who owns it, how long it must remain alive, and whether the receiver copies or borrows the data.

## Related Topics {#related-topics .skip}

- [Images](images.md) covers decoding and encoding image formats.
- [Color Handling](color_handling.md) covers channel encoding, color spaces, and alpha.
- [2D Raster Graphics](../graphics/2d_raster_graphics.md) and [2D Renderers](../graphics/2d_renderers.md) cover drawing and presentation.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
