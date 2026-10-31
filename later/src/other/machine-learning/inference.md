<!-- TODO Reviewed ??-??-2026 -->

# Run a Model Trained Elsewhere (ONNX)

{{#include inference.incl.md}}

## Score with `ort`, and Hot-swap the Model {#onnx}

{{#badges ort science=Science}}

[`ort`][c~ort~docs]↗{{hi:ort}} wraps [ONNX Runtime](https://onnxruntime.ai/)↗. The usual workflow is to train in Python (scikit-learn, PyTorch...), export to ONNX, and serve from Rust. The recipe loads a tiny logistic regression exported with `skl2onnx` (the generator script is in the examples' `models` folder), scores a feature vector, then swaps in a new model version without a restart using [`arc-swap`][c~arc-swap~docs]↗{{hi:arc-swap}}.

- **Thread safety**: `Session::run` takes `&mut self`. Share the model through an `Arc`, and use a mutex, a pool of sessions, or one session per worker.
- **Model versioning**: ship the version with the model, return it with each score, and log it. Validate the model's input and output metadata when you load it.
- **Build**: `ort` downloads ONNX Runtime binaries at build time (or can load a shared library at run time, with the `load-dynamic` feature). The example is behind the `onnx` feature of the examples crate for that reason.

```rust,editable
{{#include ../../../crates/other/examples/machine_learning/onnx_infer.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [ML examples](https://github.com/john-cd/rust_howto/issues/1929)
</div>
