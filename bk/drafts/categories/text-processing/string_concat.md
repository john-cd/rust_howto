<!-- TODO Reviewed ??-??-2026 -->

# String Concatenation

{{#include string_concat.incl.md}}

## Concatenate Strings at Runtime {#string-concat}

For runtime data, choose a method that fits the ownership and allocation needs. The `+` operator appends an `&str` to a `String` and moves the left-hand `String`; `format!` is convenient when formatting several pieces; `push_str` appends into an existing buffer. If the output size is known, reserve it with [`String::with_capacity`][c~std::string::String::with_capacity~docs]↗{{hi:String::with_capacity}}. Slice `.concat()` joins without a separator, while `.join(separator)` inserts one. The standard `concat!` macro concatenates literal tokens at compile time.

[![std][c~std~docs~badge]][c~std~docs] [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[![concat-string][c~concat-string~docs~badge]][c~concat-string~docs]{{hi:concat-string}} [![joinery][c~joinery~docs~badge]][c~joinery~docs] [![joinery~crates.io][c~joinery~crates.io~badge]][c~joinery~crates.io] [![joinery~repo][c~joinery~repo~badge]][c~joinery~repo] [![joinery~lib.rs][c~joinery~lib.rs~badge]][c~joinery~lib.rs]{{hi:joinery}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}} [![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

The example compares `+`, `format!`, `push_str`, capacity reservation, slice `concat` and `join`, and the `concat_string!` and `joinery` crate APIs.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_concat/string_concat.rs:example}}
```

## Concatenate Constant Strings at Compile Time {#compile-time}

[![constcat][c~constcat~docs~badge]][c~constcat~docs]{{hi:constcat}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

When the parts are constant string slices, [`constcat`][c~constcat~docs]↗{{hi:constcat}} provides a `concat!` macro that accepts constants and produces the combined string during compilation. Use the standard `concat!` macro for literals; use `constcat` when the inputs are named `const` values.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_concat/string_concat2.rs:example}}
```

## References {#references .skip}

- [String concatenation benchmark][concat~benchmark~repo]↗.

## Related Topics {#related-topics .skip}

- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  This review separates runtime concatenation methods from compile-time concatenation, adds the requested capacity-reservation example, and gives the existing second example a visible `constcat` section. The obsolete `concat_strs` dependency and duplicated recipes were removed.
</div>
