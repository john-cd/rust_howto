# Science

[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

Solving problems involving physics, chemistry, biology, machine learning, geoscience, and other scientific fields.

## Machine Learning

{{#include _machine_learning.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[science/_index: write P1]https://github.com/john-cd/rust_howto/issues/472

[Why Rust is becoming a contender in AI development][why rust is becoming a contender in ai development]

[why rust is becoming a contender in ai development]: <https://www.analyticsinsight.net/artificial-intelligence/why-rust-is-becoming-a-contender-in-ai-development>

## Rust Crates for Machine Learning

The Rust ML ecosystem is still evolving, but several promising crates are available. Note that some areas might have fewer mature options compared to Python's ecosystem.

| Topic | Rust Crates Examples | Notes |
|---|---|---|
| General Purpose ML Frameworks | `linfa`, `tch-rs` bindings to PyTorch | `linfa` aims to be a comprehensive ML library. `tch-rs` provides Rust bindings to the popular PyTorch framework. |
| Linear Algebra & Numerical Computation | `nalgebra`, `ndarray`, `matrix` | These crates provide efficient array and matrix operations, essential for many ML algorithms. |
| Data Manipulation & Preprocessing | `polars`, `dataframe` | `polars` and `dataframe` are designed for efficient data manipulation and analysis, similar to Pandas in Python. |
| Machine Learning Algorithms | `linfa`, `smartcore` | `linfa` implements various ML algorithms. `smartcore` is another option that provides implementations of common algorithms. |
| Deep Learning | `tch-rs`, `burn` | `tch-rs` provides access to PyTorch, enabling deep learning models. `burn` is a new deep learning framework written in Rust. |
| Model Training & Evaluation | Often integrated into ML frameworks | Many ML frameworks handle training loops and evaluation metrics. |
| Data Visualization | `plotters`, `iced`, `egui` | While not strictly ML-specific, these crates are essential for visualizing data and model performance. |
| Natural Language Processing NLP | `tokenizers`, `rust-bert` | `tokenizers` provides fast tokenization, and `rust-bert` offers pre-trained BERT models. |
| Computer Vision | `image`, `opencv-rs` bindings to OpenCV | `image` is for image processing. `opencv-rs` provides bindings to the popular OpenCV library. |
| Reinforcement Learning | `rl-rs`, `gym-rs` | `rl-rs` is a reinforcement learning library, and `gym-rs` provides bindings to the OpenAI Gym environment. |
| ONNX Runtime | `onnxruntime` | Allows running models in the ONNX format. |
| Serving/Deployment | Still developing | This area is still under heavy development in the Rust ML ecosystem. |

## Key Considerations

* Maturity: The Rust ML ecosystem is relatively young compared to Python's. Some areas may have fewer mature options or require more manual implementation.
* Performance: Rust's performance can be a significant advantage for ML tasks, especially when dealing with large datasets or computationally intensive models.
* Interoperability: Crates like `tch-rs` demonstrate a path toward leveraging existing frameworks like PyTorch from Rust. FFI Foreign Function Interface can be used to integrate with other languages.
* Community: The Rust ML community is growing, and more resources and libraries are becoming available.

</div>
