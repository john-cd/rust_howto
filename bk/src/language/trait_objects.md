# Trait Objects and Dynamic Dispatch

{{#include trait_objects.incl.md}}

Trait objects enable "dynamic dispatch", a form of polymorphism. They allow you to treat different concrete types that implement a specific trait as if they were the same at runtime. They are particularly useful when you need to create collections of different types that share common behavior, or when the exact type of an object isn't known until runtime.

## Trait Objects Basics {#trait-objects-basics}

Recall that a [trait][p~traits]{{hi:Traits}} (also called contract or interface in other programming languages) represents a behavior that multiple types can implement. In the example below, the `Draw` trait describes the ability to be drawn on a screen. A trait is at its core a set of function signatures (and associated types and constants).

A "trait object", denoted `dyn SomeTrait` where [`dyn`][keyword~dyn]↗{{hi:dyn}} is a keyword and `SomeTrait` is a trait (or a set thereof, see below), is an object that implements that specific trait, but which underlying concrete type is _not known at compile time_. In the example below, `dyn Draw` may be either a `Button` or a `Text` instance, since both types implement the `Draw` trait.

Because it can host different concrete types at runtime, a trait object is "unsized"{{hi:Unsized}}, a.k.a. dynamically sized, which implies that it is only allowed to show up behind a reference or a smart pointer like [`std::boxed::Box`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}}:

- `&dyn MyTrait` is a reference to a trait object (which can be anywhere, stack or heap),
- `Box<dyn MyTrait>` is a heap-allocated trait object,
- `Rc<dyn MyTrait>` or thread-safe `Arc<dyn MyTrait>` are used for shared ownership of trait objects.

Under the covers, a trait object pointer is a "fat pointer" consisting of two parts: a pointer to an instance of a type `T` implementing `SomeTrait`; and a 'vtable' (virtual method table){{hi:Vtable}} pointing to the specific implementation of each function in the `SomeTrait` trait for the concrete type `T`.

Trait objects permit "late binding" of methods. Calling a method on a trait object results in dynamic dispatch at runtime: that is, a function pointer is loaded from the trait object vtable and invoked indirectly. The actual implementation for each vtable entry can vary on an object-by-object basis.

The following example demonstrates the use of trait objects to store a heterogenous collection of UI widgets and dynamic dispatch:

```rust,editable
{{#include ../../crates/language/examples/trait_objects/trait_objects.rs:example}}
```

Note the following:

- Type information erasure: The concrete type is "erased" at compile time for the part of the code holding the trait object. The code only knows it has something that implements a trait.
- Object safety: Traits must be "dyn-compatible" (a.k.a. "object-safe") to be made into trait objects (see below).

## Decide When to Use Trait Objects (and When Not To) {#when-to-use-trait-objects}

Use trait objects when:

- You need a heterogeneous collection of types that share a common interface:

```rust,editable,noplayground
// Cats and Dogs:
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
    Box::new(Dog),
];

for animal in animals {
    animal.make_noise(); // Dynamically dispatches to `Dog::make_noise` or `Cat::make_noise`, depending on the object, at run time.
}
```

- You want to return different concrete types implementing the same trait from a function, and the caller doesn't need to know the specific concrete type:

```rust,editable,noplayground
fn get_animal(is_dog: bool) -> Box<dyn Animal> {
    if is_dog {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

let my_dog = get_animal(true);
my_dog.make_noise();
```

- You are implementing a plugin system where the types of plugins are not known at compile time.
- You need to reduce compile times or binary size. Compared to generic types, trait objects can lead to smaller compiled binary sizes, because specialized (monomorphized) code is not generated for each concrete type; the trait-handling code is reused.

Consider alternatives (like generics or enums) when:

- Performance is absolutely critical and the overhead of dynamic dispatch is unacceptable (profile first!). Dynamic dispatch incurs a small runtime overhead. As discussed above, there is an indirection when calling a method: first, the vtable pointer is dereferenced, then the method pointer within the vtable is dereferenced and called. This indirection can sometimes hinder compiler optimizations like inlining.

- You have a _small, fixed_ set of types that can implement the behavior. An enum with methods might be simpler and offer static dispatch:

```rust,editable
enum Shape {
    Circle(CircleData),
    Square(SquareData),
}

impl Shape {
    fn draw(&self) {
        match self {
            Shape::Circle(data) => data.draw_circle(),
            Shape::Square(data) => data.draw_square(),
        }
    }
}
```

## Static Dispatch vs. Dynamic Dispatch {#static-dispatch-vs-dynamic-dispatch}

Trait objects enable _dynamic dispatch_. This means that the decision of which concrete method implementation to call is made _at runtime_, rather than at compile time. This is in contrast to _static dispatch_, which Rust uses with generics (and `impl Trait`, see below), where the compiler generates specialized code for each concrete type used by a generic item.

| Feature | Static Dispatch (Generics) | Dynamic Dispatch (Trait Objects) |
|---------|----------------------------|---------------------------------|
| Mechanism | Monomorphization (code duplication at compile time) | vtable lookups at runtime |
| Performance | Generally faster (no runtime lookup overhead) | Slightly slower (due to vtable indirection) |
| Flexibility | Less flexible for heterogeneous collections | More flexible for heterogeneous collections |
| Code Size | Can lead to larger binaries (due to monomorphization) | Can lead to smaller binaries (no code duplication) |
| Type Known | At compile time | At runtime (for the trait object itself) |
| Syntax | `fn foo<T: MyTrait>(item: T)` | `fn foo(item: &dyn MyTrait)` |

### `impl Trait` vs `dyn Trait` {#impl-trait-vs-dyn-trait}

`impl Trait`, where `impl` is a keyword and `Trait` is a trait name, specifies an unnamed but concrete type that implements a specific trait.

Both `impl Trait` and `dyn Trait` allow you to work with types that implement a particular trait, providing a form of abstraction or polymorphism. However, they differ significantly in how they achieve this: `impl Trait` uses static dispatch (method calls are resolved at compile time); `dyn Trait` uses dynamic dispatch (resolved at run time).

See [[impl_trait | Impl Trait]] for more details.

## Trait Object Restrictions {#skip}

### Trait Objects Can Have Only One Base Trait {#only-one-base-trait}

Trait objects can implement _only one_ base [trait][p~traits]. If you need a trait object for two or more traits, create a new trait that e.g. uses them as supertraits.

Note, however, two exceptions:

- Types that implement a trait must implement its supertraits.{{hi:Supertraits}} As a result, you can call supertrait methods on a trait object:

```rust,editable
{{#include ../../crates/language/examples/trait_objects/dyn_supertraits.rs:example}}
```

- Trait objects can include "auto traits".{{hi:Auto traits}}

Auto traits are [special traits][book~rust-reference~special-traits]↗, one of [`std::marker::Send`][c~std::marker::Send~docs]↗{{hi:std::marker::Send}}, [`std::marker::Sync`][c~std::marker::Sync~docs]↗{{hi:std::marker::Sync}}, [`std::marker::Unpin`][c~std::marker::Unpin~docs]↗{{hi:std::marker::Unpin}}, [`std::panic::UnwindSafe`][c~std::panic::UnwindSafe~docs]↗{{hi:std::panic::UnwindSafe}}, and [`std::panic::RefUnwindSafe`][c~std::panic::RefUnwindSafe~docs]↗{{hi:std::panic::RefUnwindSafe}}. The compiler automatically implements these autotraits for types if certain conditions are met.

For example, the following are valid trait objects:

```rust,compile_fail,noplayground
dyn Trait // Base trait.
dyn Trait + Send // Base + autotrait. The order does not matter.
dyn Trait + Send + Sync // Base + autotraits.

// Note: there may be also (at most one) lifetime parameter:
dyn Trait + 'static
dyn Trait + Send + 'static
```

Common examples include `Send` and `Sync` for thread safety. A type is `Send` if it can be safely sent to another thread, and `Sync` if it can be safely shared between threads. To use a trait object in a multithreaded environment, you will often need one or both of these autotraits:

```rust,editable
{{#include ../../crates/language/examples/trait_objects/dyn_autotraits.rs:example}}
```

### Dyn Compatibility / Object Safety {#dyn-compatibility}

Only a trait that is "dyn-compatible" (or "object-safe") can be made into a trait object. The rules for dyn compatibility are rather complicated, constraining both the trait and the methods within, and (as of June 2025) not consistently documented in the [Rust reference][book~rust-reference~object-safe]↗. When designing a trait for use in a trait object, let the compiler's error messages guide you.

The dyn compatibility restrictions stem from the fact that trait objects are inherently `!Sized` (dynamically sized types), because their concrete type (`Self`) isn't known at compile time:

- The underlying trait cannot require `Self: Sized`.
- All methods must not have `Self` as a return type. If a method returns `Self`, the compiler wouldn't know the concrete size of `Self` at runtime when dealing with a trait object.
- All methods must not use generic type parameters. Generics are not compatible with vtables. If a method had a generic type parameter (e.g., `fn foo<T>(&self, arg: T)`), the compiler couldn't fill in the concrete type for `T` at runtime.
- For the same reason, opaque return type (impl Trait, `async fn`) are not allowed.

In addition,

- The trait must not have any associated constant or any associated type with generics.
- All supertraits, if any, must also be dyn-compatible.

```rust
{{#include ../../crates/language/examples/trait_objects/dyn_compat.rs:example}}
```

## Related Topics {#related-topics}

- [Trait Objects (docs)][book~rust~trait-objects]↗{{hi:Trait objects}}.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
