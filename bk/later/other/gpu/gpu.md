# Use GPUs with Rust

{{#include gpu.incl.md}}

## `rust-gpu` {#rust-gpu}

[![rust_gpu~github][rust_gpu~github~badge]][rust_gpu~github]{{hi:rust_gpu}}

[`rust-gpu`][rust-gpu~github]↗{{hi:rust-gpu}} is still heavily in development and is at an early stage. Compiling and running simple shaders works, and a significant portion of the core library also compiles. However, many things aren't implemented yet. That means that while being technically usable, this project is not yet production-ready.

```rust,editable
{{#include ../../../crates/other/examples/gpu/rust_gpu.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[gpu: write](https://github.com/john-cd/rust_howto/issues/607)
Review [Reddit blog][blog~reddit~rust_gpu_the_future_of_gpu_programming]↗.

Reference [`rust-gpu`][rust-gpu~github]↗.

Crates:

- [`rust-gpu` (GitHub)][rust-gpu~github]↗.
- [`rust-gpu`][rust-gpu~website]↗.
- [Rust GPU Dev Guide][rust-gpu~book]↗.
- [OpenCL for Rust][ocl-rust~github]↗.
- [`wgpu`][c~wgpu~website]↗ [(GitHub)][c~wgpu~github]↗.
  - [(Examples)][c~wgpu~examples]↗.
- [`ash`][c~ash~docs]↗.
- [Rust-CUDA][rust-gpu~Rust-CUDA]↗ [(GitHub)][rust-gpu~Rust-CUDA~github]↗
- [`rust-gpu` Blog][blog~rust-gpu].

Examples to look into:

<https://github.com/MWATelescope/mwa_hyperdrive/tree/main>
<https://github.com/TheZoq2/locksort>

[Rust on every GPU][blog~rust-on-every-gpu].
[blog~rust-on-every-gpu]: https://rust-gpu.github.io/blog/2025/07/25/rust-on-every-gpu/

</div>
