<!-- TODO Reviewed ??-??-2026. -->

# Computer Vision

[![cat~computer-vision][cat~computer-vision~badge]][cat~computer-vision]{{hi:Computer vision}}

Computer-vision applications turn images or video frames into measurements, detections, or decisions. Choose a crate for the pipeline stage you need; image decoding, geometric transforms, classical algorithms, and model inference have different data and runtime requirements.

| Need | Common starting point | Considerations |
| --- | --- | --- |
| Decode, resize, crop, or filter image buffers | [`image`][c~image~docs]↗{{hi:image}}, [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}}, [`fast-image-resize`][c~fast_image_resize~docs]↗{{hi:fast-image-resize}} | Check pixel type, color channels, and image dimensions at each stage. See [Images](https://john-cd.com/rust_howto/categories/multimedia_images/index.html). |
| Use a broad set of established algorithms or interoperate with existing CV systems | [`opencv`][c~opencv~docs]↗{{hi:opencv}} | The Rust crate binds the native OpenCV library; install a compatible SDK and linker toolchain. See the examples below. |
| Run an existing trained model | [`ort`][c~ort~docs]↗{{hi:ort}}, [`tract`][c~tract~docs]↗{{hi:tract}}, or [`tch`][c~tch~docs]↗{{hi:tch}} | Match the model format, operators, device backend, and preprocessing pipeline. See [Machine Learning in Practice](https://john-cd.com/rust_howto/other/machine-learning/index.html). |
| Work with geometry and numerical arrays | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} or [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} | Keep coordinate conventions and units explicit when moving points or boxes between images and world coordinates. |

{{#include opencv.incl.md}}

## Keep the Pipeline Consistent

Check whether the next operation expects RGB or BGR channels, interleaved or planar data, normalized floating-point values, or integer pixel values. A model's resize, crop, and normalization steps must match the steps used during training. If a detector reports coordinates in a resized or cropped frame, retain the transform needed to map them back to the source image.

OpenCV's image-loading APIs normally use BGR channel order; other image libraries commonly use RGB. Do not pass a buffer between libraries without checking the API's channel and stride conventions.

## Related Topics {#related-topics .skip}

- [Images](https://john-cd.com/rust_howto/categories/multimedia_images/index.html) and [[multimedia_video | Multimedia: Video]].
- [Machine Learning in Practice](https://john-cd.com/rust_howto/other/machine-learning/index.html).
- [[science_robotics | Robotics]] and [Rendering](../rendering/index.md).
- [[mathematics | Mathematics]] for arrays and linear algebra.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
