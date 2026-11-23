<!-- TODO Reviewed ??-??-2026. -->

# CUDA and OpenCL Compute Examples

These examples double 130 unsigned integers and compare the complete readback with a CPU reference. They share a lockfile and have independent Cargo features. The compile check uses dynamic loading and does not need a GPU or either vendor runtime.

From the repository root:

```console
just cargo check --locked --manifest-path bk/crates/gpu/vendor_compute/Cargo.toml --all-targets --all-features
```

For the CUDA run, make the NVIDIA driver and CUDA 12.8 NVRTC library available to the dynamic loader. For the OpenCL run, install an OpenCL loader and a GPU device implementation.

```console
just cargo run --locked --manifest-path bk/crates/gpu/vendor_compute/Cargo.toml --features cuda --example cuda_double
just cargo run --locked --manifest-path bk/crates/gpu/vendor_compute/Cargo.toml --features opencl --example opencl_double
```

Both programs print the selected device and `Verified 130 doubled values` on success.
