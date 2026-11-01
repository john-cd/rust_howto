<!-- TODO Reviewed ??-??-2026 -->

| Recipe                                                                                | Crates                                   | Categories                                             |
| ------------------------------------------------------------------------------------- | ---------------------------------------- | ------------------------------------------------------ |
| [Mark Scopes with `profiling`][ex~development-tools_profiling~profiling-scopes]       | {{#crate profiling}}                     | {{#categories development-tools::profiling=Profiling}} |
| [Record and Serve Frames with `puffin`][ex~development-tools_profiling~puffin-server] | {{#crate puffin}} {{#crate puffin_http}} | {{#categories development-tools::profiling=Profiling}} |
| [Time a Code Block with `measure_time`][ex~development-tools_profiling~measure-time]  | {{#crate measure_time}}                  | {{#categories development-tools::profiling=Profiling}} |

An instrumentation profiler records the scopes that you mark in the code, with exact names and durations, frame by frame. A sampling profiler such as [`samply`][c~samply~docs]↗{{hi:samply}}, `perf` or [`cargo flamegraph`][c~flamegraph~docs]↗{{hi:cargo-flamegraph}} instead interrupts the program at intervals and records where it is: it needs no changes to the code, but it cannot tell you what a frame or a request spent its time on.
