# Parsing and Displaying

## Examine the date and time

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the current UTC [`DateTime`] and its hour/minute/second via [`Timelike`]
and its year/month/day/weekday via [`Datelike`].

```rust,editable
{#include ../../../deps/examples/current.rs}
```

## Convert date to UNIX timestamp and vice versa

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Converts a date given by [`NaiveDate::from_ymd`] and [`NaiveTime::from_hms`]
to [UNIX timestamp] using [`NaiveDateTime::timestamp`].
Then it calculates what was the date after one billion seconds
since January 1, 1970 0:00:00 UTC, using [`NaiveDateTime::from_timestamp`].

```rust,editable
{#include ../../../deps/examples/timestamp.rs}
```

## Display formatted date and time

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the current time in UTC using [`Utc::now`]. Formats the
current time in the well-known formats [RFC 2822] using [`DateTime::to_rfc2822`]
and [RFC 3339] using [`DateTime::to_rfc3339`], and in a custom format using
[`DateTime::format`].

```rust,editable
{#include ../../../deps/examples/format.rs}
```

## Parse string into DateTime struct

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Parses a [`DateTime`] struct from strings representing the well-known formats
[RFC 2822], [RFC 3339], and a custom format, using
[`DateTime::parse_from_rfc2822`], [`DateTime::parse_from_rfc3339`], and
[`DateTime::parse_from_str`] respectively.

Escape sequences that are available for the [`DateTime::parse_from_str`] can be
found at [`chrono::format::strftime`]. Note that the [`DateTime::parse_from_str`]
requires that such a DateTime struct must be creatable that it uniquely
identifies a date and a time. For parsing dates and times without timezones use
[`NaiveDate`], [`NaiveTime`], and [`NaiveDateTime`].

```rust,editable
{#include ../../../deps/examples/string.rs}
```

[`NaiveDate::from_ymd`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDate.html#method.from_ymd
[`NaiveDateTime::from_timestamp`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp
[`NaiveDateTime::timestamp`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html#method.timestamp
[`NaiveTime::from_hms`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveTime.html#method.from_hms
[UNIX timestamp]: https://en.wikipedia.org/wiki/Unix_time
[`Datelike`]: https://docs.rs/chrono/*/chrono/trait.Datelike.html
[`Timelike`]: https://docs.rs/chrono/*/chrono/trait.Timelike.html
[`Utc::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Utc.html#method.now
[RFC 2822]: https://www.ietf.org/rfc/rfc2822.txt
[RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
[`chrono::format::strftime`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[`DateTime::parse_from_rfc2822`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_rfc2822
[`DateTime::parse_from_rfc3339`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_rfc3339
[`DateTime::parse_from_str`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_str
[`DateTime::to_rfc2822`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc2822
[`DateTime::to_rfc3339`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc3339
[`DateTime`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html
[`NaiveDate`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDate.html
[`NaiveDateTime`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html
[`NaiveTime`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveTime.html
{{#include ../refs/link-refs.md}}
