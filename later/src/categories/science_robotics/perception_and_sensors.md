<!-- TODO Reviewed ??-??-2026 -->

# Perception and Sensors

{{#include perception_and_sensors.incl.md}}

Sensor fusion / integration, SLAM (Simultaneous Localization and Mapping).

Sensor integration often requires specialized crates that interact with specific sensor hardware.

## Sensor Data Processing {#sensor-data-processing .skip}

[`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}: N-dimensional tensor arithmetic

## Computer Vision and Geometry

Use [`opencv`][c~opencv~docs] for bindings to native OpenCV algorithms, or [`image`][c~image~docs] and [`imageproc`][c~imageproc~docs] for image buffers and processing in Rust. [`kornia-rs`][kornia-rs~repo] provides computer-vision building blocks; [`sophus-rs`][sophus-rs~repo] covers geometry for vision and robotics. See [Computer Vision](../computer-vision/index.md) for algorithms and native-library prerequisites.

## Find Neighbors in a Point Cloud

[`nalgebra`][c~nalgebra~docs] represents vectors and transforms, while [`parry3d`][c~parry3d~docs] provides collision and geometric queries. Use [`kiddo`][c~kiddo~docs] to index a low-dimensional point cloud for nearest-neighbor lookup. An index identifies a measurement; keep its position associated with the original sensor record.

This example queries three generated points and checks both the selected index and squared Euclidean distance. It needs no camera, ROS installation, or graphics device:

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/perception_and_sensors/nearest_neighbor.rs:example}}
```

Run `cargo run -p science_robotics --example perception_and_sensors` from `later/crates`.

## Capture Camera Frames

[`nokhwa`][c~nokhwa~docs] exposes camera capture through platform backends. Enumerate available cameras, choose a supported format and frame rate, and handle disconnection and acquisition failures. A capture buffer still needs color conversion and processing appropriate to its pixel format; a camera test requires real hardware and operating-system permissions.

## Fuse Measurements and Estimate State

Timestamp observations and transform them into a consistent coordinate frame before combining them. Filtering and state estimation depend on the motion model, sensor uncertainty, and sampling interval. See [Control Systems](control_systems.md) for feedback and estimator context, and [Hardware Integration](hardware_integration.md) for reading devices.

## Handling PointCloud2 Messages in ROS {#handling-pointcloud2 .skip}

`ros_pointcloud2`

## Simulate Sensor Interactions {#simulate-sensor-interactions .skip}

[`rapier`][rapier.rs~website]↗{{hi:rapier}}: Physics engine

See [[simulation_visualization | Simulation Visualization]].

## Related Topics {#related-topics .skip}

- [[computer-vision | Computer Vision]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
