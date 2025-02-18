# OpenCV

{{#include opencv.incl.md}}

## Analyze images with `opencv` {#opencv}

[![opencv][c-opencv-badge]][c-opencv]{{hi:opencv}}
[![opencv-crates.io][c-opencv-crates.io-badge]][c-opencv-crates.io]
[![opencv-github][c-opencv-github-badge]][c-opencv-github]
[![opencv-lib.rs][c-opencv-lib.rs-badge]][c-opencv-lib.rs]
[![cat-computer-vision][cat-computer-vision-badge]][cat-computer-vision]

[OpenCV (example)][c-opencv-example]{{hi:opencv}}⮳

'OpenCV' (Open Source Computer Vision Library) is a widely-used open-source library for computer vision, machine learning, and image processing. It provides a comprehensive set of tools and algorithms for various tasks, including:

- Image and Video Processing: Reading, writing, manipulating, and displaying images and videos.
- Object Detection: Identifying and locating objects in images and videos (e.g., face detection, object tracking).
- Machine Learning: Integrating machine learning algorithms for tasks like image classification, pattern recognition, and more.
- 3D Vision: Working with depth cameras, stereo vision, and 3D reconstruction.

OpenCV is written in C++ but offers interfaces for other languages, including Rust. It's cross-platform, running on various operating systems (Windows, Linux, macOS, Android, iOS), and is used in a wide range of applications, including:

- Computer Vision Research: Providing a standard platform for implementing and testing computer vision algorithms.
- Industrial Automation: Enabling tasks like quality control, object recognition, and robotic vision.
- Security and Surveillance: Supporting applications like facial recognition, motion detection, and video analytics.
- Medical Imaging: Assisting in tasks like image analysis, diagnosis, and treatment planning.
- Augmented Reality: Enabling the creation of AR applications that overlay digital information onto the real world.

The [`opencv`][c-opencv]⮳{{hi:opencv}} Rust crate provides bindings to OpenCV.

```rust,editable
{{#include ../../../crates/cats/computer_vision/tests/opencv.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[opencv: expand, add example (P2)](https://github.com/john-cd/rust_howto/issues/257)

Several Rust crates cater to various aspects of computer vision.  Here's a breakdown of some prominent ones and their functionalities:

- image: This crate focuses on image decoding and encoding for various formats (JPEG, PNG, GIF, TIFF, etc.).  It's essential for loading and saving images in different formats, a fundamental step in many computer vision pipelines.
- imageproc: Building on the image crate, imageproc provides image processing algorithms, such as filtering, resizing, color manipulation, and drawing.  It's useful for image preprocessing and basic image manipulation tasks.
- nalgebra and ndarray: These crates provide linear algebra and multi-dimensional array capabilities, respectively.  They are frequently used in computer vision for tasks involving matrix operations, image representation, and algorithm implementations.  nalgebra is often preferred for smaller, fixed-size matrices, while ndarray is better suited for larger, dynamically sized arrays.
- rust-cv: This project aims to provide a pure Rust computer vision library, offering an alternative to using OpenCV bindings. It's still under development but has made progress in implementing various algorithms.
- vision:  This crate provides some higher-level computer vision algorithms, including feature detection and matching.
- tch: This crate provides bindings to the PyTorch machine learning framework, enabling the use of PyTorch models in Rust.  While not strictly a computer vision crate, it's crucial for deep learning-based computer vision tasks.
- tract: This crate provides a framework for running and training neural networks, including those used in computer vision.
- candle: This crate provides a minimal and efficient tensor library for machine learning, often used in computer vision applications.
- fast-image-resize: This crate provides a fast image resizing implementation, which can be useful when performance is critical.

The choice of crate depends on the specific computer vision task.  For general-purpose computer vision, the opencv crate is often a good starting point.  For image I/O and manipulation, image and imageproc are essential.  For more specialized tasks or when a pure Rust implementation is desired, other crates like rust-cv, vision, or machine learning-focused crates like tch, tract, or candle might be more appropriate.
</div>
