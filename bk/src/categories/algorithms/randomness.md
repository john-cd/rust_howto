# Random Value Generation

{{#include randomness.incl.md}}

`rand` is the _de facto_ standard for random number generation in Rust. Use `rand_distr` for random distributions beyond what is available in `rand`. You may also consider lightweight alternatives to `rand` such as [`fastrand`][c~fastrand~docs]. Honorable mentions include:

- `oorandom`, a minimalistic pseudorandom number generator, when the `rand` crate is just too big.
- `getrandom`, a low-level library for accessing the operating system's random number generator.

## Generate Random Numbers {#generate-random-numbers}

[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~repo][c~rand~repo~badge]][c~rand~repo] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

This example demonstrates the basic usage of the `rand` crate for generating random numbers {{hi:Random numbers}}:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand1.rs:example}}
```

## Generate Random Numbers Within a Range {#generate-random-numbers-within-a-range}

[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~repo][c~rand~repo~badge]][c~rand~repo] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

This example generates a random value{{hi:Random value}} within the half-open `[0, 10)` range (not including `10`):

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_range.rs:example}}
```

Using `rand::distr::Uniform` has the same effect but may be faster than `rand::Rng::random_range` when repeatedly generating numbers in the same range:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_range1.rs:example}}
```

## Generate Random Numbers Within a Given Distribution {#generate-random-numbers-within-a-given-distribution}

[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~repo][c~rand~repo~badge]][c~rand~repo] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[![rand_distr~website][c~rand_distr~website~badge]][c~rand_distr~website] [![rand_distr][c~rand_distr~docs~badge]][c~rand_distr~docs] [![rand_distr~crates.io][c~rand_distr~crates.io~badge]][c~rand_distr~crates.io] [![rand_distr~repo][c~rand_distr~repo~badge]][c~rand_distr~repo] [![rand_distr~lib.rs][c~rand_distr~lib.rs~badge]][c~rand_distr~lib.rs]{{hi:rand_distr}}{{hi:Random}}{{hi:Probability}}{{hi:Distribution}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

By default, random numbers{{hi:Random numbers}} in the [`rand`][c~rand~docs]↗{{hi:rand}} crate have [uniform distribution][wikipedia~uniform-distribution]↗{{hi:Uniform distribution}}. The [`rand_distr`][c~rand_distr~docs]↗{{hi:rand_distr}} crate is a super-set of the `rand::distr` module and provides other kinds of statistical distributions{{hi:Distributions}}. Sample from a distribution using the `rand::distr::Distribution::sample` function, passing a random-number generator (implementing [`rand::Rng`][c~rand::Rng~docs]↗{{hi:rand::Rng}}).

An example using the [`rand_distr::Normal`][c~rand_distr::Normal~docs]↗{{hi:rand_distr::Normal}} distribution is shown below:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_distr.rs:example}}
```

## Generate Random Values of a Custom Type {#generate-random-values-custom-type}

[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~repo][c~rand~repo~badge]][c~rand~repo] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The following randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`, by implementing the `Distribution` trait:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_custom.rs:example}}
```

## Create Random Passwords from a Set of Alphanumeric Characters {#generate-random-values-from-set-alphanumeric}

[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~repo][c~rand~repo~badge]][c~rand~repo] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} {{hi:Algorithms}}{{hi:Random passwords}}

In this example, we randomly generate strings of given length from ASCII characters in the range `A-Z, a-z, 0-9`, using the `Alphanumeric` distribution:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_alphanumeric.rs:example}}
```

## Create Random Passwords from a Set of User-defined Characters {#create-random-passwords-from-a-set-of-user-defined-characters}

[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~repo][c~rand~repo~badge]][c~rand~repo] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The following randomly generates a string of given length (e.g. a password) using a predefined character set:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/rand_password.rs:example}}
```

## Generate Random Numbers with `fastrand` {#fastrand}

[![fastrand][c~fastrand~docs~badge]][c~fastrand~docs] [![fastrand~crates.io][c~fastrand~crates.io~badge]][c~fastrand~crates.io] [![fastrand~repo][c~fastrand~repo~badge]][c~fastrand~repo] [![fastrand~lib.rs][c~fastrand~lib.rs~badge]][c~fastrand~lib.rs]{{hi:fastrand}}{{hi:Rand}}{{hi:Random}}{{hi:Wyrand}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

[`fastrand`][c~fastrand~docs]↗{{hi:fastrand}} is a simple and fast random number generator. It has no dependencies and lower complexity than [`rand`][c~rand~docs]↗{{hi:rand}} but is NOT cryptographically secure:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/rand/fastrand.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[algorithms | Algorithms]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- `passwords` provides useful tools to generate multiple readable passwords, as well as analyze and score them.
- `rand_regex` generates random strings and byte strings matching a regex.

</div>
