# Parsing and Displaying

{{#include parse.incl.md}}

## Examine the {{i:date and time}}

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets the current UTC [`{{i:DateTime}}`][chrono::DateTime]⮳ and its {{i:hour/minute/second}} via [`{{i:Timelike}}`][chrono::Timelike]⮳ and its {{i:year/month/day/weekday}} via [`{{i:Datelike}}`][c-chrono::Datelike]⮳

```rust,editable
{{#include ../../../deps/tests/current.rs}}
```

## Convert date to UNIX timestamp and vice versa

[![chrono][chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Converts a {{i:date}} given by [`{{i:NaiveDate::from_ymd}}`][c-chrono::naive::NaiveDate::from_ymd]⮳ and [`{{i:NaiveTime::from_hms}}`][c-chrono::naive::NaiveTime::from_hms]⮳ to [UNIX time stamp][wikipedia-unix-timestamp]⮳ using [`{{i:NaiveDateTime::timestamp}}`][c-chrono::naive::NaiveDateTime::timestamp]⮳

Then it calculates what was the date after one billion seconds since January 1, 1970 0:00:00 UTC, using [`{{i:NaiveDateTime::from_timestamp}}`][c-chrono::naive::NaiveDateTime::from_timestamp]⮳.

```rust,editable
{{#include ../../../deps/tests/timestamp.rs}}
```

## Display formatted date and time

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the {{i:current time}} in UTC using [`{{i:Utc::now}}`][c-chrono::offset::Utc::now]⮳.

Formats the current time in the well-known [{{i:RFC 2822 format}}][rfc-2822]⮳ using [`{{i:DateTime::to_rfc2822}}`][c-chrono::DateTime::to_rfc2822]⮳ and [`{{i:RFC 3339}}`][rfc-3339]⮳ using [`{{i:DateTime::to_rfc3339}}`][c-chrono::DateTime::to_rfc3339]⮳ and in a custom format using [`{{i:DateTime::format}}`][c-chrono::DateTime::format]⮳.

```rust,editable
{{#include ../../../deps/tests/format.rs}}
```

## Parse string into DateTime struct

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Parses a [`{{i:DateTime}}`][c-chrono::DateTime]⮳ struct from strings representing the well-known
[RFC 2822 format][rfc-2822]⮳ and [{{i:RFC 3339 format}}][rfc-3339]⮳, and a custom format, using
[`{{i:DateTime::parse_from_rfc2822}}`][c-chrono::DateTime::parse_from_rfc2822]⮳  [`{{i:DateTime::parse_from_rfc3339}}`][c-chrono::DateTime::parse_from_rfc2822]⮳ and
[`{{i:DateTime::parse_from_str}}`][c-chrono::DateTime::parse_from_str]⮳ respectively.

Escape sequences that are available for the [`{{i:DateTime::parse_from_str}}`][chrono::DateTime::parse_from_str]⮳ can be found at [`{{i:chrono::format::strftime}}`][chrono::format::strftime]⮳. Note that the [`{{i:DateTime::parse_from_str}}`][c-chrono::DateTime::parse_from_str]⮳ requires that such a {{i:DateTime}} struct must be creatable that it uniquely identifies a date and a time. For parsing dates and times without {{i:timezones}} use [`{{i:NaiveDate}}`][c-chrono::naive::NaiveDate]⮳  [`{{i:NaiveTime}}`][c-chrono::naive::NaiveTime]⮳ and [`{{i:NaiveDateTime}}`][c-chrono::naive::NaiveDateTime]⮳.

```rust,editable
{{#include ../../../deps/tests/string.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
