# Borrow

{{#include borrow.incl.md}}

## Write Generic Functions with the `Borrow` trait {#borrow}

[![std][c~std~docs~badge]][c~std~docs]

Rust data types often have multiple representations to suit different needs. For example, pointer types like `Box<T>` or `Arc<T>` allow you to choose how a value is stored and managed. Some types, such as `String`, extend a more basic type (`str`) by adding features like mutability and dynamic growth, which require extra metadata. This design lets you use lightweight, immutable, borrowed types when possible, and switch to more flexible, feature-rich, memory-owning types when necessary. Common type pairs include:

| Borrowed Type | Owned Type |
|---|---|
| `&str` | `String` |
| `&CStr` | `CString` |
| `&OsStr` | `OsString` |
| `&Path` | `PathBuf` |
| `&[T]` | `Vec<T>` |
| `&[T]` | `[T; N]` |
| `&T` | `Box<T>` |
| `&T` | `Arc<T>` |

Types express that they can be borrowed as some type `T` by implementing `Borrow<T>`, providing a reference to a `T` in the trait's `borrow` method. A type is free to borrow as several different types.

If it wishes to mutably borrow as the type, allowing the underlying data to be modified, it can additionally implement the [`BorrowMut`](https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html)⮳{{hi:std::borrow::BorrowMut}} trait.

The [`Borrow<T>`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)⮳{{hi:std::borrow::Borrow}} trait, part of the `std::borrow` module, allows a type to provide a reference `&T` to another type `T`. For instance, a `Box<T>` can be borrowed as `T`, while a `String` can be borrowed as `&str`.

With `Borrow`, it is possible to write generic code that accept `&T`, and therefore works with both such owned and borrowed data.

It is a form of trait-based polymorphism, which enables flexible APIs that accept multiple forms of a type.

`Borrow` is particularly useful when you are using (or writing) a data structure, and you want to use either an owned or borrowed type as synonymous.

For example, as a data collection, `HashMap<K, V>` owns both keys and values. If the key's actual data is wrapped in a managing type of some kind, it should, however, still be possible to search for a value using a reference to the key's data. For instance, if the key is a string, then it is likely stored with the hash map as a `String`, while it should be possible to search using a `&str`. The `Borrow` trait enables this: you can insert with a `String`, but retrieve values using a `&str` reference, allowing for flexible and efficient key access without unnecessary allocations or conversions. Specifically, `HashMap<K, V>` functions like `get` accept `&Q` where `K: Borrow<Q>`. This means that you can store `String` keys in a `HashMap` but look them up using `&str`, since `String` is `Borrow<str>`.

BEWARE: `Borrow` is different from `AsRef<U>` in that `Borrow` is intended for _equivalence_ - meaning the borrowed value should behave identically to the owned one.
In particular, `Eq`, `Ord` and `Hash` must be equivalent for borrowed and owned values: `x.borrow() == y.borrow()` should give the same result as `x == y`.

```rust,editable
{{#include ../../crates/cats/standard_library/examples/borrow/borrow.rs:example}}
```

## Write a Generic Function over `Borrow` {#generic-function-over-borrow}

[![std][c~std~docs~badge]][c~std~docs]

You can write a function that accepts any type that can borrow a string, be it `String`, `&String`, or `&str`, and use it in lookups seamlessly:

```rust,editable
{{#include ../../crates/cats/standard_library/examples/borrow/borrow2.rs:example}}
```

## Implement `Borrow` for a Custom Type {#implement-borrow}

[![std][c~std~docs~badge]][c~std~docs]

You can implement `Borrow` on your own types to integrate them into APIs expecting a particular key type.

```rust,editable
{{#include ../../crates/cats/standard_library/examples/borrow/borrow3.rs:example}}
```

These patterns scale to any type `T` and borrowed form `&U` as long as you implement `Borrow<U>`.

## Differences between `Borrow`, `Deref`, and `AsRef` in Generic Code {#differences}

The Borrow and AsRef traits are very similar, but different.

- Use `Borrow` when you want to abstract over different kinds of borrowing, or when you’re building a data structure that treats owned and borrowed values in equivalent ways, such as hashing and comparison.
- Use `AsRef` when you want to convert something to a reference directly, and you’re writing generic code.

## Use Borrow in combination with custom hashing or comparison logic {#borrow-and-custom-hashing}

## Related Topics {#skip}

- [[asref | AsRef]].
- [[ownership_borrowing | Ownership and Borrowing]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
