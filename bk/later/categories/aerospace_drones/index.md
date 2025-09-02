# Aerospace - Drones

[![cat~aerospace::drones][cat~aerospace::drones~badge]][cat~aerospace::drones]{{hi:Aerospace::Drones}}

{{#include drones.incl.md}}

## Core Flight Control & Communication

- Flight Control Algorithms: PID controllers, sensor fusion (IMU, GPS, barometer), state estimation, path planning, trajectory generation.
- Communication Protocols: MAVLink (Micro Air Vehicle Link), other custom protocols. Serial communication (UART, I2C, SPI).
- Real-time Systems: Handling sensor data and control loops within strict timing constraints.
- Embedded Systems: Interfacing with hardware, memory management, `no-std`{{hi:no-std}} programming (if applicable).

Consider using:

- [`mavlink`][c~mavlink~docs]↗{{hi:mavlink}} for MAVLink encoding/decoding.
- [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} for hardware abstraction (if working with embedded systems).
- [`pid`][c~pid~docs]↗{{hi:pid}} for PID controller implementation.

## Sensor Integration

- IMU (Inertial Measurement Unit) Data Processing: Accelerometer, gyroscope, magnetometer data. Sensor fusion for orientation estimation.
- GPS Integration: Receiving and processing GPS data for localization and navigation.
- Barometer/Altimeter: Altitude measurement.
- Other Sensors: Cameras, LiDAR, ultrasonic sensors.

Consider using:

- [`i2cdev`][c~i2cdev~docs]↗{{hi:i2cdev}}, [`spidev`][c~spidev~docs]↗{{hi:spidev}}, [`serialport`][c~serialport~docs]↗{{hi:serialport}} for low-level hardware communication.
- [`gpsd-client`][c~gpsd_client~docs]↗{{hi:gpsd-client}} for interacting with the gpsd daemon (if using a GPS receiver that works with gpsd).

Sensor-specific crates are common: Often, you'll use crates provided by the sensor manufacturer or community-developed drivers.

## Computer Vision & Image Processing

- Object Detection: Detecting objects in images/video (e.g., obstacles, landing pads).
- Image Processing: Image filtering, feature extraction.
- SLAM (Simultaneous Localization and Mapping): Building a map of the environment and localizing the drone within it.

Consider using:

- [`opencv`][c~opencv~docs]↗{{hi:opencv}}: Bindings for the OpenCV library (powerful for computer vision).
- [`image`][c~image~docs]↗{{hi:image}} for image loading and manipulation.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} for efficient array operations (often used in image processing).

## See Also {#see-also .skip}

- [[additional_numeric_types | Additional Numeric Types]].
- [[algorithms | Algorithms]].
- [[computer-vision | Computer Vision]].
- [[hardware-support | Hardware Support]].
- [[images | Images]].
- [[linear_algebra | Linear Algebra]].
- [[mathematics | Mathematics]].
- [[multimedia_images | Multimedia Images]].
- [[linear_algebra | Linear Algebra]].

Consider using:

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} for linear algebra and control algorithms.
- [`num-traits`][c~num-traits~docs]↗{{hi:num-traits}} & [`num-complex`][c~num-complex~docs]↗{{hi:num-complex}} for numerical and complex number operations.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/193)
</div>
