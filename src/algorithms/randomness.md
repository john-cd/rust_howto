# Generate Random Values

## Generate random numbers

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates random numbers with help of random-number
generator [`rand::Rng`][rand::Rng] obtained via [`rand::thread_rng`][rand::Rng] obtained via [rand::thread_rng] Each thread has an initialized generator. Integers are uniformly distributed over the range of the type, and floating point numbers are uniformly distributed from 0 up to but not including 1.

```rust,editable
{{#include ../../deps/examples/rand.rs}}
```

## Generate random numbers within a range

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates a random value within half-open `[0, 10)` range (not including `10`) with [`Rng::gen_range`][Rng::gen_range] range.

```rust,editable
{{#include ../../deps/examples/rand-range.rs}}
```

[`Uniform`][Uniform] can obtain values with [uniform distribution].
This has the same effect, but may be faster when repeatedly generating numbers in the same range.

```rust,editable
{{#include ../../deps/examples/rand-range1.rs}}
```

## Generate random numbers with given distribution

[![rand-distr-badge]][rand-distr] [![cat-science-badge]][cat-science]

By default, random numbers in the `rand` crate have [uniform distribution]. The [`rand-distr`][rand-distr] crate provides
other kinds of distributions. To use them, you instantiate a distribution, then sample from that distribution using[`Distribution::sample`][Distribution::sample] with help of a random-number generator [`rand::Rng`][rand::Rng].
The [distributions available are documented here][rand-distributions]. An example using the [`Normal`][Normal] distribution is shown below.

```rust,editable,ignore
{{#include ../../deps/examples/rand-dist.rs}}
```

## Generate random values of a custom type

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`.
Implements the [`Distribution`][Distribution] trait on type Point for [`Standard`][Standard] trait in order to allow random generation.

```rust,editable
{{#include ../../deps/examples/rand-custom.rs}}
```

## Create random passwords from a set of alphanumeric characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters in the range `A-Z, a-z, 0-9`, with [`Alphanumeric`][Alphanumeric] sample.

```rust,editable
{{#include ../../deps/examples/rand-passwd.rs}}
```

## Create random passwords from a set of user-defined characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters with custom user-defined bytestring, with [`gen_range`][gen_range].

```rust,editable
{{#include ../../deps/examples/rand-choose.rs}}
```

{{#include ../refs/link-refs.md}}
