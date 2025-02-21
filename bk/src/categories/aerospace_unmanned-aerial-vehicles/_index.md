# Aerospace - Unmanned Aerial Vehicles

[![cat-aerospace::unmanned-aerial-vehicles][cat-aerospace::unmanned-aerial-vehicles-badge]][cat-aerospace::unmanned-aerial-vehicles]{{hi:Aerospace::Unmanned aerial vehicles}}

## Unmanned Aerial Vehicles {#uavs}

{{#include uavs.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aerospace_unmanned-aerial-vehicles: write (P3)](https://github.com/john-cd/rust_howto/issues/205)

Key UAV Communication & Control Protocols:

- MAVLink (Micro Air Vehicle Link): The dominant protocol for communication between drones and ground control stations (GCS). It's message-based, covering everything from basic flight control to sensor data and telemetry.
- Link to `mavlink` crate
- `DroneCAN`: A CAN-based protocol specifically designed for UAVs and robotics. It aims for reliability and real-time performance.
- `can-rs` (general CAN crate) and socketcan (Linux-specific) are relevant. However, DroneCAN-specific higher-level crates might require custom work or FFI with existing C/C++ implementations.
- `sbus`/`s.bus`: A serial communication protocol often used for radio control (RC) receivers and servos.
- serial port crates (`serialport`) and implement the sbus/s.bus decoding logic yourself.
- Spektrum DSM/DSMX: Another common RC protocol.
- FrSky SmartPort/FPort: A telemetry protocol used by FrSky RC systems.

- Link to Parsing Binary Data: Crates like nom or binascii
- Link to Data Structures and Serialization: Define Rust structs to represent the message formats of the protocols. `serde` can be used for serialization/deserialization.
- Link to FFI (Foreign Function Interface): If existing C/C++ libraries are available, FFI can be a viable option, but it adds complexity.
- Link to `no_std` (for embedded systems): If your target is a flight controller running on an embedded system, `no_std` is essential.

</div>
