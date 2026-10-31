<!-- TODO Reviewed ??-??-2026 -->

# Code Coverage

{{#include code_coverage.incl.md}}

Code coverage records which instrumented parts of a program ran during a selected test run. It helps find code that tests never reached; it cannot tell whether the tests checked the right result or whether the behavior is correct.

## Choose and Read Coverage Metrics

Tools may report line, function, region, or branch coverage. These measurements answer different questions, and not every tool or configuration reports every metric. A line marked covered only means execution reached it; it does not prove that an assertion would detect an incorrect result.

Compare reports only when the source revision, test selection, features, target, and instrumentation settings are comparable. Include meaningful test configurations, such as feature combinations or platform-specific code, and decide whether examples and documentation tests belong in the report.

Use uncovered lines and branches to identify behaviors that lack tests. Exclude code from a report only for a clear reason, and keep exclusions narrow. Treat a percentage as a diagnostic signal rather than a quality target.

## Measure Coverage with cargo llvm-cov {#cargo-llvm-cov}

[`cargo llvm-cov`][cargo-llvm-cov~repo] wraps Rust's LLVM source-based coverage instrumentation and can run tests and produce summary, HTML, or LCOV reports.

Install the Cargo subcommand, then run a workspace report:

```sh
cargo install cargo-llvm-cov --locked
cargo llvm-cov --workspace --all-features --html --open
```

For a report that a CI service can consume, write an LCOV file:

```sh
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
```

Coverage of documentation tests may require explicit options and toolchain support. Check the tool's current documentation and make sure the generated report includes the test types you intend to measure.

## Collect Coverage with grcov {#coverage_testing}

{{#badges grcov command-line-utilities="Command line utilities" development-tools="Development tools" development-tools::testing=Testing}}{{hi:Coverage}}

grcov collects and combines Rust coverage data and can emit formats such as HTML and LCOV. It is useful when a project already has an LLVM instrumentation workflow or needs its supported report formats. Follow its documentation for the compiler flags and profile-data handling required by your setup.

## Measure Coverage with cargo-tarpaulin {#cargo-tarpaulin}

[![cargo-tarpaulin~repo][c~cargo-tarpaulin~repo~badge]][c~cargo-tarpaulin~repo]{{hi:cargo-tarpaulin}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

cargo-tarpaulin runs Rust tests and reports coverage. Its available backends and supported targets differ by operating system, so check those limits before adding it to local or continuous-integration workflows.

```sh
cargo tarpaulin --workspace --out Html
```

## Read an LCOV Report with `lcov` {#lcov}

{{#badges lcov development-tools::testing=}}

Coverage tools such as [`cargo llvm-cov`][cargo-llvm-cov~repo] and grcov write LCOV files. [`lcov::Reader`][c~lcov~docs]↗{{hi:lcov}} streams the records of a report (`SF` source file, `LF` lines found, `LH` lines hit, `end_of_record`). The example computes line coverage per file and fails if a file is below a threshold, which is a simple coverage gate for CI.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_testing/examples/cli_testing/lcov_summary.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[testing | Testing]]
- [[property_based_testing | Property-based Testing]]
- [[fuzzing | Fuzzing]]
- [[test_runners | Test Runners]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  write
</div>
