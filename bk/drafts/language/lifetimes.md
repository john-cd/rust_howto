# Lifetimes

{{#include lifetimes.incl.md}}

## Lifetime {#lifetime}

[![Rust by example - Lifetimes][book-rust-by-example-lifetimes-badge]][book-rust-by-example-lifetimes]{{hi:Lifetimes}}

Lifetimes are a mechanism that the Rust compiler uses to ensure that references are valid for as long as they are used (i.e. ensure all borrows are valid.)
Lifetimes help prevent dangling references, which occur when a reference points to memory that has been deallocated or otherwise invalidated.{{hi:Dangling references}}

A lifetime represents the scope for which a reference is valid. Lifetime names are always prefixed with an apostrophe (e.g., `'a`, `'b`, `'static`). By convention, short, lowercase names are usually used. `'static` is a special lifetime that means the reference is valid for the entire duration of the program.

Lifetimes can be added to function or method signatures, struct definitions, `impl` blocks, and references. When explicitly added to references, the lifetime annotation is inserted after `&` and before the `mut` keywords or the type:

- `&'a i32`: a shared reference with an explicit lifetime `'a`.
- `&i32`: a shared reference with an implicit lifetime (very common).
- `&'a mut i32`: a mutable reference with an explicit lifetime `'a`.
- `&mut i32`: a mutable reference with an implicit lifetime (very common).

Lifetimes are annotations, not types: Lifetimes don't change the underlying type of a variable. A `&i32` is still a reference to an `i32`, regardless of its lifetime annotation. The lifetime annotation just provides extra information to the compiler about how long that reference is valid.

The compiler infers most lifetimes, but explicit lifetime annotations are sometimes necessary: When the compiler can't figure out the relationships between the lifetimes of different references (especially in function signatures or struct definitions), you need to provide explicit lifetime annotations.

## Lifetime Annotations in Functions {#lifetimes-in-functions}

The generic lifetime{{hi:Lifetimes}} `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`:

```rust,editable
{{#include ../../crates/language/tests/lifetimes/generic_lifetime.rs:example}}
```

## Lifetime Annotations in Struct Definitions and Methods {#lifetime-annotations}

Lifetime annotations are used to specify the relationships between the lifetimes of different references.

```rust,noplayground
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}
```

```rust,editable
{{#include ../../crates/language/tests/lifetimes/lifetime.rs:example}}
```

## Static Lifetime {#static-lifetime}

```rust,editable
{{#include ../../crates/language/tests/lifetimes/static_lifetime.rs:example}}
```

## Related Topics {#skip}

- [[cow | COW]].
- [[memory-management | Memory Management]].
- [[ownership_borrowing | Ownership & Borrowing]].
- [[rust-patterns | Rust Patterns]].
- [[typecasts | Typecasts]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[polish NOW](https://github.com/john-cd/rust_howto/issues/547)
</div>
