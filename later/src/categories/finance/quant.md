<!-- TODO Reviewed ??-??-2026 -->

# Quantitative Financial Analysis

{{#include quant.incl.md}}

## `RustQuant` {#rustquant}

[![rustquant][c~RustQuant~docs~badge]][c~RustQuant~docs] [![rustquant~crates.io][c~RustQuant~crates.io~badge]][c~RustQuant~crates.io] [![rustquant~repo][c~RustQuant~repo~badge]][c~RustQuant~repo] [![rustquant~lib.rs][c~RustQuant~lib.rs~badge]][c~RustQuant~lib.rs]{{hi:rustquant}}{{hi:Finance}}{{hi:Monte-Carlo}}{{hi:Option-pricing}}{{hi:Quantitative}}{{hi:Quantlib}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~simulation][cat~simulation~badge]][cat~simulation]{{hi:Simulation}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~finance][cat~finance~badge]][cat~finance]

[`RustQuant`][c~RustQuant~lib.rs]↗ is a Rust library for quantitative [finance][p~finance].

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

## Trading Calendars {#trading-days}

[![trading-calendar][c~trading-calendar~docs~badge]][c~trading-calendar~docs] [![trading-calendar~crates.io][c~trading-calendar~crates.io~badge]][c~trading-calendar~crates.io] [![trading-calendar~repo][c~trading-calendar~repo~badge]][c~trading-calendar~repo] [![trading-calendar~lib.rs][c~trading-calendar~lib.rs~badge]][c~trading-calendar~lib.rs]{{hi:trading-calendar}} [![cat~finance][cat~finance~badge]][cat~finance]

[`trading-calendar`][c~trading-calendar~docs]↗{{hi:trading-calendar}} knows the holidays, early closes and hours of major exchanges (NYSE, NASDAQ, LSE, TSE, TSX). The recipe counts NYSE trading days across Thanksgiving and adds a number of trading days to a date. The crate supports a limited range of years: check its documentation.

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
  [quant: write](https://github.com/john-cd/rust_howto/issues/364)
  [examples](https://github.com/john-cd/rust_howto/issues/1949)
</div>
