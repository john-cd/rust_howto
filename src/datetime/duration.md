# Duration and Calculation

## Measure the elapsed time between two code sections

[![std-badge]][std] [![cat-time-badge]][cat-time]

Measures `[time::Instant::elapsed]` since `[time::Instant::now]`

Calling `[time::Instant::elapsed]` returns a `[time::Duration]` that we print at the end of the example.
This method will not mutate or reset the `[time::Instant]` object.

```rust,editable
{#include ../../deps/examples/profile.rs}
```

## Perform checked date and time calculations

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Calculates and displays the date and time two weeks from now using
`[DateTime::checked_add_signed]` and the date of the day before that using
`[DateTime::checked_sub_signed]` The methods return None if the date and time
cannot be calculated.

Escape sequences that are available for the
`[DateTime::format]` can be found at `[chrono::format::strftime]`

```rust,editable
{#include ../../deps/examples/checked.rs}
```

## Convert a local time to another timezone

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the local time and displays it using `[offset::Local::now]` and then converts it to the UTC standard using the `[DateTime::from_utc]` struct method. A time is then converted using the `[offset::FixedOffset]` struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust,editable
{#include ../../deps/examples/timezone.rs}
```

[time::Duration]: https://doc.rust-lang.org/std/time/struct.Duration.html
[time::Instant::elapsed]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed
[time::Instant::now]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[time::Instant]:https://doc.rust-lang.org/std/time/struct.Instant.html
[chrono::format::strftime]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
[DateTime::checked_add_signed]: https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_add_signed
[DateTime::checked_sub_signed]: https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_sub_signed
[DateTime::format]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[DateTime::from_utc]:https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.from_utc
[offset::FixedOffset]: https://docs.rs/chrono/*/chrono/offset/struct.FixedOffset.html
[offset::Local::now]: https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now
{{#include ../refs/link-refs.md}}
