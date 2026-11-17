<!-- TODO Reviewed ??-??-2026. -->

# Find Missing Assertions with Mutation Testing {#mutation-testing}

Line coverage says code ran; it does not show that a test would catch a changed boundary. This function classifies values below ten:

```rust
{{#include ../../../crates/cats/mutation_boundary/src/lib.rs:example}}
```

The first test used only `5`, so changing `< 10` to `<= 10` left it green. With `cargo-mutants` 27.1.0, list and run that one mutation from `bk/crates`:

```sh
cargo mutants -p mutation_boundary --file cats/mutation_boundary/src/lib.rs --list
cargo mutants -p mutation_boundary --file cats/mutation_boundary/src/lib.rs --re 'replace < with <=' --timeout 30 --in-place
```

With only `classify(5)` asserted, cargo-mutants reported `MISSED` for the selected mutation. Add `assert_eq!(classify(10), "large")` to the test and run the same command again; the mutant is now detected. The checked-in test includes that assertion. The `--in-place` flag is needed for this repository because an external workspace path is not copied into cargo-mutants' scratch tree; use it only in an isolated, clean checkout and confirm it restores the source. Use `cargo test -p mutation_boundary` for the ordinary test run. Mutation tools can also produce equivalent mutants that cannot be detected by any test and mutants that time out; review each surviving result before treating it as a missing assertion. A focused CI run on changed files or selected functions can be more practical than mutating an entire workspace. See the [cargo-mutants guide](https://mutants.rs/) and a [worked example](https://blog.frankel.ch/mutation-testing-rust/).
