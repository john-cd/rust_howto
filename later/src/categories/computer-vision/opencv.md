<!-- TODO Reviewed ??-??-2026 -->

# OpenCV

{{#include opencv.incl.md}}

The [`opencv`][c~opencv~docs] crate generates Rust bindings to the native OpenCV library. Install the native SDK, its development metadata, and libclang; the crate alone does not supply all system libraries. On Ubuntu, the examples need `libopencv-dev`, `clang`, `libclang-dev`, and `pkg-config`. This workspace enables them on Linux behind its optional `opencv` feature.

## Load an Image and Convert to Grayscale {#opencv}

[![opencv][c~opencv~docs~badge]][c~opencv~docs]

Load a color image with `imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)` and check for an empty `Mat` before processing it. OpenCV's normal color order is BGR. This helper converts a BGR frame to one grayscale channel while preserving its dimensions:

```rust,noplayground
{{#include ../../../crates/cats/computer_vision/examples/opencv/opencv.rs:grayscale}}
```

The host test constructs a 24-by-32 color `Mat` in memory and checks both the dimensions and channel count. The complete workflow below also tests reading and writing a generated image in an isolated temporary directory.

## Detect Edges with Canny {#canny}

Blur a grayscale frame to reduce noise, then compute an edge mask with `imgproc::canny`. The thresholds depend on the image and desired sensitivity; they are example parameters rather than universal settings.

```rust,noplayground
{{#include ../../../crates/cats/computer_vision/examples/opencv/opencv.rs:canny}}
```

A generated constant frame produces an empty mask with the same dimensions. A separate generated rectangle fixture verifies that the complete pipeline detects nonzero edges.

## Detect Faces with a Haar Cascade {#haar-faces}

`objdetect::CascadeClassifier` loads an XML model and detects candidate rectangles in a grayscale frame. Use an explicit model path rather than depending on a machine-specific data directory. The [OpenCV cascade tutorial][opencv~cascade-tutorial] explains model loading and detection; the [upstream frontal-face model][opencv~haar-model] is suitable for exercising this example.

```rust,noplayground
{{#include ../../../crates/cats/computer_vision/examples/opencv/faces.rs:example}}
```

Set `OPENCV_HAAR_CASCADE` to the downloaded model's path before running this recipe. The model-dependent test is ignored by default and checks that a generated 64-by-64 blank frame produces no face rectangles. This verifies loading and API integration, not detection quality on a representative face dataset.

## Run the Complete Image Workflow

From `later/crates` on a configured Linux host:

```sh
cargo run -p computer_vision --example opencv --features opencv
cargo test -p computer_vision --all-targets --features opencv
```

To include the classifier test after selecting a model:

```sh
OPENCV_HAAR_CASCADE=/path/to/haarcascade_frontalface_default.xml cargo test -p computer_vision --all-targets --features opencv -- --include-ignored
```

The complete file-processing example generates an input image, reads it, converts it, detects edges, and writes the result. File and native-library errors are propagated:

```rust,noplayground
{{#include ../../../crates/cats/computer_vision/examples/opencv/opencv.rs:example}}
```

## Pure Rust Alternatives

For image buffers and common processing operations without the OpenCV SDK, compare [`image`][c~image~docs] and [`imageproc`][c~imageproc~docs]. See [Images](../multimedia_images/images.md) and the [Computer Vision index](index.md) for related processing and geometry tools.

## Related Topics {#related-topics .skip}

- [Images](../multimedia_images/images.md).
- [Perception and Sensors](../science_robotics/perception_and_sensors.md).
- [Linear algebra](https://github.com/john-cd/rust_howto/blob/develop/bk/src/categories/mathematics/linear_algebra.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
