# Duration and Calculation

{{#include duration.incl.md}}

## Measure the elapsed time between two code sections {#measure-elapsed-time}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]{{hi:Date and time}}{{hi:Elapsed time}}

Measures [`std::time::Instant::elapsed`][c-std::time::Instant::elapsed]{{hi:std::time::Instant::elapsed}}⮳ since [`std::time::Instant::now`][c-std::time::Instant::now]{{hi:std::time::Instant::now}}⮳.

Calling [`std::time::Instant::elapsed`][c-std::time::Instant::elapsed]{{hi:std::time::Instant::elapsed}}⮳ returns a [`std::time::Duration`][c-std::time::Duration]{{hi:std::time::Duration}}⮳ that we print at the end of the example. This method will not mutate or reset the [`std::time::Instant`][c-std::time::Instant]{{hi:std::time::Instant}}⮳ object.

```rust,editable
{{#include ../../../crates/cats/date_and_time/tests/duration/profile.rs:example}}
```

## Perform checked date and time calculations {#perform-checked-date-time-calc}

[![chrono][c-chrono-badge]][c-chrono]{{hi:chrono}} [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Calculates and displays the [date and time][p-date-and-time]{{hi:Date and time}} two weeks from now using [`chrono::Date::checked_add_signed`][c-chrono::Date::checked_add_signed]{{hi:chrono::Date::checked_add_signed}}⮳ and the date of the day before that using [`chrono::Date::checked_sub_signed`][c-chrono::Date::checked_sub_signed]{{hi:chrono::Date::checked_sub_signed}}⮳.

The methods return None if the [date and time][p-date-and-time] cannot be calculated.

Escape sequences that are available for the
[`chrono::DateTime::format`][c-chrono::DateTime::format]{{hi:chrono::DateTime::format}}⮳ can be found at [`chrono::format::strftime`][c-chrono::format::strftime]{{hi:chrono::format::strftime}}⮳.

```rust,editable
{{#include ../../../crates/cats/date_and_time/tests/chrono/checked.rs:example}}
```

## Convert a local time to another timezone {#convert-local-time}

[![chrono][c-chrono-badge]][c-chrono]{{hi:chrono}} [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]

Gets the local time{{hi:Time}} and displays it using [`chrono::offset::Local::now`][c-chrono::offset::Local::now]{{hi:chrono::offset::Local::now}}⮳ and then converts it to the UTC{{hi:UTC}} standard using the [`chrono::DateTime::from_utc`][c-chrono::DateTime::from_utc]{{hi:chrono::DateTime::from_utc}}⮳ struct method. A time is then converted using the [`chrono::offset::FixedOffset`][c-chrono::offset::FixedOffset]{{hi:chrono::offset::FixedOffset}}⮳ struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust,editable
{{#include ../../../crates/cats/date_and_time/tests/chrono/timezone.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P1 review](https://github.com/john-cd/rust_howto/issues/913)
</div>
