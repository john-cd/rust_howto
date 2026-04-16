# Date and Time

[![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}

Manage the complexity of dealing with the fourth dimension.

There are two key libraries:

- [`chrono`][c~chrono~docs]↗{{hi:chrono}}: a comprehensive, full-featured, yet complex date and time library.
- [`time`][c~time~docs]↗{{hi:time}}: a smaller, simpler library with limited functionality.

There is no clear answer as to which is best between [`time`][c~time~docs]↗{{hi:time}} and [`chrono`][c~chrono~docs]↗{{hi:chrono}}. Evaluate for yourself between these two, but both are trusted and well-maintained.

| Topic | Rust Crates |
|---|---|
| Date and Time Types | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} (newer crate, often preferred) |
| Time Zones | [`chrono-tz`][c~chrono-tz~docs]↗{{hi:chrono-tz}}, [`time`][c~time~docs]↗{{hi:time}} (built-in support) |
| Formatting and Parsing | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Durations and Time Intervals | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Clock and Time Measurement | [`std::time`][c~std::time~docs]↗{{hi:std::time}} (for basic timekeeping), [`measure_time`][c~measure_time~docs]↗{{hi:measure_time}} (for convenient benchmarking) |

## Duration and Calculation

{{#include duration.incl.md}}

## Parsing and Displaying

{{#include parse.incl.md}}

## Using the `time` Crate

{{#include time_crate.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review in depth](https://github.com/john-cd/rust_howto/issues/1188)

add examples for humantime

</div>
