# Duration and Calculation

{{#include duration.incl.md}}

## Measure the Elapsed Time Between Two Code Sections {#measure-elapsed-time}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}{{hi:Elapsed time}}

Measures [`std::time::Instant::elapsed`][c~std::time::Instant::elapsed~docs]{{hi:std::time::Instant::elapsed}}⮳ since [`std::time::Instant::now`][c~std::time::Instant::now~docs]{{hi:std::time::Instant::now}}⮳.

Calling [`std::time::Instant::elapsed`][c~std::time::Instant::elapsed~docs]{{hi:std::time::Instant::elapsed}}⮳ returns a [`std::time::Duration`][c~std::time::Duration~docs]{{hi:std::time::Duration}}⮳ that we print at the end of the example. This method will not mutate or reset the [`std::time::Instant`][c~std::time::Instant~docs]{{hi:std::time::Instant}}⮳ object.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/duration/profile.rs:example}}
```

## Perform Checked Date and Time Calculations {#perform-checked-date-time-calc}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

Calculates and displays the [date and time][p~date-and-time]{{hi:Date and time}} two weeks from now using [`chrono::Date::checked_add_signed`][c~chrono::Date::checked_add_signed~docs]{{hi:chrono::Date::checked_add_signed}}⮳ and the date of the day before that using [`chrono::Date::checked_sub_signed`][c~chrono::Date::checked_sub_signed~docs]{{hi:chrono::Date::checked_sub_signed}}⮳.

The methods return None if the [date and time][p~date-and-time] cannot be calculated.

Escape sequences that are available for the
[`chrono::DateTime::format`][c~chrono::DateTime::format~docs]{{hi:chrono::DateTime::format}}⮳ can be found at [`chrono::format::strftime`][c~chrono::format::strftime~docs]{{hi:chrono::format::strftime}}⮳.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/checked.rs:example}}
```

## Convert a Local Time to Another Timezone {#convert-local-time}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

Gets the local time{{hi:Time}} and displays it using [`chrono::offset::Local::now`][c~chrono::offset::Local::now~docs]{{hi:chrono::offset::Local::now}}⮳ and then converts it to the UTC{{hi:UTC}} standard using the [`chrono::DateTime::from_utc`][c~chrono::DateTime::from_utc~docs]{{hi:chrono::DateTime::from_utc}}⮳ struct method. A time is then converted using the [`chrono::offset::FixedOffset`][c~chrono::offset::FixedOffset~docs]{{hi:chrono::offset::FixedOffset}}⮳ struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/timezone.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/913)
</div>
