# Aerospace Protocols

[![cat~aerospace::protocols][cat~aerospace::protocols~badge]][cat~aerospace::protocols]{{hi:Aerospace protocols}}

{{#include aerospace_protocols.incl.md}}

## Common Aerospace Protocols

### MAVLink (Micro Air Vehicle Link)

MAVLink is a very widely-used protocol for communication between drones (and other unmanned vehicles) and ground control stations. It's message-based and defines a standard set of messages for various purposes (e.g., flight control, telemetry, sensor data).

[`mavlink`][c~mavlink~docs]↗{{hi:mavlink}} is the primary crate for MAVLink. It provides encoding and decoding of MAVLink messages.

### CAN (Controller Area Network)

CAN is a robust and reliable serial communication protocol often used in aerospace for communication between various electronic components (e.g., sensors, actuators, controllers).

Consider using:

- [`socketcan`][c~socketcan~docs]↗{{hi:socketcan}} for interacting with CAN buses on Linux systems (often used with USB-CAN adapters).
- [`can-rs`][c~can~docs]↗{{hi:can-rs}}, which is a general CAN crate.

For embedded systems, you might use crates like [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and implement CAN communication using the microcontroller's peripherals.

### ARINC 429

ARINC 429 is a widely used standard for data transfer in commercial aircraft avionics. It's a unidirectional, broadcast-oriented protocol.

Direct ARINC 429 support in Rust is limited. You'll likely have to use FFI (Foreign Function Interface) to interact with existing C/C++ libraries or implement the protocol yourself based on the specification.

See also [[development-tools_ffi | FFI]].

### MIL-STD-1553

MIL-STD-1553 is a military standard for a serial data bus. It's highly reliable and fault-tolerant, often used in critical aerospace applications.

Similar to ARINC 429, direct MIL-STD-1553 support in Rust is limited. FFI or custom implementations are the most likely approaches.

### SpaceWire

SpaceWire is a high-speed serial data bus commonly used in spacecraft and other space applications.

SpaceWire support in Rust is very limited. FFI or custom implementations are the most likely approaches.

### CCSDS (Consultative Committee for Space Data Systems)

CCSDS is a set of standards for space data systems, including packet telemetry and telecommand protocols.

FFI or custom implementations are likely needed.

## See Also {#see-also .skip}

- [[aerospace | Aerospace]].
- [[aerospace_drones | Drones]].
- [[aerospace_simulation | Aerospace Simulation]].
- [[aerospace_space-protocols | Space Protocols]].
- [[aerospace_unmanned-aerial-vehicles | Unmanned Aerial Vehicles]].
- [[hardware-support | Hardware Support]].
- [[simulation | Simulation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aerospace protocols: write](https://github.com/john-cd/rust_howto/issues/196)
</div>
