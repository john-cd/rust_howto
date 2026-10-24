<!-- TODO Reviewed ??-??-2026 -->
# Date and Time

[![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}

Manage the complexity of dealing with the fourth dimension.

[`chrono`][c~chrono~docs]↗{{hi:chrono}} provides a broad date-and-time API, including the system's local time zone. [`time`][c~time~docs]↗{{hi:time}} has a modular API and supports `no_std` configurations. Choose based on the types, time-zone data, and runtime features your application needs.

| Topic | Rust Crates |
|---|---|
| Date and Time Types | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Named Time Zones | [`chrono-tz`][c~chrono-tz~docs]↗{{hi:chrono-tz}} supplies IANA time-zone data for `chrono`; `time` supports UTC offsets and can optionally query the system's local offset. |
| Formatting and Parsing | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Human-Readable Durations | [`humantime`][ex~date-and-time~humantime] parses and formats `std::time::Duration` values. |
| Durations and Time Intervals | [`std::time::Duration`][c~std::time::Duration~docs]↗{{hi:std::time::Duration}}, [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} |
| Elapsed-Time Measurement | [`std::time::Instant`][c~std::time::Instant~docs]↗{{hi:std::time::Instant}} is the baseline; [`measure_time`][c~measure_time~docs]↗{{hi:measure_time}} adds scope-based logging and printing. |

## Duration and Calculation

{{#include duration.incl.md}}

## Parsing and Displaying

{{#include parse.incl.md}}

## Using the `time` Crate

{{#include time_crate.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
The `time` crate represents UTC offsets and can query the system offset when its `local-offset` feature is enabled; it does not provide a bundled IANA time-zone database. `chrono-tz` supplies that database for Chrono. Keep this category in drafts while its detail pages still contain unresolved placeholders.

</div>
