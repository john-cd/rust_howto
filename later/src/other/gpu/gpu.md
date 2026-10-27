<!-- TODO Reviewed ??-??-2026 -->

# Use GPUs with Rust

{{#include gpu.incl.md}}

GPU work can mean portable compute, vendor-specific compute, or shader code compiled from Rust. Choose the API based on the devices and runtime you must support; examples that dispatch kernels need a compatible GPU driver or software adapter.

## Portable compute with `wgpu` {#wgpu}

[![wgpu~website][c~wgpu~website~badge]][c~wgpu~website] [![wgpu][c~wgpu~docs~badge]][c~wgpu~docs] [![wgpu~crates.io][c~wgpu~crates.io~badge]][c~wgpu~crates.io] [![wgpu~repo][c~wgpu~repo~badge]][c~wgpu~repo]{{hi:wgpu}}{{hi:GPU}}

Use [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} for portable GPU compute with WGSL shaders. The linked example starts an adapter and device; a compute pass binds storage buffers and dispatches workgroups, as in this WGSL kernel that doubles each value:

```rust,editable
{{#include ../../../crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs:example}}
```

```wgsl
@group(0) @binding(0)
var<storage, read_write> values: array<u32>;

@compute @workgroup_size(64)
fn double_values(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x < arrayLength(&values)) {
        values[id.x] *= 2;
    }
}
```

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

SPIR-V uses rust-gpu's target backend. Gating these entry points behind `target_arch = "wasm32"` would select WebAssembly rather than compile them as SPIR-V. The shared `other` crate therefore has no rust-gpu example target or shader dependency. For a stable host-side GPU example, use the `wgpu` compute example above.

## Call CUDA from Rust with `cudarc` {#cudarc}

[![cudarc][c~cudarc~docs~badge]][c~cudarc~docs] [![cudarc~crates.io][c~cudarc~crates.io~badge]][c~cudarc~crates.io]{{hi:cudarc}}{{hi:CUDA}}

Use [`cudarc`][c~cudarc~docs]↗{{hi:cudarc}} when targeting NVIDIA CUDA and calling its driver or runtime APIs from a Rust host program. It requires a compatible NVIDIA driver and CUDA setup; kernel code still needs to be compiled or loaded for the selected device.

```rust,ignore
let context = cudarc::driver::CudaContext::new(0)?;
let stream = context.default_stream();
```

## Use OpenCL with `opencl3` {#opencl3}

[![opencl3][c~opencl3~docs~badge]][c~opencl3~docs] [![opencl3~crates.io][c~opencl3~crates.io~badge]][c~opencl3~crates.io]{{hi:opencl3}}{{hi:OpenCL}}

Use [`opencl3`][c~opencl3~docs]↗{{hi:opencl3}} to call the OpenCL 3.0 API across devices that provide an OpenCL implementation. The host selects a platform and device, builds a kernel program, and enqueues its work on a command queue; OpenCL feature support varies by device.

```c
__kernel void double_values(__global uint *values) {
    size_t index = get_global_id(0);
    values[index] *= 2;
}
```

## Further Reading {#further-reading .skip}

- [Rust-GPU repository][rust-gpu~repo]↗ and [development guide][rust-gpu~book]↗.
- [Rust GPU's community transition and project status][rust-gpu~transition]↗.
- Real-world Rust GPU projects: [`mwa_hyperdrive`][mwa-hyperdrive~repo]↗ and [`locksort`][locksort~repo]↗.

## Related Topics {#related-topics .skip}

[[OpenCL]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
