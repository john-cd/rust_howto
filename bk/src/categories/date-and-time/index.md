<!-- TODO Reviewed ??-??-2026. -->

# Date and Time

[![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}

For new code, start with [Jiff](#using-jiff) when you need named time zones and calendar arithmetic. Choose [`time`](time_crate.md) when fixed offsets, format descriptions, or `no_std` support fit the application.

[`chrono`][c~chrono~docs]↗{{hi:chrono}} is widely used and is soft-deprecated, as noted by the [Blessed.rs crate directory](https://blessed.rs/crates). The Chrono examples remain useful for existing code and for dependencies that expose Chrono types.

| Topic | Rust Crates |
| --- | --- |
| Date and Time Types | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Named Time Zones | [`chrono-tz`][c~chrono-tz~docs]↗{{hi:chrono-tz}} supplies IANA time-zone data for `chrono`; `time` supports UTC offsets and can optionally query the system's local offset. |
| Formatting and Parsing | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Human-Readable Durations | [Jiff's `SignedDuration`](#using-jiff) parses human-readable elapsed time. [`humantime`][ex~date-and-time~humantime] is documented for existing code. |
| Durations and Time Intervals | [`std::time::Duration`][c~std::time::Duration~docs]↗{{hi:std::time::Duration}}, [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Elapsed-Time Measurement | [`std::time::Instant`][c~std::time::Instant~docs]↗{{hi:std::time::Instant}} is the baseline; [`measure_time`][c~measure_time~docs]↗{{hi:measure_time}} adds scope-based logging and printing. |

## Duration and Calculation

{{#include duration.incl.md}}

## Parsing and Displaying

{{#include parse.incl.md}}

## Using the `time` Crate

{{#include time_crate.incl.md}}

## Using Jiff {#using-jiff}

[`jiff`][c~jiff~docs]↗{{hi:jiff}} distinguishes a `Timestamp` (an instant), a `civil::DateTime` (a calendar date and wall-clock time), and a `Zoned` value (an instant viewed in a named time zone).

Parse an RFC 3339 timestamp and display the same instant in a named time zone.

```rust,editable,noplayground
{{#include ../../../crates/cats/date_and_time/examples/jiff_basics.rs:timestamp}}
```

Use `SignedDuration` for fixed elapsed time. The alternate display format emits a compact duration string that Jiff can parse.

```rust,editable,noplayground
{{#include ../../../crates/cats/date_and_time/examples/jiff_basics.rs:duration}}
```

Run these examples from `bk/crates` with `cargo run --locked -p date_and_time --example jiff_basics`. For calendar days and ambiguous local times, see [Resolve Daylight Saving Time with Jiff](jiff_dst.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
