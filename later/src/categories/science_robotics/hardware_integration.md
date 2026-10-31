<!-- TODO Reviewed ??-??-2026 -->

# Hardware Integration

{{#include hardware_integration.incl.md}}

Robotics systems interact with physical environments through sensors, actuators, serial links, fieldbus networks, and embedded microcontrollers. Rust provides both low-level `no_std` abstractions for microcontroller firmware and Linux-based userspace drivers for single-board computers (SBCs) like the Raspberry Pi or NVIDIA Jetson.

For microcontroller firmware setup and bare-metal environments, see [[embedded | Embedded Systems]] and [[hardware-support | Hardware Abstraction]].

## Serial Communication {#serial-communication}

{{#badges serialport}}

Serial ports (UART, RS-232, RS-485) are standard interfaces for communicating with motor controllers, IMUs, GPS receivers, and power distribution boards.

- [`serialport`][c~serialport~docs]↗{{hi:serialport}} provides cross-platform serial port access (configuring baud rates, data bits, parity, and stop bits) on Linux, macOS, and Windows. Check its current maintenance status and platform-specific device permissions before adopting it.

## Linux SBC Peripherals {#linux-sbc-peripherals}

{{#badges linux-embedded-hal}}

Robots running Linux on single-board computers interface with onboard GPIO pins, I2C buses, and SPI devices using Linux kernel driver interfaces (`sysfs`, `gpio-cdev`, `/dev/i2c-*`, `/dev/spidev*`).

- [`linux-embedded-hal`][c~linux-embedded-hal~docs]↗{{hi:linux-embedded-hal}} adapts Linux peripheral device nodes into standard [`embedded-hal`][c~embedded-hal~docs]↗ traits. Drivers can share code across compatible implementations, but permissions, device configuration, timing, and hardware behavior still need platform-specific checks.
- [`rppal`][c~rppal~docs]↗{{hi:rppal}} provides Raspberry Pi-specific GPIO, I2C, SPI, PWM, and UART peripherals. Its support targets Raspberry Pi boards; do not assume the same peripherals are available on other Linux SBCs.

## Portable Device Drivers with `embedded-hal` 1.0 {#embedded-hal}

{{#badges embedded-hal embedded="Embedded development" no-std="No standard library"}}

The [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} crate defines hardware abstraction traits for I2C, SPI, digital I/O, PWM, and delay functions. A driver generic over these traits can be reused with compatible bare-metal or Linux implementations and host-side mocks. The traits do not guarantee identical hardware, timing, or configuration behavior; test on each target.

The following example implements a sensor driver generic over any `embedded_hal::i2c::I2c` implementation. Its address, registers, device ID, and signed temperature scale describe a fictional teaching sensor; replace them with values from the actual device datasheet. Host-side tests use [`embedded-hal-mock`][c~embedded-hal-mock~docs] to assert the exact address, register bytes, transaction order, negative-temperature decoding, and error propagation without physical hardware.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/hardware_integration/i2c_driver.rs:example}}
```

## CAN Bus Communication {#can-bus}

{{#badges socketcan}}

Controller Area Network (CAN) bus is a real-time, fault-tolerant differential serial bus standard widely used in robotics, mobile manipulators, and automotive systems for high-speed motor controller and sensor communication.

- [`socketcan`][c~socketcan~docs]↗{{hi:socketcan}} provides an interface to Linux SocketCAN subsystem, allowing applications to open raw CAN sockets (`can0`), send frames, filter incoming CAN IDs, and interface with CANopen or J1939 higher-layer protocols.

## HMI and PLC Systems {#hmi-and-plc-systems}

Human-Machine Interface (HMI) dashboards and Programmable Logic Controller (PLC) runtimes enable control room operation and real-time machine execution.

- [`ehmi`][ehmi~repo]↗ provides human-machine interface components and widgets for [`egui`][c~egui~docs]↗.
- [`RoboPLC`][roboplc~repo]↗ is a Rust framework and toolkit for programmable logic controllers and real-time hard/soft control services.

## Robot Models and Logs {#robot-models-and-logs}

Robots require structural kinematic models (URDF) and standardized logging formats for telemetry playback.

- [`urdf-rs`][c~urdf-rs~crates.io]↗{{hi:urdf-rs}} loads Unified Robot Description Format (URDF) XML files describing robot links, joint limits, and visual geometries.
- [`mcap`][c~mcap~crates.io]↗{{hi:mcap}} reads and writes MCAP log container files for storing heterogeneous robotics data streams.
- [`assimp-rs`][c~assimp~docs]↗{{hi:assimp-rs}} provides Rust bindings to the Assimp library for importing 3D mesh assets.

## Device Drivers {#device-drivers}

Sensor and actuator drivers wrapper crates:

- [`freenect-rs`][c~freenect-rs~docs]↗{{hi:freenect-rs}} - Freenect library bindings for Kinect depth sensors.
- [`hls_lfcd_lds_rs`][c~hls_lfcd_lds_rs~docs]↗{{hi:hls_lfcd_lds_rs}} - ROBOTIS HLDS HLS-LFCD-LDS 2D laser scanner SDK.
- [`dynpick-force-torque-sensor`][c~dynpick-force-torque-sensor~docs]↗{{hi:dynpick-force-torque-sensor}} - Wacoh-tech 6-axis force-torque sensor driver.
- [`leptrino-force-torque-sensor`][c~leptrino-force-torque-sensor~docs]↗{{hi:leptrino-force-torque-sensor}} - Leptrino 6-axis force sensor driver.
- [`realsense-rust`][c~realsense-rust~docs]↗{{hi:realsense-rust}} - Intel RealSense depth camera bindings for Rust.

## ROS 2 Hardware Integration {#ros2-hardware-integration}

In ROS 2 architectures, hardware integration nodes interface sensor and motor drivers with ROS 2 topics and actions. Libraries such as [`r2r`][c~r2r~docs]↗ or [`safe_drive`][c~safe_drive~docs]↗ connect hardware loops to ROS 2 publish/subscribe channels. See [Robot Operating Systems](robot_operating_systems.md) for detailed ROS client usage.

## Related Topics {#related-topics .skip}

- [[embedded | Embedded Systems]].
- [[hardware-support | Hardware Abstraction]].
- [Perception and Sensors](perception_and_sensors.md).
- [Control Systems](control_systems.md).
- [Robot Operating Systems](robot_operating_systems.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
