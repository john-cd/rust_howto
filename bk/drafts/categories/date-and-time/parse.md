<!-- TODO Reviewed ??-??-2026 -->
# Parsing and Displaying

{{#include parse.incl.md}}

## Examine the Date and Time {#examine-date-and-time}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}

Gets the current UTC [`chrono::DateTime`][c~chrono::DateTime~docs]↗{{hi:chrono::DateTime}} and its hour/minute/second{{hi:Hour/minute/second}} via [`chrono::Timelike`][c~chrono::Timelike~docs]↗{{hi:chrono::Timelike}} and its year/month/day/weekday{{hi:year/month/day/weekday}} via [`chrono::Datelike`][c~chrono::Datelike~docs]↗{{hi:chrono::Datelike}}.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/current.rs:example}}
```

## Convert Date to UNIX Timestamp and Vice Versa {#unix-timestamp}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

Builds a `NaiveDateTime` with Chrono's checked date and time constructors, then converts it to a [Unix timestamp][wikipedia~unix-timestamp]↗ with `and_utc().timestamp()`.

It also converts one billion seconds after the Unix epoch to a UTC `DateTime` with `DateTime::from_timestamp`. The checked constructors return `Option`, so the example propagates out-of-range values instead of unwrapping them.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/timestamp.rs:example}}
```

## Display Formatted Date and Time {#display-formatted-date-time}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

Gets and displays the current time{{hi:Current time}} in UTC using [`chrono::offset::Utc::now`][c~chrono::offset::Utc::now~docs]↗{{hi:chrono::offset::Utc::now}}.

Formats the current time in the well-known [RFC 2822 format][ietf~rfc-2822]↗{{hi:RFC-2822 format}} using [`chrono::DateTime::to_rfc2822`][c~chrono::DateTime::to_rfc2822~docs]↗{{hi:chrono::DateTime::to_rfc2822}} and [`RFC 3339`][ietf~rfc-3339]↗{{hi:RFC-3339}} using [`chrono::DateTime::to_rfc3339`][c~chrono::DateTime::to_rfc3339~docs]↗{{hi:chrono::DateTime::to_rfc3339}} and in a custom format using [`chrono::DateTime::format`][c~chrono::DateTime::format~docs]↗{{hi:chrono::DateTime::format}}.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/format.rs:example}}
```

## Parse a String into a `DateTime` Struct {#parse-string-into-datetime-struct}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

[`chrono`][c~chrono~docs]↗{{hi:chrono}} parses an offset-aware [`DateTime`][c~chrono::DateTime~docs]↗ from [RFC 2822][ietf~rfc-2822]↗ and [RFC 3339][ietf~rfc-3339]↗ timestamps with `parse_from_rfc2822` and `parse_from_rfc3339`. It parses a custom format with `parse_from_str`; each function returns a `Result` so malformed input can be reported.

Escape sequences for `parse_from_str` follow Chrono's [`strftime` format rules][c~chrono::format::strftime~docs]↗. An offset-aware `DateTime` must identify one instant; for date or time input without an offset, use the `NaiveDate`, `NaiveTime`, or `NaiveDateTime` types.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/parse_string_into_datetime.rs:example}}
```

## `humantime` {#humantime}

[![humantime][c~humantime~docs~badge]][c~humantime~docs] [![humantime~crates.io][c~humantime~crates.io~badge]][c~humantime~crates.io] [![humantime~repo][c~humantime~repo~badge]][c~humantime~repo] [![humantime~lib.rs][c~humantime~lib.rs~badge]][c~humantime~lib.rs]{{hi:humantime}}{{hi:Parser}}{{hi:Time}}{{hi:Human}}{{hi:Duration}}{{hi:Human-friendly}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}

A parser and formatter for `std::time::{Duration, SystemTime}`. The example parses `2h 30m 15s`, formats the resulting duration, and parses then formats `2018-01-01T12:53:00Z` as an RFC 3339 timestamp.

- Parses durations in free form like 15days 2min 2s.
- Formats durations in similar form 2years 2min 12us.
- Parses and formats timestamp in rfc3339 format: 2018-01-01T12:53:00Z.
- Also parses timestamps in a weaker format such as `2018-01-01 12:53:00`.

## Related Topics {#related-topics .skip}

- [[duration | Duration and Calculation]].
- [[time_crate | The `time` Crate]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write humantime; review](https://github.com/john-cd/rust_howto/issues/914)
</div>
