# Borrow

{{#include borrow.incl.md}}

## Borrow {#borrow}


The `Borrow` trait is a tool for writing generic code that works with both owned and borrowed data. The `Borrow<U>` trait lets a type provide a reference to another type `U`.


It is part of the `std::borrow module` and is useful when you want to abstract over types like `String` and `&str`.


 This is different from `AsRef<U>` in that `Borrow` is intended for equivalence — meaning the borrowed value should behave identically to the owned one in terms of traits like `Eq`, `Hash`, and `Ord`.


[`Borrow`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)⮳{{hi:std::borrow::Borrow}} and [`BorrowMut`](https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html)⮳{{hi:std::borrow::BorrowMut}}

`Eq`, `Ord` and `Hash` must be equivalent for borrowed and owned values: x.borrow() == y.borrow() should give the same result as x == y.


Why Use It?

- HashMap lookups: You can store String keys but look them up using &str.
- Trait-based polymorphism: Enables flexible APIs that accept multiple forms of a type.
- Avoids unn



```rust,editable
{{#include ../../crates/cats/standard_library/examples/borrow/borrow.rs:example}}
```

Generic Function Over Borrow<str>
You can write a function that accepts any type that can borrow a str, be it String, &String, or &str, and use it in lookups seamlessly.

```rust,editable
{{#include ../../crates/cats/standard_library/examples/borrow/borrow2.rs:example}}
```

Custom Type Implementing Borrow
You can also implement Borrow on your own types to integrate them into APIs expecting a particular key type.


```rust,editable
{{#include ../../crates/cats/standard_library/examples/borrow/borrow3.rs:example}}
```

These patterns scale to any type `T` and borrowed form `&U` as long as you implement `Borrow<U>`.



- How Borrow interacts with trait objects and dynamic dispatch
- Differences between Borrow, Deref, and AsRef in generic code
- Using Borrow in combination with custom hashing or comparison logic


If it wishes to mutably borrow as the type, allowing the underlying data to be modified, it can additionally implement BorrowMut<T>.

## Related Topics {#skip}

- [[asref | AsRef]].
- [[ownership_borrowing | Ownership and Borrowing]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
