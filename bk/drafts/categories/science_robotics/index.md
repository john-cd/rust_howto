# Robotics

[![cat-science::robotics][cat-science::robotics-badge]][cat-science::robotics]

Crates related to robotics.

While still developing, the Rust robotics ecosystem is gaining momentum. Rust's performance is a major advantage for robotics, especially for real-time control, sensor processing, and computationally intensive tasks. Rust's memory safety and performance make it well-suited for embedded systems and microcontrollers. Some areas might have fewer mature options compared to Python's ROS or other robotics frameworks.

## Robot Operating Systems

{{#include robot_operating_systems.incl.md}}

## Robotics Frameworks

{{#include robotics_frameworks.incl.md}}

## Hardware Integration

{{#include hardware_integration.incl.md}}

Consider [`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}} and platform-specific crates. [`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}} defines a standard trait interface for interacting with embedded hardware. [`linux-embedded-hal`][c-linux_embedded_hal]⮳{{hi:linux-embedded-hal}} provides implementations for Linux systems.

See also

- [[embedded | Embedded Systems]] Development.
- [[hardware-support | Hardware]] Abstraction.

## File Loading

Import/Export various files related with Robotics

`assimp-rs` (open-asset-importer) - Rust bindings for the Assimp library.
`mcap` - Rust library for reading and writing MCAP log files
`urdf-rs` - URDF Loader for Rust
`pcd-ros` - Read point cloud data from PCD file format

See also [[parser-implementations | Parser Implementations]].

## Control Systems: Motion control, feedback loops, PID controllers, Path Planning

{{#include control_systems.incl.md}}

## Perception and Sensors

{{#include perception_and_sensors.incl.md}}

See also the [[computer-vision | Computer Vision]] chapter: [`opencv-rs`][c-opencv]⮳{{hi:opencv-rs}}, for example, provides bindings to OpenCV for computer vision tasks.

## Artificial Intelligence and Decision-making for Robotics

{{#include artificial_intelligence.incl.md}}

See also the machine learning chapters: [[classical_machine_learning | Classical Machine Learning]] and [[deep_learning | Deep Learning]].

## Simulation and Visualization

{{#include simulation_visualization.incl.md}}

See also the [[simulation | Simulation]], [[aerospace_simulation | Aerospace Simulation]], and [[visualization | Visualization]] chpaters. [[game-engines | Game Engines]] like [`bevy`][c-bevy]⮳{{hi:bevy}} can also be adapted for robotics simulation.

## Math and Geometry-related libraries for Robotics

- `nalgebra` - Linear algebra library for Rust.
- `ncollide` - 2 and 3-dimensional collision detection library in Rust.
- `kdtree` - K-dimensional tree in Rust for fast geospatial indexing.
- `k` - Kinematics Library for rust-lang.
- `static-math` - Safe and fast mathematical operations with static arrays in Rust programming language thinked for robotics
- `ndarray` - N-dimensional tensor arithmetic library, inspired by python’s NumPy.
- `faer-rs` - Linear algebra foundation for the Rust programming language

See also the numerical computation ([[additional_numeric_types | Additional Numeric Types]], [[linear_algebra | Linear Algebra]]), [[data-processing | data]] analysis sections.

## Related Topics

Leverage existing robotics libraries (often written in C++, Python, or other languages) by creating Rust bindings using [[development-tools_ffi | FFI]]. Also utilize Rust's [[concurrency | concurrency]] crates to build robotics-specific tools.

## See also

[![robotics.rs][robotics-rs-website-badge]][robotics-rs-website] [robotics.rs][robotics-rs-website]⮳

[Robotics (lib.rs)][robotics-lib.rs]⮳ [![cat-science::robotics][cat-science::robotics-badge]][cat-science::robotics]{{hi:Robotics}}

[Why rust for robots][why-rust-for-robots]⮳

[Linux embracing Rust will boost robotics community][linux-embracing-rust-will-boost-robotics-community]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize](https://github.com/john-cd/rust_howto/issues/480)
review in depth; review https://robotics.rs/
review https://github.com/rust-embedded/wg/
</div>
