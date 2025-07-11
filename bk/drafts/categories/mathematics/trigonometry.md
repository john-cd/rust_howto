# Trigonometry

{{#include trigonometry.incl.md}}

## Calculate the Side Length of a Triangle {#calculating-the-side-length-of-a-triangle}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Calculates the length of the hypotenuse of a right-angle triangle with an angle of 2 radians and opposite side length of 80.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/trigonometry/side_length.rs:example}}
```

## Verify that `tan` is Equal to `sin` Divided by `cos` {#verifying-tan-is-equal-to-sin-divided-by-cos}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Verifies `tan(x)` is equal to `sin(x)/cos(x)` for x = 6.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/trigonometry/tan_sin_cos.rs:example}}
```

## Calculate the Distance Between two Points on Earth {#distance-between-two-points-on-earth}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

By default, Rust provides mathematical [float methods][primitive~f64]⮳ such as trigonometric functions, square root, conversion [functions][p~functions] between radians and degrees, and so forth.

The following example computes the distance in kilometers between two points on the Earth with the [Haversine][wikipedia~haversine-formula]⮳ formula. Points are expressed as pairs of latitude and longitude in degrees. Then, [`to_radians`][primitive~f64::to_radians]{{hi:to_radians}}⮳ converts them in radians. [`sin`][primitive~f64::sin]{{hi:sin}}⮳ [`cos`][primitive~f64::cos]{{hi:cos}}⮳ [`powi`][primitive~f64::powi]{{hi:powi}}⮳ and [`sqrt`][primitive~f64::sqrt]{{hi:sqrt}}⮳ compute the central angle. Finally, it's possible to calculate the distance.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[final review](https://github.com/john-cd/rust_howto/issues/938)
</div>
