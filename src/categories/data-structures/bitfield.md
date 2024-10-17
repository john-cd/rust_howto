# Custom

{{#include bitfield.incl.md}}

## Define and operate on a type represented as a bitfield

[![bitflags][c-bitflags-badge]][c-bitflags]  [![cat-no-std][cat-no-std-badge]][cat-no-std]  [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]

Creates type-safe bitfield{{hi:bitfield}} type `MyFlags` with help of [`bitflags::bitflags`][c-bitflags::bitflags]{{hi:bitflags::bitflags}}⮳ macro and implements elementary `clear` operation as well as [`std::fmt::Display`][c-std::fmt::Display]{{hi:std::fmt::Display}}⮳ trait for it. Subsequently, shows basic bitwise operations{{hi:Bitwise operations}} and formatting.

```rust
{{#include ../../../deps/tests/bitfield.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
