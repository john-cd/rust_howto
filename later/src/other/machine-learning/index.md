<!-- TODO Reviewed ??-??-2026 -->

# Machine Learning in Practice

Rust is rarely where models are researched, but it is a good place to run them: low latency, predictable memory, and a single deployable binary. This chapter covers three common situations:

{{#include index.incl.md}}

- [[inference | Run a Model Trained Elsewhere (ONNX)]]: train in Python, serve in Rust.
- [[classical_ml | Classical Machine Learning]]: train a simple model in Rust.
- [[forecasting | Forecasting]]: forecast a time series.

See also [Machine Learning in the Science chapter][p~classical-machine-learning] and [[data-processing | Data Processing]] (for example, `polars` for feature pipelines).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [ML examples](https://github.com/john-cd/rust_howto/issues/1929)
</div>
