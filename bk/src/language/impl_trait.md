# Impl Trait

{{#include impl_trait.incl.md}}

## Simplify Method Signatures with `impl Trait` {#impl-trait}

`impl Trait`, where `impl` is a keyword and `Trait` is a trait name, specifies an unnamed but concrete type that implements a specific trait. It can only appear in argument position (where it can act as an anonymous type parameter to functions) and in return position (where it can act as an opaque return type).

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

## Related Topics {#skip}

- [[closures | Closures]].
- [[generics | Generics]].
- [[iterators | Iterators]].
- [[traits | Traits]].
- [[trait_objects | Trait Objects]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
