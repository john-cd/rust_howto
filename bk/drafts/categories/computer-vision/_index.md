# Computer Vision

[![cat-computer-vision][cat-computer-vision-badge]][cat-computer-vision]{{hi:Computer vision}}

Comprehend the world from video{{hi:Video}} or images{{hi:Images}}.

## OpenCV

{{#include opencv.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

Several Rust crates cater to various aspects of computer vision. Here's a breakdown of some prominent ones and their functionalities:

- [`image`][c-image]⮳{{hi:image}}: This crate focuses on image decoding and encoding for various formats (JPEG, PNG, GIF, TIFF, etc.). It's essential for loading and saving images in different formats, a fundamental step in many computer vision pipelines.
- [`imageproc`][c-imageproc]⮳{{hi:imageproc}}: Building on the [`image`][c-image]⮳{{hi:image}} crate, [`imageproc`][c-imageproc]⮳{{hi:imageproc}} provides image processing algorithms, such as filtering, resizing, color manipulation, and drawing. It's useful for image preprocessing and basic image manipulation tasks.
- [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`ndarray`][c-ndarray]⮳{{hi:ndarray}}: These crates provide linear algebra and multi-dimensional array capabilities, respectively. They are frequently used in computer vision for tasks involving matrix operations, image representation, and algorithm implementations. [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} is often preferred for smaller, fixed-size matrices, while `ndarray` is better suited for larger, dynamically sized arrays.
- [`rust-cv`][c-rust_cv]⮳{{hi:rust-cv}}: This project aims to provide a pure Rust computer vision library, offering an alternative to using OpenCV bindings. It's still under development but has made progress in implementing various algorithms.
- [`vision`][c-vision]⮳{{hi:vision}}: This crate provides some higher-level computer vision algorithms, including feature detection and matching.
- [`tch`][c-tch]⮳{{hi:tch}}: This crate provides bindings to the PyTorch machine learning framework, enabling the use of PyTorch models in Rust. While not strictly a computer vision crate, it's crucial for deep learning-based computer vision tasks.
- [`tract`][c-tract]⮳{{hi:tract}}: This crate provides a framework for running and training neural networks, including those used in computer vision.
- `candle`: This crate provides a minimal and efficient tensor library for machine learning, often used in computer vision applications.
- [`fast-image-resize`][c-fast_image_resize]⮳{{hi:fast-image-resize}}: This crate provides a fast image resizing implementation, which can be useful when performance is critical.

For general-purpose computer vision, the [`opencv`][c-opencv]⮳{{hi:opencv}} crate is often a good starting point. For image I/O and manipulation, image and imageproc are essential. For more specialized tasks or when a pure Rust implementation is desired, other crates like `rust-cv`, `vision`, or machine learning-focused crates like `tch`, `tract`, or `candle` might be more appropriate.

[[mathematics | Mathematics]]
[[linear_algebra | Linear Algebra]]

[[_machine_learning |  Machine Learning]]

[[multimedia | Multimedia]]
[[multimedia_images | Multimedia Images]]
[[multimedia_video | Multimedia Video]]

[[science_robotics | Science Robotics]]
</div>
