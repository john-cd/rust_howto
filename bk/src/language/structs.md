# Structs

{{#include structs.incl.md}}

## Define and Create Structs {#struct}

[![Rust by example - Structs][book~rust-by-example~structs~badge]][book~rust-by-example~structs]{{hi:Structs}}

Structs are custom data types that allow you to group related data together. The following example demonstrates how to define a `struct` and create a instance of it:

```rust,editable
{{#include ../../crates/language/examples/structs/structs.rs:example}}
```

Struct fields{{hi:Fields}} follow the general rule of everything being private by default,{{hi:Private by default}} unless annotated with [`pub`][book~rust-reference~visibility-and-privacy]{{hi:Visibility}}⮳. See the [[visibility | Visibility]] chapter.

Struct fields may be a primitive type, a tuple, an array, an enum, another struct (for nested structs), a reference...

```rust,editable
{{#include ../../crates/language/examples/structs/struct_fields.rs:example}}
```

### Declare Tuple Structs and Unit-like Structs {#tuple-structs}

You can also define tuple struct (no field names) and "Unit-like" structs (without fields). Unit-like structs are so named, because they behave similarly to `()`.

```rust,editable
{{#include ../../crates/language/examples/structs/tuple_structs.rs:example}}
```

### Design Structs for Ownership {#struct-ownership}

As shown in an example above, it is possible to declare structs, which fields are references. A lifetime parameter (e.g. `<'a>`) must be specified. See the [[lifetimes | Lifetimes]] chapter for more details.

However, you will typically design `struct`s for ownership, so that the data within the struct is valid for as long as the struct itself is valid.

That means that struct fields should use `T` instead of `&T` or `&mut T`, `String` instead of `&str`, `PathBuf` instead of `&Path`, `OsString` instead of `&OsStr`, etc.

Note that self-referential structs, that is structs that hold a reference to their own fields, can be tricky due to Rust's ownership and borrowing rules.  You can easily run into issues when moved, as the reference might become invalid. There is also no way to tie the lifetime of a reference to the lifetime of the struct that contains it.

## Create an Instance of Struct from Another with the Struct Update Syntax {#update-struct}

You may update a `struct` by using the `..previous_instance` to fill in the rest.

```rust,editable
{{#include ../../crates/language/examples/structs/struct_update.rs:example}}
```

## Implement and Use the Methods and Associated Functions of a Struct {#methods}

Methods and associated functions are defined within an `impl` (implementation) block.

Methods are functions that are associated with a _specific instance_ of a struct. Their first parameter (`self`) refers to the `struct` type, or a type alias or associated type thereof.

The first parameter of a method is usually `self`, `&self`, or `&mut self`, which is shorthand syntax for `self: Self`, `self: &Self`, and `self: &mut Self`, respectively; `Self` itself is an alias for the implementing type.

- `&self` borrows the struct instance immutably. Used for methods that only read data.
- `&mut self` borrows the struct instance mutably. Used for methods that need to modify the `struct`'s data.
- `self` takes ownership of the `struct` instance. This is less common, unless the method is intended to consume the instance.

The first parameter of a method can also be `self: Box<Self>`, `self: Arc<Self>`, `self: Rc<Self>`, `self: Pin<&Self>`, or a combination of these types.

Associated functions, in contrast, do not take `self` as a parameter. They are often constructor-like, returning `Self` (like `String::new`, `String::from`):

```rust,editable
{{#include ../../crates/language/examples/structs/struct_impl.rs:example}}
```

Each struct is allowed to have multiple `impl` blocks. Multiple `impl` blocks are useful when implementing generic types and traits - see below; also refer to the [[generics | Generics]] chapters.

Note that Rust automatic references and dereferences when calling methods: When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*`, so object matches the signature of the method. In other words, the following are the same:

```rust,noplayground
p1.distance(&p2);
(&p1).distance(&p2);
```

### Initialize Structs with a Constructor-like (Associated) Function {#struct-init}

It is common to define a function or an associated function to initialize the struct:

```rust,editable
{{#include ../../crates/language/examples/structs/struct_init.rs:example}}
```

## Define an Associated Constant for a Struct {#struct-constant}

You can also define associated constants in the implementation of a `struct`:

```rust,editable
{{#include ../../crates/language/examples/structs/struct_constant.rs:example}}
```

## Implement a Trait for a Struct {#implement-trait-for-struct}

Rust achieves polymorphism through traits. A `trait` defines a set of functions and methods (and associated types and constants) that a type can implement. When a `struct` implements a `trait`, it promises to provide concrete implementations for the (non-default) functions, methods, types, constants defined by that trait.

The syntax for trait implementations is `impl TraitName for StructName { ... }`, where `impl` is the keyword to start an implementation block, `TraitName` is the name of the trait you want to implement, and `StructName` is the specific struct for which you are providing the implementation.

The items you define within the `impl` block must match the signatures (for functions: name, parameters, return type) defined in the trait.

Like regular `struct` methods (see above), trait methods use `self`, `&self`, or `&mut self` as their first parameter to specify how they interact with an instance of the struct.

Trait implementations can also include the definition of associated types and constants. See the [[traits | Traits]] chapter for more details.

The following example demonstrates various implementations:

```rust,editable
{{#include ../../crates/language/examples/structs/struct_trait.rs:example}}
```

### Implement Common Standard Library Traits {#common-traits}

Rather then forcing you to implement very common traits repeatedly, Rust provides a `derive` attribute that can autoimplement them for you:

```rust,editable
{{#include ../../crates/language/examples/structs/common_traits.rs:example}}
```

See the [[derive | Derive]] chapter for more details.

## Define Generic Structs {#define-generic-structs}

Generic structs are like templates with one or more parameters, which act as placeholders to be specified later (e.g., during variable declaration or instance creation). They are declared by adding the aforementioned parameters between `<` and `>`, after the name of the struct. Their use promote code reuse.

Generic structs can be parameterized by types and (more infrequently) lifetimes and constants.

The following example demonstrates the declaration, implementation, and instance creation of a generic struct with a type parameter `T`.

Note how multiple `impl` blocks are possible; they can implement associated functions and methods for all possible `T`s; for specific types only; or conditionally, only for types that implement certain traits. See [[generics | Generics]] for more details.

```rust,editable
{{#include ../../crates/language/examples/structs/generic_structs.rs:example}}
```

## References {#skip}

[Creating Structs In Rust: Builder Pattern, Fluent Interfaces, And More](https://zerotomastery.io/blog/rust-struct-guide)⮳.

## Related Topics {#skip}

- [[enums | Enums]].
- [[lifetimes | Lifetimes]].
- [[generics | Generics]].
- [[traits | Traits]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
