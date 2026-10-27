<!-- TODO Reviewed ??-??-2026 -->

# Market Data and Brokers

{{#include market_data.incl.md}}

Market data is licensed. Check the terms of your provider before storing or redistributing it (the unofficial Yahoo! Finance API is for personal use). Use rate limits, and paper trading before any live broker: crates such as [`ibapi`][c~ibapi~docs]↗ (Interactive Brokers) are mentioned for reference only and have no recipe here.

## Daily Price History {#yahoo-history}

[![yahoo_finance_api][c~yahoo_finance_api~docs~badge]][c~yahoo_finance_api~docs] [![yahoo_finance_api~crates.io][c~yahoo_finance_api~crates.io~badge]][c~yahoo_finance_api~crates.io] [![yahoo_finance_api~repo][c~yahoo_finance_api~repo~badge]][c~yahoo_finance_api~repo] [![yahoo_finance_api~lib.rs][c~yahoo_finance_api~lib.rs~badge]][c~yahoo_finance_api~lib.rs]{{hi:yahoo_finance_api}} [![cat~finance][cat~finance~badge]][cat~finance]

[`yahoo_finance_api`][c~yahoo_finance_api~docs]↗{{hi:yahoo_finance_api}} fetches OHLCV history. [`governor`][c~governor~docs]↗{{hi:governor}} limits the request rate. The test parses a saved JSON response, so it runs offline; the live fetch is `#[ignore]`d.

```rust,editable
{{#include ../../../crates/cats/finance/examples/market_data/yahoo_history.rs:example}}
```

## Related Topics {.skip}

- [[quant | Quantitative Analysis]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [examples](https://github.com/john-cd/rust_howto/issues/1949)
</div>
