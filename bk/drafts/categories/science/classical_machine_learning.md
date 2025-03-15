# Machine Learning

{{#include classical_machine_learning.incl.md}}

[`linfa`][c-linfa]⮳{{hi:linfa}} and [`smartcore`][c-smartcore]⮳{{hi:smartcore}} have emerged as two leading 'scikit-learn'-analogous machine learning frameworks for Rust. Both provide access to a number of [algorithms][p-algorithms] that form the backbone of machine learning analysis. [cmccomb-website][cmccomb-website]

## Use classical machine learning algorithms {#classical-ml}

### `linfa` {#skip1}

[![linfa][c-linfa-badge]][c-linfa]{{hi:linfa}} [![linfa-crates.io][c-linfa-crates.io-badge]][c-linfa-crates.io] [![linfa-website][c-linfa-website-badge]][c-linfa-website] [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}{{hi:Machine learning}}

Kin in spirit to [Python][p-python]'s `scikit-learn`, [`linfa`][c-linfa]⮳{{hi:linfa}} focuses on common pre-processing tasks and classical ML algorithms for everyday ML tasks. Provides a convenient, bundled approach to many machine learning algorithms.

```rust,editable
{{#include ../../../crates/cats/science/tests/ml/linfa.rs:example}}
```

### `smartcore` {#skip2}

[![smartcore-website][c-smartcore-website-badge]][c-smartcore-website] [![smartcore][c-smartcore-badge]][c-smartcore] [![smartcore-crates.io][c-smartcore-crates.io-badge]][c-smartcore-crates.io] [![smartcore-github][c-smartcore-github-badge]][c-smartcore-github] [![smartcore-lib.rs][c-smartcore-lib.rs-badge]][c-smartcore-lib.rs]{{hi:smartcore}}{{hi:Statistical}}{{hi:Ai}}{{hi:Machine-learning}}{{hi:Optimization}}{{hi:Linear-algebra}}[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

[SmartCore][c-smartcore-website]{{hi:smartcore}}⮳ is a comprehensive library for machine learning and numerical computing. The library provides a set of tools for linear algebra, numerical computing, optimization, and enables a generic, powerful yet still efficient approach to machine learning.

```rust,editable
{{#include ../../../crates/cats/science/tests/ml/smartcore.rs:example}}
```

## See Also

- [Are we learning yet?][are-we-learning-yet?-website]⮳.
- [Clustering](https://www.arewelearningyet.com/clustering/)⮳.
- [Decision Trees](https://www.arewelearningyet.com/decision-trees/)⮳.
- [Linear Classifiers](https://www.arewelearningyet.com/linear-classifiers/)⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize / write](https://github.com/john-cd/rust_howto/issues/473)
</div>
