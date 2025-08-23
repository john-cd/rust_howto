# Property-Based Testing

{{#include property_based_testing.incl.md}}

Property-based testing involves defining properties that your code should satisfy for a wide range of input values. You describe the general behavior of the program, and the testing framework generates random inputs to check if the properties hold true. Example:

Suppose you have a [sorting][p~sorting] function. A property you could test is that the output list should have the same length as the input list, and the elements should be in an increasing order.

Consider using:

- [`proptest`][c~proptest~docs]↗{{hi:proptest}}: A powerful crate for property-based testing, where you define properties that your code should satisfy, and [`proptest`][c~proptest~docs]↗{{hi:proptest}} generates many random inputs to verify those properties.
- [`quickcheck`][c~quickcheck~docs]↗{{hi:quickcheck}}: Another property-based testing crate. `proptest` is often preferred for its flexibility.

## `proptest` {#proptesting}

[![proptest~website][c~proptest~website~badge]][c~proptest~website] [![proptest][c~proptest~docs~badge]][c~proptest~docs] [![proptest~crates.io][c~proptest~crates.io~badge]][c~proptest~crates.io] [![proptest~repo][c~proptest~repo~badge]][c~proptest~repo] [![proptest~lib.rs][c~proptest~lib.rs~badge]][c~proptest~lib.rs]{{hi:proptest}}{{hi:Fuzz}}{{hi:Hypothesis}}{{hi:Property}}{{hi:Quickcheck}}{{hi:Testing}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

Hypothesis-like property-based [testing][p~testing] and shrinking.

```toml
[dev-dependencies]
proptest = "1.0.0"
```

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/property_based_testing/proptest.rs:example}}
```

## Test Data Generation with `fake` {#test_data_generation}

[![fake][c~fake~docs~badge]][c~fake~docs] [![fake~crates.io][c~fake~crates.io~badge]][c~fake~crates.io] [![fake~repo][c~fake~repo~badge]][c~fake~repo] [![fake~lib.rs][c~fake~lib.rs~badge]][c~fake~lib.rs]{{hi:fake}}{{hi:Random}}{{hi:Generator}}{{hi:Data}}{{hi:Faker}}

An easy to use library and command line for generating fake data like name, number, address, lorem, dates, etc.

Often done with custom [functions][p~functions] or data structures, but crates like [`fake`][c~fake~docs]↗{{hi:fake}} can be useful for generating realistic test data.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/property_based_testing/fake.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
cover property [testing][p~testing] [proptest][c~proptest~repo]
</div>
