# Robotics

[![cat~science::robotics][cat~science::robotics~badge]][cat~science::robotics]

Crates related to robotics.

While still developing, the Rust robotics ecosystem is gaining momentum. Rust's performance is a major advantage for robotics, especially for real-time control, sensor processing, and computationally intensive tasks. Rust's memory safety and performance make it well-suited for embedded systems and microcontrollers. Some areas might have fewer mature options compared to Python's ROS or other robotics frameworks.

## Robot Operating Systems

{{#include robot_operating_systems.incl.md}}

## Robotics Frameworks

{{#include robotics_frameworks.incl.md}}

## Hardware Integration

{{#include hardware_integration.incl.md}}

Consider [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and platform-specific crates. [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} defines a standard trait interface for interacting with embedded hardware. [`linux-embedded-hal`][c~linux-embedded-hal~docs]↗{{hi:linux-embedded-hal}} provides implementations for Linux systems.

See also:

- [[embedded | Embedded Systems]] Development.
- [[hardware-support | Hardware]] Abstraction.

## File Loading

Import/Export various files related with Robotics:

[`assimp-rs`][c~assimp~docs]↗{{hi:assimp-rs}} (open-asset-importer) - Rust bindings for the Assimp library.
[`mcap`][c~mcap~crates.io]↗{{hi:mcap}} - Rust library for reading and writing MCAP log files.
[`urdf-rs`][c~urdf-rs~crates.io]↗{{hi:urdf-rs}} - URDF Loader for Rust.

See also [[parser-implementations | Parser Implementations]].

## Control Systems: Motion Control, Feedback Loops, PID Controllers, Path Planning

{{#include control_systems.incl.md}}

## Perception and Sensors

{{#include perception_and_sensors.incl.md}}

See also the [[computer-vision | Computer Vision]] chapter: [`opencv-rs`][c~opencv~docs]↗{{hi:opencv-rs}}, for example, provides bindings to OpenCV for computer vision tasks.

## Artificial Intelligence and Decision-making for Robotics

{{#include artificial_intelligence.incl.md}}

See also the machine learning chapters: [[classical_machine_learning | Classical Machine Learning]] and [[deep_learning | Deep Learning]].

## Simulation and Visualization

{{#include simulation_visualization.incl.md}}

See also the [[simulation | Simulation]], [[aerospace_simulation | Aerospace Simulation]], and [[visualization | Visualization]] chapters. [[game-engines | Game Engines]] like [`bevy`][c~bevy~docs]↗{{hi:bevy}} can also be adapted for robotics simulation.

## Math and Geometry-related Libraries for Robotics

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} - Linear algebra library for Rust.
- `ncollide2d`, `ncollide3d` - 2 and 3-dimensional collision detection library in Rust.
- [`kdtree`][c~kdtree~crates.io]↗{{hi:kdtree}} - K-dimensional tree in Rust for fast geospatial indexing.
- [`k`][c~k~crates.io]↗{{hi:k}} - Kinematics Library for rust-lang.
- [`static-math`][c~static-math~docs]{{hi:static-math}} - Safe and fast mathematical operations with static arrays in Rust programming language for robotics.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} - N-dimensional tensor arithmetic library, inspired by python's NumPy.
- [`faer-rs`][faer-rs~github]↗{{hi:faer-rs}} - Linear algebra foundation for the Rust programming language.

See also the numerical computation ([[additional_numeric_types | Additional Numeric Types]], [[linear_algebra | Linear Algebra]]), [[data-processing | data]] analysis sections.

## Related Topics

Leverage existing robotics libraries (often written in C++, Python, or other languages) by creating Rust bindings using [[development-tools_ffi | FFI]]. Also utilize Rust's [[concurrency | concurrency]] crates to build robotics-specific tools.

## See Also

[![robotics.rs][robotics-rs~website~badge]][robotics-rs~website] [robotics.rs][robotics-rs~website]↗..

[Robotics (lib.rs)][robotics~lib.rs]↗ [![cat~science::robotics][cat~science::robotics~badge]][cat~science::robotics]{{hi:Robotics}}

[Why rust for robots][why-rust-for-robots~website]↗.

[Linux embracing Rust will boost robotics community][blog~linux-embracing-rust-will-boost-robotics-community]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize; review in depth](https://github.com/john-cd/rust_howto/issues/480)
review

- [robotics][robotics~website]↗.
- [rust-embedded~github][rust-embedded~github]↗.

---

[![ehmi][c~ehmi~docs~badge]][c~ehmi~docs] [![ehmi~crates.io][c~ehmi~crates.io~badge]][c~ehmi~crates.io] [![ehmi~github][c~ehmi~github~badge]][c~ehmi~github] [![ehmi~lib.rs][c~ehmi~lib.rs~badge]][c~ehmi~lib.rs]{{hi:ehmi}}{{hi:Ui}}{{hi:Plc}}{{hi:Egui}}{{hi:Hmi}}{{hi:SCADA}}

HMI components for egui.

---

[![roboplc][c~roboplc~docs~badge]][c~roboplc~docs] [![roboplc~crates.io][c~roboplc~crates.io~badge]][c~roboplc~crates.io] [![roboplc~github][c~roboplc~github~badge]][c~roboplc~github] [![roboplc~lib.rs][c~roboplc~lib.rs~badge]][c~roboplc~lib.rs]{{hi:roboplc}}{{hi:Realtime}}{{hi:Robots}}{{hi:Plc}}{{hi:Industrial}}

Framework for PLCs and real-time micro-services.

[roboplc~website][c~roboplc~website].

</div>
