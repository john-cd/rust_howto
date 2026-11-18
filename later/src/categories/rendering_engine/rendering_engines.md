<!-- TODO Reviewed ??-??-2026. -->

# Rendering Engines

{{#include rendering_engines.incl.md}}

## Choose a Render Engine {#render}

A rendering engine owns scene objects, camera transforms, materials, asset lifetimes, and the passes that produce a frame. A graphics API such as `wgpu` exposes devices, buffers, pipelines, and command submission; an application using it directly must implement those engine responsibilities itself.

| Requirement | Starting point | What to verify |
| --- | --- | --- |
| A scene, camera, and materials for a small visualization | `three-d` or `kiss3d` | Supported window backend, driver, target, asset loading, and visible output. |
| Rendering integrated with an ECS game/application | `bevy_render` within Bevy | Compatible Bevy/plugin versions, asset lifecycle, frame scheduling, and target features. |
| Explicit ownership of resources and render passes | `wgpu` | Adapter/device support, texture formats, shader validation, submission, and readback/presentation. |
| A server-side image or deterministic regression frame | An offscreen target | Pixel format, color space, row stride, completion, and image assertions. |

The [3D renderers chapter](../rendering/3d_renderers.md) surveys engine options and demonstrates camera projection. A crate appearing in that catalog is not evidence that it has been launched on every platform. Keep engine versions and plugins aligned and test the actual window or offscreen workflow used by the application.

## Render and Verify an Offscreen Frame {#offscreen}

This bounded example implements a small rendering core with `wgpu`. It draws a red triangle over a blue clear color, copies a 64×64 RGBA texture into CPU-visible memory, and verifies an interior triangle pixel and an exterior background pixel. It exercises shader compilation, an actual render pass, GPU completion, and readback without opening a window.

From the repository root:

```sh
just cargo run --manifest-path later/crates/Cargo.toml --locked -p rendering_graphics_api --features wgpu --example offscreen_render
just cargo test --manifest-path later/crates/Cargo.toml --locked -p rendering_graphics_api --features wgpu --example offscreen_render -- --include-ignored
```

On Windows, set `WGPU_BACKEND=dx12` or `WGPU_BACKEND=vulkan` in the shell to test a specific native backend. A supported native GPU adapter and driver are required. The explicit GPU test is ignored by default; the shader-validation and malformed-frame tests run without accessing a GPU. Failure to obtain an adapter is an error, not a skipped success.

```rust,ignore
{{#include ../../../crates/cats/rendering_graphics_api/examples/offscreen_render.rs:example}}
```

The 64-pixel RGBA row occupies 256 bytes, satisfying the texture-copy row-alignment requirement without padding. Other widths need a padded staging row and removal of that padding before image comparison; the [layout example](../rendering_graphics-api/frame_layout.md) tests 65-pixel rows, malformed buffers, and aspect-preserving viewports. `Rgba8Unorm` gives linear byte values here; sRGB targets and antialiased edges require color-aware comparisons and tolerances. The checks deliberately sample pixels away from triangle edges.

The map callback is serviced by polling the device, with a ten-second timeout. CPU-visible bytes must not be read before mapping completes. A windowed engine additionally needs surface configuration, resize/minimize handling, device/surface-loss recovery, and an event loop. This offscreen result does not validate Bevy, `three-d`, or a window backend; those remain separate platform checks.

## Related Topics {#related-topics .skip}

- [3D Renderers](../rendering/3d_renderers.md) covers camera projection and engine choices.
- [Native Graphics APIs](../rendering_graphics-api/native_graphics_apis.md) identifies the selected native backend.
- [GPU Abstraction Layers](../rendering_graphics-api/gpu_abstraction_layers.md) demonstrates compute submission and readback.
- [Window Creation](../gui/window_creation.md) covers presentation windows and event handling.
- [Rendering Data Formats](../rendering_data-formats/index.md) covers scene and texture inputs.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
