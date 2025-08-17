# Safe Arithmetic in Rust

{{#include overflow.incl.md}}

## Handle Overflows {#overflow-handling}

- Wrap in all modes with the `wrapping_*`{{hi:wrapping_*}} methods, such as [`wrapping_add`][primitive~u32::wrapping_add]↗{{hi:wrapping_add}}.
- Return the [`std::option::Option::None`][c~std::option::Option::None~docs]↗{{hi:std::option::Option::None}} value if there is overflow{{hi:Overflow}} with the `checked_*`{{hi:checked_*}} methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*`{{hi:overflowing_*}} methods.
- Saturate at the value's minimum or maximum values with the `saturating_*`{{hi:saturating_*}} methods.

- [`half`][c~half~lib.rs] - data structures in Rust.

{{#include refs.incl.md}}
{{#include refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
