# Complex numbers

## Creating complex numbers

[![num-badge]][num]  [![cat-science-badge]][cat-science]

Creates complex numbers of type [`num::complex::Complex`][num::complex::Complex] Both the real and imaginary part of the complex number must be of the same type.

```rust,editable
{{#include ../../deps/tests/create-complex.rs}}
```

## Adding complex numbers

[![num-badge]][num]  [![cat-science-badge]][cat-science]

Performing mathematical operations on complex numbers is the same as on built in types: the numbers in question must be of the same type (i.e. floats or integers).

```rust,editable
{{#include ../../deps/tests/add-complex.rs}}
```

## Mathematical functions

[![num-badge]][num]  [![cat-science-badge]][cat-science]

Complex numbers have a range of interesting properties when it comes to how they interact with other mathematical functions, most notibly the family of sine functions as well as the number e. To use these functions with complex numbers, the Complex type has a few built in functions, all of which can be found here: [`num::complex::Complex`][num::complex::Complex]

```rust,editable
{{#include ../../deps/tests/mathematical-functions.rs}}
```

{{#include ../refs/link-refs.md}}
