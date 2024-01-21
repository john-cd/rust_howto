# Trigonometry

## Calculating the side length of a triangle

[![std-badge]][std] [![cat-science-badge]][cat-science]

Calculates the length of the hypotenuse of a right-angle triangle with an angle of 2 radians and opposite side length of 80.

```rust,editable
{{#include ../../deps/examples/side-length.rs}}
```

## Verifying tan is equal to sin divided by cos

[![std-badge]][std] [![cat-science-badge]][cat-science]

Verifies tan(x) is equal to sin(x)/cos(x) for x = 6.

```rust,editable
{{#include ../../deps/examples/tan-sin-cos.rs}}
```

## Distance between two points on the Earth

[![std-badge]][std]

By default, Rust provides mathematical [float methods] such as
trigonometric functions, square root, conversion functions between
radians and degrees, and so forth.

The following example computes the distance in kilometers between two
points on the Earth with the [Haversine formula]. Points are expressed
as pairs of latitude and longitude in degrees. Then, `[to_radians]`
converts them in radian. [`sin`][sin] `[cos]` [`powi`][powi] and `[sqrt]`
compute the central angle. Finally, it's possible to calculate the
distance.

```rust,editable
{{#include ../../deps/examples/latitude-longitude.rs}}
```

[float methods]: https://doc.rust-lang.org/std/primitive.f64.html#methods
[to_radians]: https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians
[sin]: https://doc.rust-lang.org/std/primitive.f64.html#method.sin
[cos]: https://doc.rust-lang.org/std/primitive.f64.html#method.cos
[powi]: https://doc.rust-lang.org/std/primitive.f64.html#method.powi
[sqrt]: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
[Haversine formula]: https://en.wikipedia.org/wiki/Haversine_formula
{{#include ../refs/link-refs.md}}
