<!-- TODO Reviewed ??-??-2026 -->

# Perception and Sensors

{{#include perception_and_sensors.incl.md}}

Sensor fusion / integration, SLAM (Simultaneous Localization and Mapping).

Sensor integration often requires specialized crates that interact with specific sensor hardware.

## Sensor Data Processing {#sensor-data-processing .skip}

[`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}: N-dimensional tensor arithmetic

## Computer Vision and Geometry

[`kornia-rs`][kornia-rs~repo] is a low-level computer-vision library written in Rust. [`sophus-rs`][sophus-rs~repo] provides 2D and 3D geometry for computer vision and robotics. See also [[computer-vision | Computer Vision]].

## Handling PointCloud2 Messages in ROS {#handling-pointcloud2 .skip}

`ros_pointcloud2`

## Simulate Sensor Interactions {#simulate-sensor-interactions .skip}

[`rapier`][rapier.rs~website]↗{{hi:rapier}}: Physics engine

See [[simulation_visualization | Simulation Visualization]].

## Related Topics {#related-topics .skip}

- [[computer-vision | Computer Vision]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [write](https://github.com/john-cd/rust_howto/issues/1207)
</div>
