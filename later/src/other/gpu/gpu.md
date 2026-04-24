# Use GPUs with Rust

{{#include gpu.incl.md}}

## `rust-gpu` {#rust-gpu}

[![rust_gpu~repo][rust_gpu~repo~badge]][rust_gpu~repo]{{hi:rust_gpu}}

[`rust-gpu`][rust-gpu~repo]↗{{hi:rust-gpu}} is still heavily in development and is at an early stage. Compiling and running simple shaders works, and a significant portion of the core library also compiles. However, many things aren't implemented yet. That means that while being technically usable, this project is not yet production-ready.

```rust,editable
{{#include ../../../crates/other/examples/gpu/rust_gpu.rs:example}}
```

## Related Topics {#related-topics .skip}

[[OpenCL]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
