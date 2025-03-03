# Use GPUs with Rust

{{#include gpu.incl.md}}

## `rust-gpu` {#rust-gpu}

[![rust_gpu-github][rust_gpu-github-badge]][rust_gpu-github]{{hi:rust_gpu}}

[`rust-gpu`][rust-gpu]⮳{{hi:rust-gpu}}

```rust,editable
{{#include ../../../crates/other/tests/gpu/rust_gpu.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[gpu: write](https://github.com/john-cd/rust_howto/issues/607)

[Reddit][reddit]

[reddit]: <https://www.reddit.com/r/rust/comments/1fyown4/rust_gpu_the_future_of_gpu_programming/?share_id=in53a04f7pnykanqye5tb&utm_content=1&utm_medium=ios_app&utm_name=iossmf&utm_source=share&utm_term=22&rdt=58853>

[rust-gpu-github]: https://github.com/Rust-GPU/rust-gpu

This project is still heavily in development and is at an early stage.

Compiling and running simple shaders works, and a significant portion of the core library also compiles.

However, many things aren't implemented yet. That means that while being technically usable, this project is not yet production-ready.

[wgpu-website]: https://wgpu.rs

</div>
