# Complex Numbers

{{#include complex_numbers.incl.md}}

## Create Complex Numbers {#creating-complex-numbers}

[![num][c~num~docs~badge]][c~num~docs]{{hi:num}}
[![num~crates.io][c~num~crates.io~badge]][c~num~crates.io]
[![num~repo][c~num~repo~badge]][c~num~repo]
[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]
[![cat~science][cat~science~badge]][cat~science]
[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The following example creates complex numbers of type [`num::complex::Complex`][c~num::complex::Complex~docs]↗{{hi:num::complex::Complex}}. Note that both the real and imaginary part of the complex number must be of the same type:

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/create_complex.rs:example}}
```

## Add Complex Numbers {#adding-complex-numbers}

[![num][c~num~docs~badge]][c~num~docs]{{hi:num}}
[![num~crates.io][c~num~crates.io~badge]][c~num~crates.io]
[![num~repo][c~num~repo~badge]][c~num~repo]
[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]
[![cat~science][cat~science~badge]][cat~science]
[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The following demonstrates performing mathematical operations on complex numbers the same way than on built-in types. The numbers in question must be of the same type (i.e. floats or integers):

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/add_complex.rs:example}}
```

## Use Mathematical Functions on Complex Numbers {#mathematical-functions}

[![num][c~num~docs~badge]][c~num~docs]{{hi:num}}
[![num~crates.io][c~num~crates.io~badge]][c~num~crates.io]
[![num~repo][c~num~repo~badge]][c~num~repo]
[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]
[![cat~science][cat~science~badge]][cat~science]
[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

Complex numbers have a range of interesting properties when it comes to how they interact with other mathematical functions, most notably the family of sine functions as well as the number e. To use these [functions][p~functions] with complex numbers, the Complex type has a few built-in [functions][p~functions], all of which can be found here: [`num::complex::Complex`][c~num::complex::Complex~docs]↗{{hi:num::complex::Complex}}.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/mathematical_functions.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[additional_numeric_types | Additional Numeric Types]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[final review](https://github.com/john-cd/rust_howto/issues/935)
</div>
