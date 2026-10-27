<!-- TODO Reviewed ??-??-2026 -->

# Aerospace

[![cat~aerospace][cat~aerospace~badge]][cat~aerospace]{{hi:Aerospace}}

## Aerospace

{{#include aerospace.incl.md}}

Use these guides as starting points for aerospace software topics; check the linked standards and hardware documentation for mission-specific requirements.

## Drones

See the [Drones][p~aerospace-drones] guide.

This guide focuses on flight-control algorithms, sensor integration, and computer-vision software.

## Protocols

See the [Aerospace Protocols][p~aerospace-protocols] guide.

Compare support for aircraft and spacecraft protocols, including device and bus interfaces, in this guide.

## Simulation

See the [Aerospace Simulation][p~aerospace-simulation] guide.

This guide covers orbital propagation and numerical simulation techniques for aerospace problems.

## Space Protocols

See the [Space Protocols][p~aerospace-space-protocols] guide.

Find CCSDS packet and ECSS PUS examples, plus related on-board software support, in this guide.

## Unmanned Aerial Vehicles

See the [Unmanned Aerial Vehicles][p~aerospace-uavs] guide.

This guide focuses on vehicle communication and radio-control protocols; the Drones guide emphasizes control, sensors, and vision software.

## Related Topics

- **Code verification:** See [[code_verification | Code Verification]] and the [`kani`][c~kani~docs] tool for proving properties of Rust code.
- **Concurrency:** See [[concurrency | Concurrency]] for message passing and [`#![no_std]`][book~rust-reference~no_std] guidance for embedded memory constraints.
- **Telemetry data:** Parse CSV, binary, or custom formats with crates such as `serde`, `nom`, and `byteorder`.
- **Geoscience:** See [[science_geo | Geoscience]]; [`geo`][c~geo~docs] supports distances, bearings, and geometry, while [`proj`][c~proj~docs] converts coordinate systems such as latitude/longitude to UTM.
- **Hardware:** See [[hardware-support | Hardware Support]].
- **Mathematics:** Kalman filters, matrices and rotations with [`nalgebra`][c~nalgebra~docs], numerical integration, and quaternion representations.
- **Testing:** See [[testing | Testing]] for unit and property tests with [`proptest`][c~proptest~docs].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
