<!-- TODO Reviewed ??-??-2026. -->

# Machine Learning in Practice

Examples for the [Machine Learning in Practice chapter](../../../src/other/machine-learning/index.md).

- `feature_pipeline` fits a scaler on training data and evaluates regression on held-out data.
- `text_features` ranks documents using count vectors and cosine similarity.
- `ml` runs SmartCore classification, Linfa clustering and regression, or Candle CPU training.
- `machine_learning` runs SmartCore classifier training, seasonal forecasting, or ONNX inference with model replacement.

From the repository root:

```sh
just cargo test --manifest-path bk/crates/Cargo.toml --locked -p science --examples
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p science --example feature_pipeline
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p science --example text_features
just cargo test --manifest-path bk/crates/Cargo.toml --locked -p science --features candle --example ml
just cargo test --manifest-path bk/crates/Cargo.toml --locked -p science --features onnx --example machine_learning
```

The `candle` feature enables CPU training and a checkpoint round trip. The `onnx` feature
enables ONNX Runtime, which downloads native binaries at build time.
