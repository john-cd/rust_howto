# FRAGMENTS

||
|---|
| [Handle Overflows][ex~language~overflow-handling] |

## Handle Overflows {#overflow-handling}

- Wrap in all modes with the `wrapping_*`{{hi:wrapping_*}} methods, such as [`wrapping_add`][primitive~u32::wrapping_add]{{hi:wrapping_add}}↗.
- Return the [`std::option::Option::None`][c~std::option::Option::None~docs]↗{{hi:std::option::Option::None}} value if there is overflow{{hi:Overflow}} with the `checked_*`{{hi:checked_*}} methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value's minimum or maximum values with the `saturating_*` methods.

- [half — data structures in Rust](https://lib.rs/crates/half)

---

## `Cow` Alternatives {#cow-alternatives}

While [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)↗{{hi:std::borrow::Cow}} is a great choice for optimizing memory usage when dealing with a mix of borrowed and owned data, there are alternatives.

Instead of e.g. `Cow<str>`, you can use:

- `Arc<str>`, an atomically reference-counted shared string slice, to be used in multithreaded environments,
- `Rc<str>`, a single-threaded reference-counted string slice,
- `Box<str>` for heap-allocated string slices.

```rust,editable
{{#include ../../crates/standard_library/examples/cow/cow_alternatives.rs:example}}
```

## Arc::make_mut {#arc_make_mut}

If you need reference-counting, note that [`Rc::make_mut`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.make_mut)↗{{hi:std::rc::Rc::make_mut}} and [`Arc::make_mut`](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut)↗{{hi:std::rc::Arc::make_mut}} can provide clone-on-write functionality as well.

```rust,editable
{{#include ../../crates/standard_library/examples/arc/arc_make_mut.rs:example}}
```

---

## Create a Self-Referential Type using `Pin` {#self-referential-type}

[![std][c~std~docs~badge]][c~std~docs]

This is an advanced topic.

```rust,editable
{{#include ../../crates/language/examples/lifetimes/self_referential_struct2.rs:example}}
```

---

## Create a Plugin System {#plugins}

[![std][c~std~docs~badge]][c~std~docs]

TODO

```rust,editable
{{#include ../../crates/standard_library/examples/any/plugin.rs:example}}
```

For a true plugin architecture where plugins are compiled as separate shared libraries (`.so` on Linux, `.dll` on Windows, `.dylib` on macOS) and loaded at runtime, you would typically:

- Use the [`libloading`]()↗{{hi:libloading}} crate, which provides safe FFI (Foreign Function Interface) wrappers to dynamically load shared libraries and resolve symbols (functions).
- Define a C-compatible ABI: Because Rust's internal ABI is not stable across different compiler versions or even minor changes, you should define your plugin interface using `#[repr(C)]` structs and [`extern "C"`](https://doc.rust-lang.org/std/keyword.extern.html)↗{{hi:extern "C"}} functions.
- Define an entry point: Each plugin [`.so`]()↗{{hi:.so}}/[`.dll`]()↗{{hi:.dll}} would export a specific [`extern "C"`](https://doc.rust-lang.org/std/keyword.extern.html)↗{{hi:extern "C"}} function that the host calls to get a `Box<dyn Plugin>`.
- Version Management: Even with [`extern "C"`](https://doc.rust-lang.org/std/keyword.extern.html)↗{{hi:extern "C"}}, you need robust versioning for your common plugin interface crate to prevent issues if host and plugins are compiled with different versions of the interface. Crates like [`abi_stable`]()↗{{hi:abi_stable}} can help with this by providing more robust ABI compatibility checks.

---

## Emulate Inheritance with `AsRef` {#inheritance-in-rust}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Rust does not support traditional inheritance like object-oriented languages. Instead, it encourages using _traits_ and _composition_ to achieve similar functionality, allowing for shared behavior without the complexities and pitfalls of inheritance.

More precisely, Rust tends to model things with 'has-a' relationships (composition) instead of 'is-a' relationships (inheritance). Instead of inheritance, consider using an [`enum`](https://doc.rust-lang.org/std/keyword.enum.html)↗{{hi:enum}} (or a [`struct`](https://doc.rust-lang.org/std/keyword.struct.html)↗{{hi:struct}} type that contains fields that are always the same, plus a field with an `enum` type).

```rust,noplayground
// The base class becomes an enum.
// - The enum clearly defines all possible states/types of `Shape` in one place.
//   With inheritance, you might have to scour multiple files to understand the full hierarchy.
// - All the data for a Shape (be it a circle, square, or triangle) is contained within the Shape enum.
//   This can be more cache-friendly.
enum Shape {
    // The `derived classes` become variants.
    Circle { radius: f64 },
    Square { side: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        // `match` expressions on `enum`s are exhaustive. The compiler will force you to handle every possible variant,
        // which prevents common bugs that can arise when new subclasses are added in an inheritance hierarchy
        //  but not all code paths are updated.
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Square { side } => side * side,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

Commonly, a trait may be used to extract a shared behavior:

```rust,noplayground
trait HasArea {
    fn area(&self) -> f64;
}

// Implement the trait for the `Shape` enum.
impl HasArea for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI *radius* radius,
            Shape::Square { side } => side *side,
            Shape::Triangle { base, height } => 0.5* base * height,
        }
    }
}
```

You may also implement the [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)↗{{hi:std::convert::AsRef}} trait to retrieve the "base class":

```rust,editable
{{#include ../../crates/standard_library/examples/  /asref2.rs:example}}
```

Note that so-called "Deref polymorphism", meaning implementing [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)↗{{hi:std::ops::Deref}} to emulate inheritance between structs, is considered an [antipattern](https://github.com/rust-unofficial/patterns/blob/main/src/anti_patterns/deref.md), especially since `Deref` is an _implicit_ conversion.

### References {#references}

- [Rust's `AsRef` Explained](https://oliverjumpertz.com/blog/rusts-asref-explained/)↗.
- [Rust Is Beyond Object-Oriented, Part 3: Inheritance](https://www.thecodedmessage.com/posts/oop-3-inheritance)↗.
- [How to implement inheritance-like feature for Rust?](https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159/21)↗.

---

- [predicates](https://lib.rs/crates/predicates) boolean-valued predicate functions.
