# Lifetimes

{{#include lifetimes.incl.md}}

## Lifetime Basics {#lifetime}

[![Rust by example - Lifetimes][book~rust-by-example~lifetimes~badge]][book~rust-by-example~lifetimes]{{hi:Lifetimes}}

In Rust, _references_ allow you to access a value without taking ownership of it ("borrow" it). There are immutable references (of type `&T`), which allow read-only, shared access to a value; and mutable references (of type `&mut T`), which allow modification but enforce exclusive access.

References are similar to pointers but come with strict safety guarantees: they are aligned, not null, and pointing to memory containing a _valid_ value of `T`.

To guarantee the latter, the Rust compiler attaches a _lifetime_ to each reference. A lifetime represents the _scope_ (a section of code) for which the value (and therefore the borrow) is valid. Lifetimes therefore prevent dangling references, which occur when a reference points to memory that has been deallocated or otherwise invalidated.{{hi:Dangling references}}

The Rust compiler infers most lifetimes (see below), therefore we will most often write references with an implicit lifetime:

- `&i32`: a shared reference.
- `&mut i32`: a mutable reference with an implicit lifetime.

When the compiler can't figure out the relationships between the lifetimes of different references (especially in function signatures or struct definitions), you need to provide explicit lifetime annotations:

- Lifetime names are always prefixed with an apostrophe (e.g., `'a`, `'b`, `'static`). By convention, short, lowercase names are usually used. `'static` is a special lifetime that means the reference is valid for the entire duration of the program.
- When added to references, the lifetime annotation is inserted after `&` and before the `mut` keywords or the type:
  - `&'a i32`: a shared reference with an explicit lifetime `'a`.
  - `&'a mut i32`: a mutable reference with an explicit lifetime `'a`.

Note the following:

- Lifetimes are annotations, not types: Lifetimes don't change the underlying type of a variable. A `&i32` is a reference to an `i32`, regardless of its lifetime annotation. The lifetime annotation just provides extra information to the compiler about how long that reference is valid.
- A lifetime is said to "outlive" another one if its representative scope is as long or longer than the other. References with longer lifetimes can be freely coerced into references with shorter ones. As such, you should read a lifetime `'a` as "lives at least as long as `'a`".
- Lifetimes are compile-time only: they are erased during compilation and have no runtime overhead.

## Lifetime Elision Rules {#lifetime-elision-rules}

As discussed above, Rust's compiler infers most lifetimes. More specifically, it has "lifetime elision rules" that allow you to omit them in common, unambiguous cases. This reduces boilerplate.{{hi:Lifetime elision rules}}

- Each input lifetime parameter gets its own lifetime parameter. `fn foo(x: &str)` is automatically treated as `fn foo<'a>(x: &'a str)`.
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

## Use Lifetime Parameters {#skip}

Lifetime parameters can be added to function or method signatures, struct definitions, enumerations, unions, `impl` blocks, type aliases, traits, in the same way that a generic type parameter or constant can be added. They are used to specify the relationships between the lifetimes of different references in the function, type, or item.

Lifetime parameters are listed in angle brackets (e.g. `<'a>`), usually immediately after the name of the item. Lifetime parameters are a form of [[generics | generics]]. Lifetime parameters, type parameters, and const generics can be intermixed within `<...>`. Lifetime parameters should appear before any generic type and const parameters (e.g. `<'a, T, const N: usize>`):

```rust,editable
{{#include ../../crates/language/examples/lifetimes/lifetime_parameters.rs:example}}
```

### Use Lifetime Parameters in Functions {#lifetimes-in-functions}

In the following example, we define a function with a lifetime parameter, which is then used to specify the relationships between the lifetimes of different references. In this case, the generic lifetime parameter will get the concrete lifetime that is equal to the smaller of the lifetimes of the function arguments `x` and `y`:

```rust,editable
{{#include ../../crates/language/examples/lifetimes/lifetime_parameter_function.rs:example}}
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
- Use `Rc` or `Arc`.
- Use arena-style allocation to enforce shared lifetimes.
- Use raw pointers (Requires `unsafe` code).

```rust,editable
{{#include ../../crates/language/examples/lifetimes/self_referential_struct.rs:example}}
```

## References {#references}

- [Lifetime elision rules (Rust reference)](https://doc.rust-lang.org/reference/lifetime-elision.html)⮳.
- [Self-referential Structs](https://ksnll.github.io/rust-self-referential-structs)⮳.

## Related Topics {#related-topics}

- [[cow | COW]].
- [[memory-management | Memory Management]].
- [[ownership_borrowing | Ownership & Borrowing]].
- [[rust-patterns | Rust Patterns]].
- [[typecasts | Typecasts]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
