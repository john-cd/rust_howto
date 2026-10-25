<!-- TODO Reviewed ??-??-2026 -->

# Trigonometry

{{#include trigonometry.incl.md}}

## Calculate the Side Length of a Triangle {#calculating-the-side-length-of-a-triangle}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]{{hi:Trigonometry}}

The following example calculates the hypotenuse of a right triangle with an acute angle of 1 radian and an opposite side of length 80.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/trigonometry/side_length.rs:example}}
```

## Verify that `tan` is Equal to `sin` Divided by `cos` {#verifying-tan-is-equal-to-sin-divided-by-cos}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]

For an angle of 6 radians, this example compares [`tan(x)`][c~f64::tan~docs]↗{{hi:f64::tan}} with `sin(x) / cos(x)`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/trigonometry/tan_sin_cos.rs:example}}
```

## Calculate the Distance Between Two Points on Earth {#distance-between-two-points-on-earth}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Rust provides mathematical [floating-point methods][primitive~f64]↗ for operations such as trigonometric functions and square roots. Trigonometric methods take angles in radians; use [`to_radians`][primitive~f64::to_radians]↗ to convert degree values.

The following example uses the [Haversine formula][wikipedia~haversine-formula]↗ to estimate the great-circle distance between Paris and London. It converts their latitude and longitude coordinates from degrees to radians, computes the central angle, and multiplies it by the mean Earth radius used in the code (6371 kilometers).

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[complex_numbers | Complex Numbers]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
