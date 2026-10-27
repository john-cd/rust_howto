<!-- TODO Reviewed ??-??-2026 -->

# Neural Networks and Deep Learning

{{#include deep_learning.incl.md}}

[`candle`][c~candle-core~repo]↗{{hi:candle}} is more focused on inference and provides more pre-trained models, while [`burn`][c~burn~docs]↗{{hi:burn}} is more focused on supporting the whole workflow from training to inference. `candle` can use NVIDIA GPUs through CUDA (and cuDNN), and Apple GPUs through Metal.

## Choosing a Deep Learning Crate {#choosing}

This snapshot was checked against current crate documentation and release records on 2026-10-26. Optional backends and native runtime requirements vary by feature and platform.

| Crate                                 | Training                        | Inference                        | GPU backends / role                                 | Maintenance and notes                                      |
| ------------------------------------- | ------------------------------- | -------------------------------- | --------------------------------------------------- | ---------------------------------------------------------- |
| [`candle`][c~candle-core~repo]↗       | Basic                           | Yes; includes pre-trained models | CUDA, Metal                                         | Active; `candle-core` 0.11.0 (2026-06-26).                 |
| [`burn`][c~burn~repo]↗                | Yes                             | Yes                              | wgpu, CUDA, ROCm, Metal, Vulkan                     | Active; 0.22.1 (2026-10-20).                               |
| [`tch`][c~tch~repo]↗                  | Yes                             | Yes                              | CUDA, Apple MPS, Vulkan                             | Active; 0.26.0; bindings to the C++ PyTorch library.       |
| [`ort`][c~ort~repo]↗                  | Yes; ONNX Runtime training APIs | Yes; ONNX models                 | CUDA, TensorRT, CoreML, and other providers         | Active; 2.0.0-rc.15 (2026-10-19), a release candidate.     |
| [`tract`][c~tract~repo]↗              | No                              | Yes; ONNX and NNEF               | CPU; optional CUDA, Metal, and WebGPU               | Active; 0.23.9 (2026-10-15); Rust inference engine.        |
| [`rust-bert`][c~rust-bert~repo]↗      | Custom training through `tch`   | Yes; ready-to-use NLP pipelines  | Through LibTorch or ONNX Runtime                    | No recent release; 0.23.0 (2024-09-29).                    |
| [`tensorflow`][tensorflow-rust~repo]↗ | Yes                             | Yes                              | CUDA via the TensorFlow runtime                     | Archived upstream on 2026-03-17; consider for legacy work. |
| [`cudarc`][c~cudarc~repo]↗            | N/A                             | N/A                              | CUDA driver/runtime bindings; not a model framework | Active; current docs list 0.19.10.                         |
| [`dfdx`][c~dfdx~repo]↗                | Yes                             | Yes                              | CPU and CUDA                                        | Inactive; latest release 0.13.0 (2023-07-27).              |

Train with `burn`, `candle`, or `tch`, or train elsewhere and run an exported model with `ort` or `tract`. For a quick reference, use `candle`'s CPU support; GPU execution depends on the selected backend and its native runtime.

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
