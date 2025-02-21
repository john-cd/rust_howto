# Aerospace Protocols

[![cat-aerospace::protocols][cat-aerospace::protocols-badge]][cat-aerospace::protocols]{{hi:Aerospace protocols}}

## Aerospace Protocols

{{#include aerospace_protocols.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aerospace protocols: write (P3)](https://github.com/john-cd/rust_howto/issues/196)

Common Aerospace Protocols:

 MAVLink (Micro Air Vehicle Link): A very widely used protocol for communication between drones (and other unmanned vehicles) and ground control stations. It's message-based and defines a standard set of messages for various purposes (e.g., flight control, telemetry, sensor data).

Rust Crates: mavlink is the primary crate for MAVLink in Rust. It provides encoding and decoding of MAVLink messages.
 CAN (Controller Area Network): A robust and reliable serial communication protocol often used in aerospace for communication between various electronic components (e.g., sensors, actuators, controllers).

Rust Crates:
socketcan: For interacting with CAN buses on Linux systems (often used with USB-CAN adapters).
can-rs: A more general CAN crate, potentially supporting other platforms.
For embedded systems, you might use crates like embedded-hal and implement CAN communication using the microcontroller's peripherals.
 ARINC 429: A widely used standard for data transfer in commercial aircraft avionics. It's a unidirectional, broadcast-oriented protocol.

Rust Support: Direct ARINC 429 support in Rust is limited. You'll likely have to use FFI (Foreign Function Interface) to interact with existing C/C++ libraries or implement the protocol yourself based on the specification.
 MIL-STD-1553: A military standard for a serial data bus. It's highly reliable and fault-tolerant, often used in critical aerospace applications.

Rust Support: Similar to ARINC 429, direct MIL-STD-1553 support in Rust is limited. FFI or custom implementations are the most likely approaches.
 SpaceWire: A high-speed serial data bus commonly used in spacecraft and other space applications.

Rust Support: SpaceWire support in Rust is very limited. FFI or custom implementations are the most likely approaches.
 CCSDS (Consultative Committee for Space Data Systems): A set of standards for space data systems, including packet telemetry and telecommand protocols.

Rust Support: There might be some crates or libraries for specific CCSDS protocols (e.g., packet telemetry), but comprehensive support is unlikely. FFI or custom implementations are likely needed.

</div>
