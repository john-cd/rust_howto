# Purpose: Profile the example workload and write a flamegraph.
# Usage: Run with bash bk/crates/cats/development_tools_profiling/examples/benchmarking/flamegraph.sh from the example workspace, as described in the book.
#
cargo flamegraph -p development_tools_profiling --example benchmarking
cargo flamegraph -p development_tools_profiling --bench divan
