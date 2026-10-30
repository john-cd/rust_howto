<!-- TODO Reviewed ??-??-2026 -->

Rust does not use class inheritance for code reuse. Instead, compose values from smaller types, define shared behavior with traits, and wrap existing types with newtypes when a domain needs stronger invariants or a distinct interface. Common structural patterns include newtypes, adapters, decorators, facades, and composite trees.

## Create a Newtype for a Domain Identifier {#newtype}

Wrap a primitive in a tuple struct to make values with the same machine representation distinct in the type system. Keep the field private and expose constructors that maintain the domain's invariants.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/structural_patterns/newtype.rs:example}}
```

## Adapt a Reader into an Iterator {#adapter}

An adapter wraps one interface and presents another. This example turns a buffered line reader into an iterator of simple comma-separated rows; use a CSV parser for quoted fields and full CSV syntax.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/structural_patterns/adapter.rs:example}}
```

## Decorate a Reader with Byte Counting {#decorator}

A decorator implements the wrapped interface while adding behavior around each operation. `CountingReader` forwards `Read` calls and tracks the bytes returned without changing the reader's output.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/structural_patterns/decorator.rs:example}}
```

## Hide Subsystem Details Behind a Facade {#facade}

A facade offers a small, cohesive API over internal data structures or collaborating components. Callers use the directory's registration and lookup methods without depending on its `HashMap` representation.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/structural_patterns/facade.rs:example}}
```

## Compose a Tree with an Enum {#composite}

The composite pattern represents both leaves and groups with one interface. An enum can express the alternatives directly, and recursive methods can process a tree without a class hierarchy.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/structural_patterns/composite.rs:example}}
```

| Recipe | Crates | Categories |
| ------ | ------ | ---------- |
