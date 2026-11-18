<!-- TODO Reviewed ??-??-2026. -->

# Machine Learning in Practice

These recipes cover three practical workflows: training a small classifier, forecasting future observations, and deploying an exported model. Choose the training and serving tools for the required model, runtime dependencies, target, and performance measurements. These examples use synthetic fixtures and do not establish production accuracy or latency.

{{#include index.incl.md}}

- [Run a Model Trained Elsewhere (ONNX)](inference.md): train in Python, serve in Rust.
- [Classical Machine Learning](classical_ml.md): train a simple model in Rust.
- [Forecasting](forecasting.md): forecast a time series.

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
