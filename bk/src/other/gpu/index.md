<!-- TODO Reviewed ??-??-2026. -->

# GPU Programming

This index covers general-purpose computing on GPUs from Rust. Graphics APIs for drawing are covered in [[rendering_graphics-api | Graphics APIs]] and shader authoring is covered in [[shaders | Shaders]]. Choose an approach based on the hardware, deployment platforms, and runtime you need.

## Choose an Approach

| Goal | Libraries | Hardware and vendor support | Maturity and requirements |
| --- | --- | --- | --- |
| Portable compute | [`wgpu`][c~wgpu~docs] | Vulkan on Windows, Linux, and Android; Metal on Apple platforms; Direct3D 12 on Windows; WebGPU in browsers. Available vendors depend on the platform backend and driver. | A mature, cross-platform Rust API. Adapter availability and optional features vary by backend and device. |
| Host applications for CUDA | [`cudarc`][c~cudarc~docs] | NVIDIA GPU and a CUDA-capable driver. | Load a precompiled kernel or use NVRTC to compile CUDA C at runtime. Check the driver and toolkit requirements for the chosen CUDA version. |
| Direct control over a native API | [`ash`][c~ash~docs] for Vulkan, [`objc2-metal`][c~objc2-metal~docs] for Metal | Vulkan-capable devices or Apple platforms, respectively. | Choose these when you need API-specific features or interoperate with existing native resources. See [native graphics APIs](../../categories/rendering_graphics-api/native_graphics_apis.md). |
| Write GPU kernels in Rust | [rust-gpu][rust-gpu~repo] for SPIR-V and Vulkan; [Rust-CUDA][rust-cuda~repo] for NVIDIA CUDA. | Vulkan-capable NVIDIA, AMD, Intel, and other GPUs for SPIR-V; NVIDIA devices for CUDA. | Rust-GPU is early-stage; both Rust-authored compiler toolchains use nightly. |
| Machine-learning tensors and kernels | [`CubeCL`][c~cubecl~docs], [`Burn`][c~burn~docs], [`Candle`][c~candle-core~docs]. | Burn lists CUDA, ROCm, Metal, Vulkan, WebGPU, and CPU backends; Candle provides CPU, CUDA, and Metal devices. Backend availability depends on framework features and devices. | CubeCL is in alpha; framework APIs and backend support evolve. Select Cargo features and native SDKs for the chosen device. See the [deep-learning guide](../machine-learning/deep_learning.md). |
| OpenCL compute | [`opencl3`][c~opencl3~docs] or [`ocl`][c~ocl~docs]. | Any CPU, GPU, DSP, FPGA, or accelerator with a compatible OpenCL implementation, including vendor-provided runtimes. | `opencl3` exposes OpenCL 3.0. OpenCL 2.x and 3.0 capabilities are optional and vary by device; `ocl` is an older, slower-moving alternative. Check its maintenance and runtime compatibility before adopting it. |

Start with the [headless wgpu compute example][gpu~compute] for a complete dispatch and readback. It includes an adapter-free WGSL validation test and a separate run that needs a compute-capable adapter. The [GPU compute recipes](gpu.md) cover CUDA, OpenCL, Rust-authored shaders, and benchmarking the same blur on the CPU and GPU.

## Run a first compute kernel

1. **Choose an adapter.** Record its name, backend, and whether it is a hardware or software adapter. Check limits for buffer sizes, workgroups, and shader features such as half-precision arithmetic. Browser compute needs WebGPU and a compute-capable adapter.
2. **Prepare input and a CPU reference.** For a first test, use a predictable operation such as doubling an array of integers. Keep element counts, byte sizes, and shader layout distinct. For example, a WGSL `vec3<f32>` has 16-byte alignment even though its three components occupy 12 bytes.
3. **Dispatch work.** A workgroup contains several shader invocations. Round the dispatch up to cover the input and guard the shader's final indices. Try 130 elements with 64 invocations per group so the last group has both active and inactive invocations.
4. **Read back and compare.** Wait for the copy to complete before accessing the results. Compare every output against the CPU reference. For floating-point kernels, choose a tolerance appropriate to the operation and test how the kernel handles NaNs and other special values.

## Measure the complete workload

Compare an optimized CPU implementation and the GPU on the same input. Measure setup and shader compilation separately from steady-state runs. When the application needs the result on the CPU, include upload, dispatch, and readback in the end-to-end timing. A host timer around asynchronous submission mostly measures enqueue time. Use GPU timestamps where supported and wait for completion for wall-clock measurements. Test several input sizes and cases where intermediate results stay on the device.

## Related Topics {#related-topics .skip}

- [[rendering_graphics-api | Graphics APIs]].
- [[shaders | Shaders]].
- [Deep learning in Rust](../machine-learning/deep_learning.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
