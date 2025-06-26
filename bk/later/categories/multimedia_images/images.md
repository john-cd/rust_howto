# Images

{{#include images.incl.md}}

[`image`][c~image~docs]⮳{{hi:image}}, [`imageproc`][c~imageproc~docs]⮳{{hi:imageproc}} are essential for working with image data.

## Image Loading and Saving {#image_loading_and_saving}

[![image][c~image~docs~badge]][c~image~docs] [![image~crates.io][c~image~crates.io~badge]][c~image~crates.io] [![image~github][c~image~github~badge]][c~image~github] [![image~lib.rs][c~image~lib.rs~badge]][c~image~lib.rs]{{hi:image}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~multimedia::encoding][cat~multimedia::encoding~badge]][cat~multimedia::encoding]{{hi:Encoding}} [![cat~multimedia::images][cat~multimedia::images~badge]][cat~multimedia::images]{{hi:Images}}

Imaging library. Provides basic image processing and encoders/decoders for common image formats.

- Image Decoding and Encoding.
- Image Compression.

```rust,editable
{{#include ../../../crates/cats/multimedia_images/examples/images/images.rs:example}}
```

## Image Manipulation {#image_manipulation}

[![image][c~image~docs~badge]][c~image~docs] [![image~crates.io][c~image~crates.io~badge]][c~image~crates.io] [![image~github][c~image~github~badge]][c~image~github] [![image~lib.rs][c~image~lib.rs~badge]][c~image~lib.rs]{{hi:image}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~multimedia::encoding][cat~multimedia::encoding~badge]][cat~multimedia::encoding]{{hi:Encoding}} [![cat~multimedia::images][cat~multimedia::images~badge]][cat~multimedia::images]{{hi:Images}}

- Image Filters and Transformations.

{{#example image}}

## Image Processing {#image_processing}

[![imageproc~website][c~imageproc~website~badge]][c~imageproc~website] [![imageproc][c~imageproc~docs~badge]][c~imageproc~docs] [![imageproc~crates.io][c~imageproc~crates.io~badge]][c~imageproc~crates.io] [![imageproc~github][c~imageproc~github~badge]][c~imageproc~github] [![imageproc~lib.rs][c~imageproc~lib.rs~badge]][c~imageproc~lib.rs]{{hi:imageproc}}

Image processing operations.

Image Processing Fundamentals:

- Image Filtering (Blur, Sharpen, Edge Detection).
- Image Transformations (Resizing, Rotation).
- Image Compression.
- Color Space Conversion.

Image Analysis

{{#example imageproc}}

## GPU-accelerated Image Processing {#skip}

Low-level [graphics][p~graphics] APIs like [`wgpu`][c~wgpu~docs]⮳{{hi:wgpu}} are often used for displaying images or performing GPU-accelerated image processing.
See [[rendering_graphics-api | Rendering: Graphics API]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[images: write](https://github.com/john-cd/rust_howto/issues/420)
</div>
