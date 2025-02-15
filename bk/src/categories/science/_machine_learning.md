# Machine Learning

{{#include _machine_learning.incl.md}}

[Are we learning yet?][are-we-learning-yet?-website]⮳

## Use classical machine learning algorithms {#classical-ml}

### `linfa` {#skip1}

[![linfa][c-linfa-badge]][c-linfa]{{hi:linfa}} [![linfa-crates.io][c-linfa-crates.io-badge]][c-linfa-crates.io] [![linfa-website][c-linfa-website-badge]][c-linfa-website] [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}{{hi:Machine learning}}

Kin in spirit to Python's `scikit-learn`, [`linfa`][c-linfa]⮳{{hi:linfa}} focuses on common pre-processing tasks and classical ML algorithms for everyday ML tasks. Provides a convenient, bundled approach to many machine learning algorithms.

```rust,editable
{{#include ../../../crates/cats/science/tests/ml/linfa.rs:example}}
```

### `smartcore` {#skip2}

[![smartcore-website][c-smartcore-website-badge]][c-smartcore-website] [![smartcore][c-smartcore-badge]][c-smartcore] [![smartcore-crates.io][c-smartcore-crates.io-badge]][c-smartcore-crates.io] [![smartcore-github][c-smartcore-github-badge]][c-smartcore-github] [![smartcore-lib.rs][c-smartcore-lib.rs-badge]][c-smartcore-lib.rs]{{hi:smartcore}}{{hi:Statistical}}{{hi:Ai}}{{hi:Machine-learning}}{{hi:Optimization}}{{hi:Linear-algebra}}[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

[SmartCore][c-smartcore-website]{{hi:smartcore}}⮳ is a comprehensive library for machine learning and numerical computing. The library provides a set of tools for linear algebra, numerical computing, optimization, and enables a generic, powerful yet still efficient approach to machine learning.

```rust,editable
{{#include ../../../crates/cats/science/tests/ml/smartcore.rs:example}}
```

## Implement deep learning algorithms {#deep-learning}

### `candle` {#skip3}

[![candle-core][c-candle_core-badge]][c-candle_core] [![candle-core-crates.io][c-candle_core-crates.io-badge]][c-candle_core-crates.io] [![candle-core-github][c-candle_core-github-badge]][c-candle_core-github] [![candle-core-lib.rs][c-candle_core-lib.rs-badge]][c-candle_core-lib.rs]{{hi:candle}}{{hi:candle-core}}{{hi:BLAS}}{{hi:Machine-learning}}{{hi:Tensor}}[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

[`candle`][c-candle_core]⮳{{hi:candle}} is a minimalist ML framework for Rust with a focus on performance (including GPU support) and ease of use. Candle is like PyTorch written in Rust and focuses on neural network.

[Candle: a minimalist machine learning framework for rust that focuses on performance including GPU support and ease of use][blog-candle]⮳

```rust,editable
{{#include ../../../crates/cats/science/tests/ml/candle.rs:example}}
```

### `burn` {#skip4}

[![burn][c-burn-badge]][c-burn] [![burn-crates.io][c-burn-crates.io-badge]][c-burn-crates.io] [![burn-github][c-burn-github-badge]][c-burn-github] [![burn-lib.rs][c-burn-lib.rs-badge]][c-burn-lib.rs]{{hi:burn}}{{hi:Deep-learning}}{{hi:Machine-learning}}{{hi:Ndarray}}{{hi:PyTorch}}{{hi:Tensor}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-science][cat-science-badge]][cat-science]{{hi:Science}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

Burn is a comprehensive dynamic Deep Learning Framework built using Rust with flexibility, compute efficiency and portability as its primary goals.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[_machine_learning: organize / write (P2)](https://github.com/john-cd/rust_howto/issues/473)

linfa and smartcore have emerged as two leading scikit-learn-analogous machine learning frameworks for Rust. Both provide access to a number of algorithms that form the backbone of machine learning analysis. [cmccomb-website][cmccomb-website]

Review tch-rs, burn, ort, rust-bert, TensorFlow/rust, tract, cudarc, DFDX [ML in Rust - Ranking][ml-in-rust-ossinsight-website]

Candle is more focused on inference and provides more pre-trained models, while Burn is more focused on supporting the whole workflow from training to inference. Candle relies on the underlying cuTENSOR and cuDNNv8 libraries, enabling efficient execution on NVIDIA GPUs.

[Choosing the Right Rust Machine Learning Framework][blog-choosing-the-right-machine-learning-framework]

</div>
