# Duration and Calculation

{{#include duration.incl.md}}

## Measure the elapsed time between two code sections

[![std][c-std-badge]][c-std]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time] {{hi:elapsed time}}

Measures {{hi:time::Instant::elapsed}}[`time::Instant::elapsed`][c-std::time::Instant::elapsed]⮳ since {{hi:time::Instant::now}}[`time::Instant::now`][c-std::time::Instant::now]⮳

Calling {{hi:time::Instant::elapsed}}[`time::Instant::elapsed`][c-std::time::Instant::elapsed]⮳ returns a {{hi:time::Duration}}[`time::Duration`][c-std::time::Duration]⮳ that we print at the end of the example. This method will not mutate or reset the {{hi:time::Instant}}[`time::Instant`][c-std::time::Instant]⮳ object.

```rust,editable
{{#include ../../../deps/tests/profile.rs}}
```

## Perform checked date and time calculations

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Calculates and displays the {{i:date and time}} two weeks from now using {{hi:DateTime::checked_add_signed}}[`DateTime::checked_add_signed`][c-chrono::Date::checked_add_signed]⮳ and the date of the day before that using {{hi:DateTime::checked_sub_signed}}[`DateTime::checked_sub_signed`][c-chrono::Date::checked_sub_signed]⮳

The methods return None if the date and time cannot be calculated.

Escape sequences that are available for the
{{hi:DateTime::format}}[`DateTime::format`][c-chrono::DateTime::format]⮳ can be found at {{hi:chrono::format::strftime}}[`chrono::format::strftime`][c-chrono::format::strftime]⮳.

```rust,editable
{{#include ../../../deps/tests/checked.rs}}
```

## Convert a local time to another timezone

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets the local {{i:time}} and displays it using {{hi:offset::Local::now}}[`offset::Local::now`][c-chrono::offset::Local::now]⮳ and then converts it to the {{i:UTC}} standard using the {{hi:DateTime::from_utc}}[`DateTime::from_utc`][c-chrono::DateTime::from_utc]⮳ struct method. A time is then converted using the {{hi:offset::FixedOffset}}[`offset::FixedOffset`][c-chrono::offset::FixedOffset]⮳ struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust,editable
{{#include ../../../deps/tests/timezone.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
