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

[RFC 2822]: https://www.ietf.org/rfc/rfc2822.txt
[RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
