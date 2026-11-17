# TODO Reviewed ??-??-2026.

# Purpose: Compare two equivalent sort workloads with Hyperfine and save their measurements.
# Usage: Run with bash bk/crates/cats/development_tools_profiling/examples/benchmarking/hyperfine.sh from bk/crates.
#
cargo build --release -p development_tools_profiling --example regression_workload

binary=target/release/examples/regression_workload
if [ -f "${binary}.exe" ]; then
    binary="${binary}.exe"
fi

hyperfine --warmup 3 --runs 10 --export-markdown results.md \
    --command-name "stable sort" "${binary} stable" \
    --command-name "unstable sort" "${binary} unstable"
