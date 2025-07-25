# Generate Random Values

{{#include randomness.incl.md}}

## Generate Random Numbers {#generate-random-numbers}

[![rand][c~rand~docs~badge]][c~rand~docs] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Generates random numbers{{hi:Random numbers}} with help of the random-number generator{{hi:Random-number generator}} [`rand::Rng`][c~rand::Rng~docs]{{hi:rand::Rng}}. Each thread has an initialized generator. Integers are uniformly distributed over the range of the type, and floating point numbers are uniformly distributed from 0 up to but not including 1.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand1.rs:example}}
```

## Generate Random Numbers Within a Range {#generate-random-numbers-within-a-range}

[![rand][c~rand~docs~badge]][c~rand~docs] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Generates a random value{{hi:Random value}} within half-open `[0, 10)` range (not including `10`) with [`rand::Rng::gen_range`][c~rand::Rng::gen_range~docs]{{hi:rand::Rng::gen_range}}⮳ range.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_range.rs:example}}
```

[`rand::distributions::uniform::Uniform`][c~rand::distributions::uniform::Uniform~docs]{{hi:rand::distributions::uniform::Uniform}} can obtain values with [uniform distribution][wikipedia~uniform-distribution]{{hi:Uniform distribution}}. This has the same effect, but may be faster when repeatedly generating numbers in the same range.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_range1.rs:example}}
```

## Generate Random Numbers Within a Given Distribution {#generate-random-numbers-within-a-given-distribution}

[![rand][c~rand~docs~badge]][c~rand~docs] [![rand_distr][c~rand_distr~docs~badge]][c~rand_distr~docs] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

By default, random numbers{{hi:Random numbers}} in the [`rand`][c~rand~docs]{{hi:rand}}⮳ crate have [uniform distribution][wikipedia~uniform-distribution]{{hi:Uniform distribution}}⮳. The [`rand_distr`][c~rand_distr~docs]{{hi:rand_distr}}⮳ crate provides other kinds of distributions{{hi:Distributions}}. To use them, you instantiate a distribution, then sample from that distribution using [`rand::distributions::Distribution::sample`][c~rand::distributions::Distribution::sample~docs]{{hi:rand::distributions::Distribution::sample}}⮳ with help of a random-number generator [`rand::Rng`][c~rand::Rng~docs]{{hi:rand::Rng}}⮳. The distributions available are documented [here][c~rand_distr~docs]⮳. An example using the [`rand_distr::Normal`][c~rand_distr::Normal~docs]{{hi:rand_distr::Normal}}⮳ distribution is shown below.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_dist.rs:example}}
```

## Generate Random Values of a Custom Type {#generate-random-values-custom-type}

[![rand][c~rand~docs~badge]][c~rand~docs] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`. Implements the [`rand::distributions::Distribution`][c~rand::distributions::Distribution~docs]{{hi:rand::distributions::Distribution}}⮳ trait on type Point for [`rand::distributions::Standard`][c~rand::distributions::Standard~docs]{{hi:rand::distributions::Standard}}⮳ trait in order to allow random generation.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_custom.rs:example}}
```

## Create Random Passwords from a Set of Alphanumeric Characters {#generate-random-values-from-set-alphanumeric}

[![rand][c~rand~docs~badge]][c~rand~docs] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}{{hi:Random passwords}}

Randomly generates a string of given length ASCII characters in the range `A-Z, a-z, 0-9`, with [`rand::distributions::Alphanumeric`][c~rand::distributions::Alphanumeric~docs]{{hi:rand::distributions::Alphanumeric}}⮳ sample.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_passwd.rs:example}}
```

## Create Random Passwords from a Set of User-defined Characters {#create-random-passwords-from-a-set-of-user-defined-characters}

[![rand][c~rand~docs~badge]][c~rand~docs] [![cat~os][cat~os~badge]][cat~os]{{hi:OS}}

Randomly generates a string of given length ASCII characters with custom user-defined bytestring, with [`rand::Rng::gen_range`][c~rand::Rng::gen_range~docs]{{hi:rand::Rng::gen_range}}.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_choose.rs:example}}
```

## `fastrand` {#fastrand}

[![fastrand][c~fastrand~docs~badge]][c~fastrand~docs] [![fastrand~crates.io][c~fastrand~crates.io~badge]][c~fastrand~crates.io] [![fastrand~github][c~fastrand~github~badge]][c~fastrand~github] [![fastrand~lib.rs][c~fastrand~lib.rs~badge]][c~fastrand~lib.rs]{{hi:fastrand}}{{hi:Fast}}{{hi:Rand}}{{hi:Random}}{{hi:Simple}}{{hi:Wyrand}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

[`fastrand`][c~fastrand~docs]⮳{{hi:fastrand}} is a simple and fast random number generator. No dependencies, non-cryptographically secure random numbers, lower complexity than [`rand`][c~rand~docs]⮳{{hi:rand}}.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/fastrand.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1390)
</div>
