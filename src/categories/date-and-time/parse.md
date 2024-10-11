# Parsing and Displaying

{{#include parse.incl.md}}

## Examine the date and time

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time] {{hi:date and time}}

Gets the current UTC {{hi:DateTime}}[`DateTime`][c-chrono::DateTime]⮳ and its {{i:hour/minute/second}} via {{hi:Timelike}}[`Timelike`][c-chrono::Timelike]⮳ and its {{i:year/month/day/weekday}} via {{hi:Datelike}}[`Datelike`][c-chrono::Datelike]⮳

```rust,editable
{{#include ../../../deps/tests/current.rs}}
```

## Convert date to UNIX timestamp and vice versa

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Converts a {{i:date}} given by {{hi:NaiveDate::from_ymd}}[`NaiveDate::from_ymd`][c-chrono::naive::NaiveDate::from_ymd]⮳ and {{hi:NaiveTime::from_hms}}[`NaiveTime::from_hms`][c-chrono::naive::NaiveTime::from_hms]⮳ to [UNIX time stamp][wikipedia-unix-timestamp]⮳ using {{hi:NaiveDateTime::timestamp}}[`NaiveDateTime::timestamp`][c-chrono::naive::NaiveDateTime::timestamp]⮳

Then it calculates what was the date after one billion seconds since January 1, 1970 0:00:00 UTC, using {{hi:NaiveDateTime::from_timestamp}}[`NaiveDateTime::from_timestamp`][c-chrono::naive::NaiveDateTime::from_timestamp]⮳.

```rust,editable
{{#include ../../../deps/tests/timestamp.rs}}
```

## Display formatted date and time

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the {{i:current time}} in UTC using {{hi:Utc::now}}[`Utc::now`][c-chrono::offset::Utc::now]⮳.

Formats the current time in the well-known [{{i:RFC 2822 format}}][rfc-2822]⮳ using {{hi:DateTime::to_rfc2822}}[`DateTime::to_rfc2822`][c-chrono::DateTime::to_rfc2822]⮳ and {{hi:RFC 3339}}[`RFC 3339`][rfc-3339]⮳ using {{hi:DateTime::to_rfc3339}}[`DateTime::to_rfc3339`][c-chrono::DateTime::to_rfc3339]⮳ and in a custom format using {{hi:DateTime::format}}[`DateTime::format`][c-chrono::DateTime::format]⮳.

```rust,editable
{{#include ../../../deps/tests/format.rs}}
```

## Parse string into DateTime struct

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Parses a {{hi:DateTime}}[`DateTime`][c-chrono::DateTime]⮳ struct from strings representing the well-known
[RFC 2822 format][rfc-2822]⮳ and [{{i:RFC 3339 format}}][rfc-3339]⮳, and a custom format, using
{{hi:DateTime::parse_from_rfc2822}}[`DateTime::parse_from_rfc2822`][c-chrono::DateTime::parse_from_rfc2822]⮳  {{hi:DateTime::parse_from_rfc3339}}[`DateTime::parse_from_rfc3339`][c-chrono::DateTime::parse_from_rfc2822]⮳ and
{{hi:DateTime::parse_from_str}}[`DateTime::parse_from_str`][c-chrono::DateTime::parse_from_str]⮳ respectively.

Escape sequences that are available for the {{hi:DateTime::parse_from_str}}[`DateTime::parse_from_str`][c-chrono::DateTime::parse_from_str]⮳ can be found at {{hi:chrono::format::strftime}}[`chrono::format::strftime`][c-chrono::format::strftime]⮳. Note that the {{hi:DateTime::parse_from_str}}[`DateTime::parse_from_str`][c-chrono::DateTime::parse_from_str]⮳ requires that such a {{i:DateTime}} struct must be creatable that it uniquely identifies a date and a time. For parsing dates and times without {{i:timezones}} use {{hi:NaiveDate}}[`NaiveDate`][c-chrono::naive::NaiveDate]⮳  {{hi:NaiveTime}}[`NaiveTime`][c-chrono::naive::NaiveTime]⮳ and {{hi:NaiveDateTime}}[`NaiveDateTime`][c-chrono::naive::NaiveDateTime]⮳.

```rust,editable
{{#include ../../../deps/tests/string.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
