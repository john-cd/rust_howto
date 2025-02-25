# Aerospace - Drones

[![cat-aerospace::drones][cat-aerospace::drones-badge]][cat-aerospace::drones]{{hi:Aerospace::Drones}}

## Drones

{{#include drones.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P3](https://github.com/john-cd/rust_howto/issues/193)

## Core Flight Control & Communication

- Flight Control Algorithms: PID controllers, sensor fusion (IMU, GPS, barometer), state estimation, path planning, trajectory generation.
- Communication Protocols: MAVLink (Micro Air Vehicle Link), other custom protocols. Serial communication (UART, I2C, SPI).
- Real-time Systems: Handling sensor data and control loops within strict timing constraints.
- Embedded Systems: Interfacing with hardware, memory management, `no-std` programming (if applicable).

[[algorithms | Algorithms]]

### Rust Crates

- [`mavlink`][c-mavlink]⮳{{hi:mavlink}} for MAVLink encoding/decoding.

[[hardware-support | Hardware Support]]
[`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}}: For hardware abstraction (if working with embedded systems).
`pid` for PID controller implementation.

[[mathematics | Mathematics]]
[[linear_algebra | Linear Algebra]]
[`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}: For linear algebra (essential for control algorithms).

[`num-traits`][c-num_traits]⮳{{hi:num-traits}} & [`num-complex`][c-num_complex]⮳{{hi:num-complex}} for numerical and complex number operations.

[[additional_numeric_types | Additional Numeric Types]]

## Sensor Integration

- IMU (Inertial Measurement Unit) Data Processing: Accelerometer, gyroscope, magnetometer data. Sensor fusion for orientation estimation.
- GPS Integration: Receiving and processing GPS data for localization and navigation.
- Barometer/Altimeter: Altitude measurement.
- Other Sensors: Cameras, LiDAR, ultrasonic sensors.

### Rust Crates

- (Sensor-specific crates are common): Often, you'll use crates provided by the sensor manufacturer or community-developed drivers.
- `i2cdev`, [`spidev`][c-spidev]⮳{{hi:spidev}}, [`serialport`][c-serialport]⮳{{hi:serialport}} for low-level hardware communication.
- [`gpsd-client`][c-gpsd_client]⮳{{hi:gpsd-client}}: For interacting with the gpsd daemon (if using a GPS receiver that works with gpsd).

## Computer Vision & Image Processing

- Object Detection: Detecting objects in images/video (e.g., obstacles, landing pads).
- Image Processing: Image filtering, feature extraction.
- SLAM (Simultaneous Localization and Mapping): Building a map of the environment and localizing the drone within it.

### Rust Crates

`opencv`: Bindings for the OpenCV library (powerful for computer vision).
`image` for image loading and manipulation.
`ndarray` for efficient array operations (often used in image processing).

[[images | Images]]
[[multimedia_images | Multimedia Images]]
[[computer-vision | Computer Vision]]
[[linear_algebra | Linear Algebra]]

## See also

[[algorithms | Algorithms]]

</div>
