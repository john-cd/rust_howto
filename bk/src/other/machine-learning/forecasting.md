<!-- TODO Reviewed ??-??-2026. -->

# Forecasting

{{#include forecasting.incl.md}}

## Forecast a Seasonal Series with `augurs` {#augurs}

{{#badges augurs science=Science}}

[`augurs`][c~augurs~docs]↗{{hi:augurs}} is a time-series toolkit written in Rust. The recipe uses MSTL (multiple seasonal-trend decomposition) to remove the weekly pattern and an exponential smoothing (ETS) model for the trend, then forecasts two weeks with a 95% interval. The crate's ETS models are not seasonal themselves, which is why the seasonality is handled by MSTL. The model fits the first 56 daily observations and forecasts the following 14, which are held out of training. The example compares their mean absolute error with a seasonal-naive baseline that repeats the last training week. This is a deterministic synthetic smoke check, not a production forecasting benchmark.

From the repository root:

```sh
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p science --example machine_learning -- forecast
```

For real observations, preserve chronological order, define how missing timestamps are handled, and evaluate rolling forecast origins before selecting a model. The example checks finite interval bounds and their ordering around each predicted point; that does not establish 95% coverage of future observations.

```rust,ignore
{{#include ../../../crates/other/machine_learning/examples/machine_learning/forecast.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
