# Duration and Calculation

## Measure the elapsed time between two code sections

[![std][std-badge]][std]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Measures [`time::Instant::elapsed`][std::time::Instant::elapsed] since [`time::Instant::now`][std::time::Instant::now]

Calling [`time::Instant::elapsed`][std::time::Instant::elapsed] returns a [`time::Duration`][std::time::Duration] that we print at the end of the example. This method will not mutate or reset the [`time::Instant`][std::time::Instant] object.

```rust,editable
{{#include ../../deps/tests/profile.rs}}
```

## Perform checked date and time calculations

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Calculates and displays the date and time two weeks from now using
[`DateTime::checked_add_signed`][chrono::Date::checked_add_signed] and the date of the day before that using
[`DateTime::checked_sub_signed`][chrono::Date::checked_sub_signed] The methods return None if the date and time cannot be calculated.

Escape sequences that are available for the
[`DateTime::format`][chrono::DateTime::format] can be found at [`chrono::format::strftime`][chrono::format::strftime]

```rust,editable
{{#include ../../deps/tests/checked.rs}}
```

## Convert a local time to another timezone

[![chrono][chrono-badge]][chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets the local time and displays it using [`offset::Local::now`][chrono::offset::Local::now] and then converts it to the UTC standard using the [`DateTime::from_utc`][chrono::DateTime::from_utc] struct method. A time is then converted using the [`offset::FixedOffset`][chrono::offset::FixedOffset] struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust,editable
{{#include ../../deps/tests/timezone.rs}}
```

{{#include ../refs/link-refs.md}}
