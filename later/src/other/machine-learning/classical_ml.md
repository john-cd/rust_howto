<!-- TODO Reviewed ??-??-2026 -->
# Classical Machine Learning

{{#include classical_ml.incl.md}}

## Train a Classifier with `smartcore` {#smartcore}

[![smartcore][c~smartcore~docs~badge]][c~smartcore~docs] [![smartcore~crates.io][c~smartcore~crates.io~badge]][c~smartcore~crates.io] [![smartcore~repo][c~smartcore~repo~badge]][c~smartcore~repo] [![smartcore~lib.rs][c~smartcore~lib.rs~badge]][c~smartcore~lib.rs]{{hi:smartcore}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

[`smartcore`][c~smartcore~docs]↗{{hi:smartcore}} is a pure-Rust machine learning library (regression, classification, clustering, metrics). The recipe fits a logistic regression on a small in-memory dataset and checks the accuracy. For anything beyond a toy, hold out a test set. See also the [machine learning overview][p~classical-machine-learning]↗.

```rust,editable
{{#include ../../../crates/other/examples/machine_learning/smartcore_fit.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ML examples](https://github.com/john-cd/rust_howto/issues/1929); coordinate with #1757 (Linfa / ndarray compatibility)
</div>
