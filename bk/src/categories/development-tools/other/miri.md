<!-- TODO Reviewed ??-??-2026. -->

# Miri Interpreter

{{#include miri.incl.md}}

## Detect Undefined Behavior with the `miri` Interpreter {#miri}

[![miri~repo][miri~repo~badge]][miri~repo]{{hi:miri}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`miri`][miri~repo]↗ is an experimental interpreter for Rust's mid-level intermediate representation (MIR). It runs Rust programs and tests under interpretation and can detect certain forms of undefined behavior on the paths they execute, including some violations of unsafe-code requirements. It does not prove that unexecuted paths are free of undefined behavior.

## Install the `miri` Interpreter {#miri-installation}

Miri is distributed as a component of the nightly Rust toolchain. Install it and prepare its sysroot once:

```bash
rustup toolchain install nightly --component miri
cargo +nightly miri setup
```

Run tests or a binary with the nightly toolchain:

```bash
cargo +nightly miri test
cargo +nightly miri test test_name_filter
cargo +nightly miri run
```

Miri interprets the Rust code and can detect certain forms of undefined behavior on the paths that execute. It is especially useful for tests that exercise unsafe code, but a passing run does not prove that a program is free of undefined behavior. Some foreign-function calls and operating-system operations are unsupported or behave differently under interpretation; consult the [Miri guide][miri~docs]↗ when a test cannot run.

For CI, install the nightly toolchain with the Miri component, run `cargo +nightly miri setup`, then run the project's selected tests with `cargo +nightly miri test`.

## Related Topics {#related-topics .skip}

- [Code Verification](code_verification.md).
- [Testing](../../../../src/categories/development-tools_testing/testing.md).

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

