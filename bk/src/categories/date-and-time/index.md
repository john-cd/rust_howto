# Date and Time

[![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]{{hi:Date and time}}

Manage the complexity of dealing with the fourth dimension.

There are two key libraries:

- [`chrono`][c-chrono]⮳{{hi:chrono}}: a comprehensive, full-featured, yet complex date and time library.
- [`time`][c-time]⮳{{hi:time}}: a smaller, simpler library with limited functionality.

There is no clear answer as to which is best between [`time`][c-time]⮳{{hi:time}} and [`chrono`][c-chrono]⮳{{hi:chrono}}. Evaluate for yourself between these two, but both are trusted and well-maintained.

## Duration and Calculation

{{#include duration.incl.md}}

## Parsing and Displaying

{{#include parse.incl.md}}

## Using the `time` Crate

{{#include time_crate.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO review

Date and Time Types: [`chrono`][c-chrono]⮳{{hi:chrono}}, [`time`][c-time]⮳{{hi:time}} (newer crate, often preferred)
Time Zones: [`chrono-tz`][c-chrono_tz]⮳{{hi:chrono-tz}}, [`time`][c-time]⮳{{hi:time}} (built-in support)
Formatting and Parsing: [`chrono`][c-chrono]⮳{{hi:chrono}}, [`time`][c-time]⮳{{hi:time}}
Durations and Time Intervals: [`chrono`][c-chrono]⮳{{hi:chrono}}, [`time`][c-time]⮳{{hi:time}}
Clock and Time Measurement: [`std::time`][c-std::time]⮳{{hi:std::time}} (for basic timekeeping), [`measure_time`][c-measure_time]⮳{{hi:measure_time}} (for convenient benchmarking)

## `humantime` {#humantime}

[![humantime][c-humantime-badge]][c-humantime] [![humantime-crates.io][c-humantime-crates.io-badge]][c-humantime-crates.io] [![humantime-github][c-humantime-github-badge]][c-humantime-github] [![humantime-lib.rs][c-humantime-lib.rs-badge]][c-humantime-lib.rs]{{hi:humantime}}{{hi:Parser}}{{hi:Time}}{{hi:Human}}{{hi:Duration}}{{hi:Human-friendly}} [![cat-date-and-time][cat-date-and-time-badge]][cat-date-and-time]{{hi:Date and time}}

A parser and formatter for `std::time::{Duration, SystemTime}`.

- Parses durations in free form like 15days 2min 2s.
- Formats durations in similar form 2years 2min 12us.
- Parses and formats timestamp in rfc3339 format: 2018-01-01T12:53:00Z.
- Parses timestamps in a weaker format: 2018-01-01 12:53:00.

</div>
