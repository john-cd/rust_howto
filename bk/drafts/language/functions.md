# Functions

{{#include functions.incl.md}}

## Write a Rust Function {#function}

[![Rust by example - Functions][book-rust-by-example-fn-badge]][book-rust-by-example-fn]{{hi:fn}}{{hi:Functions}}

Functions are fundamental building blocks in Rust, used to encapsulate reusable blocks of code. Functions are defined with `fn`, followed by the function name, optional parameters between parentheses `(...)`, optional `->` followed by the return type, and curly braces `{ ... }` for the function body. Rust uses snake_case for function names by convention:

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

## Related Topics {#skip}

- [[closures | Closures]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
