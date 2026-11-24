<!-- TODO Reviewed ??-??-2026. -->

# Property-Based Testing

{{#include property_based_testing.incl.md}}

Property-based testing involves defining properties that your code should satisfy for a wide range of input values. You describe the general behavior of the program, and the testing framework generates random inputs to check if the properties hold true. Example:

A [sorting](../algorithms/sorting.md) function should return the same elements in nondecreasing order. These properties apply to empty inputs and repeated values as well as inputs with distinct elements.

Consider the following crates for property-based testing:

- [`proptest`][c~proptest~docs]↗{{hi:proptest}} generates test inputs and checks each one against the properties you specify.
- [`quickcheck`][c~quickcheck~docs]↗{{hi:quickcheck}} is another property-based testing crate. `proptest` is often preferred for its flexibility.

## `proptest` {#proptesting}

[![proptest~website][c~proptest~website~badge]][c~proptest~website] {{#badges proptest}} {{hi:Fuzz}} {{hi:Hypothesis}} {{hi:Property}} {{hi:Quickcheck}} {{hi:Testing}} {{#categories development-tools::testing=Testing}}

Property-based testing generates inputs and shrinks failing cases to a smaller counterexample.

```toml
[dev-dependencies]
proptest = "1.0.0"
```

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/property_based_testing/proptest.rs:example}}
```

## Test Data Generation with `fake` {#test_data_generation}

{{#badges fake}}{{hi:Random}}{{hi:Generator}}{{hi:Data}}{{hi:Faker}}

An easy to use library and command line for generating fake data like name, number, address, lorem, dates, etc.

Custom [functions](../../language/functions.md) and data structures can generate test data; [`fake`][c~fake~docs]↗{{hi:fake}} is useful when examples need realistic names, addresses, or other values.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p development_tools_testing --example property_based_testing`.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_testing/examples/property_based_testing/fake.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Fuzzing](fuzzing.md).
- [Testing](testing.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
