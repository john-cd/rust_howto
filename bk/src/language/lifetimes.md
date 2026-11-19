<!-- TODO Reviewed ??-??-2026. -->

# Lifetimes

{{#include lifetimes.incl.md}}

## Lifetime Basics {#lifetime}

[![Rust by example - Lifetimes][book~rust-by-example~lifetimes~badge]][book~rust-by-example~lifetimes]{{hi:Lifetimes}}

In Rust, _references_ access a value without taking ownership of it ("borrow" it). There are immutable references (of type `&T`), which allow read-only, shared access to a value; and mutable references (of type `&mut T`), which allow modification but enforce exclusive access.

References are similar to pointers but come with strict safety guarantees: they are aligned, not null, and pointing to memory containing a _valid_ value of `T`.

A reference type has a _lifetime_ that describes how long the reference may be used. The compiler checks that the referent remains valid for that lifetime. A borrow can end before the variable it borrows goes out of scope.{{hi:Dangling references}}

The Rust compiler infers most lifetimes (see below), therefore we will most often write references with an implicit lifetime:

- `&i32`: a shared reference.
- `&mut i32`: a mutable reference with an implicit lifetime.

When the compiler can't figure out the relationships between the lifetimes of different references (especially in function signatures or struct definitions), you need to provide explicit lifetime annotations:

- Lifetime names are always prefixed with an apostrophe (e.g., `'a`, `'b`, `'static`). By convention, short, lowercase names are usually used. `'static` is a special lifetime that means the reference is valid for the entire duration of the program.
- When added to references, the lifetime annotation is inserted after `&` and before the `mut` keywords or the type:
  - `&'a i32`: a shared reference with an explicit lifetime `'a`.
  - `&'a mut i32`: a mutable reference with an explicit lifetime `'a`.

Note the following:

- Lifetime parameters affect type checking. In `&'a i32`, the referent type is `i32` and the reference type includes the lifetime `'a`.
- The bound `'long: 'short` means `'long` outlives `'short`. Rust can shorten the outer lifetime of `&T` or `&mut T` when the longer borrow outlives the shorter one. Changing a lifetime inside the referent follows different rules. For example, shortening the outer borrow of `&mut &'static str` still leaves the slot holding a `&'static str`. Treating the slot as holding a shorter reference would allow a temporary string to escape through that slot.
- Lifetimes are compile-time only: they are erased during compilation and have no runtime overhead.

## Lifetime Elision Rules {#lifetime-elision-rules}

As discussed above, Rust's compiler infers most lifetimes. More specifically, it has "lifetime elision rules" that allow you to omit them in common, unambiguous cases. This reduces boilerplate.{{hi:Lifetime elision rules}}

- Each elided input lifetime becomes a distinct lifetime parameter. `fn foo(x: &str)` is automatically treated as `fn foo<'a>(x: &'a str)`.
- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters. `fn foo(x: &str) -> &str` is treated as `fn foo<'a>(x: &'a str) -> &'a str`.
- If there are multiple input lifetime parameters but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

These rules cover many common scenarios, but you will need lifetimes in more complex cases:

- Functions that take multiple references and return references. When a function takes multiple references as input and returns a reference that could be based on any of those input references, the compiler needs to know the relationship.
- Structs that hold references. If a struct needs to hold references, it must be annotated with lifetime parameters to ensure that the references it holds don't outlive the data they point to.
- Traits that involve references. When defining traits that include methods returning references, you need to specify lifetimes to ensure the returned references are valid for the expected duration.

## Use `'static` as the Lifetime of the Running Program {#static-lifetime}

`'static` indicates that the data pointed to by the reference lives for the lifetime of the running program. It can still be coerced to a shorter lifetime. String literals are a common example of a string slice with `'static` lifetime.

```rust,editable
{{#include ../../crates/language/examples/lifetimes/static_lifetime.rs:example}}
```

## Use Lifetime Parameters {#use-lifetime-parameters}

Lifetime parameters can be added to function or method signatures, struct definitions, enumerations, unions, `impl` blocks, type aliases, traits, in the same way that a generic type parameter or constant can be added. They are used to specify the relationships between the lifetimes of different references in the function, type, or item.

Lifetime parameters are listed in angle brackets (e.g. `<'a>`), usually immediately after the name of the item. Lifetime parameters are a form of [[generics | generics]]. Lifetime parameters, type parameters, and const generics can be intermixed within `<...>`. Lifetime parameters must appear before any generic type and const parameters (e.g. `<'a, T, const N: usize>`):

```rust,editable
{{#include ../../crates/language/examples/lifetimes/lifetime_parameters.rs:example}}
```

### Use Lifetime Parameters in Functions {#lifetimes-in-functions}

The function below can return either input. Both input borrows must outlive the inferred `'a`, and callers can use the result within that shared portion of their lifetimes:

```rust,editable
{{#include ../../crates/language/examples/lifetimes/lifetime_parameter_function.rs:example}}
```

The following call fails because `chosen` is used after one of its possible referents has been dropped:

```rust,compile_fail,E0597,noplayground
{{#include ../../crates/language/tests/ui/shorter_input_lifetime.rs:example}}
```

### Use Lifetimes and Lifetime Parameters in Struct Definitions and Implementations {#lifetime-annotations}

The following example shows a `struct` with a lifetime parameter and multiple implementations, two for any lifetimes, one for a specific lifetime. This is a form of _conditional implementation_.

```rust,editable
{{#include ../../crates/language/examples/lifetimes/lifetime_parameter_struct.rs:example}}
```

### Avoid Self-referential Structs {#self-referential-structs}

[![std][c~std~docs~badge]][c~std~docs]

Self-referential structs, that is structs that hold a reference to their own fields, can be tricky due to Rust's ownership and borrowing rules. You can easily run into issues when they are moved, as the references might become invalid. There is also no way to tie the lifetime of a reference to the lifetime of the struct that contains it.

Instead, you may:

- Store only owned data in the struct,
- Store the owned data outside the struct and let the struct hold only references,
- Store ranges rather than references, if the pointed-to type is a sequence (array, string, vector...),
- Store a separate owned `Rc` or `Arc` handle to shared data where a field would otherwise borrow another field.
- Use arena-style allocation to enforce shared lifetimes.
- Use raw pointers (Requires `unsafe` code).

```rust,editable
{{#include ../../crates/language/examples/lifetimes/self_referential_struct.rs:example}}
```

## Common Misconceptions {#common-misconceptions}

- **`T` only contains owned types**: In generic code, the type parameter `T` contains all types, including references (e.g., `&i32` or `&mut i32`). Therefore, `T` is a superset of `&T` and `&mut T`.
- **A `'static` bound on a type** means the type has no borrowed data that limits it to a shorter lifetime. A local `String` satisfies `T: 'static` and can be dropped immediately. A value of type `&'static T` is a reference whose referent is valid for the entire program.
- **`&'a T` and `T: 'a` are the same**: `T: 'a` means type `T` must be valid for lifetime `'a`. This accepts owned types, types containing references, and references. `&'a T` only accepts references.
- **Shared reborrows cannot overlap**: Multiple shared reborrows of the same mutable reference may overlap. While any such shared borrow remains live, the original mutable reference cannot be used for incompatible mutable access.
- **Lifetimes can grow and shrink at run-time**: Lifetimes are statically verified at compile-time. The borrow checker assumes every code path can be taken and chooses the shortest possible lifetime. Lifetimes cannot grow or shrink dynamically at run-time.

## Overlap Shared Reborrows {#shared-reborrows}

Reborrowing an `&mut T` as `&T` permits other shared reborrows of that same value to overlap. The original mutable reference stays unavailable for mutation until the shared references' last use has passed:

```rust,editable
{{#include ../../crates/language/examples/lifetimes/shared_reborrows.rs:example}}
```

A method returning `&T` from `&mut self` has the same rule. The returned reference keeps the receiver borrowed for as long as the result is live, so code cannot mutate through the receiver during that time. This does not prevent other shared references from overlapping. The borrow-checker failure for attempting that mutation is kept as a compile-fail fixture in the `language` crate.

## References {#references .skip}

- [Lifetime elision rules (Rust reference)][book~rust-reference~lifetime-elision]↗.
- [Common Rust Lifetime Misconceptions][common-rust-lifetime-misconceptions~repo]↗.
- [Self-referential Structs][blog~rust-self-referential-structs]↗.
- [The mutable reference I return from a function in Rust seems to violate the borrowing rules — how is this possible?][reddit~the_mutable_reference_i_return_from_a_function_in]↗.

## Related Topics {#related-topics .skip}

- [[clone-on-write | COW]].
- [Choose Borrowed or Owned JSON Fields](../categories/encoding/serde.md#borrowed-json) shows a deserialization result borrowing from its input.
- [Memory Management](../categories/memory-management/index.md).
- [[ownership_and_borrowing | Ownership & Borrowing]].
- [Rust Patterns](../categories/rust-patterns/index.md).
- [Typecasts and byte layout](../categories/encoding/typecasts.md).

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
