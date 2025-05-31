# Trait Objects and Dynamic Dispatch

{{#include trait_objects.incl.md}}

Trait objects enable "dynamic dispatch", a form of polymorphism. They allow you to treat different concrete types that implement a specific trait as if they were the same at runtime. They are particularly useful when you need to create collections of different types that share common behavior, or when the exact type of an object isn't known until runtime.

## Trait Objects Basics {#trait-objects-basics}

Recall that a [trait][p-traits]{{hi:Traits}} (also called contract or interface in other programming languages) represents a behavior that multiple types can implement. In the example below, the `Draw` trait describes the ability to be drawn on a screen. A trait is at its core a set of function signatures (and associated types and constants).

A "trait object", denoted `dyn SomeTrait` where [`dyn`][keyword-dyn]{{hi:dyn}} is a keyword and `SomeTrait` is a trait (or a set thereof, see below), is an object that implements that specific trait, but which underlying concrete type is _not known at compile time_. In the example below, `dyn Draw` may be either a `Button` or a `Text` instance, since both types implement the `Draw` trait.

Because it can host different concrete types at runtime, a trait object is "unsized"{{hi:Unsized}}, a.k.a. dynamically sized, which implies that it is only allowed to show up behind a reference or a smart pointer like [`std::boxed::Box`][c-std::boxed::Box]{{hi:std::boxed::Box}}⮳:

- `&dyn MyTrait` is a reference to a trait object (which can be anywhere, stack or heap),
- `Box<dyn MyTrait>` is a heap-allocated trait object,
- `Rc<dyn MyTrait>` or thread-safe `Arc<dyn MyTrait>` are used for shared ownership of trait objects.

Under the covers, a trait object pointer is a "fat pointer" consisting of two parts: a pointer to an instance of a type `T` implementing `SomeTrait`; and a 'vtable' (virtual method table){{hi:Vtable}} pointing to the specific implementation of each function in the `SomeTrait` trait for the concrete type `T`.

Trait objects permit "late binding" of methods. Calling a method on a trait object results in dynamic dispatch at runtime: that is, a function pointer is loaded from the trait object vtable and invoked indirectly. The actual implementation for each vtable entry can vary on an object-by-object basis.

The following example demonstrates the use of trait objects to store a heterogenous collection of UI widgets and dynamic dispatch:

```rust,editable
{{#include ../../crates/language/tests/trait_objects/trait_objects.rs:example}}
```

Note the following:

- Dynamic dispatch incurs a small runtime overhead. There is an indirection when calling a method: first, the vtable pointer is dereferenced, then the method pointer within the vtable is dereferenced and called. This indirection can sometimes hinder compiler optimizations like inlining.
- Compared to generic types, trait objects can lead to smaller compiled binary sizes, because specialized (monomorphized) code is not generated for each concrete type; the trait-handling code is reused.
- Type Information: The concrete type is "erased" at compile time for the part of the code holding the trait object. The code only knows it has something that implements e.g. `Animal`.
- Object Safety: Traits must be "dyn-compatible" a.k.a. "object-safe" to be made into trait objects (see below).

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

- You want to return different concrete types implementing the same trait from a function, and the caller doesn't need to know the specific concrete type

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
- You need to reduce compile times or binary size in situations where monomorphization (from generics) would lead to excessive code generation.

Consider alternatives (like generics or enums) when:

- Performance is absolutely critical and the overhead of dynamic dispatch is unacceptable (profile first!).
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

Trait objects enable dynamic dispatch. This means that the decision of which concrete method implementation to call is made _at runtime_, rather than at compile time. This is in contrast to static dispatch, which Rust uses with generics, where the compiler generates specialized code for each concrete type used with a generic function or struct.

| Feature | Static Dispatch (Generics) | Dynamic Dispatch (Trait Objects) |
|---------|----------------------------|---------------------------------|
| Mechanism | Monomorphization (code duplication at compile time) | vtable lookups at runtime |
| Performance | Generally faster (no runtime lookup overhead) | Slightly slower (due to vtable indirection) |
| Flexibility | Less flexible for heterogeneous collections | More flexible for heterogeneous collections |
| Code Size | Can lead to larger binaries (due to monomorphization) | Can lead to smaller binaries (no code duplication) |
| Type Known | At compile time | At runtime (for the trait object itself) |
| Syntax | `fn foo<T: MyTrait>(item: T)` | `fn foo(item: &dyn MyTrait)` |

### `impl Trait` vs `dyn Trait` {#impl-trait-vs-dyn-trait}

Both `impl Trait` and `dyn Trait` allow you to work with types that implement a particular trait, providing a form of abstraction or polymorphism. However, they differ significantly in how they achieve this: `impl Trait` uses static dispatch (method calls are resolved at compile time); `dyn Trait` uses dynamic dispatch (resolved at run time).

`impl Trait` specifies an unnamed but concrete type that implements a specific trait. It can only appear in argument position (where it can act as an anonymous type parameter to functions) and in return position (where it can act as an opaque return type). `impl Trait` is essentially syntactic sugar for a generic type parameter like `<T: Trait>`, except that, in argument position, the type is anonymous and doesn't appear in the generic parameter list of a function. In return position, unlike with a generic type parameter, the function, not the caller, chooses the return type.

```rust,editable
{{#include ../../crates/language/tests/trait_objects/impl_trait.rs:example}}
```

## Trait Object Restrictions {#skip}

The following is an advanced topic. Consult it if the compiler rejects your trait object definitions.

### Dyn Compatibility / Object Safety {#dyn-compatibility}

Only a trait that is "dyn-compatible" (or ["object-safe"][book-rust-reference-object-safe]⮳) can be made into a trait object. The main rules for dyn compatibility are:

- All methods must not have `Self` as a return type. If a method returns `Self`, the compiler wouldn't know the concrete size of `Self` at runtime when dealing with a trait object. `Box<Self>` is allowed.
- All methods must not use generic type parameters. If a method had a generic type parameter (e.g., `fn foo<T>(&self, arg: T)`), the compiler couldn't fill in the concrete type for `T` at runtime.
- The trait itself cannot require `Self: Sized`. Trait objects are inherently `!Sized` (dynamically sized types), because their concrete type isn't known at compile time. Note that most methods in a trait implicitly have a `Self: Sized` bound on `Self`. You can relax this with `fn method_name(&self) where Self: ?Sized;`.
- All supertraits, if any, must also be dyn-compatible.

```rust
{{#include ../../crates/language/tests/trait_objects/dyn_compat.rs:example}}
```

### Trait Objects Can Have Only One Base Trait {#only-one-base-trait}

Trait objects can implement only one base [trait][p-traits]. They however can include auto traits and a lifetime parameter.{{hi:Autotraits}}

Auto traits are [special traits][book-rust-reference-special-traits]⮳, one of [`std::marker::Send`][c-std::marker::Send]{{hi:std::marker::Send}}⮳, [`std::marker::Sync`][c-std::marker::Sync]{{hi:std::marker::Sync}}⮳, [`std::marker::Unpin`][c-std::marker::Unpin]{{hi:std::marker::Unpin}}⮳, [`std::panic::UnwindSafe`][c-std::panic::UnwindSafe]{{hi:std::panic::UnwindSafe}}⮳, and [`std::panic::RefUnwindSafe`][c-std::panic::RefUnwindSafe]{{hi:std::panic::RefUnwindSafe}}⮳. `Sync` and `Send`, in particular, are useful in multithreaded environments.

For example, the following are valid trait objects:

```rust,editable,compile_fail,noplayground
dyn Trait // Base trait.
dyn Trait + Send // Base + autotrait. The order does not matter.
dyn Trait + Send + Sync // Base + autotraits.
dyn Trait + 'static // There may be at most one lifetime parameter.
```

Note that types that implement a trait must implements its supertraits.{{hi:Supertraits}}
As a result, you can call supertrait methods on a trait object:

```rust
{{#include ../../crates/language/tests/trait_objects/dyn_supertraits.rs:example}}
```

## See Also {#skip}

- [Trait Objects (docs)][book-rust-trait-objects]{{hi:Trait objects}}⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO review
review in depth Dyn Compatibility https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
example of use of a trait object with autotraits and/or a lifetime
</div>
