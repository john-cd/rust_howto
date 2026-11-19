<!-- TODO Reviewed ??-??-2026. -->

# Science

[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Start with the quantity you want to measure or predict. Record the units,
coordinate system, and sampling assumptions before choosing a numerical method.

## Work with Measurements and Models

1. [Compare route measurements with a model](route_measurements.md). Measure a
   synthetic route and check the assumptions behind a time prediction.
2. [Simulate a process](https://github.com/john-cd/rust_howto/blob/develop/bk/src/categories/simulation/simulation.md) when the state evolves
   over time or events change the model's state.
3. Use [Geo](../science_geo/geo.md) for geographic coordinates or
   [sampled signal analysis](../science_neuroscience/neuroscience.md#fft-signal)
   for measurements recorded at a known rate.
4. Use [Machine Learning in Practice](https://john-cd.com/rust_howto/other/machine-learning/index.html) to choose a model,
   evaluate predictions, or run a trained model in Rust.

## Related Science Categories

- [Geoscience][p~science-geo] covers geographic data, coordinate systems, and spatial analysis.
- [Biology and Neuroscience][p~science-neuroscience] includes biological formats and a runnable synthetic neuroimaging example.
- [Robotics][p~science-robotics] collects robotics systems, tools, and examples.

## Related Topics

For complementary methods and tools, see [[computer-vision | Computer Vision]], [[data-processing | Data Processing]], [[visualization | Visualization]], and [[linear_algebra | Linear Algebra]].

## Further Reading

Rust's experimental [`std::autodiff`][c~std::autodiff~docs]↗ is a nightly, Enzyme-based feature tracked in [rust-lang/rust#124509][rust-autodiff~tracking-issue].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
