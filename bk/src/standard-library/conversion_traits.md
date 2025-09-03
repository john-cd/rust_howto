# Conversion Traits

{{#include conversion_traits.incl.md}}

Conversion traits like [`From`][c~std::convert::From~docs]↗{{hi:std::convert::From}}, [`Into`][c~std::convert::Into~docs]↗{{hi:std::convert::Into}}, [`TryFrom`][c~std::convert::TryFrom~docs]↗{{hi:std::convert::TryFrom}}, and [`TryInto`][c~std::convert::TryInto~docs]↗{{hi:std::convert::TryInto}} enable type-safe transformations between types. `From` is the most common trait for defining conversions - it is implemented on the destination type and lets you create an instance from another type. `Into` is automatically implemented when `From` is, allowing values to be converted with `.into()`. For fallible conversions,`TryFrom` and `TryInto` return a `Result`, adding error handling when the conversion might fail.

## Convert between Types with `From` {#from}

[![std][c~std~docs~badge]][c~std~docs]

The [`From`][c~std::convert::From~docs]↗{{hi:std::convert::From}} trait defines how one type can be turned into another. Implementing `From<T>` for the target type allow you to write conversions like `let x: Target = Target::from(source);`.

Since `From` is infallible by design, conversions must never fail. `From` enables automatic [`Into`][c~std::convert::Into~docs]↗{{hi:std::convert::Into}} implementations, so types implementing `From` can be seamlessly used with `.into()`:

```rust,editable
{{#include ../../crates/standard_library/examples/conversion_traits/from.rs:example}}
```

[`From`][c~std::convert::From~docs]↗{{hi:std::convert::From}} is often used to convert custom errors. You may also use the [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} and [`thiserror`][c~thiserror~docs]↗{{hi:thiserror}} crates.

```rust,editable
{{#include ../../crates/standard_library/examples/conversion_traits/from2.rs:example}}
```

`From` may also be used when parsing. To parse strings, implement [`std::str::FromStr`][c~std::str::FromStr~docs]↗ instead.

```rust,editable
{{#include ../../crates/standard_library/examples/conversion_traits/from3.rs:example}}
```

## Choose the Right Trait for a Conversion {#choose-conversion-trait}

[![std][c~std~docs~badge]][c~std~docs]

Paraphrasing the [`std::convert`][c~std::convert~docs]↗ module documentation, you should:

- Implement the [`From`][c~std::convert::From~docs]↗{{hi:std::convert::From}} trait for _explicit_, _infallible_, _consuming_ value-to-value conversions.
  - `From` creates new owned values and automatically implements `Into`. `From` is the only trait here that changes ownership and produces a new value, while the others below borrow existing ones.
  - Implement the [`Into`][c~std::convert::Into~docs]↗{{hi:std::convert::Into}} trait for consuming value-to-value conversions to types outside the current crate.
- Use [`TryFrom`][c~std::convert::TryFrom~docs]↗{{hi:std::convert::TryFrom}}/[`TryInto`][c~std::convert::TryInto~docs]↗{{hi:std::convert::TryInto}} when a conversion might fail. The `TryFrom` and `TryInto` traits behave like `From` and `Into` otherwise.
- Use [`AsRef`][c~std::convert::AsRef~docs]↗{{hi:std::convert::AsRef}} / [`AsMut`][c~std::convert::AsMut~docs]↗{{hi:std::convert::AsMut}} when passing around references without consuming. `AsRef` lends a borrowed view into another type without transferring ownership.
  - Implement the `AsRef` trait for _explicit_, _cheap_ _reference-to-reference_ conversions.
  - Implement the `AsMut` trait for explicit, cheap _mutable-reference-to-mutable-reference_ conversions.

Distinguish the above traits from the following:

- [`FromStr`][c~std::str::FromStr~docs]↗{{hi:std::str::FromStr}} provides _string parsing_ with idiomatic error handling, and it's typically implemented for types like [`u32`][primitive~u32]↗{{hi:u32}}, `Url`, and enumerations{{hi:enum}}. It powers [`.parse::<T>()`][str::parse~docs]↗{{hi:str::parse}}. It can only parse types that do not contain a lifetime parameter.
- [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} provides _implicit_ access to the inner data of _smart pointers_ via _dereferencing_ (`*`, `.` method calls) - e.g., from `Box<T>` to `T`. It is a key part of Rust's deref coercion system.
- [`Borrow`][c~std::borrow::Borrow~docs]↗{{hi:std::borrow::Borrow}} enables structural borrowing, especially for collections. `Borrow` is most often used in standard library collections like `HashMap`{{hi:std::collections::HashMap}} to let you use a borrowed key (e.g., `&str`) to look up an owned key (`String`). Unlike `AsRef`, which is for conversion, `Borrow` implies full _equivalence in behavior_. In particular, `Eq`, `Ord` and `Hash` _must_ be equivalent for borrowed and owned values.

## Related Topics {#related-topics .skip}

- [[asref | AsRef]].
- [[borrow | Borrow]].
- [[smart_pointers | Smart Pointers]].
- [[string_parsing | String Parsing]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
