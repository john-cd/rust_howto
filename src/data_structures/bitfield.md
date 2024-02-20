# Custom

## Define and operate on a type represented as a bitfield

[![bitflags][bitflags-badge]][bitflags]  [![cat-no-std][cat-no-std-badge]][cat-no-std]

Creates type safe bitfield type `MyFlags` with help of [`bitflags!`][bitflags!] macro and implements elementary `clear` operation as well as [`Display`][std::fmt::Display] trait for it. Subsequently, shows basic bitwise operations and formatting.

```rust,editable
{{#include ../../deps/tests/bitfield.rs}}
```

{{#include ../refs/link-refs.md}}
