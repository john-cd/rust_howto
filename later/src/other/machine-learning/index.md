<!-- TODO Reviewed ??-??-2026. -->

# Machine Learning in Practice

These recipes cover three practical workflows: training a small classifier, forecasting future observations, and deploying an exported model. Choose the training and serving tools for the required model, runtime dependencies, target, and performance measurements. These examples use synthetic fixtures and do not establish production accuracy or latency.

{{#include index.incl.md}}

- [Run a Model Trained Elsewhere (ONNX)](inference.md): train in Python, serve in Rust.
- [Classical Machine Learning](classical_ml.md): train a simple model in Rust.
- [Forecasting](forecasting.md): forecast a time series.

## Fit Preprocessing Before Evaluation

Split observations before fitting a scaler, imputer, vocabulary, or feature selector.
A preprocessing transform is part of the trained model: save its feature order and
parameters alongside the model and reuse them unchanged at inference time. For time
series, split chronologically; a random split may put future information into training.

This complete example uses `ndarray` for feature matrices and `linfa-linear` for fitting.
It scales from training rows only, handles constant columns, rejects invalid values,
and compares held-out RMSE with a training-mean baseline. The exact line is a software
fixture, not evidence of generalization on noisy observations. A separate validation
split is needed when selecting models or tuning hyperparameters.

```rust,ignore,noplayground
{{#include ../../../crates/cats/science/examples/feature_pipeline.rs:example}}
```

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p science --example feature_pipeline
```

The test with an extreme held-out value verifies that evaluation rows cannot change
training statistics. RMSE has the same units as the target; classification needs
classification metrics rather than an arbitrary RMSE threshold. Keep a final test set
untouched by preprocessing choices and hyperparameter searches.

## Choose the Execution Layer

| Task | Common crates | Coverage in these staged chapters |
| --- | --- | --- |
| Feature arrays and table preparation | `ndarray`, `polars` | Executable array preprocessing here; dataframe workflows in [Data Processing](../data-processing/index.md). |
| Classical fitting | `smartcore`, `linfa` | Held-out classification, regression and seeded clustering examples. |
| Neural-network training | `candle-core`, `candle-nn`, `burn`, `tch` | Candle CPU training and checkpoint parity in [Deep Learning](../../categories/science/deep_learning.md); Burn and tch have framework/prerequisite descriptions, not validated local training recipes. |
| Exported-model inference | `ort`, `tract` | Executable ONNX Runtime inference and replacement; tract is an alternative discussed in [inference](inference.md). |
| Forecasting | `augurs` | Chronological holdout and seasonal-naive comparison in [Forecasting](forecasting.md). |
| Text features | `tokenizers`, `candle-transformers` | Relevant for pretrained NLP models; tokenizer files, special tokens, truncation and model revision must match the model. No downloaded NLP model is required or validated by these fixtures. |

## Validate the Workflow

- Classical classification uses rows excluded from model fitting.
- Forecasting holds out later timestamps and compares with a seasonal-naive baseline.
- ONNX inference requires the optional `onnx` feature and a working native ONNX Runtime. It tests successful replacement, rejected replacement, and invalid features.
- Fit preprocessing on training data only, record feature/class order with the artifact, and evaluate deployment data separately from build/runtime checks.

Run the default Rust tests from the repository root:

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p other --example machine_learning
```

The `other` example package shares dependencies with other future workflows, so its initial build also compiles dependencies unrelated to ML. Only the `machine_learning` example target is executed by this command. ONNX setup and commands are documented in its own recipe.

See also [Machine Learning in the Science chapter][p~classical-machine-learning] and [Data Processing](../data-processing/index.md) (for example, `polars` for feature pipelines).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
