# Generate Random Values

{{#include randomness.incl.md}}

## Generate random numbers

[![rand][rand-badge]][rand]  [![cat-science][cat-science-badge]][cat-science]

Generates {{i:random numbers}} with help of the {{i:random-number generator}} [`rand::Rng`][rand::Rng]. Each thread has an initialized generator. Integers are uniformly distributed over the range of the type, and floating point numbers are uniformly distributed from 0 up to but not including 1.

```rust,editable
{{#include ../../../deps/tests/rand.rs}}
```

## Generate random numbers within a range

[![rand][rand-badge]][rand]  [![cat-science][cat-science-badge]][cat-science]

Generates a {{i:random value}} within half-open `[0, 10)` range (not including `10`) with [`Rng::gen_range`][rand::Rng::gen_range] range.

```rust,editable
{{#include ../../../deps/tests/rand-range.rs}}
```

[`Uniform`][rand::distributions::uniform::Uniform] can obtain values with [uniform distribution][wikipedia-uniform-distribution]. This has the same effect, but may be faster when repeatedly generating numbers in the same range.

```rust,editable
{{#include ../../../deps/tests/rand-range1.rs}}
```

## Generate random numbers with given distribution

[![rand_distr][rand_distr-badge]][rand_distr]  [![cat-science][cat-science-badge]][cat-science]

By default, {{i:random numbers}} in the `rand` crate have [uniform distribution][wikipedia-uniform-distribution]. The [`rand-distr`][rand_distr] crate provides other kinds of {{i:distributions}}. To use them, you instantiate a distribution, then sample from that distribution using [`Distribution::sample`][rand::distributions::Distribution::sample] with help of a random-number generator [`rand::Rng`][rand::Rng]. The [distributions available are documented here][rand_distr]. An example using the [`Normal`][rand_distr::Normal] distribution is shown below.

```rust,editable,ignore
{{#include ../../../deps/tests/rand-dist.rs}}
```

## Generate random values of a custom type

[![rand][rand-badge]][rand]  [![cat-science][cat-science-badge]][cat-science]

Randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`. Implements the [`Distribution`][rand::distributions::Distribution]trait on type Point for [`Standard`][rand::distributions::Standard] trait in order to allow random generation.

```rust,editable
{{#include ../../../deps/tests/rand-custom.rs}}
```

## Create random passwords from a set of alphanumeric characters

[![rand][rand-badge]][rand]  [![cat-os][cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters in the range `A-Z, a-z, 0-9`, with [`Alphanumeric`][rand::distributions::Alphanumeric] sample.

```rust,editable
{{#include ../../../deps/tests/rand-passwd.rs}}
```

## Create {{i:random passwords}} from a set of user-defined characters

[![rand][rand-badge]][rand]  [![cat-os][cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters with custom user-defined bytestring, with [`gen_range`][rand::Rng::gen_range].

```rust,editable
{{#include ../../../deps/tests/rand-choose.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
