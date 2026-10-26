<!-- TODO Reviewed ??-??-2026 -->

# Neural Networks and Deep Learning

{{#include deep_learning.incl.md}}

[`candle`][c~candle-core~repo]↗{{hi:candle}} is more focused on inference and provides more pre-trained models, while [`burn`][c~burn~docs]↗{{hi:burn}} is more focused on supporting the whole workflow from training to inference. `candle` can use NVIDIA GPUs through CUDA (and cuDNN), and Apple GPUs through Metal.

## Choosing a Deep Learning Crate {#choosing}

| Crate                                 | Training | Inference                    | GPU backends                   | Maintained                                                |
| ------------------------------------- | -------- | ---------------------------- | ------------------------------ | --------------------------------------------------------- |
| [`candle`][c~candle-core~repo]↗       | Basic    | Yes, many pre-trained models | CUDA, Metal                    | Yes                                                       |
| [`burn`][c~burn~repo]↗                | Yes      | Yes                          | wgpu, CUDA, Metal, LibTorch    | Yes                                                       |
| [`tch`][c~tch~repo]↗                  | Yes      | Yes                          | CUDA (through LibTorch)        | Yes; binds the C++ PyTorch library                        |
| [`ort`][c~ort~repo]↗                  | No       | Yes (ONNX models)            | CUDA, TensorRT, CoreML, others | Yes; binds ONNX Runtime                                   |
| [`tract`][c~tract~repo]↗              | No       | Yes (ONNX, NNEF)             | CPU only                       | Yes; pure Rust                                            |
| [`tensorflow`][tensorflow-rust~repo]↗ | Yes      | Yes                          | CUDA                           | Little activity; prefer the other crates for new projects |

Train with `burn` or `candle`, or train in Python and run the exported model in Rust with `ort` or `tract` (see the ONNX example in [Machine Learning][p~classical-machine-learning]). Use `tch` or `tensorflow` when you need an existing PyTorch or TensorFlow model unchanged.

## `candle` {#deep-learning}

[![candle-core][c~candle-core~docs~badge]][c~candle-core~docs] [![candle-core~crates.io][c~candle-core~crates.io~badge]][c~candle-core~crates.io] [![candle-core~repo][c~candle-core~repo~badge]][c~candle-core~repo] [![candle-core~lib.rs][c~candle-core~lib.rs~badge]][c~candle-core~lib.rs]{{hi:candle}}{{hi:candle-core}}{{hi:BLAS}}{{hi:Machine-learning}}{{hi:Tensor}}[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

[`candle`][c~candle-core~docs]↗{{hi:candle}} is a minimalist ML framework for Rust with a focus on performance (including GPU support) and ease of use. Candle is like PyTorch written in Rust and focuses on neural networks. Read [Candle][blog~candle]↗ for the motivation behind the project.

The example trains a one-layer model on generated values entirely on `Device::Cpu`, needs no model download, and is behind the `candle` Cargo feature of the `science` crate (`cargo test -p science --features candle`).

```rust,editable
{{#include ../../../crates/cats/science/examples/ml/candle.rs:example}}
```

## `burn` {#burn .skip}

[![burn][c~burn~docs~badge]][c~burn~docs] [![burn~crates.io][c~burn~crates.io~badge]][c~burn~crates.io] [![burn~repo][c~burn~repo~badge]][c~burn~repo] [![burn~lib.rs][c~burn~lib.rs~badge]][c~burn~lib.rs]{{hi:burn}}{{hi:Deep-learning}}{{hi:Machine-learning}}{{hi:Ndarray}}{{hi:PyTorch}}{{hi:Tensor}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`burn`][c~burn~repo]↗ is a comprehensive, dynamic deep learning framework built using Rust with flexibility, compute efficiency and portability as its primary goals. It supports the whole workflow, from defining and training a model to deploying it, and it swaps compute backends (CPU through `ndarray`, `wgpu`, CUDA, Metal) without changing the model code. It also supports `no_std` and WebAssembly targets.

## Bind to Existing Frameworks {#bindings .skip}

- [`tch`][c~tch~repo]↗{{hi:tch}} provides Rust bindings to the C++ API of PyTorch (LibTorch).
- [`tensorflow`][tensorflow-rust~repo]↗{{hi:tensorflow}} provides Rust bindings for TensorFlow. Its development has slowed down.
- [`mistral.rs`][mistral.rs~repo]↗ is an inference engine for running large language models in Rust.

## References {#references .skip}

- [Neural Networks][are-we-learning-yet?-neural-networks~website]↗.

## Related Topics {#related-topics .skip}

- [[computer-vision | Computer Vision]].
- [[linear_algebra | Linear Algebra]].
- [Machine Learning][p~classical-machine-learning].
- [Artificial Intelligence for Robotics][p~robotics-ai].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
