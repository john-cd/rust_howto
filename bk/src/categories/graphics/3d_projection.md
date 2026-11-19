<!-- TODO Reviewed ??-??-2026. -->

# Project 3D Points to Screen Coordinates

A 3D renderer transforms a model-space point with model, view, and projection
matrices. It then divides by the clip-space w coordinate and maps normalized
device coordinates to the viewport. The depth convention and handedness must
match the graphics API that will consume the matrices.

## Project a Vertex to the Screen {#project-a-vertex-to-the-screen}

This example performs the vertex transform on the CPU with [`glam`][c~glam~docs]↗{{hi:glam}}. It prints the screen coordinates and normalized depth for a triangle; it does not create a window or render pixels.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p graphics --example 3d_projection`.

```rust,editable,noplayground
{{#include ../../../crates/cats/graphics/examples/3d_projection/project_triangle.rs:example}}
```

```console
cargo run --locked -p graphics --example 3d_projection
```

The example uses a right-handed, Y-up camera with a depth range of 0 through 1,
as used by APIs such as DirectX, Vulkan, Metal, and WebGPU. Select camera and
projection helpers that match the conventions of your target API.

## Related Topics

- [Linear Algebra](../mathematics/linear_algebra.md) covers vectors and matrices.
- [Tessellate, Then Rasterize](2d_renderers.md#tessellate-and-rasterize) shows a
  CPU rendering pipeline in two dimensions.

{{#include 3d_projection.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
