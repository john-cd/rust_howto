<!-- TODO Reviewed ??-??-2026 -->

# Assertions

{{#include assertions.incl.md}}

Assertions make a test fail at the point where an expected condition is false. The standard library macros cover most checks; assertion crates add focused comparisons for cases such as floating-point values.

## Standard Library Assertions {#standard-library-macros}

Use `assert!` for a boolean condition, `assert_eq!` and `assert_ne!` to compare values, and `matches!` to check a pattern. Equality assertions print both values when they fail, so they usually give a more useful diagnostic than writing `assert!(left == right)`.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
enum Reply {
    Ready { items: usize },
    Missing,
}

#[test]
fn response_has_the_expected_shape() {
    let reply = Reply::Ready { items: 3 };

    assert!(matches!(reply, Reply::Ready { items } if items > 0));
    assert_eq!(reply, Reply::Ready { items: 3 });
    assert_ne!(reply, Reply::Missing);
}
```

The standard macros also accept a custom panic message. Use `debug_assert!`, `debug_assert_eq!`, and related macros only for checks that may be disabled in optimized builds; they must not enforce input validation or other required behavior.

## Emit a Custom Message {#custom-message}

[![std][c~std~docs~badge]][c~std~docs] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/assertions/tests_custom_message.rs:example}}
```

## Check a Pattern with `matches!` or `assert_matches` {#assertion_libraries}

{{#badges assert_matches no-std="No standard library"}}{{hi:Pattern}}{{hi:Assert}}{{hi:Match}}

The built-in `matches!` macro returns a boolean, so `assert!(matches!(value, Pattern))` is a stable way to assert a variant or a pattern with a guard. The third-party [`assert_matches`][c~assert_matches~docs]↗{{hi:assert_matches}} crate offers a dedicated assertion macro when that syntax is preferred.

## `approx` {#approx}

{{#badges approx development-tools::testing=Testing}}

[`approx`][c~approx~docs]↗{{hi:approx}} allows approximate floating point equality comparisons and assertions.

Floating-point calculations round, so exact equality is often the wrong requirement. Choose an absolute or relative tolerance that matches the scale and accuracy expected by the application; do not treat one default tolerance as suitable for every calculation.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/assertions/approx.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[testing | Testing]] describes unit, integration, and documentation tests.
- [[test_runners | Test Runners]] covers running and filtering tests.
- [[property_based_testing | Property-Based Testing]] checks properties across generated inputs.

Assertions are for tests and internal invariants. Return a `Result` for expected failures such as invalid user input, missing files, or network errors; panicking is not ordinary error handling.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
write

- [pretty_assertions - Rust dev tool][c~pretty_assertions~lib.rs]↗.

</div>
