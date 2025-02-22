# Aerospace

[![cat-aerospace][cat-aerospace-badge]][cat-aerospace]{{hi:Aerospace}}

## Aerospace

{{#include aerospace.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P3](https://github.com/john-cd/rust_howto/issues/190)

- Data: Parse Telemetry: demonstrate parsing common aerospace telemetry formats (e.g., CSV, binary formats, custom protocols) using crates like serde, nom, or byteorder

- Crates like [`geo`][c-geo]⮳{{hi:geo}}, `georust`, or [`proj`][c-proj]⮳{{hi:proj}} for tasks such as:
  - Calculating distances and bearings between locations. [`geo`][c-geo]⮳{{hi:geo}}
  - Converting between coordinate systems (e.g., latitude/longitude to UTM). [`proj`][c-proj]⮳{{hi:proj}}
  - Working with geometric shapes (points, lines, polygons).

- Math:
  - Kalman Filter Basics: Simple implementation or usage.
  - Matrix Operations: nalgebra example (rotations).
  - Numerical Integration: Trapezoidal rule example.
  - Quaternion Usage: Rotation representation.

- Link to Hardware section

- Concurrency/Safety:
  - Link to Safe Concurrency: Message passing example.
  - no_std Memory Safety: Example with unsafe blocks.

- Link to Testing:
  - Unit Test
  - Property Testing: proptest for a specific property.

- Aerospace Examples:
  - Orbital Parameters: Calculation from vectors.
  - Basic Attitude Control: Simplified algorithm.

- Link to formal verification and how tools like kani might be used to prove properties of Rust code.

</div>
