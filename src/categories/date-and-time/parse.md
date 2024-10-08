# Parsing and Displaying

{{#include parse.incl.md}}

## Examine the {{i:date and time}}

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets the current UTC [`DateTime`][chrono::DateTime]⮳ and its {{i:hour/minute/second}} via [`Timelike`][chrono::Timelike]⮳ and its {{i:year/month/day/weekday}} via [`Datelike`][chrono::Datelike]⮳

```rust,editable
{{#include ../../../deps/tests/current.rs}}
```

## Convert date to UNIX timestamp and vice versa

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Converts a {{i:date}} given by [`NaiveDate::from_ymd`][chrono::naive::NaiveDate::from_ymd]⮳ and [`NaiveTime::from_hms`][chrono::naive::NaiveTime::from_hms]⮳ to [UNIX time stamp][unix-timestamp]⮳ using [`NaiveDateTime::timestamp`][chrono::naive::NaiveDateTime::timestamp]⮳

Then it calculates what was the date after one billion seconds since January 1, 1970 0:00:00 UTC, using [`NaiveDateTime::from_timestamp`][chrono::naive::NaiveDateTime::from_timestamp]⮳.

```rust,editable
{{#include ../../../deps/tests/timestamp.rs}}
```

## Display formatted date and time

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the {{i:current time}} in UTC using [`Utc::now`][chrono::offset::Utc::now]⮳.

Formats the current time in the well-known [RFC 2822 format][rfc-2822]⮳ using [`DateTime::to_rfc2822`][chrono::DateTime::to_rfc2822]⮳ and [`RFC 3339`][rfc-3339]⮳ using [`DateTime::to_rfc3339`][chrono::DateTime::to_rfc3339]⮳ and in a custom format using [`DateTime::format`][chrono::DateTime::format]⮳.

```rust,editable
{{#include ../../../deps/tests/format.rs}}
```

## Parse string into DateTime struct

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Parses a [`DateTime`][chrono::DateTime]⮳ struct from strings representing the well-known
[RFC 2822 format][rfc-2822]⮳ and [RFC 3339 format][rfc-3339]⮳, and a custom format, using
[`DateTime::parse_from_rfc2822`][chrono::DateTime::parse_from_rfc2822]⮳  [`DateTime::parse_from_rfc3339`][chrono::DateTime::parse_from_rfc3339]⮳ and
[`DateTime::parse_from_str`][chrono::DateTime::parse_from_str]⮳ respectively.

Escape sequences that are available for the [`DateTime::parse_from_str`][chrono::DateTime::parse_from_str]⮳ can be found at [`chrono::format::strftime`][chrono::format::strftime]⮳. Note that the [`DateTime::parse_from_str`][chrono::DateTime::parse_from_str]⮳ requires that such a {{i:DateTime}} struct must be creatable that it uniquely identifies a date and a time. For parsing dates and times without {{i:timezones}} use [`NaiveDate`][chrono::naive::NaiveDate]⮳  [`NaiveTime`][chrono::naive::NaiveTime]⮳ and [`NaiveDateTime`][chrono::naive::NaiveDateTime]⮳.

```rust,editable
{{#include ../../../deps/tests/string.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
