<!-- TODO Reviewed ??-??-2026. -->

Benchmark first to establish a repeatable baseline; profile when you need to
find where the time or memory goes. Use representative inputs, keep the
compiler, target, machine, and workload constant between runs, and compare
several measurements instead of relying on one sample. A function-level
benchmark measures a small hot path; a command-line benchmark measures a whole
process.

| Recipe | Crates | Categories |
| --- | --- | --- |
| [`cargo flamegraph`][ex~development-tools_profiling~cargo-flamegraph] | [![cargo-flamegraph][c~flamegraph~docs~badge]][c~flamegraph~docs] | {{#categories development-tools::profiling=Profiling}} |
| [`criterion`][ex~development-tools_profiling~criterion] | {{#crate criterion}} | {{#categories development-tools::profiling=Profiling}} |
| [`divan`][ex~development-tools_profiling~divan] | {{#crate divan}} | {{#categories development-tools::profiling=Profiling}} |
| [`hyperfine`][ex~development-tools_profiling~hyperfine] | {{#crate hyperfine}} | {{#categories development-tools::profiling=Profiling}} |
| [Benchmark Hash Functions][ex~development-tools_profiling~hashing-benchmark] | {{#crate ahash}} | {{#categories development-tools::profiling=Profiling}} |
| [Benchmark Arena and Heap Allocation][ex~development-tools_profiling~arena-benchmark] | {{#crate bumpalo}} | {{#categories development-tools::profiling=Profiling}} |
| [Benchmark Hash Map Implementations][ex~development-tools_profiling~hashmap-benchmark] | {{#crate ahash}} | {{#categories development-tools::profiling=Profiling}} |
| [Compare Benchmark Results in CI][ex~development-tools_profiling~benchmark-regression] | {{#crate std}} | {{#categories development-tools::profiling=Profiling}} |
