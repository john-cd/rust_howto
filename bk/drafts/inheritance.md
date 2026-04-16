# Inheritance

{{#include inheritance.incl.md}}

Rust does not support traditional inheritance like object-oriented languages. Instead, it encourages using _traits_ and _composition_ to achieve similar functionality, allowing for shared behavior without the complexities and pitfalls of inheritance.

More precisely, Rust tends to model things with 'has-a' relationships (composition) instead of 'is-a' relationships (inheritance).

## Use Enumerations and Traits Instead of Inheritance {#inheritance-in-rust}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Instead of inheritance, consider using an [`enum`][keyword~enum]↗{{hi:enum}} (or a [`struct`][keyword~struct]↗{{hi:struct}} type that contains fields that are always the same, plus a field with an `enum` type).

```rust,noplayground
// The base class becomes an `enum`.
// - The `enum` clearly defines all possible states or subtypes of `Shape` in one place.
//   With inheritance, we might have to scour multiple files to understand the full hierarchy.
// - All the data for a `Shape` (be it a circle, square, or triangle) is contained within the `enum`.
//   This can be more cache-friendly.
enum Shape {
    // The `derived classes` become variants.
    Circle { radius: f64 },
    Square { side: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        // `match` expressions on `enum`s are exhaustive. The compiler will force us to handle every possible variant,
        // which prevents common bugs that can arise when new subclasses are added in an inheritance hierarchy,
        //  but not all code paths are updated.
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Square { side } => side * side,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

Commonly, a trait may be used to extract a shared behavior:

```rust,noplayground
trait HasArea {
    fn area(&self) -> f64;
}

// Implement the trait for the `Shape` enum.
impl HasArea for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI *radius* radius,
            Shape::Square { side } => side *side,
            Shape::Triangle { base, height } => 0.5* base * height,
        }
    }
}
```

## Emulate Inheritance with `AsRef` {#inheritance-in-rust}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

You may also implement the [`AsRef`][c~std::convert::AsRef~docs]↗{{hi:std::convert::AsRef}} trait to retrieve the "base class":

```rust,editable
{{#include ../../crates/standard-library/examples/  /asref2.rs:example}}
```

Note that so-called "Deref polymorphism", meaning implementing [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} to emulate inheritance between structs, is considered an [anti-pattern][deref-anti-pattern~repo]↗, especially since `Deref` is an _implicit_ conversion.

### References {#references .skip}

- [Rust's `AsRef` Explained][blog~rusts-asref-explained]↗.
- [Rust is Beyond Object-Oriented, Part 3: Inheritance][blog~thecodedmessage-posts-oop-3-inheritance]↗.
- [How to Implement Inheritance-like Feature for Rust?][forum~how-to-implement-inheritance-like-feature-for-rust]↗.

## dynamic dispatch with trait objects {#dynamic-dispatch}

While Rust prefers static dispatch (using generics and monomorphization), you can achieve dynamic polymorphism—similar to virtual methods in other languages—using **trait objects**.

A trait object is written as `&dyn Trait` or `Box<dyn Trait>`. It allows you to store different types that implement the same trait in a single collection.

```rust,noplayground
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) { println!("Woof!"); }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) { println!("Meow!"); }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        animal.make_sound();
    }
}
```

## Composition over Inheritance {#composition}

Instead of inheriting behavior, Rust encourages embedding one struct within another. This "has-a" relationship is often cleaner and more flexible.

```rust,noplayground
struct Engine {
    hp: u32,
}

impl Engine {
    fn start(&self) { println!("Engine started with {} HP", self.hp); }
}

struct Car {
    engine: Engine, // Composition
    model: String,
}

impl Car {
    fn start(&self) {
        println!("Starting {}", self.model);
        self.engine.start();
    }
}
```

## Related Topics {#related-topics .skip}

- [[traits | Traits]].
- [[dynamic_typing | Dynamic Typing]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
