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

</div>
