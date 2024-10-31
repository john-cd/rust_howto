# Complex numbers

{{#include complex_numbers.incl.md}}

## Creating complex numbers

[![num][c-num-badge]][c-num]{{hi:num}}  [![cat-mathematics][cat-mathematics-badge]][cat-mathematics]{{hi:Mathematics}}  [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

Creates complex numbers of type [`num::complex::Complex`][c-num::complex::Complex]{{hi:num::complex::Complex}}⮳. Both the real and imaginary part of the complex number must be of the same type.

```rust
{{#include ../../../deps/tests/create_complex.rs}}
```

## Adding complex numbers

[![num][c-num-badge]][c-num]{{hi:num}}  [![cat-mathematics][cat-mathematics-badge]][cat-mathematics]{{hi:Mathematics}}  [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

Performing mathematical operations on complex numbers is the same as on built in types: the numbers in question must be of the same type (i.e. floats or integers).

```rust
{{#include ../../../deps/tests/add_complex.rs}}
```

## Mathematical functions

[![num][c-num-badge]][c-num]{{hi:num}}  [![cat-mathematics][cat-mathematics-badge]][cat-mathematics]{{hi:Mathematics}}  [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

Complex numbers have a range of interesting properties when it comes to how they interact with other mathematical functions, most notibly the family of sine functions as well as the number e. To use these functions with complex numbers, the Complex type has a few built in functions, all of which can be found here: [`num::complex::Complex`][c-num::complex::Complex]{{hi:num::complex::Complex}}⮳.

```rust
{{#include ../../../deps/tests/mathematical_functions.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
