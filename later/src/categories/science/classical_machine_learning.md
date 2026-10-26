<!-- TODO Reviewed ??-??-2026 -->

# Machine Learning

{{#include classical_machine_learning.incl.md}}

[`linfa`][c~linfa~docs]↗{{hi:linfa}} and [`smartcore`][c~smartcore~docs]↗{{hi:smartcore}} provide classical machine-learning algorithms for Rust. Choose an algorithm for the task and keep data preparation inside the training split to avoid leaking information into evaluation.

## Choose an Algorithm by Task {#classical-ml}

| Task           | Example                                                    | Guidance                                                                     |
| -------------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Classification | Seeded [`smartcore`][c~smartcore~docs] decision tree       | Use labeled examples and evaluate predictions on held-out data.              |
| Clustering     | Seeded [`linfa-clustering`][linfa-clustering~docs] K-means | Use numeric features without labels and choose the number of clusters.       |
| Regression     | [`linfa-linear`][c~linfa-linear~docs] linear regression    | Predict a numeric target and check prediction error on held-out data.        |
| Preprocessing  | Scaling and train/test splitting                           | Split first; fit scalers and other learned transforms on training rows only. |

## Compare Linfa and SmartCore

| Crate                           | Algorithms                                          | Array integration                              | Maintenance                                                                   |
| ------------------------------- | --------------------------------------------------- | ---------------------------------------------- | ----------------------------------------------------------------------------- |
| [`linfa`][c~linfa~docs]         | Modular crates: `linfa-clustering`, `linfa-linear`. | `ndarray` records in `DatasetBase`.            | Review updates across algorithm crates; see [Linfa releases][linfa~releases]. |
| [`smartcore`][c~smartcore~docs] | Broad suite in one crate.                           | `DenseMatrix`; optional `ndarray` integration. | Follow the main crate's [release history][smartcore~releases].                |

This chapter's examples use Linfa 0.8.1 and SmartCore 0.6.16.

## Classify with SmartCore

The seeded decision-tree recipe uses a synthetic, linearly separable dataset and checks accuracy on a held-out split.

```rust,editable
{{#include ../../../crates/cats/science/examples/ml/smartcore.rs:example}}
```

## Cluster with Linfa

K-means needs only feature rows. This example creates two small synthetic groups, seeds its random initializer, and tests that every point receives a label across exactly two clusters.

```rust,editable
{{#include ../../../crates/cats/science/examples/ml/kmeans.rs:example}}
```

## Predict Numeric Values with Linfa

Linear regression fits a numeric target and predicts values for new feature rows.

```rust,editable
{{#include ../../../crates/cats/science/examples/ml/linfa.rs:example}}
```

## Prepare and Evaluate Data

- Keep features and targets aligned when shuffling or splitting rows.
- Split data before fitting a scaler, encoder, imputer, or feature selector. Learn its parameters from training rows and apply them unchanged to validation and test rows.
- Use a fixed seed when an API supports it, so tests and comparisons are reproducible.
- Choose metrics for the task: accuracy or class-specific measures for classification, a distance-based measure for clustering, and error measures for regression.
- Prefer a library's dataset and preprocessing APIs when they fit the problem; otherwise keep transformations explicit and test them independently.

## See Also {#see-also .skip}

- [Are we learning yet?][are-we-learning-yet?~website]↗.
- [Clustering][are-we-learning-yet?-clustering~website]↗.
- [Decision Trees][are-we-learning-yet?-decision-trees~website]↗.
- [Linear Classifiers][are-we-learning-yet?-linear-classifiers~website]↗.
- [Deep learning and neural networks][ex~science~deep-learning].

## Related Topics {#related-topics .skip}

- Data science and analytics.
- Deep learning and neural networks.
- Statistical modeling.
- Scientific computing in Rust.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
