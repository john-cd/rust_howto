<!-- TODO Reviewed ??-??-2026. -->

# GPU Compute Recipes

See the [GPU programming survey][gpu~survey] to choose a compute API.

{{#include gpu.incl.md}}

The recipes below require their own compiler, driver, or runtime. For portable WGSL compute, see the [book's headless wgpu example][gpu~compute].

## `rust-gpu` {#rust-gpu}

[![rust-gpu~repo][rust-gpu~repo~badge]][rust-gpu~repo]{{hi:rust-gpu}}

[`rust-gpu`][rust-gpu~repo]↗{{hi:rust-gpu}} compiles Rust shader crates to SPIR-V for Vulkan. It is experimental and requires a nightly toolchain plus `spirv-builder`; follow its [development guide][rust-gpu~book]↗ for the matching setup and compiler version.

The standalone project at `bk/crates/gpu/rust_gpu_triangle` builds this pair of shaders and embeds their SPIR-V in a wgpu host. The vertex shader supplies three positions and the fragment shader writes opaque red.

```rust,ignore
{{#include ../../../crates/gpu/rust_gpu_triangle/shader/src/lib.rs}}
```

### Build the shader separately {#rust-gpu-build}

The project pins Rust-GPU 0.11.0 and `nightly-2026-08-15` with `rust-src`, `rustc-dev`, and `llvm-tools`. The first build needs a C++ compiler for the bundled SPIR-V Tools. It compiles the shader to `spirv-unknown-vulkan1.1`, validates the resulting module for Vulkan 1.1, and embeds it in the host. See the [shader build guide][rust-gpu~shader-guide] for the toolchain details.

```console
rustup toolchain install nightly-2026-08-15 --profile minimal --component rust-src,rustc-dev,llvm-tools
just cargo +nightly-2026-08-15 test --locked --manifest-path bk/crates/gpu/rust_gpu_triangle/Cargo.toml -- --include-ignored --nocapture
```

The host selects a Vulkan adapter and the `main_vs` and `main_fs` entry points. It renders to a 64-by-64 RGBA texture, copies it to a mappable buffer, and checks a red interior pixel and a blue exterior pixel. The checked-in project can build and validate SPIR-V without rendering. Run the ignored test to check the pipeline and readback on a GPU. Keep the Rust-GPU nightly isolated from the stable book workspace.

## Call CUDA from Rust with `cudarc` {#cudarc}

[![cudarc][c~cudarc~docs~badge]][c~cudarc~docs] [![cudarc~crates.io][c~cudarc~crates.io~badge]][c~cudarc~crates.io]{{hi:cudarc}}{{hi:CUDA}}

This example compiles CUDA C to PTX at runtime with NVRTC, loads the module through the NVIDIA driver, and launches the kernel on one stream. It uploads 130 integers, doubles them in place, and compares the full readback against a CPU reference.

The checked-in example uses CUDA 12.8. Install a compatible NVIDIA driver and the matching NVRTC library, and make the library available to the dynamic loader. For a deployment that already ships PTX, see the [precompiled-kernel example][cudarc~launch].

From the repository root:

```console
just cargo run --locked --manifest-path bk/crates/gpu/vendor_compute/Cargo.toml --features cuda --example cuda_double
```

```rust,ignore,noplayground
{{#include ../../../crates/gpu/vendor_compute/examples/cuda_double.rs}}
```

The program prints the device name and `Verified 130 doubled values` on success. A missing driver and a missing NVRTC library are different setup problems. If runtime compilation fails after both load, inspect the NVRTC compile error before debugging the launch.

## Use OpenCL with `opencl3` {#opencl3}

[![opencl3][c~opencl3~docs~badge]][c~opencl3~docs] [![opencl3~crates.io][c~opencl3~crates.io~badge]][c~opencl3~crates.io]{{hi:opencl3}}{{hi:OpenCL}}

Use [`opencl3`][c~opencl3~docs]↗{{hi:opencl3}} to run a kernel on devices that provide OpenCL. You need both the [OpenCL loader][opencl~loader] and a device implementation (ICD). If the loader is present but no GPU is enumerated, check the vendor runtime and GPU support. OpenCL 3.0 devices can expose different optional features.

The host builds the kernel from source and prints the selected device. The padded dispatch submits 192 work-items for 130 elements. The guard keeps the remaining 62 work-items from accessing the buffer. The example uses blocking host transfers and waits for the kernel event before reading back.

From the repository root:

```console
just cargo run --locked --manifest-path bk/crates/gpu/vendor_compute/Cargo.toml --features opencl --example opencl_double
```

```rust,ignore,noplayground
{{#include ../../../crates/gpu/vendor_compute/examples/opencl_double.rs}}
```

The expected confirmation is `Verified 130 doubled values`. For event profiling or nonblocking transfers, see the [upstream OpenCL example][opencl3~example]. An asynchronous transfer requires its host memory to remain valid until the completion event. [`ocl`][c~ocl~docs] offers a higher-level OpenCL interface.

## Compare CPU and GPU Image Blur {#image-blur}

The standalone project at `bk/crates/gpu/image_blur` blurs a grayscale image with the same separable five-tap Gaussian approximation on both devices. The first pass sums neighboring columns with weights `[1, 4, 6, 4, 1]`. The second pass uses the same weights on rows and rounds once, after both passes. Image edges clamp to the nearest pixel. This integer formulation lets the benchmark compare every pixel exactly.

```wgsl
{{#include ../../../crates/gpu/image_blur/src/blur.wgsl}}
```

Run an optimized build from the repository root:

```console
just cargo run --release --locked --manifest-path bk/crates/gpu/image_blur/Cargo.toml
```

The CPU path distributes independent rows across Rayon workers and reuses its scratch buffers. The GPU path reuses its pipelines and buffers. Each end-to-end sample includes upload, both compute passes, synchronization, and readback into a host buffer. The resident-data column measures submission and completion for input already on the GPU. Both columns use host wall-clock time. Setup and warm-up are outside the measurement, and every end-to-end readback is checked before the next sample.

The program prints the adapter and median of seven samples at each size. A CPU-to-GPU ratio above 1 means the GPU was faster **with transfers included**. A smaller resident-data time shows what can change when the next stage also runs on the GPU.

In one Windows run using 20 Rayon threads and an RTX 3060 Laptop GPU over Vulkan, the 512-pixel case completed in 0.58 ms on the CPU and 0.47 ms on the GPU including transfers. At 2,048 pixels per row, the times were 2.20 ms on the CPU, 5.26 ms on the GPU including transfers, and 0.55 ms with the data already on the GPU. This run illustrates why input size alone cannot predict a crossover. Compare your own driver, transfer path, thread count, and workload.

Set `WGPU_BACKEND` to `vulkan`, `dx12`, or `metal` on a host with that backend. Set `WGPU_ADAPTER_NAME` to a substring of the adapter name, such as `NVIDIA`, `Intel`, or `AMD`. Record the actual device and backend printed by the program when comparing runs.

## Check the Shader in Browser WebGPU {#webgpu-smoke}

Serve the image-blur project on loopback and open `http://127.0.0.1:8000/web/` in a WebGPU-capable browser.

```sh
python -m http.server 8000 --bind 127.0.0.1 --directory bk/crates/gpu/image_blur
```

The page dispatches the same WGSL as the Rust benchmark. It checks every pixel of a 130-by-130 image against a separate, direct 5-by-5 JavaScript reference. The odd width tests the workgroup bounds checks. A successful run reports 16,900 verified pixels and the browser's adapter information. Stop the server with Ctrl+C when the check finishes.

The validation runs exercised the native blur on NVIDIA and Intel adapters over Vulkan and Direct3D 12, the Rust-GPU triangle on Intel Vulkan, and browser WebGPU in Chrome on the Intel adapter. The native benchmark can also check AMD Vulkan and Apple Metal when those devices and drivers are available.

## Further Reading {#further-reading .skip}

- [Rust-GPU repository][rust-gpu~repo]↗ and [development guide][rust-gpu~book]↗.
- [Rust GPU's community transition and project status][rust-gpu~transition]↗.
- Real-world Rust GPU projects: [`mwa_hyperdrive`][mwa-hyperdrive~repo]↗ and [`locksort`][locksort~repo]↗.

## Related Topics {#related-topics .skip}

- [GPU Programming][gpu~survey].
- [Graphics APIs][p~rendering_graphics-api].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
