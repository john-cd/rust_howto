# Parsing and Displaying

{{#include parse.incl.md}}

## Examine the date and time

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time] {{hi:date and time}}

Gets the current UTC [`chrono::DateTime`][c-chrono::DateTime]{{hi:chrono::DateTime}}⮳ and its hour/minute/second{{hi:hour/minute/second}} via [`chrono::Timelike`][c-chrono::Timelike]{{hi:chrono::Timelike}}⮳ and its year/month/day/weekday{{hi:year/month/day/weekday}} via [`chrono::Datelike`][c-chrono::Datelike]{{hi:chrono::Datelike}}⮳

```rust,editable
{{#include ../../../deps/tests/current.rs}}
```

## Convert date to UNIX timestamp and vice versa

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Converts a date{{hi:date}} given by [`chrono::naive::NaiveDate::from_ymd`][c-chrono::naive::NaiveDate::from_ymd]{{hi:chrono::naive::NaiveDate::from_ymd}}⮳ and [`chrono::naive::NaiveTime::from_hms`][c-chrono::naive::NaiveTime::from_hms]{{hi:chrono::naive::NaiveTime::from_hms}}⮳ to [UNIX time stamp][wikipedia-unix-timestamp]⮳ using [`chrono::naive::NaiveDateTime::timestamp`][c-chrono::naive::NaiveDateTime::timestamp]{{hi:chrono::naive::NaiveDateTime::timestamp}}⮳

Then it calculates what was the date after one billion seconds since January 1, 1970 0:00:00 UTC, using [`chrono::naive::NaiveDateTime::from_timestamp`][c-chrono::naive::NaiveDateTime::from_timestamp]{{hi:chrono::naive::NaiveDateTime::from_timestamp}}⮳.

```rust,editable
{{#include ../../../deps/tests/timestamp.rs}}
```

## Display formatted date and time

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the current time{{hi:current time}} in UTC using [`chrono::offset::Utc::now`][c-chrono::offset::Utc::now]{{hi:chrono::offset::Utc::now}}⮳.

Formats the current time in the well-known [RFC 2822 format][rfc-2822]{{hi:RFC 2822 format}}⮳ using [`chrono::DateTime::to_rfc2822`][c-chrono::DateTime::to_rfc2822]{{hi:chrono::DateTime::to_rfc2822}}⮳ and [`RFC 3339`][rfc-3339]{{hi:RFC 3339}}⮳ using [`chrono::DateTime::to_rfc3339`][c-chrono::DateTime::to_rfc3339]{{hi:chrono::DateTime::to_rfc3339}}⮳ and in a custom format using [`chrono::DateTime::format`][c-chrono::DateTime::format]{{hi:chrono::DateTime::format}}⮳.

```rust,editable
{{#include ../../../deps/tests/format.rs}}
```

## Parse string into DateTime struct

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Parses a [`chrono::DateTime`][c-chrono::DateTime]{{hi:chrono::DateTime}}⮳ struct from strings representing the well-known
[RFC 2822 format][rfc-2822]⮳ and [RFC 3339 format][rfc-3339]{{hi:RFC-3339 format}}⮳, and a custom format, using
[`chrono::DateTime::parse_from_rfc2822`][c-chrono::DateTime::parse_from_rfc2822]{{hi:chrono::DateTime::parse_from_rfc2822}}⮳  [`chrono::DateTime::parse_from_rfc2822`][c-chrono::DateTime::parse_from_rfc2822]{{hi:chrono::DateTime::parse_from_rfc2822}}⮳ and
[`chrono::DateTime::parse_from_str`][c-chrono::DateTime::parse_from_str]{{hi:chrono::DateTime::parse_from_str}}⮳ respectively.

Escape sequences that are available for the [`chrono::DateTime::parse_from_str`][c-chrono::DateTime::parse_from_str]{{hi:chrono::DateTime::parse_from_str}}⮳ can be found at [`chrono::format::strftime`][c-chrono::format::strftime]{{hi:chrono::format::strftime}}⮳. Note that the [`chrono::DateTime::parse_from_str`][c-chrono::DateTime::parse_from_str]{{hi:chrono::DateTime::parse_from_str}}⮳ requires that such a DateTime{{hi:DateTime}} struct must be creatable that it uniquely identifies a date and a time. For parsing dates and times without timezones{{hi:timezones}} use [`chrono::naive::NaiveDate`][c-chrono::naive::NaiveDate]{{hi:chrono::naive::NaiveDate}}⮳  [`chrono::naive::NaiveTime`][c-chrono::naive::NaiveTime]{{hi:chrono::naive::NaiveTime}}⮳ and [`chrono::naive::NaiveDateTime`][c-chrono::naive::NaiveDateTime]{{hi:chrono::naive::NaiveDateTime}}⮳.

```rust,editable
{{#include ../../../deps/tests/string.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
