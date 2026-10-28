<!-- TODO Reviewed ??-??-2026 -->

# Market Data and Brokers

{{#include market_data.incl.md}}

Market data is licensed. Check provider terms, account entitlements, request limits, and rights to store or redistribute data before using it. Keep API credentials out of source control, and use a paper account before sending live orders.

## Provider Clients

- [`yahoo_finance_api`][c~yahoo_finance_api~docs]↗ is an asynchronous client for Yahoo Finance's unofficial API. The crate documents the API as intended for personal use; endpoints, availability, and data rights can change. The example below uses a rate limiter and parses a saved response in its offline test.
- [`databento`][c~databento~docs]↗ is Databento's official Rust client for historical and live market data. Access depends on an API key and the datasets enabled for the account.
- [`apca`][c~apca~docs]↗ is an asynchronous Rust client for Alpaca trading and market-data APIs. Review its current maintenance and GPL-3.0-or-later license before choosing it for an application; also confirm which Alpaca data feeds and account features are available to you.

These clients connect to different providers and do not make their data interchangeable. Choose a source based on the market, feed, latency, retention, licensing, and cost your application requires. For spreadsheet exchange, see [[excel | Excel Spreadsheets]].

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
