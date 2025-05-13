# Closures

{{#include closures.incl.md}}

## Closure Syntax {#skip}

[![Rust by example - Closures][book-rust-by-example-closures-badge]][book-rust-by-example-closures]

Closures are anonymous functions you can define inline, often right where you need to use them.
Here is a example of a short, inline closure with a simple expression as its body:

```rust,editable
{{#include ../../crates/language/tests/closures/closures.rs:example}}
```

{{i:Closures}} can capture variables{{hi:Capture of variables}} from their surrounding scope:

- by reference: `&T`,
- by mutable reference: `&mut T`, or
- by value: `T`.

They preferentially capture variables by reference and only go lower when required.

```rust,editable
fn main() {
    let outer_variable = 10;

    // Define a simple closure that takes a parameter and captures `outer_variable`:
    let my_closure = |x| {
        println!("Parameter x = {}", x);
        println!("Outer variable = {}", outer_variable); // Captured variable.
        x + outer_variable // Optional return value.
    }; // The body can be an expression or block between `{` and `}`.

    // Call the closure:
    let result = my_closure(5);
    println!("Result from closure: {}", result); // Output: 15.
}
```

## Force Closure Capture by Value {#move-closures}

A closure can be forced to capture its environment by copying or moving values by prefixing it with the `move` keyword.
This is often used to ensure that the closure's lifetime is `'static` and common when creating a new thread.

```rust,editable
{{#include ../../crates/language/tests/closures/closures_move.rs:example}}
```

## Annotate Closure Types {#closure-with-type-annotations}

Closures can often infer parameter and return-value types, but you can add them manually:

```rust,editable
{{#include ../../crates/language/tests/closures/closures_with_type_annotations.rs:example}}
```

## Use Closures as Function Arguments {#closures-as-input-parameters}

Closures are frequently used as arguments to functions, especially higher-order functions like those found on iterators (`map`, `filter`, `fold`, etc.).

A closure automatically implements one of three special traits:

- [`std::ops::Fn`][c-std::ops::Fn]{{hi:std::ops::Fn}}⮳ if the closure uses the captured value by reference (`&T`),
- [`std::ops::FnMut`][c-std::ops::FnMut]{{hi:std::ops::FnMut}}⮳ if the closure uses the captured value by mutable reference (`&mut T`),
- [`std::ops::FnOnce`][c-std::ops::FnOnce]{{hi:std::ops::FnOnce}}⮳ if the closure uses the captured value by value (`T`) e.g. moves it.

When defining a function that accepts a closure, you use the `Fn`, `FnMut`, or `FnOnce` traits as bounds:

```rust,editable
{{#include ../../crates/language/tests/closures/closures_as_input_parameters.rs:example}}
```

## Related Topics {#skip}

- [[functions | Functions]].
- [[rust-patterns | Rust Patterns]].
- [[functional_programming | Functional Programming]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO final review
</div>
