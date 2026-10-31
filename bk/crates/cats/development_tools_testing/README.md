<!-- TODO Reviewed ??-??-2026 -->

# Development Tools Testing

Examples of advanced testing techniques including property-based testing, mocking, and fuzzing. It demonstrates integration with key ecosystem crates including `afl`, `anyhow`, `approx`, `arbitrary`, `assert_cmd`, `bolero`.

## Examples Covered

- **Assertions** (`examples/assertions/`): Demonstrates concepts like `approx`, `tests_custom_message`.
- **CLI Testing** (`examples/cli_testing/`): Demonstrates concepts like `cli_demo`, `lcov_summary`.
- **Code Coverage** (`examples/code_coverage/`): Code examples covering code coverage.
- **Fuzzing** (`examples/fuzzing/`): Demonstrates concepts like `afl`, `bolero_example`.
- **Mocking** (`examples/mocking/`): Demonstrates concepts like `faux`, `mockall`.
- **Property Based Testing** (`examples/property_based_testing/`): Demonstrates concepts like `fake`, `proptest`.
- **Predicates** (`examples/predicates/`): Demonstrates combining predicates to filter records.
- **Test Runners** (`examples/test_runners/`): Demonstrates concepts like `tests1`.
- **Test Techniques** (`examples/test_techniques/`): Demonstrates concepts like `env_vars`, `float_asserts`, `http_mock`, `paused_time`, `proptest_strategy`, `rstest_cases`, `snapshot_redactions`.
- **Testing** (`examples/testing/`): Demonstrates concepts like `insta`.

## Usage

To check or run examples in this crate:

```console
cargo check --package development_tools_testing --examples
cargo test --package development_tools_testing
```

Code examples correspond to chapters in `bk/src/categories/development_tools_testing` or related book sections.
