// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fetch daily bars from Yahoo! Finance under a rate limit.
//!
//! Respect the data provider's terms of use: Yahoo! Finance's public API is
//! unofficial and intended for personal use.

use std::num::NonZeroU32;

use governor::Quota;
use governor::RateLimiter;
use yahoo_finance_api::Quote;
use yahoo_finance_api::YResponse;
use yahoo_finance_api::YahooConnector;

/// Converts a Yahoo! chart response into daily bars.
fn parse_bars(json: serde_json::Value) -> anyhow::Result<Vec<Quote>> {
    Ok(YResponse::from_json(json)?.quotes()?)
}

/// Fetches one year of daily bars per ticker, at most two requests per
/// second.
async fn fetch(tickers: &[&str]) -> anyhow::Result<()> {
    let limiter = RateLimiter::direct(Quota::per_second(
        NonZeroU32::new(2).expect("non-zero"),
    ));
    let provider = YahooConnector::new()?;
    for ticker in tickers {
        limiter.until_ready().await;
        let response = provider.get_quote_range(ticker, "1d", "1y").await?;
        let quotes = response.quotes()?;
        println!("{ticker}: {} daily bars", quotes.len());
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(fetch(&["AAPL", "MSFT"]))
}
// ANCHOR_END: example

/// Runs the example (requires network access).
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A saved (abridged) chart response.
    const SAVED: &str = r#"{
      "chart": {
        "result": [{
          "meta": {
            "currency": "USD", "symbol": "AAPL", "exchangeName": "NMS",
            "instrumentType": "EQUITY", "fullExchangeName": "NasdaqGS",
            "hasPrePostMarketData": true, "firstTradeDate": 345479400,
            "regularMarketTime": 1700000000, "gmtoffset": -18000,
            "timezone": "EST", "exchangeTimezoneName": "America/New_York",
            "regularMarketPrice": 190.0, "chartPreviousClose": 189.0,
            "priceHint": 2, "dataGranularity": "1d", "range": "5d"
          },
          "timestamp": [1699972200, 1700058600],
          "indicators": {
            "quote": [{
              "open": [187.0, 189.5], "high": [190.0, 191.0],
              "low": [186.5, 188.9], "close": [189.0, 190.0],
              "volume": [50000000, 48000000]
            }],
            "adjclose": [{ "adjclose": [188.9, 189.9] }]
          }
        }],
        "error": null
      }
    }"#;

    #[test]
    fn parses_a_saved_response() {
        let json: serde_json::Value =
            serde_json::from_str(SAVED).expect("valid JSON");
        let bars = parse_bars(json).expect("parses");
        assert_eq!(bars.len(), 2);
        assert!(bars[0].high >= bars[0].low);
    }

    #[test]
    #[ignore = "needs network access"]
    fn fetches_history() {
        main().expect("network available");
    }
}
