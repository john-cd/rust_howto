# Rust-GPU Offscreen Triangle

This standalone project uses Rust-GPU 0.11.0 on its pinned nightly, `nightly-2026-08-15`. It needs a C++ toolchain to compile the bundled SPIR-V Tools and a Vulkan driver with graphics support for the rendering test. The build script compiles the `no_std` shader, validates it for Vulkan 1.1, and embeds the resulting SPIR-V in the wgpu host binary.

From the repository root:

```console
just cargo +nightly-2026-08-15 test --locked --manifest-path later/crates/gpu/rust_gpu_triangle/Cargo.toml
just cargo +nightly-2026-08-15 test --locked --manifest-path later/crates/gpu/rust_gpu_triangle/Cargo.toml -- --include-ignored --nocapture
```

The second command runs the offscreen test. It checks an interior red pixel and an exterior blue pixel in a 64-by-64 RGBA readback. Success proves this shader and pipeline on the printed Vulkan adapter. The host takes no vertex buffer because the Rust shader derives positions from the vertex index.
