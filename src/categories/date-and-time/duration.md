# Duration and Calculation

{{#include duration.incl.md}}

## Measure the elapsed time between two code sections

[![std][c-std-badge]][c-std]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time] {{hi:elapsed time}}

Measures [`std::time::Instant::elapsed`][c-std::time::Instant::elapsed]{{hi:std::time::Instant::elapsed}}⮳ since [`std::time::Instant::now`][c-std::time::Instant::now]{{hi:std::time::Instant::now}}⮳

Calling [`std::time::Instant::elapsed`][c-std::time::Instant::elapsed]{{hi:std::time::Instant::elapsed}}⮳ returns a [`std::time::Duration`][c-std::time::Duration]{{hi:std::time::Duration}}⮳ that we print at the end of the example. This method will not mutate or reset the [`std::time::Instant`][c-std::time::Instant]{{hi:std::time::Instant}}⮳ object.

```rust,editable
{{#include ../../../deps/tests/profile.rs}}
```

## Perform checked date and time calculations

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Calculates and displays the date and time{{hi:date and time}} two weeks from now using [`chrono::Date::checked_add_signed`][c-chrono::Date::checked_add_signed]{{hi:chrono::Date::checked_add_signed}}⮳ and the date of the day before that using [`chrono::Date::checked_sub_signed`][c-chrono::Date::checked_sub_signed]{{hi:chrono::Date::checked_sub_signed}}⮳

The methods return None if the date and time cannot be calculated.

Escape sequences that are available for the
[`chrono::DateTime::format`][c-chrono::DateTime::format]{{hi:chrono::DateTime::format}}⮳ can be found at [`chrono::format::strftime`][c-chrono::format::strftime]{{hi:chrono::format::strftime}}⮳.

```rust,editable
{{#include ../../../deps/tests/checked.rs}}
```

## Convert a local time to another timezone

[![chrono][c-chrono-badge]][c-chrono]  [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets the local time{{hi:Time}} and displays it using [`chrono::offset::Local::now`][c-chrono::offset::Local::now]{{hi:chrono::offset::Local::now}}⮳ and then converts it to the UTC{{hi:UTC}} standard using the [`chrono::DateTime::from_utc`][c-chrono::DateTime::from_utc]{{hi:chrono::DateTime::from_utc}}⮳ struct method. A time is then converted using the [`chrono::offset::FixedOffset`][c-chrono::offset::FixedOffset]{{hi:chrono::offset::FixedOffset}}⮳ struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust,editable
{{#include ../../../deps/tests/timezone.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
