# Trigonometry

{{#include trigonometry.incl.md}}

## Calculating the side length of a triangle

[![std][std-badge]][std]  [![cat-science][cat-science-badge]][cat-science]

Calculates the length of the hypotenuse of a right-angle triangle with an angle of 2 radians and opposite side length of 80.

```rust,editable
{{#include ../../../deps/tests/side-length.rs}}
```

## Verifying tan is equal to sin divided by cos

[![std][std-badge]][std]  [![cat-science][cat-science-badge]][cat-science]

Verifies `tan(x)` is equal to `sin(x)/cos(x)` for x = 6.

```rust,editable
{{#include ../../../deps/tests/tan-sin-cos.rs}}
```

## Distance between two points on the Earth

[![std][std-badge]][std]

By default, Rust provides mathematical [float methods][f64]⮳ such as trigonometric functions, square root, conversion functions between radians and degrees, and so forth.

The following example computes the distance in kilometers between two points on the Earth with the [Haversine][wikipedia-haversine-formula]⮳ formula. Points are expressed as pairs of latitude and longitude in degrees. Then, [`to_radians`][f64::to_radians]⮳ converts them in radians. [`sin`][f64::sin]⮳  [`cos`][f64::cos]⮳  [`powi`][f64::powi]⮳ and [`sqrt`][f64::sqrt]⮳ compute the central angle. Finally, it's possible to calculate the distance.

```rust,editable
{{#include ../../../deps/tests/latitude-longitude.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
