# Aerospace - Unmanned Aerial Vehicles

[![cat~aerospace::unmanned-aerial-vehicles][cat~aerospace::unmanned-aerial-vehicles~badge]][cat~aerospace::unmanned-aerial-vehicles]{{hi:Aerospace::Unmanned aerial vehicles}}

{{#include uavs.incl.md}}

## Key UAV Communication & Control Protocols

MAVLink (Micro Air Vehicle Link) is the dominant protocol for communication between drones and ground control stations (GCS). It's message-based, covering everything from basic flight control to sensor data and telemetry.

Consider using:

- [`mavlink`][c~mavlink~docs]↗{{hi:mavlink}} crate.
- [`DroneCAN`][dronecan~website]↗{{hi:DroneCAN}}: A CAN-based protocol specifically designed for UAVs and robotics. It aims for reliability and real-time performance.
- [`can-rs`][c~can~docs]↗{{hi:can-rs}} (general CAN crate) and `socketcan` (Linux-specific) are relevant. However, DroneCAN-specific higher-level crates might require custom work or FFI with existing C/C++ implementations.
- [`sbus`][c~sbus~docs]↗{{hi:sbus}}/`s.bus`{{hi:s.bus}}: A serial communication protocol often used for radio control (RC) receivers and servos.
- Serial port crates ([`serialport`][c~serialport~docs]↗{{hi:serialport}}) and implement the sbus/s.bus decoding logic yourself.
- Spektrum DSM/DSMX: Another common RC protocol.
- FrSky SmartPort/FPort: A telemetry protocol used by FrSky RC systems.

## See Also {#see-also .skip}

### Parsing Binary Data

Consider using crates like [`nom`][c~nom~docs]↗{{hi:nom}} or [`binascii`][c~binascii~docs]↗{{hi:binascii}}.

See [[parsing | Parsing]] and [[_binary_encoders |  Binary Encoders]].

### Data Structures and Serialization

Define Rust structs to represent the message formats of the protocols. [`serde`][c~serde~docs]↗{{hi:serde}} can be used for serialization/deserialization.

See [[data-structures | Data Structures]] and [[encoding | Encoding]].

### FFI (Foreign Function Interface)

If existing C/C++ libraries are available, FFI can be a viable option, but it adds complexity.

See:

- [[development-tools_ffi | FFI]].
- [[external-ffi-bindings | External FFI Bindings]].
- [[external_ffi_bindings | External FFI Bindings]].
- [[generate_ffi_bindings | Generate FFI Bindings]].

### `#![no_std]` (for Embedded systems)

If your target is a flight controller running on an embedded system, [`#![no_std]`][book~rust-reference~no_std]↗{{hi:no_std}} is essential.

See [[no_std | No Std]] and [[no_alloc | No Alloc]].

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aerospace_unmanned-aerial-vehicles: write](https://github.com/john-cd/rust_howto/issues/205)
</div>
