<!-- TODO Reviewed ??-??-2026. -->

# Train and Evaluate a Classifier

{{#include classical_ml.incl.md}}

From the repository root, run:

```sh
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p science --example machine_learning -- smartcore_fit
```

Split real data before fitting preprocessing steps. Keep related subjects or time periods in the same split when row-level random splits would leak information. Report class balance and per-class errors alongside accuracy when the cost of mistakes differs.

## Train a Classifier with `smartcore` {#smartcore}

{{#badges smartcore science=Science}}

[`smartcore`][c~smartcore~docs]↗{{hi:smartcore}} is a pure-Rust machine learning library (regression, classification, clustering, metrics). The recipe fits logistic regression on eight training rows and checks six different held-out rows. Labels in this synthetic example indicate whether the sum of the two coordinates is positive. Evaluation never reuses the training rows. A perfect result on this tiny, separable fixture verifies the workflow, not generalization to real data. See also the [machine learning overview][p~classical-machine-learning]↗.

```rust,ignore
{{#include ../../../crates/other/machine_learning/examples/machine_learning/smartcore_fit.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  ML examples; coordinate with #1757 (Linfa / ndarray compatibility)
</div>
