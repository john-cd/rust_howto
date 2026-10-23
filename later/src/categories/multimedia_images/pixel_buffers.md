<!-- TODO Reviewed ??-??-2026 -->
# Pixel Buffers and Data Structures

{{#include pixel_buffers.incl.md}}

## `pixels` {#pixel_buffers}

[![pixels][c~pixels~docs~badge]][c~pixels~docs] [![pixels~crates.io][c~pixels~crates.io~badge]][c~pixels~crates.io] [![pixels~repo][c~pixels~repo~badge]][c~pixels~repo] [![pixels~lib.rs][c~pixels~lib.rs~badge]][c~pixels~lib.rs]{{hi:pixels}}{{hi:2d}}{{hi:Framebuffer}}{{hi:Gpu}}{{hi:pixels}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}} [![cat~rendering][cat~rendering~badge]][cat~rendering]{{hi:Rendering}}

A tiny library providing a GPU-powered pixel frame buffer.

`pixels` owns a CPU-side frame buffer and presents it through a GPU-backed window surface. Its `frame_mut()` method exposes the frame as bytes; the documented frame format is RGBA8, with four bytes per pixel in red, green, blue, alpha order. A raw buffer is fast to iterate, but the application must keep its dimensions, format, and indexing rules consistent.

## Layout and Stride

For a tightly packed image, the byte offset of pixel `(x, y)` is:

```text
offset = (y * width + x) * bytes_per_pixel
```

This formula assumes rows have no padding. General image and GPU APIs may use a stride, or row pitch, larger than `width * bytes_per_pixel`; calculate the row start as `y * stride` and then add the pixel's byte offset. Some GPU transfer APIs also require row-pitch alignment, so consult the API's rules instead of assuming a tightly packed layout can be copied directly.

Keep the pixel format alongside the buffer. Four bytes might mean RGBA, BGRA, or another channel order, and a 16-bit or floating-point format has different component sizes. Likewise, an alpha channel can be straight or premultiplied; blending code must match the convention expected by the renderer.

## Update a Frame

With a configured `pixels::Pixels` value named `pixels`, fill the RGBA8 frame as a byte slice:

```rust
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
- [2D Raster Graphics](../rendering/2d_raster_graphics.md) and [2D Renderers](../rendering/2d_renderers.md) cover drawing and presentation.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1222)
</div>
