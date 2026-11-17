<!-- TODO Reviewed ??-??-2026. -->

# Rust Patterns

[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

This category collects common design patterns, Rust idioms, and ecosystem techniques. Use them when they make ownership, resource management, or API boundaries clearer; a direct function, struct, or enum is often simpler.

Start with the language patterns below when choosing ownership, error, or concurrency techniques. The design-pattern chapters group construction and behavior recipes: [Builder Pattern](builder_pattern.md), [Creational Patterns](creational_patterns.md), [Structural Patterns](structural_patterns.md), and [Behavioral Patterns](behavioral_patterns.md). [Functional Programming](functional_programming.md) covers iterator composition and related ecosystem tools. [Rust-specific Patterns](rust_specific_patterns.md) focuses on idioms built around Rust's type system.

## Basic Patterns

### Ownership & Borrowing

| Pattern | Description | Resources |
| --- | --- | --- |
| Ownership & Borrowing | Ownership determines who drops a value; borrowing grants access without transferring ownership. Rust's borrowing rules prevent aliased mutation and help make data races impossible in safe code. | [Ownership and Borrowing](../../language/ownership_and_borrowing.md). |
| RAII: Resource Acquisition Is Initialization | Tie a resource's cleanup to a value's lifetime so dropping the value releases it. The [`Drop`][c~std::ops::Drop~docs]↗{{hi:Drop}} trait supports this pattern. | [Resource Cleanup](../../standard-library/resource_cleanup.md). |
| Interior Mutability | Use interior mutability when a value must change through a shared reference. `Cell` and `RefCell` are single-threaded options; atomics and locks are for shared access across threads. | [Interior Mutability](../../standard-library/interior_mutability.md). |

### Error Handling and Customization

| Pattern | Description | Example/Resources |
| --- | --- | --- |
| Recoverable failures | Use `Option` for an absent value and `Result` when callers need the reason an operation failed. Propagate or handle expected errors; reserve panics for broken invariants. | [Recoverable Error Handling](../../standard-library/recoverable_error_handling.md). |
| Custom errors and reports | Use typed errors for library contracts, then add context or format diagnostics at an application boundary. | [Custom Errors and Reports](custom_errors.md). |

{{#include custom_errors.incl.md}}

### Generics & Traits

| Pattern | Description | Example/Resources |
| --- | --- | --- |
| Generics | Write code that works with multiple types without needing to specify them explicitly. | The Rust Book, "Generic Types, Traits, and Lifetimes". |
| Traits | Define shared behavior that types can implement. | The Rust Book, "Traits". |
| Trait Objects | Dynamically dispatch to different implementations of a trait at runtime, using `Box<dyn Trait>` or `&dyn Trait`. | The Rust Book, "Using Trait Objects That Allow for Different Types". |
| Associated Types | Define types associated with a trait. | The Rust Book, "Associated Types". |

### Concurrency and Asynchronous Programming

| Pattern | Description | Example/Resources |
| --- | --- | --- |
| Threading | Run independent work on OS threads and join their results. | [`std::thread`][c~std::thread~docs]↗{{hi:std::thread}} and `std::sync` provide basic threading and synchronization primitives. |
| Atomic Reference Counting | Use [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} to share data safely between threads. Combine with [`Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} or [`RwLock`][c~std::sync::RwLock~docs]↗{{hi:RwLock}} for mutable access. | The Rust Book, "Shared-State Concurrency". |
| Mutual Exclusion Lock | Use [`Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} for protecting shared data from concurrent access. | The Rust Book, "Shared-State Concurrency". |
| Read-Write Lock | [`RwLock`][c~std::sync::RwLock~docs]↗{{hi:RwLock}} allows multiple readers or exclusive writers to access shared data. | Standard library documentation. |
| Channels | Communicate between threads using message passing. | The Rust Book, "Message Passing Concurrency". |
| Concurrent Data Structures | Use a concurrent collection when multiple workers need coordinated access to shared data. | [Concurrent Data Structures](../concurrency/concurrent_data_structures.md) and [`crossbeam`][c~crossbeam~docs]↗{{hi:crossbeam}}. |
| Asynchronous Programming | Build asynchronous applications using e.g. the [`tokio`][c~tokio~docs]↗{{hi:tokio}} asynchronous runtime, futures, tasks, and actors. | [`tokio`][c~tokio~docs]↗{{hi:tokio}} crate documentation. |

### Functional Programming with Rust

| Pattern | Description | Example/Resources |
| --- | --- | --- |
| Closures | Use anonymous functions that can capture their environment. Fundamental language feature. | The Rust Book, "Closures". |
| Iterators | Access elements of a sequence. Fundamental language feature. | The [`itertools`][c~itertools~docs]↗{{hi:itertools}} crate provides additional iterator adapters. The Rust Book, "Iterators". |

{{#include functional_programming.incl.md}}

### Other

| Pattern | Description | Example/Resources |
| --- | --- | --- |
| Macros | Code that generates other code at compile time. Fundamental language features. | Crates like [`syn`][c~syn~docs]↗{{hi:syn}} and [`quote`][c~quote~docs]↗{{hi:quote}} are used for procedural macros. The Rust Book, "Macros". |
| Unsafe Code | Bypass Rust's safety guarantees (use with caution). | The Rust Book, "Unsafe Rust". Rustonomicon. |
| FFI (Foreign Function Interface) | Interact with code written in other languages e.g., C. | The Rust Book, "Foreign Function Interface". |
| Plugin Registration | Let independent, statically linked crates contribute commands without editing a central list. | [Register Commands Explicitly or with Inventory](plugin_registration.md). | `inventory` discovers linked registrations; it does not load dynamic libraries. |

## Design Patterns

Design patterns are recurring ways to organize software. In Rust, prefer the smallest design that makes ownership, variation, or state transitions clear; class-oriented patterns may be better expressed with enums, traits, closures, or composition.

### Creational Patterns

| Design Pattern | Description | Helpful Rust Crates | Notes |
| --- | --- | --- | --- |
| Builder | Constructing complex objects step-by-step. | Use [`derive_builder`][c~derive_builder~docs]↗{{hi:derive_builder}} for code generation. | Often implemented directly using structs and methods. [`derive_builder`][c~derive_builder~docs]↗{{hi:derive_builder}} reduces boilerplate. |
| Factory | Creating objects of different types based on some criteria. | Often implemented directly using traits, enums, or closures. | Traits and generics are frequently used. |
| Abstract Factory | Creating families of related objects. | Often implemented directly using traits and generics. | |
| Singleton | Exposing one shared instance when process-wide identity is part of the requirement. | Prefer passing a value through the application; `OnceLock` or `LazyLock` can initialize a global value. | Global state makes ownership, mutation, and tests harder to reason about. |

#### Builder Patterns

{{#include builder_pattern.incl.md}}

### Other Creational Patterns

{{#include creational_patterns.incl.md}}

### Structural Patterns

| Design Pattern | Description | Helpful Rust Crates | Notes |
| --- | --- | --- | --- |
| Adapter | Making incompatible interfaces work together. | Often implemented directly using traits. | Traits are key for implementing adapters. |
| Composite | Representing hierarchical tree-like structures. | Often implemented directly using structs and enums. | |
| Decorator | Adding behavior to objects dynamically. | Often implemented directly using traits. | Traits are commonly used for decorators. |
| Facade | Providing a simplified interface to a complex system. | | Usually a matter of structuring your code. |
| Flyweight | Sharing objects to reduce memory usage. | Often implemented directly using [`Rc`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} or [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}}. | [`Rc`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} and [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} are used for shared ownership. |
| Proxy | Controlling access to another object. | Often implemented directly. | Can be implemented with custom types and dereferencing. |

[See Structural Patterns](structural_patterns.md) for the recipes.

### Behavioral Patterns

| Design Pattern | Description | Helpful Rust Crates | Notes |
| --- | --- | --- | --- |
| Chain of Responsibility | Handling requests by passing them along a chain of handlers. | Often implemented directly using enums or function pointers | |
| Command | Encapsulating a request as an object. | Often implemented directly using structs and closures | Closures can be helpful for command implementations. |
| Interpreter | Defining a grammatical representation for a language and providing an interpreter. | Parsing crates like [`nom`][c~nom~docs]↗{{hi:nom}}, [`pest`][c~pest~docs]↗{{hi:pest}}, [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}} can be helpful. | |
| Iterator | Providing a way to access elements of a sequence. | | Iterators are a core language feature. [`itertools`][c~itertools~docs]↗{{hi:itertools}} provides additional iterator adaptors. |
| Mediator | Defining an object that controls how other objects interact. | Often implemented directly | |
| Memento | Capturing and externalizing an object's internal state. | Often implemented directly using structs and serialization | [`serde`][c~serde~docs]↗{{hi:serde}} can be useful for serialization. |
| Observer | Notifying interested parties when a state changes. | [`event-listener`][c~event-listener~docs]↗{{hi:event-listener}} | Helps with implementing the observer pattern. |
| State | Altering an object's behavior when its internal state changes. | Often implemented directly using enums. | Enums are often used to represent states. |
| Strategy | Choosing an algorithm at runtime. | Often implemented directly using trait objects or enums. | Trait objects or enums are commonly used. |
| Template Method | Defining a sequence of steps while allowing implementations to supply selected behavior. | A trait with default methods can provide the shared sequence. | Rust traits replace the inheritance-based subclass variation used in some languages. |
| Visitor | Adding operations over a set of data variants without placing every operation on each variant. | Often implemented with enums and traits. | Consider a direct `match` when the set of variants is closed. |
| Dependency Injection | Supplying a component with the services it needs. | Constructors, function arguments, and context providers are common approaches; frameworks such as [`shaku`][c~shaku~docs]↗{{hi:shaku}} offer additional support. | Prefer explicit dependencies when they keep the component easy to understand and test. |

{{#include behavioral_patterns.incl.md}}

## Related Programming Tasks

| Task | Where to continue |
| --- | --- |
| Parsing | [Parser Implementations](../parser-implementations/index.md) |
| Serialization | [Encoding and Serialization](../encoding/index.md) |
| Command-line interfaces | [Command-line Interface](../command-line-interface/index.md) |
| Logging and diagnostics | [Debugging and Observability](../development-tools_debugging/index.md) |
| Testing | [Testing](../development-tools_testing/index.md) |
| Asynchronous programming | [Asynchronous Programming](../asynchronous/index.md) |

## References

- [Rust Unofficial: patterns][rust-unofficial-patterns~repo]↗.
- [Rust: state machine pattern][rust-state-machine-pattern]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
