<!-- TODO Reviewed ??-??-2026 -->

# Time

{{#include time_crate.incl.md}}

## Use the `time` Crate {#time-crate}

[![time~website][c~time~website~badge]][c~time~website] [![time][c~time~docs~badge]][c~time~docs] [![time~crates.io][c~time~crates.io~badge]][c~time~crates.io] [![time~repo][c~time~repo~badge]][c~time~repo] [![time~lib.rs][c~time~lib.rs~badge]][c~time~lib.rs]{{hi:time}}{{hi:Duration}}{{hi:Date}}{{hi:Calendar}}{{hi:time}}[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}[![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}[![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}
[![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}
[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}
[![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`time`][c~time~docs]↗{{hi:time}} is a date-and-time library that interoperates with `std` and supports `no_std` configurations. Its `OffsetDateTime`, `PrimitiveDateTime`, and `Duration` types cover timestamp, local date-time, and arithmetic use cases. Enable the `macros` feature for compile-time format descriptions and fixed date literals. Compared with `chrono`, `time` emphasizes a smaller modular API and `no_std` support; [`jiff`][c~jiff~docs]↗ is another actively developed option with richer time-zone handling.

`OffsetDateTime::now_utc()` is suitable for a current UTC timestamp. Avoid relying on wall-clock time in tests: construct fixed values with `datetime!` and assert exact parse, format, and arithmetic results. Local offset lookup such as `OffsetDateTime::now_local()` can fail on multithreaded Unix programs because the platform's local-time APIs may mutate process-global state.

```rust,editable
{{#include ../../../crates/cats/date_and_time/examples/time.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Duration and Calculation](duration.md) explains elapsed-time measurement and checked arithmetic.
- [Parsing and Displaying](parse.md) covers date parsing and formatting.
- [Time-zone handling](duration.md#convert-local-time) demonstrates conversion with Chrono.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
