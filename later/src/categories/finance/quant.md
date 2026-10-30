<!-- TODO Reviewed ??-??-2026 -->

# Quantitative Financial Analysis

{{#include quant.incl.md}}

## Time Value of Money

The [`financial`][c~financial~docs]↗ crate provides time-value-of-money functions, including net present value, internal rate of return, present value, and future value. Its `npv` function discounts the first entry by one period, like Excel's `NPV`. Keep a time-zero initial investment separate from the future cash flows. The example checks this convention against direct discounting:

```rust,editable
{{#include ../../../crates/cats/finance/examples/quant/net_present_value.rs:example}}
```

The crate does not currently provide every Excel financial function; for example, its documentation lists PMT as future work. Check the implemented function set rather than assuming complete spreadsheet compatibility.

## Simulate an Option Payoff

Use [`rand`][c~rand~docs] for a random generator and [`rand_distr`][c~rand_distr~docs] for a specified probability distribution. This fixed-seed example draws standard-normal shocks for a one-year geometric Brownian motion model, averages European call payoffs, and discounts that average. The tests check the zero-volatility limit and compare a seeded estimate with the analytic Black-Scholes price.

```rust,editable
{{#include ../../../crates/cats/finance/examples/quant/monte_carlo.rs:example}}
```

The seed makes this example reproducible with its pinned dependency versions; `StdRng` does not promise identical output across releases. A finite Monte Carlo estimate has sampling error, and a model's assumptions still need separate validation. Use `f64` for model calculations; use decimal arithmetic for exact account balances and settled money amounts.

## Stochastic Models and Technical Indicators

For advanced stochastic processes, [`stochastic-rs`][c~stochastic-rs~docs]↗ covers simulation and quantitative-finance models. It is a broad, feature-rich library; review its optional numerical backends and platform requirements before adding it to a project.

For common technical-analysis indicators and custom indicator implementations, see [`yata`][c~yata~docs]↗.

## `RustQuant` {#rustquant}

[![rustquant][c~RustQuant~docs~badge]][c~RustQuant~docs] [![rustquant~crates.io][c~RustQuant~crates.io~badge]][c~RustQuant~crates.io] [![rustquant~repo][c~RustQuant~repo~badge]][c~RustQuant~repo] [![rustquant~lib.rs][c~RustQuant~lib.rs~badge]][c~RustQuant~lib.rs]{{hi:rustquant}}{{hi:Finance}}{{hi:Monte-Carlo}}{{hi:Option-pricing}}{{hi:Quantitative}}{{hi:Quantlib}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~simulation][cat~simulation~badge]][cat~simulation]{{hi:Simulation}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~finance][cat~finance~badge]][cat~finance]

[`RustQuant`][c~RustQuant~lib.rs]↗ is a Rust library for quantitative [finance][p~finance]. Its umbrella crate brings in a substantial dependency tree, so this workspace does not compile it. The following standalone Black-Scholes example uses no RustQuant APIs; it relies on [`statrs`][c~statrs~docs]↗ for the normal distribution; the next section shows a smaller dedicated option-pricing crate.

```rust,editable
{{#include ../../../crates/cats/finance/examples/quant/rustquant.rs:example}}
```

## Option Pricing and Implied Volatility {#black-scholes}

[![black_scholes][c~black_scholes~docs~badge]][c~black_scholes~docs] [![black_scholes~crates.io][c~black_scholes~crates.io~badge]][c~black_scholes~crates.io] [![black_scholes~repo][c~black_scholes~repo~badge]][c~black_scholes~repo] [![black_scholes~lib.rs][c~black_scholes~lib.rs~badge]][c~black_scholes~lib.rs]{{hi:black_scholes}} [![cat~finance][cat~finance~badge]][cat~finance]

[`black_scholes`][c~black_scholes~docs]↗{{hi:black_scholes}} prices European options and computes the Greeks. `call_iv` and `put_iv` solve for the implied volatility of an observed price. The recipe checks put-call parity and that price -> volatility -> price round-trips.

Use `f64` for statistics and model inputs. For money amounts (cash balances, order prices), use a decimal type such as [`rust_decimal`][c~rust_decimal~docs]↗ instead.

```rust,editable
{{#include ../../../crates/cats/finance/examples/quant/black_scholes.rs:example}}
```

## Rolling Statistics {#rolling-stats}

[![ta-statistics][c~ta-statistics~docs~badge]][c~ta-statistics~docs] [![ta-statistics~crates.io][c~ta-statistics~crates.io~badge]][c~ta-statistics~crates.io] [![ta-statistics~repo][c~ta-statistics~repo~badge]][c~ta-statistics~repo] [![ta-statistics~lib.rs][c~ta-statistics~lib.rs~badge]][c~ta-statistics~lib.rs]{{hi:ta-statistics}} [![cat~finance][cat~finance~badge]][cat~finance]

[`ta-statistics`][c~ta-statistics~docs]↗{{hi:ta-statistics}} computes rolling-window statistics (mean, standard deviation, z-score, drawdown, correlation, beta...) without allocating per update. The recipe computes daily returns and a 20-day rolling standard deviation, annualizes it with the square root of 252, and checks it against a hand-written version.

```rust,editable
{{#include ../../../crates/cats/finance/examples/quant/rolling_stats.rs:example}}
```

## Trading Days {#trading-days}

This example uses [`trading-calendar`][c~trading-calendar~docs]↗{{hi:trading-calendar}} for the NYSE holiday schedule and [`chrono`][c~chrono~docs]↗{{hi:chrono}} for the date arithmetic: it counts trading days in a range and steps forward by a number of trading days. Verify the crate's holiday and early-close data against the exchange's published calendar before relying on it in production.

```rust,editable
{{#include ../../../crates/cats/finance/examples/quant/trading_days.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[mathematics | Mathematics]].
- [[linear_algebra | Linear Algebra]].
- [[statistics | Statistics]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [examples](https://github.com/john-cd/rust_howto/issues/1949)
</div>
