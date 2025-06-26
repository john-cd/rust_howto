# OpenCV

{{#include opencv.incl.md}}

## Analyze Images with `opencv` {#opencv}

[![opencv][c~opencv~docs~badge]][c~opencv~docs]{{hi:opencv}}
[![opencv~crates.io][c~opencv~crates.io~badge]][c~opencv~crates.io]
[![opencv~github][c~opencv~github~badge]][c~opencv~github]
[![opencv~lib.rs][c~opencv~lib.rs~badge]][c~opencv~lib.rs]
[![cat~computer-vision][cat~computer-vision~badge]][cat~computer-vision]

See also [OpenCV (example)][c~opencv~example]{{hi:opencv}}⮳.

'OpenCV' (Open Source Computer Vision Library) is a widely-used open-source library for computer vision, [machine learning][p~machine-learning], and image processing. It provides a comprehensive set of tools and algorithms for various tasks, including:

- Image and [Video][p~video] Processing: Reading, writing, manipulating, and displaying [images][p~images] and videos.
- Object Detection: Identifying and locating objects in [images][p~images] and videos (e.g., face detection, object tracking).
- Machine Learning: Integrating [machine learning][p~machine-learning] algorithms for tasks like image classification, pattern recognition, and more.
- 3D Vision: Working with depth cameras, stereo vision, and 3D reconstruction.

[OpenCV][c~opencv~website]{{hi:opencv}}⮳ is written in C++ but offers interfaces for other languages, including Rust. It is [cross-platform][p~cross-platform], running on various operating systems (Windows, Linux, macOS, Android, iOS), and is used in a wide range of applications, including:

- Computer Vision Research: Providing a standard platform for implementing and [testing][p~testing] computer vision algorithms.
- Industrial Automation: Enabling tasks like quality control, object recognition, and robotic vision.
- Security and Surveillance: Supporting applications like facial recognition, motion detection, and [video][p~video] analytics.
- Medical Imaging: Assisting in tasks like image analysis, diagnosis, and treatment planning.
- Augmented Reality: Enabling the creation of AR applications that overlay digital information onto the real world.

The [`opencv`][c~opencv~docs]⮳{{hi:opencv}} Rust crate provides bindings to OpenCV.

```rust,editable
{{#include ../../../crates/cats/computer_vision/examples/opencv/opencv.rs:example}}
```

## See Also

- [[algorithms | Algorithms]].
- [[mathematics | Mathematics]].
- [[linear_algebra | Linear Algebra]].
- [[_machine_learning |  Machine Learning]].
- [[multimedia | Multimedia]].
- [[multimedia_images | Multimedia Images]].
- [[multimedia_video | Multimedia Video]].
- [[science_robotics | Science Robotics]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[opencv: expand, add example](https://github.com/john-cd/rust_howto/issues/257)
</div>
