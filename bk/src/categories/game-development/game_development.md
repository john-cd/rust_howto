<!-- TODO Reviewed ??-??-2026. -->

# Game Development

{{#include game_development.incl.md}}

Start with [Collect Three Markers](../games/index.md#collect-markers) for a
complete input, update, and drawing loop. Use the smaller examples here when
you need to isolate game state or vector math from the frame loop.

## Query Shape Overlap with Parry {#game-development}

Use `parry2d` when a game needs a direct geometric answer, such as whether
a trigger overlaps the player or how far a projectile is from an obstacle.
Describe each shape in its own local coordinates, then give the query a pose
for each shape. A `Cuboid` takes half-extents, so `(1, 1)` makes a square
two world units wide and two units tall.

In this example a radius-one ball at the origin overlaps a square centered
at `(1.5, 0)`. The intersection query reports overlap, the distance query
returns zero, and the contact reports a negative signed separation. Use the
contact normal to identify the surface direction. Its prediction distance
also lets the query report near contacts before the shapes touch. A physics
engine adds the time integration and collision response when you need bodies
to move and react. See [Physics Engines](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/simulation/index.md#physics-engines).

Run from the repository root:

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p game_development --example game_development -- game_development1
```

```rust,editable
{{#include ../../../crates/cats/game_development/examples/game_development/game_development1.rs:example}}
```

## Work with Game Vectors and Transforms Using `glam` {#glam}

{{#badges glam game-engines="Game engines" no-std="No standard library"}}{{hi:Gamedev}}{{hi:Math}}{{hi:Matrix}}{{hi:Quaternion}}{{hi:Vector}}

[`glam`][c~glam~docs]↗{{hi:glam}} supplies 2D and 3D vectors, matrices,
and quaternions. Use `Vec2` for planar gameplay and `Vec3` for positions
and directions in 3D. Add a displacement to move a position. Use a dot
product to measure alignment, normalizing both inputs first when you need
the cosine of the angle. Use `try_normalize` if a zero-length input is
possible. The 3D cross product gives a perpendicular
vector whose direction depends on operand order.

The example builds one local-to-world transform from scale, a 45-degree
rotation around Y, and translation. `transform_point3` applies those
operations in that order. The local point `(1, 0, 0)` lands near
`(4.707, 5, 5.293)`. The quaternion constructor takes radians, so the
example converts degrees explicitly. Use `transform_vector3` for a
displacement when translation should be excluded. Keep units and axis
conventions consistent at the boundaries with your renderer and physics
library, and compare computed floating-point coordinates with a tolerance.

Run the example to see the vector arithmetic and transformed point:

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p game_development --example game_development -- glam
```

```rust,editable
{{#include ../../../crates/cats/game_development/examples/game_development/glam.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[game-engines | Game Engines]].
- [[games | Games]] written in Rust.
- [[graphics | Graphics]] and [[rendering_graphics-api | Rendering Graphics API]].
- [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
