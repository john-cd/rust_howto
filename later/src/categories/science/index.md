<!-- TODO Reviewed ??-??-2026 -->

# Science

[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

This section collects Rust crates and examples for scientific computing, machine learning, geospatial data, and neuroscience. Compare runtime and accelerator requirements, model or data compatibility, and project activity before choosing a crate.

## Classical Machine Learning

The [Classical Machine Learning][p~classical-machine-learning] chapter covers common algorithms and links to runnable examples.

{{#include classical_machine_learning.incl.md}}

## Neural Networks and Deep Learning

The [Deep Learning][p~deep-learning] chapter covers model training and inference. Its [crate comparison table][p~deep-learning-comparison] summarizes framework support, accelerator backends, and recent project activity.

{{#include deep_learning.incl.md}}

## Related Science Categories

- [Geoscience][p~science-geo] covers geographic data, coordinate systems, and spatial analysis.
- [Biology and Neuroscience][p~science-neuroscience] includes biological formats and a runnable synthetic neuroimaging example.
- [Robotics][p~science-robotics] collects robotics systems, tools, and examples.

## Related Topics

For complementary methods and tools, see [[computer-vision | Computer Vision]], [[data-processing | Data Processing]], [[visualization | Visualization]], and [[linear_algebra | Linear Algebra]].

## Further Reading

Rust's experimental [`std::autodiff`][c~std::autodiff~docs]↗ is a nightly, Enzyme-based feature tracked in [rust-lang/rust#124509][rust-autodiff~tracking-issue].

Learn more: [Awesome Rust Neural Network][awesome-rust-neural-network~project-list]↗ is a curated list of Rust neural-network projects.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
