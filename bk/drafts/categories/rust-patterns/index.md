# Rust Patterns

[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

Shared solutions for particular situations specific to programming in Rust.

## Basic Patterns

### Ownership & Borrowing

| Pattern | Description | Resources |
|---|---|---|
| Ownership & Borrowing | Ownership ensure automatic memory clean up when a variable that owns data goes out of scope. This prevents memory leaks. Borrowing allows using a value without taking ownership of it. The borrowing rules prevent data races by ensuring that either multiple parts of the code can read the data, or one part can modify it, but not both at the same time. These are fundamental language features. | Rust Book, "Understanding Ownership". |
| RAII: Resource Acquisition Is Initialization | Manage resources memory, files, network connections by tying them to the lifetime of an object. Automatic cleanup via `Drop` trait. | Rust Book, "Understanding Ownership". |
| Interior Mutability | Allow mutation of data even when there are immutable references. Use `RefCell`, `Rc<RefCell<T>>`, `Cell` types. | `std::cell`, [`std::sync::atomic`][c~std::sync::atomic~docs]⮳{{hi:std::sync::atomic}} documentation. The Rust Book, "Interior Mutability". |

### Error Handling & Customization

| Pattern | Description | Example/Resources |
|---|---|---|
| Error Handling | Represent the possibility of failure via `Result` or absence via `Option`. `Result` and `Option` are part of the prelude. Use `match`, `unwrap`, `expect`, and `?` operator. Crates like [`anyhow`][c~anyhow~docs]⮳{{hi:anyhow}} and [`thiserror`][c~thiserror~docs]⮳{{hi:thiserror}} can help with error management. | The Rust Book, "Error Handling". |
| `?` Operator | Propagate errors concisely. | The Rust Book, "Error Handling". |
| `From` Trait | Define conversions between types with `From`, often used for error handling. | Standard library documentation. |

#### Error Handling

{{#include error_handling/error_handling.incl.md}}

#### Error Customization

{{#include error_handling/error_customization.incl.md}}

### Generics & Traits

| Pattern | Description | Example/Resources |
|---|---|---|
| Generics | Write code that works with multiple types without needing to specify them explicitly. | The Rust Book, "Generic Types, Traits, and Lifetimes". |
| Traits | Define shared behavior that types can implement. | The Rust Book, "Traits". |
| Trait Objects | Dynamically dispatch to different implementations of a trait at runtime, using `Box<dyn Trait>` or `&dyn Trait`. | The Rust Book, "Using Trait Objects That Allow for Different Types". |
| Associated Types | Define types associated with a trait. | The Rust Book, "Associated Types". |

### Concurrency and Asynchronous Programming

| Pattern | Description | Example/Resources |
|---|---|---|
| Threading | FIXME | `std::thread` and `std::sync` provide basic threading and synchronization primitives. |
| Atomically Reference Counting | Use `Arc` to share data safely between threads. Combine with `Mutex` or `RwLock` for mutable access. | The Rust Book, "Shared-State Concurrency". |
| Mutual Exclusion Lock | Use `Mutex` for protecting shared data from concurrent access. | The Rust Book, "Shared-State Concurrency". |
| Read-Write Lock | `RwLock` allows multiple readers or exclusive writers to access shared data. | Standard library documentation. |
| Channels | Communicate between threads using message passing. | The Rust Book, "Message Passing Concurrency". |
| Concurrent Data Structures | | [`crossbeam`][c~crossbeam~docs]⮳{{hi:crossbeam}} offers advanced concurrent data structures. |
| Asynchronous Programming | Build asynchronous applications using e.g. the [`tokio`][c~tokio~docs]⮳{{hi:tokio}} asynchronous runtime, futures, tasks, and actors. | [`tokio`][c~tokio~docs]⮳{{hi:tokio}} crate documentation. |

### Functional Programming with Rust

| Pattern | Description | Example/Resources |
|---|---|---|
| Closures | Use anonymous functions that can capture their environment. Fundamental language feature. | The Rust Book, "Closures". |
| Iterators | Access elements of a sequence. Fundamental language feature. | The [`itertools`][c~itertools~docs]⮳{{hi:itertools}} crate provides additional iterator adapters. The Rust Book, "Iterators". |

{{#include functional_programming.incl.md}}

### Other

| Pattern | Description | Example/Resources |
|---|---|---|
| Macros | Code that generates other code at compile time. Fundamental language features. | Crates like [`syn`][c~syn~docs]⮳{{hi:syn}} and [`quote`][c~quote~docs]⮳{{hi:quote}} are used for procedural macros. The Rust Book, "Macros". |
| Unsafe Code | Bypass Rust's safety guarantees (use with caution). | The Rust Book, "Unsafe Rust". Rustonomicon. |
| FFI (Foreign Function Interface) | Interact with code written in other languages e.g., C. | The Rust Book, "Foreign Function Interface". |

## Design Patterns

Design patterns are well-proven blueprints or best practices for solving recurring problems in software design.

### Creational Patterns

| Design Pattern | Description | Helpful Rust Crates | Notes |
|---|---|---|---|
| Builder | Constructing complex objects step-by-step. | Use [`derive_builder`][c~derive_builder~docs]⮳{{hi:derive_builder}} for code generation. | Often implemented directly using structs and methods. [`derive_builder`][c~derive_builder~docs]⮳{{hi:derive_builder}} reduces boilerplate. |
| Factory | Creating objects of different types based on some criteria. | Often implemented directly using traits, enums, or closures. | Traits and generics are frequently used. |
| Abstract Factory | Creating families of related objects. | Often implemented directly using traits and generics. | |
| Singleton | Ensuring a class has only one instance. | Often implemented directly using static variables or lazy initialization. | Can be implemented with [`lazy_static`][c~lazy_static~docs]⮳{{hi:lazy_static}}, though Singletons are often discouraged in Rust. |

#### Builder Patterns

{{#include builder_pattern.incl.md}}

### Other Creational Patterns

{{#include creational_patterns.incl.md}}

### Structural Patterns

| Design Pattern | Description | Helpful Rust Crates | Notes |
|---|---|---|---|
| Adapter | Making incompatible interfaces work together. | Often implemented directly using traits. | Traits are key for implementing adapters. |
| Composite | Representing hierarchical tree-like structures. | Often implemented directly using structs and enums. | |
| Decorator | Adding behavior to objects dynamically. | Often implemented directly using traits. | Traits are commonly used for decorators. |
| Facade | Providing a simplified interface to a complex system. |  | Usually a matter of structuring your code. |
| Flyweight | Sharing objects to reduce memory usage. | Often implemented directly using `Rc` or `Arc`. | `Rc` and `Arc` are used for shared ownership. |
| Proxy | Controlling access to another object. | Often implemented directly. | Can be implemented with custom types and dereferencing. |

{{#include structural_patterns.incl.md}}

### Behavioral Patterns

| Design Pattern | Description | Helpful Rust Crates | Notes |
|---|---|---|---|
| Chain of Responsibility | Handling requests by passing them along a chain of handlers. | Often implemented directly using enums or function pointers | |
| Command | Encapsulating a request as an object. | Often implemented directly using structs and closures | Closures can be helpful for command implementations. |
| Interpreter | Defining a grammatical representation for a language and providing an interpreter. | Parsing crates like [`nom`][c~nom~docs]⮳{{hi:nom}}, [`pest`][c~pest~docs]⮳{{hi:pest}}, [`lalrpop`][c~lalrpop~docs]⮳{{hi:lalrpop}} can be helpful. | |
| Iterator | Providing a way to access elements of a sequence. | | Iterators are a core language feature. [`itertools`][c~itertools~docs]⮳{{hi:itertools}} provides additional iterator adaptors. |
| Mediator | Defining an object that controls how other objects interact. | Often implemented directly | |
| Memento | Capturing and externalizing an object's internal state. | Often implemented directly using structs and serialization | [`serde`][c~serde~docs]⮳{{hi:serde}} can be useful for serialization. |
| Observer | Notifying interested parties when a state changes. | [`event-listener`][c~event_listener~docs]⮳{{hi:event-listener}} | Helps with implementing the observer pattern. |
| State | Altering an object's behavior when its internal state changes. | Often implemented directly using enums. | Enums are often used to represent states. |
| Strategy | Choosing an algorithm at runtime. | Often implemented directly using trait objects or enums. | Trait objects or enums are commonly used. |
| Template Method | Defining the skeleton of an algorithm and letting subclasses define specific steps. | Often implemented directly using traits | Traits are helpful for defining the template. |
| Visitor | Adding new operations to objects without changing their classes. | Often implemented directly using traits | Traits are usually used for visitor implementations. |
| Dependency Injection | Providing dependencies to components. | Dependency injection frameworks (`shaku`) exist, but it's often handled manually, especially in smaller projects. [`yew`][c~yew~docs]⮳{{hi:yew}} uses dependency injection for its component system. |

{{#include behavioral_patterns.incl.md}}

## Related Programming Tasks

| Pattern | Description | Example/Resources |
|---|---|---|
| Parsing | [`nom`][c~nom~docs]⮳{{hi:nom}}, [`pest`][c~pest~docs]⮳{{hi:pest}}, [`lalrpop`][c~lalrpop~docs]⮳{{hi:lalrpop}} | These crates are useful for parsing various input formats. |
| Serialization (Serde) | [`serde`][c~serde~docs]⮳{{hi:serde}}, [`serde_json`][c~serde_json~docs]⮳{{hi:serde_json}}, [`serde_yml`][c~serde_yml~docs]⮳{{hi:serde_yml}}, [`toml`][c~toml~docs]⮳{{hi:toml}} | [`serde`][c~serde~docs]⮳{{hi:serde}} is a powerful framework for serialization and deserialization. |
| CLI Argument Parsing | [`clap`][c~clap~docs]⮳{{hi:clap}}, [`structopt`][c~structopt~docs]⮳{{hi:structopt}} | These crates help with parsing command-line arguments. |
| Logging | [`log`][c~log~docs]⮳{{hi:log}}, [`env_logger`][c~env_logger~docs]⮳{{hi:env_logger}}, [`tracing`][c~tracing~docs]⮳{{hi:tracing}} | [`log`][c~log~docs]⮳{{hi:log}} is a logging facade, and [`env_logger`][c~env_logger~docs]⮳{{hi:env_logger}} and [`tracing`][c~tracing~docs]⮳{{hi:tracing}} are logging implementations. |
| Testing | Built-in | Rust has built-in support for unit and integration testing. Crates like [`rstest`][c~rstest~docs]⮳{{hi:rstest}} can help with testing. |
| Asynchronous Programming | [`tokio`][c~tokio~docs]⮳{{hi:tokio}}, [`async-std`][c~async_std~docs]⮳{{hi:async-std}}, [`futures`][c~futures~docs]⮳{{hi:futures}} | [`tokio`][c~tokio~docs]⮳{{hi:tokio}} and [`async-std`][c~async_std~docs]⮳{{hi:async-std}} are asynchronous runtimes. [`futures`][c~futures~docs]⮳{{hi:futures}} provides utilities for working with futures. |

## References

- [Rust Unofficial: patterns][rust-unofficial-patterns~github]⮳.
- [Rust: state machine pattern][rust-state-machine-pattern]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[index: organize / write / add cross links](https://github.com/john-cd/rust_howto/issues/469)

- [patterns: A catalog of Rust design patterns, anti-patterns and idioms](https://github.com/rust-unofficial/patterns)

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/scopeguard.rs:example}}
```

</div>
