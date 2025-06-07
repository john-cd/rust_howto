# Lifetimes

{{#include lifetimes.incl.md}}

## Lifetime {#lifetime}

[![Rust by example - Lifetimes][book-rust-by-example-lifetimes-badge]][book-rust-by-example-lifetimes]{{hi:Lifetimes}}

In Rust, _references_ allow you to access a value without taking ownership of it ("borrow" it). There are immutable references (of type `&T`), which allow read-only, shared access to a value; and mutable references (of type `&mut T`), which allow modification but enforce exclusive access.

References are similar to pointers but come with strict safety guarantees: they are aligned, not null, and pointing to memory containing a valid value of `T`.

To guarantee the latter, the Rust compiler attaches a _lifetime_ to each reference. A lifetime represents the _scope_ (a section of code) for which the value (and therefore the borrow) is valid. Lifetimes therefore prevent dangling references, which occur when a reference points to memory that has been deallocated or otherwise invalidated.{{hi:Dangling references}}

Lifetime names are always prefixed with an apostrophe (e.g., `'a`, `'b`, `'static`). By convention, short, lowercase names are usually used. `'static` is a special lifetime that means the reference is valid for the entire duration of the program.

When explicitly added to references, the lifetime annotation is inserted after `&` and before the `mut` keywords or the type:

- `&'a i32`: a shared reference with an explicit lifetime `'a`.
- `&i32`: a shared reference with an implicit lifetime (very common).
- `&'a mut i32`: a mutable reference with an explicit lifetime `'a`.
- `&mut i32`: a mutable reference with an implicit lifetime (very common).

Lifetimes are annotations, not types: Lifetimes don't change the underlying type of a variable. A `&i32` is a reference to an `i32`, regardless of its lifetime annotation. The lifetime annotation just provides extra information to the compiler about how long that reference is valid.

A lifetime is said to "outlive" another one if its representative scope is as long or longer than the other. References with longer lifetimes can be freely coerced into references with shorter ones. As such, you should read a lifetime `'a` as "lives at least as long as `'a`".

The compiler infers most lifetimes, eliminating most boilerplate, but explicit lifetime annotations are sometimes necessary: When the compiler can't figure out the relationships between the lifetimes of different references (especially in function signatures or struct definitions), you need to provide explicit lifetime annotations.

## Use `'static` as the Lifetime of the Running Program {#static-lifetime}

`'static` indicates that the data pointed to by the reference lives for the lifetime of the running program. It can still be coerced to a shorter lifetime. String literals are a common example of a string slice with `'static` lifetime.

```rust,editable
{{#include ../../crates/language/tests/lifetimes/static_lifetime.rs:example}}
```

## Use Lifetime Parameters {#skip}

Lifetime parameters can be added to function or method signatures, struct definitions, enumerations, unions, `impl` blocks, type aliases, traits, in the same way that a generic type parameter or constant can be added. They can be used to specify the relationships between the lifetimes of different references in the function, type, or item.

Lifetime parameters are listed in angle brackets (e.g. `<'a>`), usually immediately after the name of the item. Lifetime parameters are a form of [[generics | generics]]. Lifetime parameters, type parameters, and const generics can be intermixed within `<...>`. Lifetime parameters should appear before any generic type and const parameters (e.g. `<'a, T, const N: usize>`):

```rust,editable
{{#include ../../crates/language/tests/lifetimes/lifetime_parameter.rs:example}}
```

### Use Lifetime Parameters in Functions {#lifetimes-in-functions}

In the following example, we define a function with a lifetime parameter, which is then used to specify the relationships between the lifetimes of different references. In this case, the generic lifetime parameter will get the concrete lifetime that is equal to the smaller of the lifetimes of the function arguments `x` and `y`:

```rust,editable
{{#include ../../crates/language/tests/lifetimes/lifetime_parameter_function.rs:example}}
```

## Use Lifetimes and Lifetime Parameters in Struct Definitions and Implementations {#lifetime-annotations}

The following example shows a `struct` with a lifetime parameter and multiple implementations, two for any lifetimes, one for a specific lifetime.
This is a form of _conditional implementation_.

```rust,editable
{{#include ../../crates/language/tests/lifetimes/lifetime_parameter_struct.rs:example}}
```

### Avoid Self-referential Structs {#self-referential-structs}

Self-referential structs, that is structs that hold a reference to their own fields, can be tricky due to Rust's ownership and borrowing rules. You can easily run into issues when they are moved, as the references might become invalid. There is also no way to tie the lifetime of a reference to the lifetime of the struct that contains it.

Instead, you may:

- Store only owned data in the struct,
- Store the owned data outside the struct and let the struct hold only references,
- Store ranges rather than references, if the pointed-to type is a sequence (array, string, vector...),
- Use `Rc` or `Arc`.
- Use raw pointers,
- Use arena-style allocation to enforce shared lifetimes.

```rust,editable
{{#include ../../crates/language/tests/lifetimes/self_referential_struct.rs:example}}
```

## References {#skip}

[Self-referential Structs](https://ksnll.github.io/rust-self-referential-structs)⮳.

## Related Topics {#skip}

- [[cow | COW]].
- [[memory-management | Memory Management]].
- [[ownership_borrowing | Ownership & Borrowing]].
- [[rust-patterns | Rust Patterns]].
- [[typecasts | Typecasts]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
