# AsRef and &T {#asref}

{{#include asref.incl.md}}

## Accept Arguments of Multiple Types with `AsRef` {#asref}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

The [`AsRef`][c~std::convert::AsRef~docs]↗{{hi:std::convert::AsRef}} trait is used for _cheap reference-to-reference conversions_ (without allocating new memory). It provides a way to convert an object into a reference to another type.

The primary use case for [`AsRef<T>`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)↗{{hi:std::convert::AsRef}} is generic programming, especially for function arguments, to provide ergonomics and flexibility to the caller. In other words, this trait is often used to allow [functions][p~functions] to accept arguments in multiple forms.

For example, [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html)↗{{hi:std::path::Path}}, [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html)↗{{hi:std::path::PathBuf}}, [`str`](https://doc.rust-lang.org/std/primitive.str.html)↗{{hi:str}}, [`String`](https://doc.rust-lang.org/std/string/struct.String.html)↗{{hi:std::string::String}}, [`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html)↗{{hi:std::ffi::OsString}}, [`OsStr`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html)↗{{hi:std::ffi::OsStr}}, `Cow<'_, OsStr>`... all implement `AsRef<Path>`. The `std::path`{{hi:std::path}} standard library module therefore contains many functions that accepts `AsRef<Path>` and therefore any of the aforementioned types as a argument. Other common implementations include `AsRef<str>`, `AsRef<OsStr>`, `AsRef<[u8]>`, and `AsRef<[T]>`:

```rust,editable
{{#include ../../crates/standard_library/examples/asref/asref.rs:example}}
```

## Use `as_ref` to Get a Reference to the Contained Value of a Smart Pointer {#asref-smart-pointer}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

For smart pointers like [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html)↗{{hi:std::boxed::Box}}, [`Rc<T>`](https://doc.rust-lang.org/std/rc/struct.Rc.html)↗{{hi:std::rc::Rc}}, [`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html)↗{{hi:std::sync::Arc}}, etc., their `as_ref()` methods typically provide a `&T`, which is a reference to the contained value. This is distinct from simply using `&` on the smart pointer itself, which would give you a reference to the smart pointer (`&Box<T>`), not its contents.

```rust,editable
{{#include ../../crates/standard_library/examples/asref/smart_pointer_asref.rs:example}}
```

Note that `Option::as_ref()` and `Result::as_ref()` are inherent methods on the [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)↗{{hi:std::option::Option}} and [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)↗{{hi:std::result::Result}} types respectively, not implementations of the `std::convert::AsRef` trait. They transform an `Option<T>` into an `Option<&T>` (or `Result<T, E>` into `Result<&T, &E>`), which is useful for working with references inside these enums without consuming the original value.

```rust,editable
{{#include ../../crates/standard_library/examples/asref/option_asref.rs:example}}
```

## `AsRef` vs. `Deref` vs. `Borrow` {#asref-vs-deref-vs-borrow}

These traits are related but have distinct purposes:

- [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)↗{{hi:std::convert::AsRef}} enables _explicit_ (via `.as_ref()`) cheap reference-to-reference conversions, often used in generic bounds for flexibility.
- [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)↗{{hi:std::ops::Deref}} enables _implicit_ coercions (e.g., [`String`](https://doc.rust-lang.org/std/string/struct.String.html)↗{{hi:std::string::String}} to [`&str`](https://doc.rust-lang.org/std/primitive.str.html)↗{{hi:&str}}).
- [`Borrow`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)↗{{hi:std::borrow::Borrow}} is similar to `AsRef` but imposes additional constraints (e.g., [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html)↗{{hi:std::hash::Hash}}, [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html)↗{{hi:std::cmp::Eq}}, [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)↗{{hi:std::cmp::Ord}} must be equivalent for the borrowed value and the owned value). It's commonly used for keys in collections like [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)↗{{hi:std::collections::HashMap}}.

For non-generic contexts or when `Deref` coercion suffices, `&T` remains the simpler and often preferred choice.

If you need to do a _costly_ conversion, it is better to implement [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)↗{{hi:std::convert::From}} instead or write a custom function.

## References {#references}

- [When and why to use `AsRef<T>` instead of `&T`][stackoverflow~asref]↗.
- [Rust's `AsRef` Explained](https://oliverjumpertz.com/blog/rusts-asref-explained/)↗.

## Related Topics {#related-topics}

- [[box | `Box`]].
- [[borrow | `Borrow`]].
- [[conversion_traits | Conversion Traits]].
- [[reference_counting | `Rc` and `Arc`]].
- [[smart_pointers | Smart Pointers]].

Note that [`AsMut`][c~std::convert::AsMut~docs]↗{{hi:std::convert::AsMut}} can be used for converting between mutable references.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
