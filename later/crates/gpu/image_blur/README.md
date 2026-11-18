# CPU and GPU Image Blur

This standalone project compares the same separable 5-tap grayscale blur on Rayon and wgpu. Each timed GPU sample includes a host upload, two compute passes, a copy into a mappable buffer, and readback. Both paths allocate and warm up before measurement. The program compares every returned pixel and reports medians of seven samples. A separate resident-data column measures submission and completion with the same input already on the device. Use a release build for meaningful timings.

From the repository root:

```console
just cargo run --release --locked --manifest-path later/crates/gpu/image_blur/Cargo.toml
just cargo test --release --locked --manifest-path later/crates/gpu/image_blur/Cargo.toml
```

Set `WGPU_BACKEND` to `vulkan`, `dx12`, or `metal` on a host that supports that backend. Set `WGPU_ADAPTER_NAME` to a case-insensitive substring of an adapter name. The program prints the chosen backend and device and reports an error if the requested name is unavailable. Change `RAYON_NUM_THREADS` to compare CPU configurations, and record it alongside the measured times.

To check the same WGSL in a browser, serve this directory over loopback:

```sh
python -m http.server 8000 --bind 127.0.0.1 --directory later/crates/gpu/image_blur
```

Open `http://127.0.0.1:8000/web/` from a browser with WebGPU. The page verifies every pixel of a 130-by-130 image against a direct 5-by-5 JavaScript reference and reports the adapter information exposed by the browser. The 130-pixel width exercises partially filled workgroups.
