# Borrowed Types with `Borrow`

{{#include borrow.incl.md}}

## Use the `Borrow` Trait to Work with Borrowed Types Synonymous with Owned Types {#borrow}

[![std][c~std~docs~badge]][c~std~docs]

Rust data types often have multiple representations to suit different needs. For example, smart pointer types like [`Box<T>`][c~std::boxed::Box~docs]↗ or [`Arc<T>`][c~std::sync::Arc~docs]↗ allow you to choose how a value is stored and managed. Some types, such as [`String`][c~std::string::String~docs]↗, extend a more basic type ([`str`][primitive~str]↗{{hi:str}}) by adding features like mutability and dynamic growth, which require extra metadata. This design lets you use lightweight, immutable, borrowed types when possible, and switch to more flexible, feature-rich, memory-owning types when necessary. Common type pairs include:

| Borrowed Type | Owned Type |
|---|---|
| [`&str`][primitive~str]↗{{hi:&str}} | [`String`][c~std::string::String~docs]↗ |
| [`&CStr`][c~std::ffi::CStr~docs]↗{{hi:std::ffi::CStr}} | [`CString`][c~std::ffi::CString~docs]↗{{hi:std::ffi::CString}} |
| [`&OsStr`][c~std::ffi::OsStr~docs]↗{{hi:std::ffi::OsStr}} | [`OsString`][c~std::ffi::OsString~docs]↗{{hi:std::ffi::OsString}} |
| [`&Path`][c~std::path::Path~docs]↗{{hi:std::path::Path}} | [`PathBuf`][c~std::path::PathBuf~docs]↗{{hi:std::path::PathBuf}} |
| [`&[T]`][primitive~slice]↗{{hi:slice}} | [`Vec<T>`][c~std::vec::Vec~docs]↗{{hi:std::vec::Vec}} |
| `&[T]` | [`[T; N]`][primitive~array]↗{{hi:array}} |
| [`&T`][primitive~reference]↗{{hi:reference}} | [`Box<T>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}} |
| `&T` | [`Arc<T>`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} |

Types express that they can be _borrowed as_ some type `T` by implementing [`Borrow<T>`][c~std::borrow::Borrow~docs]↗{{hi:std::borrow::Borrow}}. Use the trait's `borrow` method to return a reference `&T`. For instance, a `Box<T>` can be borrowed as `&T`, while a `String` can be borrowed as `&str`.

A type is free to borrow as several different types.

If a type wishes to mutably borrow as another type, allowing the underlying data to be modified, it can additionally implement the [`BorrowMut`][c~std::borrow::BorrowMut~docs]↗{{hi:std::borrow::BorrowMut}} trait.

With `Borrow<T>` and `BorrowMut<T>`, it is possible to write generic code that accept `&T`, and therefore works with both such owned and borrowed data. It is a form of trait-based polymorphism, which enables flexible APIs that accept multiple forms of a type.

`Borrow` is particularly useful when you are using (or implementing) a data structure, and you want to use either an owned or borrowed type as _synonymous_.

For example, as a data collection, `HashMap<K, V>` owns both keys and values. If the key's actual data is wrapped in a managing type of some kind, it should, however, still be possible to search for a value using a reference to the key's data. For instance, if the key is a string, then it is likely stored with the hash map as a [`String`][c~std::string::String~docs]↗, while it should be possible to search using a [`&str`][primitive~str]↗. The `Borrow` trait enables this: you can insert with a `String`, but retrieve values using a `&str` reference, allowing for flexible and efficient key access without unnecessary allocations or conversions. Specifically, `HashMap<K, V>` functions like `get` accept `&Q` where `K: Borrow<Q>` and `String` is `Borrow<str>`:

```rust,editable
{{#include ../../crates/standard_library/examples/borrow/borrow.rs:example}}
```

You can, of course, write a generic function that accepts any type that can borrow as e.g. a string slice, be it `String`, `&String`, or `&str`, and use it in lookups seamlessly:

```rust,editable
{{#include ../../crates/standard_library/examples/borrow/borrow2.rs:example}}
```

## Implement `Borrow` for a Custom Type {#implement-borrow}

[![std][c~std~docs~badge]][c~std~docs]

You can implement [`Borrow`][c~std::borrow::Borrow~docs]↗{{hi:std::borrow::Borrow}} on your own types.

This said, BEWARE: `Borrow` is different from `AsRef<T>` in that `Borrow` is intended for _equivalence_ - meaning the borrowed value should behave identically to the owned one. In particular, `Eq`, `Ord` and `Hash` must be equivalent for borrowed and owned values: `x.borrow() == y.borrow()` should give the same result as `x == y`:

```rust,editable
{{#include ../../crates/standard_library/examples/borrow/borrow3.rs:example}}
```

## Differences between `Borrow`, `Deref`, and `AsRef` in Generic Code {#differences-borrow-deref-asref}

The [`Borrow`][c~std::borrow::Borrow~docs]↗{{hi:std::borrow::Borrow}} and [`AsRef`][c~std::convert::AsRef~docs]↗ traits are very similar, but different in purposes.

- Use `Borrow` when you want to abstract over different kinds of borrowing, or when you're building a data structure that treats owned and borrowed values in _equivalent_ ways, such as hashing and comparison.
- Use `AsRef` when you want to convert something to a reference directly or you're writing generic code.
- [`Deref`][c~std::ops::Deref~docs]↗ should be implemented by smart pointers only.

See [[asref | AsRef]], [[conversion_traits | Conversion Traits]] and [[smart_pointers | Smart Pointers]] for more details.

## Related Topics {#related-topics .skip}

- [[asref | AsRef]].
- [[conversion_traits | Conversion Traits]].
- [[ownership_borrowing | Ownership and Borrowing]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
