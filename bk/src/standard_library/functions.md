# Functions

{{#include functions.incl.md}}

## Write a Rust Function {#function}

[![Rust by example - Functions][book-rust-by-example-fn-badge]][book-rust-by-example-fn]{{hi:fn}}{{hi:Functions}}

Functions are fundamental building blocks in Rust, used to encapsulate reusable blocks of code. Functions are defined with `fn`, followed by the function name, optional parameters between parentheses `(...)`, optional `->` followed by the return type, and curly braces `{ ... }` for the function body. Rust uses snake_case for function names by convention.

Here are examples of functions with or without parameters or return value:

```rust,editable
{{#include ../../crates/language/tests/functions/functions.rs:example}}
```

## Write a Generic Function {#write-generic-functions}

[![Rust by example - generic functions][book-rust-by-example-generic_functions-badge]][book-rust-by-example-generic_functions]

Functions can be generic, meaning they can operate on parameters of various types without needing to be rewritten for each type:

```rust,editable
{{#include ../../crates/language/tests/functions/generic_functions.rs:example}}
```

Type parameters can be constrained to implement specified traits:

```rust,editable
{{#include ../../crates/language/tests/functions/generic_functions2.rs:example}}
```

## Diverging Functions {#diverging-functions}

Functions that are guaranteed never to return (e.g., they loop forever or exit the process) have the special return type `!` (the "never" type).{{hi:Diverging functions}}

```rust,editable,should_panic
{{#include ../../crates/language/tests/functions/diverging_functions.rs:example}}
```

## Function Pointers {#function-pointers}

Functions are first-class citizens in Rust. This means you can treat them like any other value: assign them to variables, pass them as arguments to other functions, etc. The type of a function pointer looks like `fn(i32) -> i32`.

```rust,editable
{{#include ../../crates/language/tests/functions/function_pointers.rs:example}}
```

## Return a Reference from a Function {#return-reference}

You cannot directly return a reference to a local variable created within and owned by a function:

- Local variables reside on the stack. This memory is typically deallocated when the function finishes executing.
- Returning a reference to a local variable would result in a dangling reference. The reference would point to a memory location that is no longer valid or might be overwritten later, leading to undefined behavior.

In Rust, the borrow checker prevents returning references to local data, enforcing memory safety:

```rust,compile_fail
// Compile Error[E0515]: cannot return reference to temporary value.
fn try_create<'a>() -> &'a String {
    let s = String::new();
    &s
    // References are pointers to memory locations. Once the function is executed, the local variable is popped off the execution stack.
}

#[test]
fn test() {
    let _ = try_create();
}
```

Instead, you should typically:

- Return an owned value (`String` instead of `&str`, `Vec<T>` instead of `&[T]`, etc.).
- Pass a reference as an argument: If the goal is to modify a variable, pass a mutable reference to a variable that exists in the calling scope. The function can then directly modify the original variable.
- Use a literal, a static or a constant: Since a static variable lives as long as the process runs, its references will be pointing to the same memory location both inside and outside the function.

```rust,editable
{{#include ../../crates/language/tests/functions/return_reference.rs:example}}
```

You can also use `std::borrow::Cow` to generalize over owned data and unowned references. See the [[cow | COW]] chapter.

## Related Topics {#skip}

- [[closures | Closures]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
