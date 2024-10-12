# Generate Random Values

{{#include randomness.incl.md}}

## Generate random numbers

[![rand][c-rand-badge]][c-rand]  [![cat-science][cat-science-badge]][cat-science]

Generates random numbers{{hi:Random numbers}} with help of the random-number generator{{hi:Random-number generator}} [`rand::Rng`][c-rand::Rng]{{hi:rand::Rng}}. Each thread has an initialized generator. Integers are uniformly distributed over the range of the type, and floating point numbers are uniformly distributed from 0 up to but not including 1.

```rust,editable
{{#include ../../../deps/tests/rand.rs}}
```

## Generate random numbers within a range

[![rand][c-rand-badge]][c-rand]  [![cat-science][cat-science-badge]][cat-science]

Generates a random value{{hi:Random value}} within half-open `[0, 10)` range (not including `10`) with [`rand::Rng::gen_range`][c-rand::Rng::gen_range]{{hi:rand::Rng::gen_range}}⮳ range.

```rust,editable
{{#include ../../../deps/tests/rand-range.rs}}
```

[`rand::distributions::uniform::Uniform`][c-rand::distributions::uniform::Uniform]{{hi:rand::distributions::uniform::Uniform}} can obtain values with [uniform distribution][wikipedia-uniform-distribution]{{hi:Uniform distribution}}. This has the same effect, but may be faster when repeatedly generating numbers in the same range.

```rust,editable
{{#include ../../../deps/tests/rand-range1.rs}}
```

## Generate random numbers with given distribution

[![rand_distr][c-rand_distr-badge]][c-rand_distr]  [![cat-science][cat-science-badge]][cat-science]

By default, random numbers{{hi:Random numbers}} in the [`rand`][c-rand]{{hi:rand}}⮳ crate have [uniform distribution][wikipedia-uniform-distribution]{{hi:Uniform distribution}}⮳. The [`rand-distr`][c-rand_distr]{{hi:rand-distr}}⮳ crate provides other kinds of distributions{{hi:Distributions}}. To use them, you instantiate a distribution, then sample from that distribution using [`rand::distributions::Distribution::sample`][c-rand::distributions::Distribution::sample]{{hi:rand::distributions::Distribution::sample}}⮳ with help of a random-number generator [`rand::Rng`][c-rand::Rng]{{hi:rand::Rng}}⮳. The distributions available are documented [here][c-rand_distr]⮳. An example using the [`rand_distr::Normal`][c-rand_distr::Normal]{{hi:rand_distr::Normal}}⮳ distribution is shown below.

```rust,editable,ignore
{{#include ../../../deps/tests/rand-dist.rs}}
```

## Generate random values of a custom type

[![rand][c-rand-badge]][c-rand]  [![cat-science][cat-science-badge]][cat-science]

Randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`. Implements the [`rand::distributions::Distribution`][c-rand::distributions::Distribution]{{hi:rand::distributions::Distribution}}⮳ trait on type Point for [`rand::distributions::Standard`][c-rand::distributions::Standard]{{hi:rand::distributions::Standard}}⮳ trait in order to allow random generation.

```rust,editable
{{#include ../../../deps/tests/rand-custom.rs}}
```

## Create random passwords from a set of alphanumeric characters

[![rand][c-rand-badge]][c-rand]  [![cat-os][cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters in the range `A-Z, a-z, 0-9`, with [`rand::distributions::Alphanumeric`][c-rand::distributions::Alphanumeric]{{hi:rand::distributions::Alphanumeric}}⮳ sample.

```rust,editable
{{#include ../../../deps/tests/rand-passwd.rs}}
```

## Create random passwords from a set of user-defined characters

[![rand][c-rand-badge]][c-rand]  [![cat-os][cat-os-badge]][cat-os] {{hi:Random passwords}}

Randomly generates a string of given length ASCII characters with custom user-defined bytestring, with [`rand::Rng::gen_range`][c-rand::Rng::gen_range]{{hi:rand::Rng::gen_range}}.

```rust,editable
{{#include ../../../deps/tests/rand-choose.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
TODO:
</div>
