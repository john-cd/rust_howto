<!-- TODO Reviewed ??-??-2026. -->

# Inheritance

{{#include inheritance.incl.md}}

Rust has no class inheritance. Instead, model data relationships with composition or enums, and shared behavior with traits. These approaches make the stored data and supported operations explicit.

## Use an Enum for a Closed Set of Variants {#enum-variants}

When the set of cases is known and owned by one part of the program, an enum keeps the cases together and lets `match` check that each is handled.

```rust,noplayground
// Each variant contains the data specific to that shape.
enum Shape {
    Circle { radius: f64 },
    Square { side: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Square { side } => side * side,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

## Use Traits for Shared Behavior {#shared-behavior}

Traits describe behavior that different types can implement. Use a generic function when the caller's concrete type can be selected at compile time; use a trait object when one value or collection must hold different implementations at runtime.

```rust,noplayground
trait Animal {
    fn make_sound(&self) -> &'static str;
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> &'static str { "Woof!" }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> &'static str { "Meow!" }
}

fn sounds(animals: &[Box<dyn Animal>]) -> Vec<&'static str> {
    animals.iter().map(|animal| animal.make_sound()).collect()
}
```

The trait defines shared behavior, while each concrete type supplies its own implementation. `dyn Animal` selects the implementation at runtime. A trait is not a parent class: it does not provide inherited fields or make one struct a subtype of another. See [Trait Objects and Dynamic Dispatch](../../language/trait_objects_and_dynamic_dispatch.md) for more.

## Prefer Composition; Use `AsRef` Only for a Reference Conversion {#asref-projection}

Embed a value when another value contains it. This is a `has-a` relationship, not an `is-a` relationship. A narrow accessor is usually clearest when callers need only a few fields. A domain-specific trait is better when unrelated types should expose the same behavior. Implement `AsRef` when callers genuinely need a cheap, explicit reference conversion to the embedded value.

```rust,noplayground
{{#include ../../../crates/standard_library/examples/asref/asref_inheritance.rs:example}}
```

Do not implement `Deref` to imitate inheritance. Deref coercion is implicit and should model pointer-like access, not a general substitute for a base class.

## Related Topics {#related-topics .skip}

- [Enumerations](../../language/enumerations.md).
- [Traits](../../language/traits.md).
- [Trait Objects and Dynamic Dispatch](../../language/trait_objects_and_dynamic_dispatch.md).
- [Reference-to-reference Conversions](../../standard-library/reference-to-reference_conversions.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
