<!-- TODO Reviewed ??-??-2026 -->

# Creational Patterns

{{#include creational_patterns.incl.md}}

Creational patterns describe how code constructs values and hides construction details. They are options, not requirements: Rust constructors, enums, traits, and builder APIs often express the same ideas with less indirection. Choose a pattern when it makes a real variation or invariant easier to understand.

## Implement a Factory {#factory}

A factory centralizes a decision about which concrete value to create. In Rust, a function returning an enum is often a good fit when the alternatives are known; a trait object or generic interface can help when callers need to work with different implementations.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/creational_patterns/factory.rs:example}}
```

## Implement an Abstract Factory {#abstract-factory}

An abstract factory creates a family of related values through one interface. This is useful when a selected configuration, platform, or theme must provide compatible products together. It can be excessive when there is only one family or the caller can construct the values directly.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/creational_patterns/abstract_factory.rs:example}}
```

See also:

- [abstract-factory in rust][abstract-factory-in-rust~website]↗.

## Implement a Singleton {#singleton}

A singleton exposes one shared instance, often through a global lazy initializer. Global state can make initialization order, mutation, and tests harder to reason about. Prefer passing a value or a reference through the owning application when possible; use a singleton only when process-wide identity is part of the requirement. If shared mutation is necessary, protect it with synchronization rather than a mutable static.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/creational_patterns/singleton.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[builder_pattern | Builder Pattern]] is a common Rust construction API for values with optional configuration.
- [[structural_patterns | Structural Patterns]] cover composition and relationships between types.
- [[behavioral_patterns | Behavioral Patterns]] cover communication and state transitions.
- [[lazy_initialization | Lazy Initialization]] describes safe one-time initialization.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1391)
</div>
