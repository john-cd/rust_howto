# Parsing and Displaying

## Examine the date and time

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the current UTC [`DateTime`][DateTime] and its hour/minute/second via [`Timelike`][Timelike] and its year/month/day/weekday via [`Datelike`][Datelike]

```rust,editable
{{#include ../../deps/examples/current.rs}}
```

## Convert date to UNIX timestamp and vice versa

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Converts a date given by [`NaiveDate::from_ymd`][NaiveDate::from_ymd] and [`NaiveTime::from_hms`][NaiveTime::from_hms] to [UNIX timestamp][UNIX timestamp] using [`NaiveDateTime::timestamp`][NaiveDateTime::timestamp] Then it calculates what was the date after one billion seconds since January 1, 1970 0:00:00 UTC, using [`NaiveDateTime::from_timestamp`][NaiveDateTime::from_timestamp]

```rust,editable
{{#include ../../deps/examples/timestamp.rs}}
```

## Display formatted date and time

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the current time in UTC using [`Utc::now`][Utc::now] Formats the current time in the well-known formats [RFC 2822][RFC 2822] using [`DateTime::to_rfc2822`][DateTime::to_rfc2822] and [RFC 3339][RFC 3339] using [`DateTime::to_rfc3339`][DateTime::to_rfc3339] and in a custom format using
[`DateTime::format`][DateTime::format]

```rust,editable
{{#include ../../deps/examples/format.rs}}
```

## Parse string into DateTime struct

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Parses a [`DateTime`][DateTime] struct from strings representing the well-known formats
[RFC 2822], [RFC 3339][RFC 3339], and a custom format, using
[`DateTime::parse_from_rfc2822`][DateTime::parse_from_rfc2822] [`DateTime::parse_from_rfc3339`][DateTime::parse_from_rfc3339] and
[`DateTime::parse_from_str`][DateTime::parse_from_str] respectively.

Escape sequences that are available for the [`DateTime::parse_from_str`][DateTime::parse_from_str] can be found at [`chrono::format::strftime`][chrono::format::strftime] Note that the [`DateTime::parse_from_str`][DateTime::parse_from_str] requires that such a DateTime struct must be creatable that it uniquely identifies a date and a time. For parsing dates and times without timezones use
[`NaiveDate`][NaiveDate] [`NaiveTime`][NaiveTime] and [`NaiveDateTime`][NaiveDateTime]

```rust,editable
{{#include ../../deps/examples/string.rs}}
```

{{#include ../refs/link-refs.md}}
