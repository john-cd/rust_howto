# Science

[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Solving problems involving physics, chemistry, biology, machine learning, geoscience, and other scientific fields.

## Machine Learning

The Rust ML ecosystem is relatively young compared to Python's. However, Rust's performance can be a significant advantage for ML tasks, especially when dealing with large datasets or computationally intensive models.

| Topic | Rust Crate(s) | Notes |
|---|---|---|
| Classical Machine Learning Algorithms | [`linfa`][c~linfa~docs]↗{{hi:linfa}}, [`smartcore`][c~smartcore~docs]↗{{hi:smartcore}} | [`linfa`][c~linfa~docs]↗{{hi:linfa}} implements various ML algorithms. [`smartcore`][c~smartcore~docs]↗{{hi:smartcore}} is another option that provides implementations of common algorithms. |
| Deep Learning | [`tch-rs`][c~tch~docs]↗{{hi:tch-rs}}, [`burn`][c~burn~docs]↗{{hi:burn}} | [`tch-rs`][c~tch~docs]↗{{hi:tch-rs}} provides access to PyTorch, enabling deep learning models. [`burn`][c~burn~docs]↗{{hi:burn}} is a new deep learning framework written in Rust. |
| Natural Language Processing NLP | [`tokenizers`][c~tokenizers~docs]↗{{hi:tokenizers}}, [`rust-bert`][c~rust-bert~docs]↗{{hi:rust-bert}} | [`tokenizers`][c~tokenizers~docs]↗{{hi:tokenizers}} provides fast tokenization, and [`rust-bert`][c~rust-bert~docs]↗{{hi:rust-bert}} offers pre-trained BERT models. |
| Reinforcement Learning | [`rl-rs`][c~rl~docs]↗{{hi:rl-rs}}, [`gym-rs`][c~gym~docs]↗{{hi:gym-rs}} | [`rl-rs`][c~rl~docs]↗{{hi:rl-rs}} is a reinforcement learning library, and [`gym-rs`][c~gym~docs]↗{{hi:gym-rs}} provides bindings to the OpenAI Gym environment. |
| ONNX Runtime | [`onnxruntime`][c~onnxruntime~docs]↗{{hi:onnxruntime}} | Allows running models in the ONNX format. |

## Classical Machine Learning

{{#include classical_machine_learning.incl.md}}

## Neural Networks and Deep Learning

{{#include deep_learning.incl.md}}

## Related Topics

| Topic | Rust Crate(s) | Notes |
|---|---|---|
| [[computer-vision | Computer Vision]] | [`image`][c~image~docs]↗{{hi:image}}, [`opencv-rs`][c~opencv~docs]↗{{hi:opencv-rs}} bindings to OpenCV | [`image`][c~image~docs]↗{{hi:image}} is for image manipulation. [`opencv-rs`][c~opencv~docs]↗{{hi:opencv-rs}} provides bindings to the popular OpenCV library. |
| Data Manipulation & [[data-processing | Processing]]  | [`polars`][c~polars~docs]↗{{hi:polars}}, [`dataframe`][c~dataframe~docs]↗{{hi:dataframe}} | [`polars`][c~polars~docs]↗{{hi:polars}} and [`dataframe`][c~dataframe~docs]↗{{hi:dataframe}} are designed for efficient data manipulation and analysis, similar to Pandas in Python. |
| Data [[visualization | Visualization]] | [`plotters`][c~plotters~docs]↗{{hi:plotters}}, [`iced`][c~iced~docs]↗{{hi:iced}}, [`egui`][c~egui~docs]↗{{hi:egui}} | While not strictly ML-specific, these crates are essential for visualizing data and model performance. |
| [[linear_algebra | Linear Algebra]] & Numerical Computation | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}, [`matrix`][c~matrix~docs]↗{{hi:matrix}} | These crates provide efficient array and matrix operations, essential for many ML algorithms. |

[[development-tools_ffi | FFI]] (Foreign Function Interface) can be used to integrate with ML frameworks written in other languages.

## References

- [Why Rust is becoming a contender in AI development][blog~why-rust-is-becoming-a-contender-in-ai-development]↗.
- [Choosing the Right Rust Machine Learning Framework][blog~choosing-the-right-machine-learning-framework]↗.
- [ML in Rust - Ranking][ml-in-rust-ossinsight~website]↗.
- [Data Processing][are-we-learning-yet?-data-preprocessing~website]↗.
- [Data Structures][are-we-learning-yet?-data-structures~website]↗.
- [GPU Computing][are-we-learning-yet?-gpu-computing~website]↗.
- [Metaheuristics][are-we-learning-yet?-metaheuristics~website]↗.
- [MLOps][are-we-learning-yet?-mlops~website]↗.
- [Natural Language Processing][are-we-learning-yet?-nlp~website]↗.
- [Reinforcement learning][are-we-learning-yet?-reinforcement~website]↗.
- [Scientific Computing][are-we-learning-yet?-scientific-computing~website]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/472)
Review tch-rs, burn, ort, rust-bert, TensorFlow/rust, tract, cudarc, DFDX.

- [Awesome-Rust-Neural-Network][awesome-rust-neural-network~repo]↗: A curated collection of Rust projects related to neural networks, designed to complement "Are We Learning Yet?".
- [`std::autodiff`][c~std::autodiff~docs]↗.

[`burn.dev`][burn~website]↗.

</div>
