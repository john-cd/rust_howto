<!-- TODO Reviewed ??-??-2026. -->

# Neural Networks and Deep Learning

{{#include deep_learning.incl.md}}

[`candle`][c~candle-core~repo]↗{{hi:candle}} is more focused on inference and provides more pre-trained models, while [`burn`][c~burn~docs]↗{{hi:burn}} is more focused on supporting the whole workflow from training to inference. `candle` can use NVIDIA GPUs through CUDA (and cuDNN), and Apple GPUs through Metal.

## Choosing a Deep Learning Crate {#choosing}

Choose the execution contract before a framework: train in Rust, load a model
exported elsewhere, or bind an existing native runtime. Accelerator availability is
feature-, operator-, and platform-dependent; a backend name does not establish that
a particular model runs there. Use the exact versions/features in the Cargo manifest.

| Crate | Primary integration boundary | What the example or application must supply |
| --- | --- | --- |
| [`candle`][c~candle-core~repo]↗ | Rust tensors, autograd, layers and model implementations | Architecture, weights, preprocessing, and explicit CPU/accelerator choice. The CPU training/checkpoint recipe below is executable. |
| [`burn`][c~burn~repo]↗ | Training and inference with a selectable backend | Compatible backend/autodiff configuration, model record format and training loop. Framework overview only in this chapter. |
| [`tch`][c~tch~repo]↗ | Rust bindings to LibTorch | A compatible LibTorch installation, library discovery and model/tensor contracts. Not validated by the Candle test. |
| [`ort`][c~ort~repo]↗ | ONNX Runtime sessions and execution providers | Compatible native runtime, supported graph operators and tensor contracts. See the [ONNX recipe](../../other/machine-learning/inference.md). |
| [`tract`][c~tract~repo]↗ | Rust graph inference, including ONNX/NNEF | Operator and shape compatibility for the exact exported graph. An alternative to evaluate, not a tested substitute for this ONNX Runtime session. |
| [`tokenizers`](https://docs.rs/tokenizers/) | Pretrained text preprocessing | The tokenizer artifact and configuration belonging to the model revision. Tokenization alone does not run a language model. |
| [`cudarc`][c~cudarc~repo]↗ | CUDA driver/runtime interaction | Device/toolkit setup and your kernels or framework integration; it is not itself a model/training framework. |

Keep model artifacts, tokenizer/preprocessing configuration, feature/class order and
runtime requirements together. Do not infer model compatibility from matching file
extensions. Start with a CPU fixture whose output can be checked, then validate the
actual accelerator/provider and model operators required by deployment.

## `candle` {#deep-learning}

{{#badges candle-core=}} {{hi:candle}} {{hi:candle-core}} {{hi:BLAS}} {{hi:Machine-learning}} {{hi:Tensor}} {{#categories science=Science}}

[`candle`][c~candle-core~docs]↗{{hi:candle}} is a minimalist ML framework for Rust with a focus on performance (including GPU support) and ease of use. Candle is like PyTorch written in Rust and focuses on neural networks. Read [Candle][blog~candle]↗ for the motivation behind the project.

The example trains a one-layer model on generated values entirely on `Device::Cpu`.
It initializes weights explicitly, checks predictions on held-out inputs, saves a
Safetensors checkpoint to a temporary directory, constructs a fresh model, reloads
its parameters, and checks prediction parity before removing the directory.
`VarMap::load` updates variables already registered by the model architecture;
a weights file alone does not describe the architecture or preprocessing contract.
See the [VarMap API](https://docs.rs/candle-nn/0.11.1/candle_nn/struct.VarMap.html).

No model download or GPU is required. Run the feature explicitly:

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p science --features candle --example ml
```

This single-layer regression teaches tensors, autograd, optimization, and persistence.
It is not a pretrained transformer or a benchmark. A larger neural model also needs
an explicit architecture, a training/validation protocol, and a compatible tokenizer
or feature transform. CUDA/Metal features and LibTorch dependencies require separate
platform checks; CPU success establishes none of those backends.

```rust,ignore,noplayground
{{#include ../../../crates/cats/science/examples/ml/candle.rs:example}}
```

## `burn` {#burn .skip}

{{#badges burn embedded="Embedded development" no-std="No standard library" science=Science wasm=WebAssembly}}{{hi:Deep-learning}}{{hi:Machine-learning}}{{hi:Ndarray}}{{hi:PyTorch}}{{hi:Tensor}}

[`burn`][c~burn~repo]↗ is a comprehensive, dynamic deep learning framework built using Rust with flexibility, compute efficiency and portability as its primary goals. It supports the whole workflow, from defining and training a model to deploying it, and it swaps compute backends (CPU through `ndarray`, `wgpu`, CUDA, Metal) without changing the model code. It also supports `no_std` and WebAssembly targets.

## Bind to Existing Frameworks {#bindings .skip}

- [`tch`][c~tch~repo]↗{{hi:tch}} provides Rust bindings to the C++ API of PyTorch (LibTorch).
- [`tensorflow`][tensorflow-rust~repo]↗{{hi:tensorflow}} provides Rust bindings for TensorFlow. The upstream repository is archived, so treat it as a legacy option and assess alternatives for new projects.
- [`mistral.rs`][mistral.rs~repo]↗ is an inference engine for running large language models in Rust.

## References {#references .skip}

- [Neural Networks][are-we-learning-yet?-neural-networks~website]↗.

## Related Topics {#related-topics .skip}

- [Computer Vision](../computer-vision/index.md).
- [Linear Algebra](https://john-cd.com/rust_howto/categories/mathematics/linear_algebra.html).
- [Machine Learning][p~classical-machine-learning].
- [Artificial Intelligence for Robotics][p~robotics-ai].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
