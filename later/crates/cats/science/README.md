<!-- TODO Reviewed ??-??-2026. -->

# Science

Scientific computing and machine learning examples using Ndarray, Linfa, and Candle. It demonstrates integration with key ecosystem crates including `candle-core`, `candle-nn`, `clap`, `linfa`, `linfa-clustering`, `linfa-linear`.

## Examples Covered

- **Ml** (`examples/ml/`): Demonstrates concepts like `candle`, `kmeans`, `linfa`, `smartcore`.

- **Feature pipeline** (`examples/feature_pipeline.rs`): Fits training-only scaling and measures held-out regression against a baseline.
- **Text features** (`examples/text_features.rs`): Fits a fixed vocabulary and ranks count vectors with cosine similarity.

## Reproduce the CPU Recipes

From the repository root:

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p science --features candle --examples
just cargo run --manifest-path later/crates/Cargo.toml --locked -p science --example feature_pipeline
just cargo run --manifest-path later/crates/Cargo.toml --locked -p science --example text_features
```

The `candle` feature adds CPU training and a temporary checkpoint round trip; it does
not validate an accelerator. See the [Machine Learning in Practice chapter](../../../../bk/src/other/machine-learning/index.md).

## Usage

To check or run examples in this crate:

```console
cargo check --package science --examples
cargo test --package science
```

Code examples correspond to chapters in `later/src/categories/science` or related book sections.
