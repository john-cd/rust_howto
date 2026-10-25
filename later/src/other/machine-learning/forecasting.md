<!-- TODO Reviewed ??-??-2026 -->
# Forecasting

{{#include forecasting.incl.md}}

## Forecast a Seasonal Series with `augurs` {#augurs}

[![augurs][c~augurs~docs~badge]][c~augurs~docs] [![augurs~crates.io][c~augurs~crates.io~badge]][c~augurs~crates.io] [![augurs~repo][c~augurs~repo~badge]][c~augurs~repo] [![augurs~lib.rs][c~augurs~lib.rs~badge]][c~augurs~lib.rs]{{hi:augurs}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

[`augurs`][c~augurs~docs]↗{{hi:augurs}} is a time-series toolkit written in Rust. The recipe uses MSTL (multiple seasonal-trend decomposition) to remove the weekly pattern and an exponential smoothing (ETS) model for the trend, then forecasts two weeks with a 95% interval. The crate's ETS models are not seasonal themselves, which is why the seasonality is handled by MSTL. The crate's API is still evolving: pin the version.

`augurs` also provides a Prophet implementation (running Stan compiled to WebAssembly, so that no C++ toolchain is needed), which is not covered here.

```rust,editable
{{#include ../../../crates/other/examples/machine_learning/forecast.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ML examples](https://github.com/john-cd/rust_howto/issues/1929)
</div>
