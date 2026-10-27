<!-- TODO Reviewed ??-??-2026 -->

# Aerospace - Drones

[![cat~aerospace::drones][cat~aerospace::drones~badge]][cat~aerospace::drones]{{hi:Aerospace::Drones}}

Rust's memory safety, zero-cost abstractions, and fearless concurrency make it a highly compelling language for aerospace and drone applications. Drones (Unmanned Aerial Vehicles) require real-time processing, extreme reliability, and efficient resource management. This chapter covers the ecosystem of crates suitable for writing flight control software, handling sensor integration, implementing communication protocols, and processing data for computer vision.

{{#include drones.incl.md}}

## Core Flight Control & Communication

Flight-control algorithms include PID control, sensor fusion for IMU, GPS, and barometer data, state estimation, path planning, and trajectory generation. Control loops must meet strict real-time timing while interfacing with hardware and managing constrained memory; use `no_std` when the controller requires it. Use [`mavlink`][c~mavlink~docs] for ground-station messages and [`embedded-hal`][c~embedded-hal~docs] for portable I2C/SPI abstractions; choose UART or custom-protocol support for the target and see [[aerospace_protocols | Aerospace Protocols]] for protocol choices.

## Sensor Integration

Sensor integration may include IMU accelerometer, gyroscope, and magnetometer processing, GPS for localization and navigation, barometric altitude, and cameras, lidar, or ultrasonic rangefinders. Prefer target-compatible [`embedded-hal`][c~embedded-hal~docs] drivers where available; vendor or community drivers and calibration depend on the hardware. The [`ahrs`][c~ahrs~docs] crate provides sensor-fusion algorithms for orientation estimates.

## Computer Vision & Image Processing

Vision tasks include obstacle and landing-pad detection, image filtering and feature extraction, and simultaneous localization and mapping (SLAM). Use [`opencv`][c~opencv~docs] for computer-vision algorithms, [`image`][c~image~docs] for image loading and manipulation, and [`ndarray`][c~ndarray~docs] for n-dimensional arrays; see [[computer-vision | Computer Vision]], [[images | Images]], and [[multimedia_images | Multimedia Images]] for broader guidance.

## Related Topics

The [[aerospace_unmanned-aerial-vehicles | Unmanned Aerial Vehicles]] page covers radio-control and vehicle protocol choices; this page focuses on combining flight-control, sensor, and vision software.

- [[additional_numeric_types | Additional Numeric Types]].
- [[algorithms | Algorithms]].
- [[computer-vision | Computer Vision]].
- [[hardware-support | Hardware Support]].
- [[images | Images]].
- [[mathematics | Mathematics]].
- [[multimedia_images | Multimedia Images]].
- [[linear_algebra | Linear Algebra]].

Consider using:

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} for linear algebra and control algorithms.
- [`num-traits`][c~num-traits~docs]↗{{hi:num-traits}} & [`num-complex`][c~num-complex~docs]↗{{hi:num-complex}} for numerical and complex number operations.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
