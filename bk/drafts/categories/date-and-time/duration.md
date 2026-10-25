<!-- TODO Reviewed ??-??-2026 -->

# Duration and Calculation

{{#include duration.incl.md}}

## Measure the Elapsed Time Between Two Code Sections {#measure-elapsed-time}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}{{hi:Elapsed time}}

Measures [`std::time::Instant::elapsed`][c~std::time::Instant::elapsed~docs]↗{{hi:std::time::Instant::elapsed}} since [`std::time::Instant::now`][c~std::time::Instant::now~docs]↗{{hi:std::time::Instant::now}}.

Calling [`std::time::Instant::elapsed`][c~std::time::Instant::elapsed~docs]↗{{hi:std::time::Instant::elapsed}} returns a [`std::time::Duration`][c~std::time::Duration~docs]↗{{hi:std::time::Duration}} that we print at the end of the example. This method will not mutate or reset the [`std::time::Instant`][c~std::time::Instant~docs]↗{{hi:std::time::Instant}} object.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/duration/profile.rs:example}}
```

## Perform Checked Date and Time Calculations {#perform-checked-date-time-calc}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

Calculates and displays a [date and time][p~date-and-time]{{hi:Date and time}} three weeks after a fixed UTC timestamp, then subtracts one day. The example uses [`chrono::DateTime::checked_add_signed`][c~chrono::DateTime::checked_add_signed~docs]↗{{hi:chrono::DateTime::checked_add_signed}} and [`chrono::DateTime::checked_sub_signed`][c~chrono::DateTime::checked_sub_signed~docs]↗{{hi:chrono::DateTime::checked_sub_signed}} so overflow is represented as `None`.

The methods return None if the [date and time][p~date-and-time] cannot be calculated.

Escape sequences that are available for the[`chrono::DateTime::format`][c~chrono::DateTime::format~docs]↗{{hi:chrono::DateTime::format}} can be found at [`chrono::format::strftime`][c~chrono::format::strftime~docs]↗{{hi:chrono::format::strftime}}.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/checked.rs:example}}
```

## Convert a UTC Time to Fixed Time Zones {#convert-local-time}

[![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]

This example converts a fixed UTC timestamp to fixed offsets, so its output is deterministic. Use a time-zone database such as [`chrono-tz`][c~chrono-tz~docs]↗ when daylight-saving rules and named zones matter; fixed offsets do not model those rules.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/chrono/timezone.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Parsing and Displaying](parse.md) covers date parsing and formatting.
- [Time](time_crate.md) demonstrates the `time` crate.
- [Benchmarking](../development-tools_profiling/benchmarking.md) uses a benchmark harness for repeatable measurements; `Instant` is useful for elapsed-time diagnostics, not a substitute for benchmarking.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
