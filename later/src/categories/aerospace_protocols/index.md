<!-- TODO Reviewed ??-??-2026 -->

# Aerospace Protocols

[![cat~aerospace::protocols][cat~aerospace::protocols~badge]][cat~aerospace::protocols]{{hi:Aerospace protocols}}

{{#include aerospace_protocols.incl.md}}

This guide surveys protocol choices for aircraft, uncrewed vehicles, and spacecraft. Use the linked protocol or device documentation for implementation details; this page focuses on crate support and hardware boundaries.

## Common Aerospace Protocols

### MAVLink (Micro Air Vehicle Link)

MAVLink is a widely used protocol for communication between drones and other unmanned vehicles and ground control stations. It is message-based and defines standard messages for flight control, telemetry, and sensor data.

The maintained [`mavlink`][c~mavlink~docs] crate provides message encoding and decoding. The selected dialect and autopilot determine which messages and commands are supported.

### CAN (Controller Area Network)

CAN is a robust serial communication protocol used between electronic components such as sensors, actuators, and controllers.

For Linux systems, [`socketcan`][c~socketcan~docs] exposes SocketCAN interfaces and frames, often through USB-CAN adapters. Embedded targets can use the [`embedded-can`][c~embedded-can~docs] traits with a compatible hardware driver; [`embedded-hal`][c~embedded-hal~docs] provides the broader hardware abstraction layer used by many such drivers. These crates cover different layers, so check which one the device and driver need.

### ARINC 429

ARINC 429 is a widely used standard for data transfer in commercial aircraft avionics. It is a unidirectional, broadcast-oriented protocol.

No maintained ARINC 429 crate is available to recommend; the small [`arinc429`][c~arinc429~docs] implementation is a candidate for evaluation. Check its release activity, test coverage, and conformance against the licensed standard and target hardware; existing C or C++ libraries may require [[development-tools_ffi | FFI]].

### MIL-STD-1553

MIL-STD-1553 is a serial data bus used in reliable, fault-tolerant military and aerospace systems.

The maintained [`mil_std_1553b`][c~mil_std_1553b~docs] crate parses and constructs MIL-STD-1553B messages. Mission-ready support still depends on compatible bus-controller or remote-terminal hardware and a validated driver.

### SpaceWire

SpaceWire is a high-speed serial data bus commonly used in spacecraft and other space applications.

I found no maintained SpaceWire crate to recommend. A hardware vendor's SDK or a project-specific implementation may be needed; validate link initialization, routing, and error handling against the device and applicable standard. FFI may be required when integrating an existing vendor library.

### CCSDS

For CCSDS packet and PUS guidance, see the [Space Protocols][p~space-protocols] category.

## Related Topics

- [[aerospace | Aerospace]].
- [[aerospace_drones | Drones]].
- [[aerospace_simulation | Aerospace Simulation]].
- [[aerospace_space-protocols | Space Protocols]].
- [[aerospace_unmanned-aerial-vehicles | Unmanned Aerial Vehicles]].
- [[hardware-support | Hardware Support]].
- [[simulation | Simulation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
