# Macros

{{#include macros.incl.md}}

## Macro Syntax {#macro-syntax}

[![Rust by example - macros][book~rust-by-example~macros~badge]][book~rust-by-example~macros]{{hi:Macros}}

Macros are code that writes other code - all at compile time.

 known as metaprogramming{{hi:Metaprogramming}}

Macros allow you to define reusable code snippets that can be expanded into more complex code at compile time. Macros are more flexible than functions, because they can handle a variable number of arguments and can manipulate code structure directly.

Unlike macros in C and other languages, Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs.

There are two main types of macros in Rust:

- Declarative Macros (aka "macros by example"). These are the most common and look like pattern-matching rules. You’ve probably used them already with `println!`, `vec!`, or `format!`.

- Procedural Macros are more advanced and allow you to operate on the abstract syntax tree (AST) of Rust code. They come in three flavors:
  - Custom derive: e.g., `#[derive(Debug)]`.
  - Attribute-like: e.g., `#[route(GET, "/")]`.
  - Function-like: e.g., `my_macro!(...)`. These macros look like functions, except that their name ends with a bang `!`.

## Macros By Example {#macros-by-example}

The `macro_rules!` macro is used to define a macro by example. The syntax consists of a name for the macro (e.g., `create_function`), followed by a set of rules enclosed in curly braces `{}`. Each rule has a pattern and a corresponding code block that will be substituted when the macro is invoked.

In the following example, the pattern `($func_name:ident)` matches an identifier (like a function name), and the code block defines a function with that name. The `$func_name` is a macro variable that will be replaced with the actual identifier provided when the macro is called.

- Pattern Matching: Macros match input syntax using patterns like $var:expr, $var:ident, etc.
- Repetition: You can repeat patterns using *, +, or ?, similar to regular expressions.
- Metavariables: These are the $name placeholders that capture parts of the input.
- Transcription: Once a pattern matches, the macro expands into the corresponding code.

```rust,editable
macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = 0;
            $(
                temp += $x;
            )*
            temp
        }
    };
}

fn main() {
    println!("{}", sum!(1, 2, 3, 4)); // Outputs 10
}
```

## Macro Uses {#macro-uses}

The following example demonstrates various ways macros can be used in Rust.

```rust,editable
{{#include ../../crates/language/examples/macros/macros.rs:example}}
```

## Procedural Macros {#procedural-macros}

Procedural macros are written in their own crate with `proc-macro = true` set in `Cargo.toml`.

## Common Macros in the Standard Library {#common-macros}

TODO cover common macros in std lib: https://doc.rust-lang.org/std/#macros

## Common Pitfalls {#pitfalls}

Here are some common pitfalls to watch out for:

- Overusing Macros: Sometimes a regular function or trait does the job just fine. Macros can obscure logic and make code harder to read or debug if used unnecessarily.
- Poor Macro Hygiene. If your macro introduces variables without care, it can clash with names in the surrounding scope. This can lead to confusing bugs. Rust tries to help with hygiene, but it’s not foolproof - especially in procedural macros.
- Complex or Unreadable Macros. Macros that try to do too much can become cryptic. If you can’t understand what your macro expands to without a mental workout, it might be time to simplify or rethink the design.
- Lack of Testing. Macros don’t get tested like functions do. If you don’t write tests that exercise all the macro’s patterns and edge cases, you might miss subtle bugs.
- Unexpected Expansion Behavior. Macros expand before type checking, so they can produce code that compiles incorrectly or fails in surprising ways. Debugging macro expansion can be tricky.
- Lifetime and Type Issues. Especially in procedural macros, forgetting to handle lifetimes or generic parameters properly can lead to compiler errors that are hard to trace.
- Unclear Error Messages. When something goes wrong inside a macro, the compiler often points to the macro invocation site, not the actual problem. Using compile_error! can help provide clearer diagnostics.

## References {#skip}

- [Rust reference - Macros][book~rust-reference~macros]⮳.
- [Rust by example - Macros][book~rust-by-example~macros]⮳ [![Rust by example - macros][book~rust-by-example~macros~badge]][book~rust-by-example~macros]{{hi:Macros}}⮳.
- The [Little Book of Rust Macros][book~rust-macros]⮳.
- [Macros in Rust: A tutorial with examples](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples)⮳.

## Related Topics {#skip}

- [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]].
  - [[compile_macros | Compile Macros]].
  - [[macro_tools | Macro Tools]].
  - [[write_proc_macros | Write Proc Macros]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[macros: write NOW](https://github.com/john-cd/rust_howto/issues/548)
</div>
