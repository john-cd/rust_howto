# Aerospace

[![cat-aerospace][cat-aerospace-badge]][cat-aerospace]{{hi:Aerospace}}

## Aerospace

{{#include aerospace.incl.md}}

## See also

### Code Verification

See [[code_verification | Code Verification]].

- How tools like `kani` might be used to prove properties of Rust code

### Concurrency

- Safe [[concurrency | Concurrency]]: Message passing example.
- `no_std` Memory Safety: Example with unsafe blocks.

### Data

- Data: Parse Telemetry: demonstrate parsing common aerospace telemetry formats (e.g., CSV, binary formats, custom protocols) using crates like serde, nom, or byteorder

### Geoscience

See [[science_geo | Geoscience]].

Consider using crates like [`geo`][c-geo]⮳{{hi:geo}} or [`proj`][c-proj]⮳{{hi:proj}} for tasks such as:

- Calculating distances and bearings between locations. [`geo`][c-geo]⮳{{hi:geo}}
- Converting between coordinate systems (e.g., latitude/longitude to UTM). [`proj`][c-proj]⮳{{hi:proj}}
- Working with geometric shapes (points, lines, polygons).

### Hardware

See [[hardware-support | Hardware Support]].

### Mathematics

- Kalman Filters.
- Matrix Operations: `nalgebra` (rotations).
- Numerical Integration: Trapezoidal rule example.
- Quaternion: Rotation representation.

### Testing

- Unit Test.
- Property Testing: `proptest` for a specific property.

See [[testing | Testing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/190)

</div>
