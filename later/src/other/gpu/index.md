<!-- TODO Reviewed ??-??-2026 -->

# GPU Programming

This index covers general-purpose computing on GPUs from Rust. Graphics APIs for drawing are covered in [[rendering_graphics-api | Rendering: Graphics API]] and shader authoring is covered in [[shaders | Shaders]]. Choose an approach based on the hardware, deployment platforms, and runtime you need.

## Choose an Approach

| Goal                                 | Libraries                                                                                                                                   | Hardware and vendor support                                                                                                                                                   | Maturity and requirements                                                                                                                                                                                        |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Portable compute                     | [`wgpu`][c~wgpu~docs]                                                                                                                       | Vulkan on Windows, Linux, and Android; Metal on Apple platforms; Direct3D 12 on Windows; WebGPU in browsers. Available vendors depend on the platform backend and driver.     | A mature, cross-platform Rust API. Adapter availability and optional features vary by backend and device.                                                                                                        |
| Write GPU kernels in Rust            | [rust-gpu][rust-gpu~repo] for SPIR-V and Vulkan; [Rust-CUDA][rust-cuda~repo] for NVIDIA CUDA; [`cudarc`][c~cudarc~docs] for CUDA host APIs. | Vulkan-capable NVIDIA, AMD, Intel, and other GPUs for SPIR-V; NVIDIA devices for CUDA.                                                                                        | Rust-GPU is early-stage; both Rust-authored compiler toolchains use nightly. `cudarc` provides host-side CUDA bindings and requires a compatible driver/runtime.                                                 |
| Machine-learning tensors and kernels | [`CubeCL`][c~cubecl~docs], [`Burn`][c~burn~docs], [`Candle`][c~candle-core~docs].                                                           | Burn lists CUDA, ROCm, Metal, Vulkan, WebGPU, and CPU backends; Candle provides CPU, CUDA, and Metal devices. Backend availability depends on framework features and devices. | CubeCL is in alpha; framework APIs and backend support evolve. Select Cargo features and native SDKs for the chosen device. See the [deep-learning guide][ex~gpu~deep-learning].                                 |
| OpenCL compute                       | [`opencl3`][c~opencl3~docs] or [`ocl`][c~ocl~docs].                                                                                         | Any CPU, GPU, DSP, FPGA, or accelerator with a compatible OpenCL implementation, including vendor-provided runtimes.                                                          | `opencl3` exposes OpenCL 3.0. OpenCL 2.x and 3.0 capabilities are optional and vary by device; `ocl` is an older, slower-moving alternative. Check its maintenance and runtime compatibility before adopting it. |

{{#include gpu.incl.md}}

Continue to the [GPU programming guide](gpu.md) for setup details and examples.

## Related Topics {#related-topics .skip}

- [[rendering_graphics-api | Rendering: Graphics API]].
- [[shaders | Shaders]].
- [Deep learning in Rust][ex~gpu~deep-learning].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
