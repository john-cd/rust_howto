<!-- TODO Reviewed ??-??-2026. -->

# Heap Storage

{{#include heap_storage.incl.md}}

## Store Data on the Heap with `Box` {#box}

[![book~rust~box][book~rust~box~badge]][book~rust~box]{{hi:Box}} [![Rust by example - box][book~rust-by-example~box~badge]][book~rust-by-example~box] [![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

[`Box<T>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}} owns its pointee and drops it when the box is dropped. With the default allocator, a nonzero-sized pointee lives in a heap{{hi:Heap}} allocation. A zero-sized pointee needs no allocation. For sized `T`, the default `Box<T>` has the representation of one pointer. For dynamically sized types, a pointer also needs metadata, such as a slice length or trait-object vtable. A box can itself be a local, a struct field, or an element in a collection.{{hi:Stack}}

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_basics.rs:example}}
```

The `Box<T>` type is a smart pointer{{hi:Smart pointers}}, because it implements the [`std::ops::Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} trait, which allows `Box<T>` values to be treated just like a reference. You can use the de-reference operator{{hi:Dereference operator}} `*`{{hi:*}} or 'deref coercion' with the `.` operator to use its inner value:

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_deref.rs:example}}
```

### Decide When to Use `Box` {#box-use-cases}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

[`Box<T>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}} supports several use cases.

- Move large data to the heap to reduce stack usage.
- Own a dynamically sized value through a pointer such as `Box<[T]>`.
- Give a recursive type a finite size through indirection. A linked-list node can contain an `Option<Box<Node>>` because the box has a known size.
- Own a value through its trait interface with `Box<dyn Trait>`.

Pass a value directly to transfer ownership when no indirection is needed. Use [`Arc<T>`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} or [`Rc<T>`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} for shared ownership.

Use `&T` or `&mut T` when you only need to borrow a value. A small type can still benefit from boxing when it needs indirection, for example to break a recursive type cycle.

## Use `Box` to Store Dynamically Sized Types {#box-dst}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Dynamically Sized Types (DSTs), also known as "unsized types," are types whose size cannot be determined at compile time. The most common DSTs are:

- Slices: [`[T]`][primitive~slice]↗ (e.g., `[i32]`, `[u8]`),
- [`str`][primitive~str]↗{{hi:str}} - the string slice type,
- Trait Objects: [`dyn Trait`][keyword~dyn]↗{{hi:dyn}} (e.g., `dyn std::io::Read`).

The following demonstrates boxes containing slices and string slices:

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_dst.rs:example}}
```

## Use `Box` as a Smart Pointer to Owned Trait Objects {#box-trait-objects}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Trait objects `dyn SomeTrait` are opaque values that implement a base trait `SomeTrait`. The purpose of trait objects is to permit "late binding", a.k.a. virtual dispatch. When you call a function through a `dyn SomeTrait`, the compiler inserts code to lookup the function to call _at runtime_, allowing for polymorphism.

Because their underlying concrete types are obscured, trait objects are dynamically sized types (DSTs). Like all DSTs, trait objects must be used behind some type of pointer, for example `&dyn SomeTrait` or `Box<dyn SomeTrait>`. If you want an owned trait object use `Box<dyn Trait>`, and if you want a borrowed trait object use `&dyn Trait`.

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_trait_objects.rs:example}}
```

## Implement Recursive Data Structures with `Box` {#box-recursive-data-structures}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

The Rust compiler needs to know the exact size of a type at compile time, but a recursive data structure's size is potentially unbounded.

For example, in the following, we define a linked list that has an unknown number of nodes, thus its size is not fixed. It could not be stored directly on the stack. By using [`Box`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}}, which has a defined size on the stack, we can create a local variable. The actual `Node` data will be stored on the heap.

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_recursive.rs:example}}
```

Rust 1.100 stabilized [`Allocator`][std~alloc~allocator]↗ and per-instance allocation for selected standard types, including `Box` and `Vec`. Check the stability of the collection methods you need before choosing a minimum Rust version.

With Rust 1.100 or newer, a `Vec` and `Box` can use an allocator passed when they are constructed:

```rust
{{#include ../../crates/rust_1_100/src/lib.rs:allocator}}
```

`System` is a demonstration allocator; choose a custom allocator only when its lifetime, alignment, failure behavior, and measurement justify the added complexity. Other collections do not necessarily expose the same stable allocator API.

## Related Topics {#related-topics .skip}

- [Memory Management](../categories/memory-management/index.md).
- [[ownership_and_borrowing | Ownership & Borrowing]].
- [Rust Patterns](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/rust-patterns/index.md).

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
