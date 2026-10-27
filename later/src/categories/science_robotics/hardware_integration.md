<!-- TODO Reviewed ??-??-2026 -->

# Hardware Integration

{{#include hardware_integration.incl.md}}

Low-level hardware access and [embedded][p~embedded] systems.

[`embedded-hal`][c~embedded-hal~docs] defines a standard trait interface for embedded hardware. [`linux-embedded-hal`][c~linux-embedded-hal~docs] provides implementations for Linux systems; see also [[embedded | Embedded Systems]] and [[hardware-support | Hardware Abstraction]].

## Device Drivers {#device-drivers .skip}

Robotics-related sensor/motor drivers:

- [`freenect-rs`][c~freenect-rs~docs]{{hi:freenect-rs}} - Freenect wrapper for rust.
- Slamtec RPLIDAR public SDK for Rust.
- [`hls_lfcd_lds_rs`][c~hls_lfcd_lds_rs~docs]{{hi:hls_lfcd_lds_rs}} - ROBOTIS HLDS HLS-LFCD-LDS SDK for RUST.
- [`dynpick-force-torque-sensor`][c~dynpick-force-torque-sensor~docs]{{hi:dynpick-force-torque-sensor}} - Wacoh-tech 6-axis force sensor (Dynpick)↗ driver.
- [`leptrino-force-torque-sensor`][c~leptrino-force-torque-sensor~docs]{{hi:leptrino-force-torque-sensor}} - Leptrino 6-axis force sensor driver.
- [`realsense-rust`][c~realsense-rust~docs]{{hi:realsense-rust}} - RealSense Bindings for Rust.

## HMI and PLC Systems {#hmi-and-plc-systems}

[`ehmi`][ehmi~repo] provides human-machine interface components for [`egui`][c~egui~docs]. [`RoboPLC`][roboplc~repo] is a Rust framework and toolkit for programmable logic controllers and real-time services.

## Robot Models and Logs {#robot-models-and-logs}

- [`assimp-rs`][c~assimp~docs]{{hi:assimp-rs}} provides Rust bindings to the Assimp asset importer.
- [`mcap`][c~mcap~crates.io]{{hi:mcap}} reads and writes MCAP log files.
- [`urdf-rs`][c~urdf-rs~crates.io]{{hi:urdf-rs}} loads URDF robot descriptions. See also [[parser-implementations | Parser Implementations]].

## Related Topics {#related-topics .skip}

- Embedded hardware interfaces.
- Sensor and actuator drivers.
- Robotics control loops.
- Low-level systems integration.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [write](https://github.com/john-cd/rust_howto/issues/1211)
</div>
