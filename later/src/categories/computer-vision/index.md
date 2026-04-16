# Computer Vision

[![cat~computer-vision][cat~computer-vision~badge]][cat~computer-vision]{{hi:Computer vision}}

Computer Vision refers to comprehending the world from video{{hi:Video}} or images{{hi:Images}}.

For general-purpose computer vision, the [`opencv`][c~opencv~docs]↗{{hi:opencv}} crate is often a good starting point.

{{#include opencv.incl.md}}

Several other pure-Rust crates cater to various aspects of computer vision:

- [`rust-cv`][c~rust-cv~docs]↗{{hi:rust-cv}} aims to provide a pure Rust computer vision library, offering an alternative to using OpenCV bindings. It's still under development.
- [`vision`][c~vision~docs]↗{{hi:vision}} provides some higher-level computer vision algorithms, including feature detection and matching.

## Related Topics

For image I/O and manipulation, [`image`][c~image~docs]↗{{hi:image}} and [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}} are essential. For more specialized tasks, machine learning-focused crates like [`tch`][c~tch~docs]↗{{hi:tch}}, [`tract`][c~tract~docs]↗{{hi:tract}}, or [`candle`][c~candle_core~repo]↗{{hi:candle}} are appropriate.

## Multimedia

See:

- [[multimedia | Multimedia]].
- [[multimedia_images | Multimedia: Images]].
- [[multimedia_video | Multimedia: Video]].

## Image Processing

- [`image`][c~image~docs]↗{{hi:image}} focuses on image decoding and encoding for various formats (JPEG, PNG, GIF, TIFF, etc.). It's essential for loading and saving images in different formats.
- [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}}: Building on the [`image`][c~image~docs]↗{{hi:image}} crate, [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}} provides image processing algorithms, such as filtering, resizing, color manipulation, and drawing. It is useful for image preprocessing and basic image manipulation tasks.
- [`fast-image-resize`][c~fast_image_resize~docs]↗{{hi:fast-image-resize}} provides a fast image resizing implementation, which can be useful when performance is critical.

See [[multimedia_images | Multimedia: Images]].

## Linear Algebra

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} and [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} provide linear algebra and multi-dimensional array capabilities, respectively. They are frequently used in computer vision for tasks involving matrix operations and algorithm implementations. [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} is often preferred for smaller, fixed-size matrices, while [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} is better suited for larger, dynamically sized arrays.

See [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]].

## Machine Learning

- [`tch`][c~tch~docs]↗{{hi:tch}} provides bindings to the PyTorch machine learning framework, enabling the use of PyTorch models in Rust. While not strictly a computer vision crate, it is useful for deep learning-based computer vision tasks.
- [`tract`][c~tract~docs]↗{{hi:tract}} provides a framework for running and training neural networks, including those used in computer vision.
- [`candle`][c~candle_core~docs]↗{{hi:candle}} provides a minimal and efficient tensor library for machine learning.

See [[_machine_learning |  Machine Learning]].

## Robotics

See [[science_robotics | Science: Robotics]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
