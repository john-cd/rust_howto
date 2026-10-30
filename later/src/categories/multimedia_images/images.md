<!-- TODO Reviewed ??-??-2026 -->

# Images

{{#include images.incl.md}}

Use `image` for buffers and common codecs, `kamadak-exif` to inspect camera metadata, and `oxipng` to recompress PNGs. These recipes generate their own data and do not need downloaded photographs. For parallel processing of a directory of files, use the published [Rayon thumbnail recipe][images~parallel-thumbnails] instead of duplicating its traversal and worker setup here.

## Create and Save an Image {#create-image}

An `ImageBuffer` holds pixels in memory; `save` chooses the file format from the extension. The example builds an RGB gradient in memory and writes it to the PNG path given on the command line (or `generated_image.png` by default).

```rust,editable
{{#include ../../../crates/cats/multimedia_images/examples/images/images.rs:example}}
```

## Resize and Crop

[![image][c~image~docs~badge]][c~image~docs]

Resize a generated gradient and convert it to grayscale. Choose an interpolation filter for the desired quality and cost; `crop_imm` selects a rectangular region without resizing it.

```rust,editable
{{#include ../../../crates/cats/multimedia_images/examples/images/resize.rs:example}}
```

## Convert Formats

[![image][c~image~docs~badge]][c~image~docs]

Encode an image to PNG and JPEG in memory. JPEG is lossy and does not retain an alpha channel; format conversion can also discard metadata, so preserve metadata explicitly when the application needs it.

```rust,editable
{{#include ../../../crates/cats/multimedia_images/examples/images/convert.rs:example}}
```

## Apply Filters

[![image][c~image~docs~badge]][c~image~docs]

Apply Gaussian blur to an in-memory image. `imageproc` supplies additional analysis and filtering operations; OpenCV recipes are covered in the [computer vision chapter](../computer-vision/opencv.md).

```rust,editable
{{#include ../../../crates/cats/multimedia_images/examples/images/blur.rs:example}}
```

## Read EXIF Metadata

[![kamadak-exif][c~kamadak-exif~docs~badge]][c~kamadak-exif~docs]

The Cargo package `kamadak-exif` exposes the Rust crate `exif`. Use `Reader::read_from_container` with a buffered file reader for JPEG/TIFF input. This self-contained example uses `read_raw` on a tiny generated TIFF record and checks the Orientation tag. Reading the tag does not rotate the pixels; orientation-aware display or conversion must apply that transformation separately.

```rust,editable
{{#include ../../../crates/cats/multimedia_images/examples/images/exif_metadata.rs:example}}
```

## Optimize PNGs

[![oxipng][c~oxipng~docs~badge]][c~oxipng~docs]

`oxipng::optimize_from_memory` changes PNG encoding choices while retaining the image. The test compares every decoded pixel before and after optimization; it does not assume every small input becomes smaller. Metadata stripping is a separate policy choice and is not requested here.

```rust,editable
{{#include ../../../crates/cats/multimedia_images/examples/images/optimize_png.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Multimedia encoding](../multimedia_encoding/encoding.md).
- [Computer vision](../computer-vision/index.md).
- [Graphics APIs](../rendering_graphics-api/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
