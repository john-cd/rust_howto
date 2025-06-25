# Neural Networks and Deep Learning

{{#include deep_learning.incl.md}}

`candle` is more focused on inference and provides more pre-trained models, while [`burn`][c-burn]⮳{{hi:burn}} is more focused on supporting the whole workflow from training to inference. `candle` relies on the underlying cuTENSOR and cuDNNv8 libraries, enabling efficient execution on NVIDIA GPUs.

## `candle` {#deep-learning}

[![candle-core][c-candle_core-badge]][c-candle_core] [![candle-core-crates.io][c-candle_core-crates.io-badge]][c-candle_core-crates.io] [![candle-core-github][c-candle_core-github-badge]][c-candle_core-github] [![candle-core-lib.rs][c-candle_core-lib.rs-badge]][c-candle_core-lib.rs]{{hi:candle}}{{hi:candle-core}}{{hi:BLAS}}{{hi:Machine-learning}}{{hi:Tensor}}[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

[`candle`][c-candle_core]⮳{{hi:candle}} is a minimalist ML framework for Rust with a focus on performance (including GPU support) and ease of use. Candle is like PyTorch written in Rust and focuses on neural network.

[Candle: a minimalist machine learning framework for rust that focuses on performance including GPU support and ease of use][blog-candle]⮳.

```rust,editable
{{#include ../../../crates/cats/science/examples/ml/candle.rs:example}}
```

## `burn` {#skip4}

[![burn][c-burn-badge]][c-burn] [![burn-crates.io][c-burn-crates.io-badge]][c-burn-crates.io] [![burn-github][c-burn-github-badge]][c-burn-github] [![burn-lib.rs][c-burn-lib.rs-badge]][c-burn-lib.rs]{{hi:burn}}{{hi:Deep-learning}}{{hi:Machine-learning}}{{hi:Ndarray}}{{hi:PyTorch}}{{hi:Tensor}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-science][cat-science-badge]][cat-science]{{hi:Science}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

Burn is a comprehensive dynamic Deep Learning Framework built using Rust with flexibility, compute efficiency and portability as its primary goals.

## Related Topics {#skip}

- [[computer-vision | Computer Vision]].
- [[linear_algebra | Linear Algebra]].

## References

- [Neural Networks](https://www.arewelearningyet.com/neural-networks/)🔗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1205)

- [burn: Burn is a new comprehensive dynamic Deep Learning Framework built using Rust with extreme flexibility, compute efficiency and portability as its primary goals.](https://github.com/tracel-ai/burn)
- [tensorflow: Rust language bindings for TensorFlow](https://github.com/tensorflow/rust)

</div>
