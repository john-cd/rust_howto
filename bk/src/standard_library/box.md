# Box

{{#include box.incl.md}}

## Store Data on the Heap with `Box` {#box}

[![book~rust~box][book~rust~box~badge]][book~rust~box]{{hi:Box}} [![Rust by example - box][book~rust-by-example~box~badge]][book~rust-by-example~box] [![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

All values in Rust are stack-allocated by default. [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html)↗{{hi:std::boxed::Box}} allow you to store data on the heap{{hi:Heap}} rather than the stack{{hi:Stack}}. What remains on the stack is the pointer to the heap data. `Box<T>` owns its inner data and drop its contents when it goes out of scope:

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_basics.rs:example}}
```

The `Box<T>` type is a smart pointer{{hi:Smart pointers}}, because it implements the [`std::ops::Deref`][c~std::ops::Deref~docs]{{hi:std::ops::Deref}}↗ trait, which allows `Box<T>` values to be treated just like a reference. You can use the de-reference operator{{hi:Dereference operator}} `*`{{hi:*}} or 'deref coercion' with the `.` operator to use its inner value:

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_deref.rs:example}}
```

### `Box` Use Cases {#box-use-cases}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Use [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html)↗{{hi:std::boxed::Box}}↗ when

- you don't want to rely on stack space;
- you have a large amount of data and you want to avoid copying it on the stack: If you have a very large struct, you might want to allocate it on the heap using `Box` to avoid stack overflow.
- you have a dynamically sized type, whose size can't be known at compile time. This is common with recursive data structures like linked lists, trees, or enums where one variant contains another instance of the same enum.
- you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type.
- When you need to transfer ownership of data between functions - although [`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html)↗{{hi:std::sync::Arc}} or [`Rc<T>`](https://doc.rust-lang.org/std/rc/struct.Rc.html)↗{{hi:std::rc::Rc}} might be more appropriate for shared ownership.

Do not use `Box` for small, fixed-size types, or when you only need a reference: If you just need to borrow a value, use `&T` (immutable reference) or `&mut T` (mutable reference).

## Use `Box` to Store Dynamically Sized Types {#box-dst}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Dynamically Sized Types (DSTs), also known as "unsized types," are types whose size cannot be determined at compile time. The most common DSTs are:

- Slices: [`[T]`](https://doc.rust-lang.org/std/primitive.slice.html)↗ (e.g., `[i32]`, `[u8]`),
- [`str`](https://doc.rust-lang.org/std/primitive.str.html)↗{{hi:str}}: The string slice type,
- Trait Objects: [`dyn Trait`](https://doc.rust-lang.org/std/keyword.dyn.html)↗{{hi:dyn}} (e.g., `dyn std::io::Read`).

The following demonstrates boxes containing slices and string slices:

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_dst.rs:example}}
```

## Use `Box` as a Smart Pointer to Owned Trait Objects {#box-trait-objects}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Trait objects `dyn SomeTrait` are opaque values that implements a base trait `SomeTrait`. The purpose of trait objects is to permit "late binding", a.k.a. virtual dispatch. When you call a function through a `dyn SomeTrait`, the compiler inserts code to lookup the function to call _at runtime_, allowing for polymorphism.

Because their underlying concrete types are obscured, trait objects are dynamically sized types (DSTs). Like all DSTs, trait objects must be used behind some type of pointer, for example `&dyn SomeTrait` or `Box<dyn SomeTrait>`. If you want an owned trait object use `Box<dyn Trait>`, and if you want a borrowed trait object use `&dyn Trait`.

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_trait_objects.rs:example}}
```

## Implement Recursive Data Structures with `Box` {#box-recursive-data-structures}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

The Rust compiler needs to know the exact size of a type at compile time, but a recursive data structure's size is potentially unbounded.

For example, in the following, we define a linked list that has an unknown number of nodes, thus its size is not fixed. It could not be stored directly on the stack. By using [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)↗{{hi:std::boxed::Box}}, which has a defined size on the stack, we can create a local variable. The actual `Node` data will be stored on the heap.

```rust,editable
{{#include ../../crates/standard_library/examples/box/box_recursive.rs:example}}
```

## Related Topics {#related-topics}

- [[memory-management | Memory Management]].
- [[ownership_borrowing | Ownership & Borrowing]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
