# Property-Based Testing

{{#include property_based_testing.incl.md}}

Property-based testing involves defining properties that your code should satisfy for a wide range of input values. You describe the general behavior of the program, and the testing framework generates random inputs to check if the properties hold true. Example:

Suppose you have a sorting function. A property you could test is that the output list should have the same length as the input list, and the elements should be in an increasing order.

Consider using:

- `proptest`: A powerful crate for property-based testing, where you define properties that your code should satisfy, and proptest generates many random inputs to verify those properties.
- `quickcheck`: Another property-based testing crate. proptest is often preferred for its flexibility.

## `proptest` {#proptesting}

[![proptest-website][c-proptest-website-badge]][c-proptest-website] [![proptest][c-proptest-badge]][c-proptest] [![proptest-crates.io][c-proptest-crates.io-badge]][c-proptest-crates.io] [![proptest-github][c-proptest-github-badge]][c-proptest-github] [![proptest-lib.rs][c-proptest-lib.rs-badge]][c-proptest-lib.rs]{{hi:proptest}}{{hi:Fuzz}}{{hi:Hypothesis}}{{hi:Property}}{{hi:Quickcheck}}{{hi:Testing}} [![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::testing]{{hi:Testing}}

Hypothesis-like property-based testing and shrinking.

```toml
[dev-dependencies]
proptest = "1.0.0"
```

{{#example proptest}}

## Test Data Generation {#test_data_generation}

[![faker][c-faker-badge]][c-faker] [![faker-crates.io][c-faker-crates.io-badge]][c-faker-crates.io] [![faker-github][c-faker-github-badge]][c-faker-github] [![faker-lib.rs][c-faker-lib.rs-badge]][c-faker-lib.rs]{{hi:faker}}

A library for generating fake data such as names, addresses, and phone numbers.

Often done with custom functions or data structures, but crates like `faker` can be useful for generating realistic test data.

{{#example faker}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 cover property [testing][p-testing] [proptest](https://github.com/proptest-rs/proptest)

</div>
