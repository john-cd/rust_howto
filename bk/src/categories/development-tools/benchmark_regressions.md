<!-- TODO Reviewed ??-??-2026. -->

# Compare Benchmark Results in CI

A benchmark compares the same representative workload at two revisions under the same build profile, toolchain, and machine. This example sorts a fixed input and prints a checksum so the comparison can reject a change in output:

```rust
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/regression_workload.rs:example}}
```

Build the baseline and candidate from separate checkouts of the same repository, using the same release profile. Keep their Cargo target directories separate so the second build cannot replace the first executable. For this book's workspace, from each checkout run `cargo build --release --manifest-path bk/crates/Cargo.toml -p development_tools_profiling --example regression_workload`. Copy the two resulting executables to `baseline` and `candidate` paths, then compare them:

```console
python bk/scripts/compare_benchmarks.py baseline candidate --threshold 0.10 --output benchmark-results.json
```

On Windows, use the `.exe` filenames. The script warms both binaries, checks that their output matches, and interleaves 15 timed runs per binary. It reports medians and a trimmed range divided by the median. A ratio above 1.10 fails only when both spreads are at most 5%; noisier results are inconclusive and should be rerun on a stable runner. The threshold is a policy choice, not a universal performance guarantee. Keep the raw timings in CI artifacts and inspect a regression before changing code; hosted runners, CPU contention, thermal state, input size, and I/O can all affect results. For serious performance gates, use a dedicated runner and multiple independent CI runs.

For an actual CI job, check out the base and proposed revision into separate directories, build both with the same pinned toolchain, and invoke the command above with their executable paths. See [This Week in Rust #677](https://this-week-in-rust.org/blog/2026/11/11/this-week-in-rust-677/) for the article that prompted this recipe.
