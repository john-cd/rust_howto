<!-- TODO Reviewed ??-??-2026. -->

# Use GPUs with Rust

{{#include gpu.incl.md}}

GPU work can mean portable compute, vendor-specific compute, or shader code compiled from Rust. Choose the API based on the devices and runtime you must support; examples that dispatch kernels need a compatible GPU driver or software adapter.

## Portable compute with `wgpu` {#wgpu}

[![wgpu~website][c~wgpu~website~badge]][c~wgpu~website] [![wgpu][c~wgpu~docs~badge]][c~wgpu~docs] [![wgpu~crates.io][c~wgpu~crates.io~badge]][c~wgpu~crates.io] [![wgpu~repo][c~wgpu~repo~badge]][c~wgpu~repo]{{hi:wgpu}}{{hi:GPU}}

Use [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} for portable GPU compute with WGSL shaders. The headless compute example from the Graphics APIs chapter uses a storage buffer for the kernel, then copies the results into a separate buffer that the CPU can map. The GPU dispatch rounds up to whole workgroups, so the shader checks the buffer bounds before writing. The test checks the values after readback and needs a compute-capable adapter, including a compatible software adapter.

```rust,ignore,noplayground
{{#include ../../../../bk/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs:example}}
```

When deciding whether to move work to the GPU, measure the complete operation, including copying the data to the device and reading the result back. For kernels written in a Rust-based language with a choice of runtimes, see [`cubecl`][c~cubecl~docs].

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
use cudarc::driver::CudaContext;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = CudaContext::new(0)?;
    let _stream = context.default_stream();
    Ok(())
}
```

This checks whether CUDA can open the first device and obtain its default stream. Follow the [`cudarc` launch example][cudarc~launch] to load compiled kernel code, move buffers to the device, and launch work on the stream.

## Use OpenCL with `opencl3` {#opencl3}

[![opencl3][c~opencl3~docs~badge]][c~opencl3~docs] [![opencl3~crates.io][c~opencl3~crates.io~badge]][c~opencl3~crates.io]{{hi:opencl3}}{{hi:OpenCL}}

Use [`opencl3`][c~opencl3~docs]↗{{hi:opencl3}} to call the OpenCL 3.0 API across devices that provide an OpenCL implementation. The host selects a platform and device, builds a kernel program, and enqueues its work on a command queue; OpenCL feature support varies by device.

```c
__kernel void double_values(__global uint *values) {
    size_t index = get_global_id(0);
    values[index] *= 2;
}
```

Launch this kernel with one work-item per element. If the host pads the global size to a workgroup boundary, pass the element count and guard the index before accessing the buffer. See the [`opencl3` host example][opencl3~example] for the full device, queue, buffer, and readback sequence. [`ocl`][c~ocl~docs] offers a higher-level OpenCL interface.

## Further Reading {#further-reading .skip}

- [Rust-GPU repository][rust-gpu~repo]↗ and [development guide][rust-gpu~book]↗.
- [Rust GPU's community transition and project status][rust-gpu~transition]↗.
- Real-world Rust GPU projects: [`mwa_hyperdrive`][mwa-hyperdrive~repo]↗ and [`locksort`][locksort~repo]↗.

## Related Topics {#related-topics .skip}

[[OpenCL]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
