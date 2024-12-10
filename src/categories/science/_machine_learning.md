# Machine Learning

{{#include _machine_learning.incl.md}}

[Are we learning yet?][are-we-learning-yet?-website]⮳

## Use classical machine learning algorithms

### `linfa` {#linfa}

[![linfa][c-linfa-badge]][c-linfa]{{hi:linfa}} [![linfa-crates.io][c-linfa-crates.io-badge]][c-linfa-crates.io] [![linfa-website][c-linfa-website-badge]][c-linfa-website] [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}{{hi:Machine learning}}

Kin in spirit to Python's `scikit-learn`, `linf` focuses on common preprocessing tasks and classical ML algorithms for everyday ML tasks. Provides a convenient, bundled approach to many machine learning algorithms.

```rust,editable
{{#include ../../../deps/tests/categories/science/linfa.rs:example}}
```

### `smartcore` {#smartcore}

[![smartcore-website][c-smartcore-website-badge]][c-smartcore-website] [![smartcore][c-smartcore-badge]][c-smartcore] [![smartcore-crates.io][c-smartcore-crates.io-badge]][c-smartcore-crates.io] [![smartcore-github][c-smartcore-github-badge]][c-smartcore-github] [![smartcore-lib.rs][c-smartcore-lib.rs-badge]][c-smartcore-lib.rs]{{hi:smartcore}}{{hi:Statistical}}{{hi:Ai}}{{hi:Machine-learning}}{{hi:Optimization}}{{hi:Linear-algebra}}[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

[SmartCore][c-smartcore-website]{{hi:smartcore}}⮳ is a comprehensive library for machine learning and numerical computing. The library provides a set of tools for linear algebra, numerical computing, optimization, and enables a generic, powerful yet still efficient approach to machine learning.

```rust,editable
{{#include ../../../deps/tests/categories/science/smartcore.rs:example}}
```

## Implement deep learning algorithms

### `candle` {#candle}

[![candle-core][c-candle_core-badge]][c-candle_core] [![candle-core-crates.io][c-candle_core-crates.io-badge]][c-candle_core-crates.io] [![candle-core-github][c-candle_core-github-badge]][c-candle_core-github] [![candle-core-lib.rs][c-candle_core-lib.rs-badge]][c-candle_core-lib.rs]{{hi:candle}}{{hi:candle-core}}{{hi:Blas}}{{hi:Machine-learning}}{{hi:Tensor}}[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

`candle` is a minimalist ML framework for Rust with a focus on performance (including GPU support) and ease of use. Candle is like PyTorch written in Rust and focuses on neural network.

[Candle: a minimalist machine learning framework for rust that focuses on performance including GPU support and ease of use][blog-candle]⮳

```rust,editable
{{#include ../../../deps/tests/categories/science/candle.rs:example}}
```

### `burn` {#burn}

[![burn][c-burn-badge]][c-burn] [![burn-crates.io][c-burn-crates.io-badge]][c-burn-crates.io] [![burn-github][c-burn-github-badge]][c-burn-github] [![burn-lib.rs][c-burn-lib.rs-badge]][c-burn-lib.rs]{{hi:burn}}{{hi:Deep-learning}}{{hi:Machine-learning}}{{hi:Ndarray}}{{hi:Pytorch}}{{hi:Tensor}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-science][cat-science-badge]][cat-science]{{hi:Science}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

Burn is a comprehensive dynamic Deep Learning Framework built using Rust with flexibility, compute efficiency and portability as its primary goals.

## Use genetic algorithms {#watchmaker}

[Watchmaker (genetic algos in Rust)][c-watchmaker]{{hi:watchmaker}}⮳ [![watchmaker][c-watchmaker-badge]][c-watchmaker]{{hi:watchmaker}} [![watchmaker-github][c-watchmaker-github-badge]][c-watchmaker-github] [![watchmaker-crates.io][c-watchmaker-crates.io-badge]][c-watchmaker-crates.io] [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

```rust,editable
{{#include ../../../deps/tests/categories/science/watchmaker.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 organize / write

linfa and smartcore have emerged as two leading scikit-learn-analogous machine learning frameworks for Rust. Both provide access to a number of algorithms that form the backbone of machine learning analysis. https://cmccomb.com/smartcore_vs_linfa/

Review tch-rs, burn, ort, rust-bert, tensorflow/rust, tract, cudarc, DFDX
https://ossinsight.io/collections/ml-in-rust/?monthly-rankings=prs

Candle is more focused on inference and provides more pre-trained models, while Burn is more focused on supporting the whole workflow from training to inference. Candle relies on the underlying cuTENSOR and cuDNNv8 libraries, enabling efficient execution on NVIDIA GPUs.

https://medium.com/@athan.seal/choosing-the-right-rust-machine-learning-framework-candle-burn-dfdx-or-tch-rs-17501f6cd765
</div>
