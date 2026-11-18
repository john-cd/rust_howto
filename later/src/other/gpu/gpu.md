<!-- TODO Reviewed ??-??-2026. -->

# GPU Compute Recipes

See the [GPU programming survey][gpu~survey] to choose a compute API.

{{#include gpu.incl.md}}

The recipes below require their own compiler, driver, or runtime. For portable WGSL compute, see the [book's headless wgpu example][gpu~compute].

## `rust-gpu` {#rust-gpu}

[![rust-gpu~repo][rust-gpu~repo~badge]][rust-gpu~repo]{{hi:rust-gpu}}

[`rust-gpu`][rust-gpu~repo]↗{{hi:rust-gpu}} compiles Rust shader crates to SPIR-V for Vulkan. It is experimental and requires a nightly toolchain plus `spirv-builder`; follow its [development guide][rust-gpu~book]↗ for the matching setup and compiler version.

The following is shader-crate source, shown as an ignored snippet because it requires the rust-gpu compiler backend. It emits a triangle with a solid red fragment color. It belongs in a separate shader crate's `src/lib.rs`, outside the shared `later/crates` workspace.

```rust,ignore
#![no_std]

use spirv_std::glam::{vec4, Vec4};
use spirv_std::spirv;

/// Writes the clip-space position for each of the triangle's three vertices.
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vertex_index: i32,
    #[spirv(position)] position: &mut Vec4,
) {
    *position = match vertex_index {
        0 => vec4(-0.5, -0.5, 0.0, 1.0),
        1 => vec4(0.5, -0.5, 0.0, 1.0),
        _ => vec4(0.0, 0.5, 0.0, 1.0),
    };
}

/// Writes opaque red to the first color attachment.
#[spirv(fragment)]
pub fn main_fs(#[spirv(location = 0)] output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0);
}
```

### Build the shader separately {#rust-gpu-build}

1. Select a rust-gpu release or Git revision and copy its `rust-toolchain.toml` into a separate project. The compiler backend requires that exact nightly; the book's stable toolchain and nightly formatter pin serve different purposes.
2. Create a shader library with `crate-type = ["dylib"]` and a `spirv-std` dependency matching the selected release or revision. Keep it in its own workspace so its toolchain and dependencies are independent of this book's examples.
3. In a host crate in that project, add the matching `spirv-builder` as a build dependency. Its `build.rs` compiles the shader crate for a SPIR-V target such as `spirv-unknown-vulkan1.1`; follow the [shader build guide][rust-gpu~shader-guide] for the builder API and required toolchain components of the chosen version.
4. Load the generated SPIR-V module in a compatible graphics application, select `main_vs` and `main_fs` as the pipeline entry points, and draw three vertices. A shader defines GPU stages; the host must still create the device, pipeline, render target, and draw commands.

Check the generated SPIR-V module before loading it. Validate it for the target Vulkan environment and inspect the vertex and fragment entry points. A runtime check should render to an offscreen target and compare a pixel inside the triangle with the expected red and a background pixel with the clear color. The shader source alone cannot exercise pipeline state, vertex ordering, or readback.

SPIR-V uses rust-gpu's target backend. Gating these entry points behind `target_arch = "wasm32"` selects WebAssembly. Keep the shader project and its compiler pin separate from the book's stable Cargo workspace.

## Call CUDA from Rust with `cudarc` {#cudarc}

[![cudarc][c~cudarc~docs~badge]][c~cudarc~docs] [![cudarc~crates.io][c~cudarc~crates.io~badge]][c~cudarc~crates.io]{{hi:cudarc}}{{hi:CUDA}}

This example compiles CUDA C to PTX at runtime with NVRTC, loads the module through the NVIDIA driver, and launches the kernel on one stream. It uploads 130 integers, doubles them in place, and compares the full readback against a CPU reference.

Create a binary Cargo package, save the snippet as `src/main.rs`, and use the following dependency for CUDA 12.8. The NVIDIA driver must support the CUDA version you select. Install the matching NVRTC library and ensure the dynamic loader can find it. For a deployment that already ships PTX, see the [precompiled-kernel example][cudarc~launch].

```toml
[dependencies]
cudarc = { version = "=0.19.10", default-features = false, features = [
  "std",
  "driver",
  "nvrtc",
  "dynamic-loading",
  "cuda-12080",
] }
```

```rust,ignore,noplayground
{{#include examples/cuda_double.rs}}
```

Run `cargo run`. The program prints the device name and `Verified 130 doubled values` on success. A missing driver and a missing NVRTC library are different setup problems. If runtime compilation fails after both load, inspect the NVRTC compile error before debugging the launch.

## Use OpenCL with `opencl3` {#opencl3}

[![opencl3][c~opencl3~docs~badge]][c~opencl3~docs] [![opencl3~crates.io][c~opencl3~crates.io~badge]][c~opencl3~crates.io]{{hi:opencl3}}{{hi:OpenCL}}

Use [`opencl3`][c~opencl3~docs]↗{{hi:opencl3}} to run a kernel on devices that provide OpenCL. You need both the [OpenCL loader][opencl~loader] and a device implementation (ICD). If the loader is present but no GPU is enumerated, check the vendor runtime and GPU support. OpenCL 3.0 devices can expose different optional features.

Create a binary Cargo package, save the snippet as `src/main.rs`, and add the dependency below. The host builds the kernel from source and prints the selected device. The padded dispatch submits 192 work-items for 130 elements. The guard keeps the remaining 62 work-items from accessing the buffer. The example uses blocking host transfers and waits for the kernel event before reading back.

```toml
[dependencies]
opencl3 = "=0.12.4"
```

```rust,ignore,noplayground
{{#include examples/opencl_double.rs}}
```

Run `cargo run` and look for `Verified 130 doubled values`. For event profiling or nonblocking transfers, see the [upstream OpenCL example][opencl3~example]. An asynchronous transfer requires its host memory to remain valid until the completion event. [`ocl`][c~ocl~docs] offers a higher-level OpenCL interface.

## Further Reading {#further-reading .skip}

- [Rust-GPU repository][rust-gpu~repo]↗ and [development guide][rust-gpu~book]↗.
- [Rust GPU's community transition and project status][rust-gpu~transition]↗.
- Real-world Rust GPU projects: [`mwa_hyperdrive`][mwa-hyperdrive~repo]↗ and [`locksort`][locksort~repo]↗.

## Related Topics {#related-topics .skip}

[[OpenCL]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
