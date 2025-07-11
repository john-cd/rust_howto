# Parsing and Displaying

{{#include parse.incl.md}}

## Examine the Date and Time {#examine-date-and-time}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}

Gets the current UTC [`chrono::DateTime`][c~chrono::DateTime~docs]{{hi:chrono::DateTime}}⮳ and its hour/minute/second{{hi:Hour/minute/second}} via [`chrono::Timelike`][c~chrono::Timelike~docs]{{hi:chrono::Timelike}}⮳ and its year/month/day/weekday{{hi:year/month/day/weekday}} via [`chrono::Datelike`][c~chrono::Datelike~docs]{{hi:chrono::Datelike}}⮳.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/current.rs:example}}
```

## Convert Date to UNIX Timestamp and Vice Versa {#unix-timestamp}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

Converts a date{{hi:date}} given by [`chrono::naive::NaiveDate::from_ymd`][c~chrono::naive::NaiveDate::from_ymd~docs]{{hi:chrono::naive::NaiveDate::from_ymd}}⮳ and [`chrono::naive::NaiveTime::from_hms`][c~chrono::naive::NaiveTime::from_hms~docs]{{hi:chrono::naive::NaiveTime::from_hms}}⮳ to [UNIX time stamp][wikipedia~unix-timestamp]⮳ using [`chrono::naive::NaiveDateTime::timestamp`][c~chrono::naive::NaiveDateTime::timestamp~docs]{{hi:chrono::naive::NaiveDateTime::timestamp}}⮳.

Then it calculates what was the date after one billion seconds since January 1, 1970 0:00:00 UTC, using [`chrono::naive::NaiveDateTime::from_timestamp`][c~chrono::naive::NaiveDateTime::from_timestamp~docs]{{hi:chrono::naive::NaiveDateTime::from_timestamp}}⮳.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/timestamp.rs:example}}
```

## Display Formatted Date and Time {#display-formatted-date-time}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

Gets and displays the current time{{hi:Current time}} in UTC using [`chrono::offset::Utc::now`][c~chrono::offset::Utc::now~docs]{{hi:chrono::offset::Utc::now}}⮳.

Formats the current time in the well-known [RFC 2822 format][ietf~rfc-2822]{{hi:RFC-2822 format}}⮳ using [`chrono::DateTime::to_rfc2822`][c~chrono::DateTime::to_rfc2822~docs]{{hi:chrono::DateTime::to_rfc2822}}⮳ and [`RFC 3339`][ietf~rfc-3339]{{hi:RFC-3339}}⮳ using [`chrono::DateTime::to_rfc3339`][c~chrono::DateTime::to_rfc3339~docs]{{hi:chrono::DateTime::to_rfc3339}}⮳ and in a custom format using [`chrono::DateTime::format`][c~chrono::DateTime::format~docs]{{hi:chrono::DateTime::format}}⮳.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/format.rs:example}}
```

## Parse a String into a `DateTime` Struct {#parse-string-into-datetime-struct}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

[`chrono`][c~chrono~docs]⮳{{hi:chrono}} parses a [`chrono::DateTime`][c~chrono::DateTime~docs]{{hi:chrono::DateTime}}⮳ struct from strings representing the well-known
[RFC 2822 format][ietf~rf-2822]⮳ and [RFC 3339 format][ietf~rfc-3339]{{hi:RFC-3339 format}}⮳, and a custom format, using
[`chrono::DateTime::parse_from_rfc2822`][c~chrono::DateTime::parse_from_rfc2822~docs]{{hi:chrono::DateTime::parse_from_rfc2822}}⮳ [`chrono::DateTime::parse_from_rfc2822`][c~chrono::DateTime::parse_from_rfc2822~docs]{{hi:chrono::DateTime::parse_from_rfc2822}}⮳ and
[`chrono::DateTime::parse_from_str`][c~chrono::DateTime::parse_from_str~docs]{{hi:chrono::DateTime::parse_from_str}}⮳ respectively.

Escape sequences that are available for the [`chrono::DateTime::parse_from_str`][c~chrono::DateTime::parse_from_str~docs]{{hi:chrono::DateTime::parse_from_str}}⮳ can be found at [`chrono::format::strftime`][c~chrono::format::strftime~docs]{{hi:chrono::format::strftime}}⮳. Note that the [`chrono::DateTime::parse_from_str`][c~chrono::DateTime::parse_from_str~docs]{{hi:chrono::DateTime::parse_from_str}}⮳ requires that such a DateTime{{hi:DateTime}} struct must be creatable that it uniquely identifies a date and a time. For parsing dates and times without timezones{{hi:timezones}} use [`chrono::naive::NaiveDate`][c~chrono::naive::NaiveDate~docs]{{hi:chrono::naive::NaiveDate}}⮳ [`chrono::naive::NaiveTime`][c~chrono::naive::NaiveTime~docs]{{hi:chrono::naive::NaiveTime}}⮳ and [`chrono::naive::NaiveDateTime`][c~chrono::naive::NaiveDateTime~docs]{{hi:chrono::naive::NaiveDateTime}}⮳.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/parse_string_into_datetime.rs:example}}
```

## `humantime` {#humantime}

[![humantime][c~humantime~docs~badge]][c~humantime~docs] [![humantime~crates.io][c~humantime~crates.io~badge]][c~humantime~crates.io] [![humantime~github][c~humantime~github~badge]][c~humantime~github] [![humantime~lib.rs][c~humantime~lib.rs~badge]][c~humantime~lib.rs]{{hi:humantime}}{{hi:Parser}}{{hi:Time}}{{hi:Human}}{{hi:Duration}}{{hi:Human-friendly}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}

A parser and formatter for `std::time::{Duration, SystemTime}`.

- Parses durations in free form like 15days 2min 2s.
- Formats durations in similar form 2years 2min 12us.
- Parses and formats timestamp in rfc3339 format: 2018-01-01T12:53:00Z.
- Parses timestamps in a weaker format: 2018-01-01 12:53:00.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write humantime; review](https://github.com/john-cd/rust_howto/issues/914)
</div>
