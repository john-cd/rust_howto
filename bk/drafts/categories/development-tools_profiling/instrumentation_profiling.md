<!-- TODO Reviewed ??-??-2026 -->

# Instrumentation Profiling

{{#include instrumentation_profiling.incl.md}}

## Mark Scopes with `profiling` {#profiling-scopes}

[![profiling][c~profiling~docs~badge]][c~profiling~docs]{{hi:profiling}} [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}{{hi:Instrumentation}}{{hi:Scope}}

[`profiling`][c~profiling~docs]↗{{hi:profiling}} is a facade: annotate functions with `#[profiling::function]` and blocks with `profiling::scope!`, then choose the backend (`puffin`, `tracy-client`, `optick`, and others) with a Cargo feature of the `profiling` crate. Without a backend feature, the macros expand to nothing, so the annotations can stay in release builds.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/examples/profiling_scopes.rs:example}}
```

Select the backend in one place, the `Cargo.toml` of the application:

```toml
[dependencies]
profiling = { version = "1", features = ["profile-with-puffin"] }
```

## Record and Serve Frames with `puffin` {#puffin-server}

[![puffin][c~puffin~docs~badge]][c~puffin~docs]{{hi:puffin}} [![puffin_http][c~puffin_http~docs~badge]][c~puffin_http~docs]{{hi:puffin_http}} [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}{{hi:Frame}}

[`puffin`][c~puffin~docs]↗{{hi:puffin}} records scopes into frames. Turn recording on with `puffin::set_scopes_on(true)`, mark the end of each loop iteration with `puffin::GlobalProfiler::lock().new_frame()`, and start a [`puffin_http::Server`][c~puffin_http::Server~docs]↗{{hi:puffin_http::Server}} to stream the frames to `puffin_viewer`.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/examples/puffin_server.rs:example}}
```

To look at the profile, install the viewer with `cargo install puffin_viewer`, run `puffin_viewer`, and connect it to `127.0.0.1:8585` while the application runs.

To test instrumented code without a server or a viewer, add a sink with `puffin::GlobalFrameView::default()`, run the code, finish a frame, and inspect `latest_frame()`. The unit tests of this example do so.

## Time a Code Block with `measure_time` {#measure-time}

[![measure_time][c~measure_time~docs~badge]][c~measure_time~docs]{{hi:measure_time}} [![measure_time~crates.io][c~measure_time~crates.io~badge]][c~measure_time~crates.io] [![measure_time~repo][c~measure_time~repo~badge]][c~measure_time~repo] [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}{{hi:Timing}}

[`measure_time`][c~measure_time~docs]↗{{hi:measure_time}} is the lightest form of instrumentation: `print_time!("label")` starts a timer that prints the elapsed time when the enclosing scope ends. Use it for a quick look at how long a block takes, not for statistically sound measurements; for those, see [Benchmarking][p~benchmarking].

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/src/memory_usage_analysis/measure_time.rs:example}}
```

## Practical Notes {#practical-notes}

- **Overhead**: each recorded scope costs tens of nanoseconds. Keep scopes out of the innermost loops, and turn recording off (`set_scopes_on(false)`) when nobody is looking. A failed `Server::new`, for instance because the port is taken, should disable profiling, not crash the application.
- **Security**: the stream is not authenticated and reveals function names and timings. Bind the server to `127.0.0.1` unless you really need remote access.
- **Other backends**: [`tracy-client`][c~tracy-client~docs]↗{{hi:tracy-client}} and [`optick`][c~optick~docs]↗{{hi:optick}} feed the Tracy and Optick profilers, which offer a richer GUI at the price of an external tool.
- **Complementary tools**: use a sampling profiler to find which function is slow in a whole program, and instrumentation to find which part of a frame is slow. See [Benchmarking][p~benchmarking] for micro-benchmarks and [Memory Usage Analysis][p~memory-usage-analysis] for allocations.

## Related Topics {#related-topics .skip}

- [Benchmarking][p~benchmarking] to measure a function in isolation.
- [Memory Usage Analysis][p~memory-usage-analysis] for allocation profiling.
- [Tracing][p~tracing] for event-based diagnostics.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [instrumentation profiling: write](https://github.com/john-cd/rust_howto/issues/1934)
</div>
