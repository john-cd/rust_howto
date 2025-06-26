# Use GPUs with Rust

{{#include gpu.incl.md}}

## `rust-gpu` {#rust-gpu}

[![rust_gpu~github][rust_gpu~github~badge]][rust_gpu~github]{{hi:rust_gpu}}

[`rust-gpu`][rust-gpu]⮳{{hi:rust-gpu}} is still heavily in development and is at an early stage. Compiling and running simple shaders works, and a significant portion of the core library also compiles. However, many things aren't implemented yet. That means that while being technically usable, this project is not yet production-ready.

```rust,editable
{{#include ../../../crates/other/examples/gpu/rust_gpu.rs:example}}
```

[reddit]: https://www.reddit.com/r/rust/comments/1fyown4/rust_gpu_the_future_of_gpu_programming/?share_id=in53a04f7pnykanqye5tb&utm_content=1&utm_medium=ios_app&utm_name=iossmf&utm_source=share&utm_term=22&rdt=58853

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[gpu: write](https://github.com/john-cd/rust_howto/issues/607)
Review [Reddit blog][reddit]
Reference [rust-gpu~github][rust-gpu~github]

Crates:
https://github.com/Rust-GPU
https://github.com/Rust-GPU/rust-gpu https://rust-gpu.github.io/ https://rust-gpu.github.io/rust-gpu/book/
https://github.com/cogciprocate/ocl
https://wgpu.rs/  https://github.com/gfx-rs/wgpu https://github.com/gfx-rs/wgpu/tree/v24/examples#readme
https://docs.rs/ash/latest/ash/
https://rust-gpu.github.io/Rust-CUDA/index.html
https://github.com/Rust-GPU/Rust-CUDA  https://rust-gpu.github.io/blog/2025/01/27/rust-cuda-reboot/
Examples to look into
https://github.com/MWATelescope/mwa_hyperdrive/tree/main
https://github.com/TheZoq2/locksort

</div>
