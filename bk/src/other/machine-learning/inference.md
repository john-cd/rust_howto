<!-- TODO Reviewed ??-??-2026. -->

# Run a Model Trained Elsewhere (ONNX)

{{#include inference.incl.md}}

## Score with `ort`, and Hot-swap the Model {#onnx}

{{#badges ort science=Science}}

[`ort`][c~ort~docs]↗{{hi:ort}} wraps [ONNX Runtime](https://onnxruntime.ai/)↗. The usual workflow is to train in Python (scikit-learn, PyTorch...), export to ONNX, and serve from Rust. The recipe loads a tiny logistic regression exported with `skl2onnx` (the generator script is in the examples' `models` folder), scores a feature vector, then swaps in a new model version without a restart using [`arc-swap`][c~arc-swap~docs]↗{{hi:arc-swap}}.

- **Thread safety**: `Session::run` takes `&mut self`. Share the model through an `Arc`, and use a mutex, a pool of sessions, or one session per worker.
- **Replacement safety**: load and probe a candidate before atomically publishing it. A failed replacement leaves the previous model available. Readers retain one model snapshot for the whole request.
- **Model versioning**: ship the version with the model, return it with each score, and log it. Validate input names, types, dimensions, preprocessing, and class order against the training/export contract. This example probes the named `X` input and requires a finite `[1, 2]` floating-point probability output. A probe does not prove model quality or feature semantics.
- **Build**: `ort` downloads ONNX Runtime binaries at build time (or can load a shared library at run time, with the `load-dynamic` feature). The example is behind the `onnx` feature of the examples crate for that reason.

From the repository root:

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p other --features onnx --example machine_learning
just cargo run --manifest-path later/crates/Cargo.toml --locked -p other --features onnx --example machine_learning -- onnx_infer
```

The committed models let Rust tests run without Python. `models/make_models.py` documents their optional regeneration; regenerate both artifacts deliberately and review changed predictions before committing them. Keep downloaded runtime artifacts in the configured shared `ORT_CACHE_DIR` (`D:/packages/ort` on this Windows machine), rather than beside the source.

```rust,ignore
{{#include ../../../../later/crates/other/examples/machine_learning/onnx_infer.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
