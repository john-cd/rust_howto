<!-- TODO Reviewed ??-??-2026. -->

# Resolve Daylight Saving Time with Jiff

A civil date and time is a wall-clock label. It is not an instant until you combine it with a time zone and resolve any daylight-saving transition. In New York, `01:30` occurs twice on November 3, 2024, while `02:30` does not occur on March 10, 2024. This recipe selects the earlier and later occurrences of the first, then moves the nonexistent time forward explicitly.

```rust
{{#include ../../../crates/cats/date_and_time/examples/jiff_dst.rs:example}}
```

Run `cargo test -p date_and_time --example jiff_dst` from `bk/crates`. The fixture uses Jiff's bundled IANA time-zone database, so it needs no service or machine-specific zoneinfo. The assertions check both local time and UTC offset. Save an instant (or a zoned value with its offset) after disambiguating a scheduled event. Revisit future appointments when time-zone rules change.

Adding one calendar day preserves the intended wall-clock time across the spring transition; adding 24 elapsed hours lands at a different hour. The existing [Chrono examples](https://github.com/john-cd/rust_howto/tree/develop/bk/crates/cats/date_and_time/examples/chrono) and [time example](https://github.com/john-cd/rust_howto/blob/develop/bk/crates/cats/date_and_time/examples/time.rs) show other date-time APIs. See [Jiff's civil-time guide](https://docs.rs/jiff/latest/jiff/civil/) for the ambiguity rules and [time-zone database documentation](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZoneDatabase.html) for deployment choices.
