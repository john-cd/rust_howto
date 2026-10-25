<!-- TODO Reviewed ??-??-2026 -->

Benchmark first to establish a repeatable baseline; profile when you need to
find where the time or memory goes. Use representative inputs, keep the
compiler, target, machine, and workload constant between runs, and compare
several measurements instead of relying on one sample. A function-level
benchmark measures a small hot path; a command-line benchmark measures a whole
process.

| Recipe                                                                | Crates                                                            | Categories                                                                                                      |
| --------------------------------------------------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| [`cargo flamegraph`][ex~development-tools_profiling~cargo-flamegraph] | [![cargo-flamegraph][c~flamegraph~docs~badge]][c~flamegraph~docs] | [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling] |
| [`criterion`][ex~development-tools_profiling~criterion]               | [![criterion][c~criterion~docs~badge]][c~criterion~docs]          | [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling] |
| [`divan`][ex~development-tools_profiling~divan]                       | [![divan][c~divan~docs~badge]][c~divan~docs]                      | [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling] |
| [`hyperfine`][ex~development-tools_profiling~hyperfine]               | [![hyperfine][c~hyperfine~docs~badge]][c~hyperfine~docs]          | [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling] |
