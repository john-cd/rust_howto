<!-- TODO Reviewed ??-??-2026. -->

# Clone-on-Write

{{#include clone-on-write.incl.md}}

## Clone Data Lazily with `Cow` {#clone-data-lazily-with-cow}

[![std][c~std~docs~badge]][c~std~docs]{{hi:Clone-on-write}}

The type [`std::borrow::Cow`][c~std::borrow::Cow~docs]↗{{hi:std::borrow::Cow}} is a smart pointer providing clone-on-write functionality: it encloses and provides immutable access to borrowed data, and clones the data lazily when mutation or ownership is required.

`Cow` optimizes memory usage by delaying cloning until mutation is required, if it is required. It is especially useful in cases where:

- Cloning is costly (long strings, large arrays...),
- Needing to modify the underlying value is rare,
- The stored value is mostly used for read-only purposes.

`Cow`, as a smart pointer, implements [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}}, which means that you can call non-mutating methods directly on the data it encloses. If mutation is desired, [`to_mut`][c~std::borrow::Cow::to_mut~docs]↗{{hi:std::borrow::Cow::to_mut}} will obtain a mutable reference to an owned value, cloning if necessary.

## Accept Either an Owned or Borrowed Value as the Input of a Function {#accept-either-owned-or-borrowed-values}

Since [`Cow`][c~std::borrow::Cow~docs]↗{{hi:std::borrow::Cow}} allows borrowing until mutation is needed, it's ideal for functions that take either borrowed or owned strings without unnecessary cloning.

```rust,editable
{{#include ../../crates/standard_library/examples/cow/cow_as_function_param.rs:example}}
```

## Modify a `Cow` In-place {#modify-cow-in-place}

You can of course pass a `&mut Cow<T>` to a function. Modify the underlying value in place with [`to_mut`][c~std::borrow::Cow::to_mut~docs]↗{{hi:std::borrow::Cow::to_mut}}:

```rust,editable
{{#include ../../crates/standard_library/examples/cow/modify_cow_in_place.rs:example}}
```

## Return a `Cow` from a Function {#return-cow-from-function}

It is common to return a [`Cow`][c~std::borrow::Cow~docs]↗{{hi:std::borrow::Cow}} from a function, if the (borrowed) input is returned unmodified in most, but not all, cases.

```rust,editable
{{#include ../../crates/standard_library/examples/cow/function_returning_cow.rs:example}}
```

## Efficiently Construct a `Cow` with `into` {#into-cow}

Use the [`Into`][c~std::convert::Into~docs]↗{{hi:std::convert::Into}} trait to construct `Cow`.

The `Into` trait is the dual of [`From`][c~std::convert::From~docs]↗{{hi:std::convert::From}} and is implemented for all types that implement `From`. Since `impl<'a> From<&'a str> for Cow<'a, str>` and `impl<'a> From<String> for Cow<'a, str>` are in the standard library, we can simply call `into()` to convert a `&str` or a `String` into `Cow`.

[`Cow`][c~std::borrow::Cow~docs]↗{{hi:std::borrow::Cow}} implements `From` for a number of other common types as well: (references to) `Vec`, `Path`, `PathBuf`, `OsStr`, `OsString`, etc.

```rust,editable
{{#include ../../crates/standard_library/examples/cow/into_cow.rs:example}}
```

## Convert a `Cow` to a Borrowed or Owned Type {#convert-cow-to-str}

Pass a reference to the `Cow<str>` where a function accepts `&str`. To keep the text after its input buffer goes away, call `into_owned()`. It clones borrowed text or moves an existing `String`.

This example reads a status label through a slice, then saves it before dropping the input buffer.

```rust,editable
{{#include ../../crates/standard_library/examples/cow/cow_to_borrowed_owned.rs:example}}
```

## Use `Arc`, `Rc`, `Box` as `Cow` Alternatives {#cow-alternatives}

Taking the example of strings: using `&str` for borrowing and `String` for owned data might be sufficient if you don't need the flexibility of [`Cow<str>`][c~std::borrow::Cow~docs]↗{{hi:std::borrow::Cow}}. You can also use:

- [`Arc<str>`][c~std::sync::Arc~docs]↗, an atomically reference-counted shared string slice, to be used in multithreaded environments,
- [`Rc<str>`][c~std::rc::Rc~docs]↗, a single-threaded reference-counted string slice,
- [`Box<str>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}} for heap-allocated immutable string slices:

```rust,editable
{{#include ../../crates/standard_library/examples/cow/cow_alternatives.rs:example}}
```

## References {#references .skip}

- [Using Cow in Rust for efficient memory utilization][blog~using-cow-rust-efficient-memory-utilization]↗.
- [The Secret Life of Cows][blog~secret-life-of-cows]↗.

## Related Topics {#related-topics .skip}

- [[lifetimes | Lifetimes]].
- [Memory Management](../categories/memory-management/index.md).
- [[ownership_and_borrowing | Ownership & Borrowing]].
- [Rust Patterns](../categories/rust-patterns/index.md).
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
