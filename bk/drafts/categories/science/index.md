# Science

[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

Solving problems involving physics, chemistry, biology, machine learning, geoscience, and other scientific fields.

## Machine Learning

{{#include _machine_learning.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write P1]https://github.com/john-cd/rust_howto/issues/472

[Why Rust is becoming a contender in AI development][why rust is becoming a contender in ai development]

[why rust is becoming a contender in ai development]: <https://www.analyticsinsight.net/artificial-intelligence/why-rust-is-becoming-a-contender-in-ai-development>

## Rust Crates for Machine Learning

The Rust ML ecosystem is still evolving, but several promising crates are available. Note that some areas might have fewer mature options compared to Python's ecosystem.

| Topic | Rust Crates Examples | Notes |
|---|---|---|
| General Purpose ML Frameworks | [`linfa`][c-linfa]⮳{{hi:linfa}}, [`tch-rs`][c-tch]⮳{{hi:tch-rs}} bindings to PyTorch | [`linfa`][c-linfa]⮳{{hi:linfa}} aims to be a comprehensive ML library. [`tch-rs`][c-tch]⮳{{hi:tch-rs}} provides Rust bindings to the popular PyTorch framework. |
| Linear Algebra & Numerical Computation | [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}, [`ndarray`][c-ndarray]⮳{{hi:ndarray}}, [`matrix`][c-matrix]⮳{{hi:matrix}} | These crates provide efficient array and matrix operations, essential for many ML algorithms. |
| Data Manipulation & Preprocessing | [`polars`][c-polars]⮳{{hi:polars}}, [`dataframe`][c-dataframe]⮳{{hi:dataframe}} | [`polars`][c-polars]⮳{{hi:polars}} and [`dataframe`][c-dataframe]⮳{{hi:dataframe}} are designed for efficient data manipulation and analysis, similar to Pandas in Python. |
| Machine Learning Algorithms | [`linfa`][c-linfa]⮳{{hi:linfa}}, [`smartcore`][c-smartcore]⮳{{hi:smartcore}} | [`linfa`][c-linfa]⮳{{hi:linfa}} implements various ML algorithms. [`smartcore`][c-smartcore]⮳{{hi:smartcore}} is another option that provides implementations of common algorithms. |
| Deep Learning | [`tch-rs`][c-tch]⮳{{hi:tch-rs}}, [`burn`][c-burn]⮳{{hi:burn}} | [`tch-rs`][c-tch]⮳{{hi:tch-rs}} provides access to PyTorch, enabling deep learning models. [`burn`][c-burn]⮳{{hi:burn}} is a new deep learning framework written in Rust. |
| Model Training & Evaluation | Often integrated into ML frameworks | Many ML frameworks handle training loops and evaluation metrics. |
| Data Visualization | [`plotters`][c-plotters]⮳{{hi:plotters}}, [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}} | While not strictly ML-specific, these crates are essential for visualizing data and model performance. |
| Natural Language Processing NLP | [`tokenizers`][c-tokenizers]⮳{{hi:tokenizers}}, [`rust-bert`][c-rust_bert]⮳{{hi:rust-bert}} | [`tokenizers`][c-tokenizers]⮳{{hi:tokenizers}} provides fast tokenization, and [`rust-bert`][c-rust_bert]⮳{{hi:rust-bert}} offers pre-trained BERT models. |
| Computer Vision | [`image`][c-image]⮳{{hi:image}}, [`opencv-rs`][c-opencv]⮳{{hi:opencv-rs}} bindings to OpenCV | [`image`][c-image]⮳{{hi:image}} is for image processing. [`opencv-rs`][c-opencv]⮳{{hi:opencv-rs}} provides bindings to the popular OpenCV library. |
| Reinforcement Learning | [`rl-rs`][c-rl]⮳{{hi:rl-rs}}, [`gym-rs`][c-gym]⮳{{hi:gym-rs}} | [`rl-rs`][c-rl]⮳{{hi:rl-rs}} is a reinforcement learning library, and [`gym-rs`][c-gym]⮳{{hi:gym-rs}} provides bindings to the OpenAI Gym environment. |
| ONNX Runtime | [`onnxruntime`][c-onnxruntime]⮳{{hi:onnxruntime}} | Allows running models in the ONNX format. |
| Serving/Deployment | Still developing | This area is still under heavy development in the Rust ML ecosystem. |

## Key Considerations

* Maturity: The Rust ML ecosystem is relatively young compared to Python's. Some areas may have fewer mature options or require more manual implementation.
* Performance: Rust's performance can be a significant advantage for ML tasks, especially when dealing with large datasets or computationally intensive models.
* Interoperability: Crates like [`tch-rs`][c-tch]⮳{{hi:tch-rs}} demonstrate a path toward leveraging existing frameworks like PyTorch from Rust. FFI Foreign Function Interface can be used to integrate with other languages.
* Community: The Rust ML community is growing, and more resources and libraries are becoming available.

[Clustering](https://www.arewelearningyet.com/clustering/)
[Data Processing](https://www.arewelearningyet.com/data-preprocessing/)
[Data Structures](https://www.arewelearningyet.com/data-structures/)
[Decision Trees](https://www.arewelearningyet.com/decision-trees/)
[GPU Computing](https://www.arewelearningyet.com/gpu-computing/)
[Linear Classifiers](https://www.arewelearningyet.com/linear-classifiers/)
[Metaheuristics](https://www.arewelearningyet.com/metaheuristics/)
[MLOps](https://www.arewelearningyet.com/mlops/)
[Neural Networks](https://www.arewelearningyet.com/neural-networks/)
[Natural Language Processing](https://www.arewelearningyet.com/nlp/)
[Reinforcement learning](https://www.arewelearningyet.com/reinforcement/)
[Scientific Computing](https://www.arewelearningyet.com/scientific-computing/)
</div>
