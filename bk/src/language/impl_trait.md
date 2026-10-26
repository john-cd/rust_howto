<!-- TODO Reviewed ??-??-2026 -->

# Impl Trait

{{#include impl_trait.incl.md}}

[![Rust by Example - impl Trait][book~rust-by-example~impl_trait~badge]][book~rust-by-example~impl_trait]

## Simplify Method Signatures with `impl Trait` {#impl-trait}

`impl Trait`, where `impl` is a keyword and `Trait` is a trait name, specifies an unnamed but concrete type that implements a specific trait. It can appear in argument position (where it acts as an anonymous type parameter) and in return position (where it acts as an opaque return type). Return-position `impl Trait` is also allowed in trait methods and their implementations; this is called return-position `impl Trait` in traits (RPITIT) and has been stable since Rust 1.75. In Rust 2024, return-position `impl Trait` captures all in-scope generic lifetime, type, and const parameters by default. Use a `use<...>` bound to specify a narrower set of captured parameters; precise capturing syntax has been stable since Rust 1.82.

`impl Trait` is essentially syntactic sugar for a generic type parameter with a trait bound, like `<T: Trait>`, except that, in argument position, the type is anonymous and doesn't appear in the generic parameter list of a function. In return position, unlike with a generic type parameter, the function, not the caller, chooses the return type:

```rust,editable
{{#include ../../crates/language/examples/impl_trait/impl_trait.rs:example}}
```

Do not confuse `impl Trait` with `dyn Trait`. The [[trait_objects | Trait Objects]] chapter explains the difference.

## Return Opaque Types (esp. Closures and Iterators) with Return-position `impl Trait` {#return-position-impl-trait}

As discussed above, you can use `impl Trait` in the return type of a function to indicate that the function returns a type that implements a specific trait, without specifying the exact type.

This is useful when the exact type is complex, not relevant to the caller, or impossible to write explicitly, and especially for closures and iterators:

```rust,editable
{{#include ../../crates/language/examples/impl_trait/rpit.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[closures | Closures]].
- [[generics | Generics]].
- [[iterators | Iterators]].
- [[traits | Traits]].
- [[trait_objects | Trait Objects]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [[rust-patterns | Rust Patterns]].

accuracy review: traits, iterators, attributes and macros

</div>
