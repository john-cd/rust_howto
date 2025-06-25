# Complex Numbers

{{#include complex_numbers.incl.md}}

## Create Complex Numbers {#creating-complex-numbers}

[![num][c-num-badge]][c-num]{{hi:num}}
[![num-crates.io][c-num-crates.io-badge]][c-num-crates.io]
[![num-github][c-num-github-badge]][c-num-github]
[![num-lib.rs][c-num-lib.rs-badge]][c-num-lib.rs]
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Creates complex numbers of type [`num::complex::Complex`][c-num::complex::Complex]{{hi:num::complex::Complex}}⮳. Both the real and imaginary part of the complex number must be of the same type.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/create_complex.rs:example}}
```

## Add Complex Numbers {#adding-complex-numbers}

[![num][c-num-badge]][c-num]{{hi:num}}
[![num-crates.io][c-num-crates.io-badge]][c-num-crates.io]
[![num-github][c-num-github-badge]][c-num-github]
[![num-lib.rs][c-num-lib.rs-badge]][c-num-lib.rs]
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Performing mathematical operations on complex numbers is the same as on built-in types: the numbers in question must be of the same type (i.e. floats or integers).

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/add_complex.rs:example}}
```

## Use Mathematical Functions on Complex Numbers {#mathematical-functions}

[![num][c-num-badge]][c-num]{{hi:num}}
[![num-crates.io][c-num-crates.io-badge]][c-num-crates.io]
[![num-github][c-num-github-badge]][c-num-github]
[![num-lib.rs][c-num-lib.rs-badge]][c-num-lib.rs]
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Complex numbers have a range of interesting properties when it comes to how they interact with other mathematical functions, most notably the family of sine functions as well as the number e. To use these [functions][p-functions] with complex numbers, the Complex type has a few built-in [functions][p-functions], all of which can be found here: [`num::complex::Complex`][c-num::complex::Complex]{{hi:num::complex::Complex}}⮳.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/mathematical_functions.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[final review](https://github.com/john-cd/rust_howto/issues/935)
</div>
