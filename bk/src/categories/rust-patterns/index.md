# Rust Patterns

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

Shared solutions for particular situations specific to programming in Rust.

## Handle errors

{{#include error_handling.incl.md}}

## Customize errors

{{#include error_customization.incl.md}}

## Rust design patterns

{{#include design_patterns.incl.md}}

## Functional programming

{{#include functional_programming.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

## Rust idioms

{{#include _rust_idioms.incl.md}}

## Builder patterns

{{#include _builder_pattern.incl.md}}

[index: organize / write P1](https://github.com/john-cd/rust_howto/issues/469)

## Rust Patterns Organized by Topic

| Topic | Pattern | Description | Example/Resources |
|---|---|---|---|
| Ownership & Borrowing | RAII Resource Acquisition Is Initialization | Managing resources memory, files, network connections by tying them to the lifetime of an object. Automatic cleanup via `Drop` trait. | Standard Rust practice. See the Rust Book. |
| | Ownership & Borrowing | Core concepts of Rust's memory management. Moving, borrowing mutable & immutable, and copying data. | The Rust Book, "Understanding Ownership" |
| | Interior Mutability | Allowing mutation of data even when there are immutable references. Using `RefCell`, `Rc<RefCell<T>>`, `Cell`, `Atomic*` types. | The Rust Book, "Interior Mutability" |
| Error Handling | `Result` and `Option` | Representing the possibility of failure `Result` or absence `Option`. Using `match`, `unwrap`, `expect`, `?` operator. | The Rust Book, "Error Handling" |
| | `?` Operator | Propagating errors concisely. | The Rust Book, "Error Handling" |
| | `From` Trait | Defining conversions between types, often used for error handling. | Standard library documentation |
| Generics & Traits | Generics | Writing code that works with multiple types without needing to specify them explicitly. | The Rust Book, "Generic Types, Traits, and Lifetimes" |
| | Traits | Defining shared behavior that types can implement. | The Rust Book, "Traits" |
| | Trait Objects | Dynamically dispatching to different implementations of a trait at runtime. Using `Box<dyn Trait>` or `&dyn Trait`. | The Rust Book, "Using Trait Objects That Allow for Different Types" |
| | Associated Types | Defining types associated with a trait. | The Rust Book, "Associated Types" |
| Concurrency | `Arc` Atomically Reference Counted | Sharing data safely between threads. Combined with `Mutex` or `RwLock` for mutable access. | The Rust Book, "Shared-State Concurrency" |
| | `Mutex` Mutual Exclusion | Protecting shared data from concurrent access. | The Rust Book, "Shared-State Concurrency" |
| | `RwLock` Read-Write Lock | Allowing multiple readers or exclusive writers to access shared data. | Standard library documentation |
| | Channels mpsc | Communicating between threads using message passing. | The Rust Book, "Message Passing Concurrency" |
| | [`tokio`][c-tokio]⮳{{hi:tokio}} Asynchronous Runtime | Building asynchronous applications using futures, tasks, and actors. | [`tokio`][c-tokio]⮳{{hi:tokio}} crate documentation |
| Design Patterns | Builder | Creating complex objects step-by-step. ||
| | Factory | Creating objects of different types based on some criteria. ||
| | Strategy | Choosing an algorithm at runtime. ||
| | Observer | Notifying interested parties when a state changes. ||
| | Dependency Injection | Providing dependencies to components. | Often used with dependency injection frameworks |
| Other | Closures | Anonymous functions that can capture their environment. | The Rust Book, "Closures" |
| | Iterators | Providing a way to access elements of a sequence. | The Rust Book, "Iterators" |
| | Macros | Code that generates other code at compile time. | The Rust Book, "Macros" |
| | Unsafe Code | Bypassing Rust's safety guarantees use with extreme caution!. | The Rust Book, "Unsafe Rust" |
| | FFI Foreign Function Interface | Interacting with code written in other languages e.g., C. | The Rust Book, "Foreign Function Interface" |

| Topic/Pattern | Rust Crates Examples | Notes |
|---|---|---|
| Ownership & Borrowing | | These are fundamental language features. |
| Interior Mutability | `std::cell`, [`std::sync::atomic`][c-std::sync::atomic]⮳{{hi:std::sync::atomic}} | `Cell`, `RefCell`, `Atomic*` types are in the standard library. |
| Error Handling | | `Result` and `Option` are part of the prelude. Crates like [`anyhow`][c-anyhow]⮳{{hi:anyhow}} and [`thiserror`][c-thiserror]⮳{{hi:thiserror}} can help with error management. |
| Generics & Traits | | Fundamental language features. |
| Concurrency | `std::thread`, `std::sync`, [`crossbeam`][c-crossbeam]⮳{{hi:crossbeam}}, [`tokio`][c-tokio]⮳{{hi:tokio}}, [`async-std`][c-async_std]⮳{{hi:async-std}} | `std::thread` and `std::sync` provide basic threading and synchronization primitives. [`crossbeam`][c-crossbeam]⮳{{hi:crossbeam}} offers more advanced concurrent data structures. [`tokio`][c-tokio]⮳{{hi:tokio}} and [`async-std`][c-async_std]⮳{{hi:async-std}} are asynchronous runtimes. |
| Builder | Often implemented directly, no specific crate | While often implemented directly, the [`derive_builder`][c-derive_builder]⮳{{hi:derive_builder}} crate can help generate boilerplate for the builder pattern. |
| Factory | Often implemented directly, no specific crate | Commonly implemented using traits and generics. |
| Strategy | Often implemented directly, no specific crate | Often implemented using trait objects or enums. |
| Observer | [`event-listener`][c-event_listener]⮳{{hi:event-listener}} | Helps implement the observer pattern. |
| Dependency Injection | `di`, [`yew`][c-yew]⮳{{hi:yew}} for front-end | Dependency injection frameworks exist, but it's often handled manually, especially in smaller projects. [`yew`][c-yew]⮳{{hi:yew}} uses dependency injection for its component system. |
| Closures | | Fundamental language features. |
| Iterators | | Fundamental language features, but the [`itertools`][c-itertools]⮳{{hi:itertools}} crate provides additional iterator adapters. |
| Macros | | Fundamental language features. Crates like [`syn`][c-syn]⮳{{hi:syn}} and [`quote`][c-quote]⮳{{hi:quote}} are used for procedural macros. |
| Unsafe Code | | A language feature use with extreme care!. |
| FFI Foreign Function Interface| | A language feature. |
| Parsing | [`nom`][c-nom]⮳{{hi:nom}}, [`pest`][c-pest]⮳{{hi:pest}}, [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}} | These crates are useful for parsing various input formats. |
| Serialization Serde | [`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}, [`serde_yaml`][c-serde_yaml]⮳{{hi:serde_yaml}}, [`toml`][c-toml]⮳{{hi:toml}} | [`serde`][c-serde]⮳{{hi:serde}} is a powerful framework for serialization and deserialization. |
| CLI Argument Parsing | [`clap`][c-clap]⮳{{hi:clap}}, [`structopt`][c-structopt]⮳{{hi:structopt}} | These crates help with parsing command-line arguments. |
| Logging | [`log`][c-log]⮳{{hi:log}}, [`env_logger`][c-env_logger]⮳{{hi:env_logger}}, [`tracing`][c-tracing]⮳{{hi:tracing}} | [`log`][c-log]⮳{{hi:log}} is a logging facade, and [`env_logger`][c-env_logger]⮳{{hi:env_logger}} and [`tracing`][c-tracing]⮳{{hi:tracing}} are logging implementations. |
| Testing | Built-in - no specific crates | Rust has built-in support for unit and integration testing. Crates like [`rstest`][c-rstest]⮳{{hi:rstest}} can help with testing. |
| Asynchronous Programming | [`tokio`][c-tokio]⮳{{hi:tokio}}, [`async-std`][c-async_std]⮳{{hi:async-std}}, [`futures`][c-futures]⮳{{hi:futures}} | [`tokio`][c-tokio]⮳{{hi:tokio}} and [`async-std`][c-async_std]⮳{{hi:async-std}} are asynchronous runtimes. [`futures`][c-futures]⮳{{hi:futures}} provides utilities for working with futures. |

| Design Pattern | Description | Helpful Rust Crates | Notes |
|---|---|---|---|
| **Creational Patterns** | | | |
| Builder | Constructing complex objects step-by-step. | [`derive_builder`][c-derive_builder]⮳{{hi:derive_builder}} for code generation | Often implemented directly using structs and methods. [`derive_builder`][c-derive_builder]⮳{{hi:derive_builder}} reduces boilerplate. |
| Factory | Creating objects of different types based on some criteria. | Often implemented directly using traits, enums, or closures | Traits and generics are frequently used. |
| Abstract Factory | Creating families of related objects. | Often implemented directly using traits and generics | |
| Singleton | Ensuring a class has only one instance. | Often implemented directly using static variables or lazy initialization | Can be implemented with [`lazy_static`][c-lazy_static]⮳{{hi:lazy_static}} though often discouraged in modern Rust. |
| **Structural Patterns** | | | |
| Adapter | Making incompatible interfaces work together. | Often implemented directly using traits | Traits are key for implementing adapters. |
| Composite | Representing hierarchical tree-like structures. | Often implemented directly using structs and enums | |
| Decorator | Adding behavior to objects dynamically. | Often implemented directly using traits | Traits are commonly used for decorators. |
| Facade | Providing a simplified interface to a complex system. |  | Usually a matter of structuring your code. |
| Flyweight | Sharing objects to reduce memory usage. | Often implemented directly using `Rc` or `Arc` | `Rc` and `Arc` are used for shared ownership. |
| Proxy | Controlling access to another object. | Often implemented directly | Can be implemented with custom types and dereferencing. |
| **Behavioral Patterns** | | | |
| Chain of Responsibility | Handling requests by passing them along a chain of handlers. | Often implemented directly using enums or function pointers | |
| Command | Encapsulating a request as an object. | Often implemented directly using structs and closures | Closures can be helpful for command implementations. |
| Interpreter | Defining a grammatical representation for a language and providing an interpreter. | Parsing crates like [`nom`][c-nom]⮳{{hi:nom}}, [`pest`][c-pest]⮳{{hi:pest}}, [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}} can be helpful | |
| Iterator | Providing a way to access elements of a sequence. | | Iterators are a core language feature. [`itertools`][c-itertools]⮳{{hi:itertools}} provides additional iterator adaptors. |
| Mediator | Defining an object that controls how other objects interact. | Often implemented directly | |
| Memento | Capturing and externalizing an object's internal state. | Often implemented directly using structs and serialization | [`serde`][c-serde]⮳{{hi:serde}} can be useful for serialization. |
| Observer | Notifying interested parties when a state changes. | [`event-listener`][c-event_listener]⮳{{hi:event-listener}} | Helps with implementing the observer pattern. |
| State | Altering an object's behavior when its internal state changes. | Often implemented directly using enums | Enums are often used to represent states. |
| Strategy | Choosing an algorithm at runtime. | Often implemented directly using trait objects or enums | Trait objects or enums are commonly used. |
| Template Method | Defining the skeleton of an algorithm and letting subclasses define specific steps. | Often implemented directly using traits | Traits are helpful for defining the template. |
| Visitor | Adding new operations to objects without changing their classes. | Often implemented directly using traits | Traits are usually used for visitor implementations. |

</div>
