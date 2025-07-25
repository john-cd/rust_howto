# Macros

{{#include macros.incl.md}}

Rust macros enable metaprogramming, allowing you to write code that generates other code. This capability allows for code reuse by reducing boilerplate and enhancing readability through more expressive syntax.

- Macros can implement common traits or create domain-specific languages.
- Macros are commonly used as variadic interfaces: They are more flexible than functions, because they can handle a variable number of arguments. An example is `println!`, which can take any number of arguments, depending on the format string.
- Macros run at compile time, so any errors in the generated code are caught early. Macros can also achieve performance optimizations through compile-time computation and code transformation.

## Use Common Macros from the Standard Library {#common-macros}

Commonly used macros include:

- `println!`, `eprintln!`, `print!`, `eprint!` handle formatted output to standard output (`println!`, `print!`) or standard error (`eprintln!`, `eprint!`). They work similarly to `C`'s `printf` and support various formatting specifiers. `println!` and `eprintln!` automatically add a newline at the end.
- `format!` is similar to `print!`, but instead of printing to the console, it constructs and returns a `String` with the formatted content.
- `concat!` concatenates literals into a static string slice.
- `vec!` allows for convenient creation and initialization of `Vec` (vectors). For example, `vec![1, 2, 3]` creates a vector with integers 1, 2, and 3.
- `dbg!` is a debugging macro. It prints the value of an expression to standard error along with its file, line number, and the expression itself, and then returns the expression's value. This is useful for quickly inspecting values during development.
- `assert!`, `assert_eq!`, `assert_ne!` are used for assertions, primarily in testing.
  - `assert!(condition)` panics if the condition is false.
  - `assert_eq!(left, right)` panics if `left` is not equal to `right`, providing a helpful message showing both values.
  - `assert_ne!(left, right)` panics if `left` is equal to `right`.
  - There are also `debug_assert`, `debug_assert_eq`, `debug_assert_ne` statements that are only enabled in non-optimized builds by default.
- `unreachable!` is used to mark code paths that should logically never be reached. If the code reaches an `unreachable!` macro, it will cause a panic, indicating a logical error in the program.
- `todo!` is a placeholder for not-yet-implemented code. If executed, it will cause a panic with a message indicating that the functionality is "not yet implemented." It's useful during development to mark areas that still need attention.
- `panic!` immediately terminates the current thread with a panic. You can provide a custom message that will be displayed when the panic occurs. It's used for unrecoverable errors.
- `cfg!` evaluates boolean combinations of configuration flags at compile-time. It is often used with or instead of the `#[cfg]` attribute.

## Use Macros {#use-macros}

The following example demonstrates various ways macros can be used in Rust. Macros can be used as expressions or statements; in a pattern, type alias, or declaration; or as an associated item. It is also possible to call macros within a macro:

```rust,editable
{{#include ../../crates/language/examples/macros/macros.rs:example}}
```

## Write Macros {#write-macros}

There are two main types of macros in Rust:

- _Declarative Macros_ (a.k.a. "macros by example") are the most common and look like pattern-matching rules. They are expanded at compile-time, meaning they incur no runtime performance cost, and their output is checked syntactically and type-checked. Examples in the Standard Library include `println!(...)`, `vec![...]`, and `format!(...)`.

- _Procedural Macros_ are more advanced and allow you to operate on `TokenStreams` (representing the source code). They are compiled before the main program and can perform more complex code generation. They come in three flavors:
  - Custom derive macros e.g., `#[derive(Debug)]`, automatically generate trait implementations for structs and enums (here the `Debug` trait).
  - Attribute-like macros e.g., `#[route(GET, "/")]`, apply custom attributes to items like functions or modules to modify their behavior.
  - Function-like macros e.g., `my_macro!(...)` look like functions, except that their name ends with a bang `!`.

### Write Macros By Example {#macros-by-example}

[![Rust by example - macros][book~rust-by-example~macros~badge]][book~rust-by-example~macros]{{hi:Macros}}{{hi:Metaprogramming}}

Macros-by-example allow you to define reusable code snippets. They are excellent for simple syntactic transformations and reducing boilerplate based on structural patterns.

They are defined using the `macro_rules!` macro and work by pattern matching, where you provide patterns and corresponding code templates. They are expanded into source code that gets compiled with the rest of the program.

The `macro_rules!` syntax, which resembles a `match` expression, consists of a name for the macro (e.g., `say_hello`), followed by a set of rules enclosed in curly braces `{}` and separated by semicolons. Each rule has a _matcher_, a pattern describing the syntax that it matches; and, followed by `=>`, a _transcriber_, code that will be substituted when the macro is invoked.

For example, the following defines a basic `say_hello` macro (with one rule without arguments), then calls it:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example.rs:example}}
```

You can use metavariables to capture parts of the input. They are prefixed with a dollar sign (`$`). Inside matchers, `$name:fragment-specifier` matches a Rust syntax fragment of the kind specified and binds it to the metavariable `$name`. Valid fragment specifiers are:

- `block`: a block expression e.g. `{ let x = 1; x + 2 }`,
- `expr`: an expression that produces a value, e.g. `a + 2` or `some_function()`,
- `ident`: a variable or function name, e.g. `foo` or `_identifier` or even a raw identifier like `r#true`,
- `item`: an item, i.e. a function, module, use declaration, type alias, struct, enum, constant, trait, `impl` block...
- `lifetime`: a lifetime token e.g. `'a`,
- `literal`: a literal e.g. `'c'`, `42`, `"hello"`,
- `meta`: the contents of an attribute,
- `pat`: a pattern (in `match`, `if let`, `for`... expressions) e.g., `Message::Move{ x, y: 0 }` or `Some((a, _))`,
- `pat_param`: (part of) a pattern without `|`,
- `path`: a type path, e.g. `std::future::Future` or `std::ops::FnOnce(isize) -> isize`,
- `stmt`: a statement without the trailing semicolon (except for item statements that require semicolons) , e.g. `let x = 5;`
- `tt`: a token tree, i.e. a single token (identifier, Rust keyword, operator, or literal) or tokens in matching delimiters `()`, `[]`, or `{}`,
- `ty`: a type, e.g. `bool`, `[(i32, i32); 3]`, `impl Trait`, `dyn Trait + Send + Sync`...
- `vis`: a possibly empty visibility qualifier, e.g. `pub`, `pub(crate)`...

In the following example, `$e:expr`, `$e1:expr`, `$e2:expr` each capture an expression:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example2.rs:example}}
```

Repetitions are indicated by placing the tokens to be repeated inside `$(…)`, followed by a repetition operator ( `*`, `+`, or `?`, similar to regular expressions), optionally with a separator token between.

In the example below, `$($x:expr),*` is read as follows:

- `$()` groups a pattern.
- `$x:expr` captures an expression into metavariable `$x`.
- `,` is the literal comma that separates the expressions.
- `*` is a repetition operator, meaning "zero or more" occurrences of the preceding pattern. Other operators include `+` (one or more) and `?` (zero or one).

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example3.rs:example}}
```

In the transcriber (the code block after `=>`), metavariables are referred to simply by `$name`. Metavariables are replaced with the syntax element that matched them. The special metavariable `$crate` can be used to refer to the crate defining the macro. Repetition is possible: In the example above, the `$(...)*` syntax repeats the `temp += $x;` statement for each captured expression `$x`.

Note that macros-by-example have "mixed-site hygiene". This means that loop labels, block labels, and local variables are looked up at the macro definition site, while other symbols are looked up at the macro invocation site. Refer to the [Rust Reference - Macros][book~rust-reference~macros]⮳ for full details.

### Create a Domain-Specific Language (DSL) with Macros {#dsl-macros}

Macros can be used to create domain-specific languages (DSLs). A DSL is a specialized language tailored to a specific problem domain, making the code easier to read and write. The following example demonstrates how to create a simple configuration DSL using a declarative macro:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_dsl.rs:macro}}
```

### Write Procedural Macros {#procedural-macros}

Procedural macros are more flexible than declarative macros (`macro_rules!`). While `macro_rules!` macros work by pattern matching and substitution on tokens, procedural macros are essentially Rust functions that run _at compile time_ and receive the Rust code they are applied to as a "token stream". They then process this token stream and return a new token stream, which the compiler then uses to replace the original code.

Procedural macros are one of the more complex but powerful parts of Rust, allowing for complex code generation and manipulation of the abstract syntax tree (AST):

- Custom derive attributes (e.g., `#[derive(Serialize, Deserialize)]`) automatically implement traits for structs and enums.
- Attribute-like macros (e.g., `#[route("/users", method = "GET")]`) define custom attributes that can be applied to items (functions, structs, modules...) to modify their behavior or generate additional code.
- Function-like macros (e.g., `custom!(...)`) are similar to macros created with `macro_rules!`, but with the full power of Rust for parsing and code generation.

Note the following before writing procedural macros:

- Procedural macros must be defined in their own crate. This is because they are compiled for the host (the compiler's environment) rather than the target (where your program will run). The need for a separate crate adds a bit of organizational overhead to projects.
- The `proc-macro = true` key must be set in `Cargo.toml`:

  ```toml
  [lib]
  proc-macro = true
  ```

- Procedural macros must be defined in the root of that crate (in `lib.rs`).
- You will always use types from the `proc_macro` crate (provided by the compiler) to interact with token streams.
  - All procedural macros take one or two `proc_macro::TokenStream` as input and return a `proc_macro::TokenStream`.
  - A token stream is roughly equivalent to `Vec<TokenTree>` where a `TokenTree` can roughly be thought of as a lexical token. For example `foo` is an `Ident` token, `.` is a `Punct` token, and `1.2` is a `Literal` token [(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.proc_macro)[(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.token).
- While you could manually parse and generate token streams, it is highly recommended to use helper crates (found on `crates.io`):
  - `syn` is a parser for Rust syntax. It allows you to parse a `proc_macro::TokenStream` into an Abstract Syntax Tree (AST) that's easy to work with (e.g., `syn::ItemStruct`, `syn::FnArg`).
  - `quote` is a quasi-quoting library that makes it easy to generate Rust code from the parsed AST. It allows you to write Rust code directly and "splice in" variables.
- Procedural macros are "unhygienic." They behave as if the output token stream was simply written inline to the code it's next to. This means that it's affected by external items and also affects external imports. Use full absolute paths to items in libraries (for example, `::std::option::Option` instead of `Option`) and make sure that generated functions have names that are unlikely to clash with other functions (like `__internal_foo` instead of `foo`) [(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.hygiene).
- While `syn` and `proc_macro::Span` help, providing clear and precise error messages from a macro can be tricky.
  - Procedural macros have two ways of reporting errors. The first is to `panic!`. The second is to invoke the `compile_error!` macro.
- Debugging macros can be challenging. `cargo expand` is your best friend for seeing the generated code. IDE support for macros is limited.
- Procedural macros can increase compilation times, especially for large projects or complex macros.

### Derive Custom Traits with Procedural Macros {#derive-macros}

Derive macros define new inputs for the `derive` attribute. These macros can create new items, when applied to a struct, enum, or union.

- Create a separate crate and add the following to its `Cargo.toml`:

```toml
[lib]
proc-macro = true

# Typical dependencies for procedural macros:
[dependencies]
syn = { version = "2.0", features = ["full"] } # "full" feature is often needed for parsing various Rust constructs.
quote = "1.0"
proc-macro2 = "1.0" # TokenStream manipulation.
```

- Add the following to `lib.rs`, the root of the new crate:

```rust,editable
{{#include ../../crates/proc_macros/src/lib.rs:derive_macro}}
```

- To use the derive macro, add it to your main crate's `Cargo.toml`:

```toml
[dependencies]
proc-macros = { path = "../proc_macros" } # Adjust the path as necessary.
```

- Then, in your main crate, use the macro like this:

```rust,editable
{{#include ../../crates/language/examples/macros/proc_macro_derive.rs:example}}
```

Derive macros can also add additional attributes ("derive macro helper attributes") into the scope of the item they are on [(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.derive.attributes). This is useful for adding metadata or additional functionality to the item being derived.

```rust,editable
# [derive(MyCustomDeriveMacro)]
struct MyStruct {
    // `helper` attributes can be added here:
    #[helper] field: ()
}
```

### Create Custom Attributes with Procedural Macros {#attribute-macros}

Attribute procedural macros define new outer attributes.

- Create a separate crate and add it to your main crate's `Cargo.toml`, as above.

- Add the following to `lib.rs`, the root of the new crate:

```rust,editable
{{#include ../../crates/proc_macros/src/lib.rs:attribute_macro}}
```

- Then, in your main crate, use the macro like this:

```rust,editable
{{#include ../../crates/language/examples/macros/proc_macro_attribute.rs:example}}
```

### Create Function-like Procedural Macros {#function-like-procedural-macros}

- Create a separate crate and add it to your main crate's `Cargo.toml`, as above.

- Add the following to `lib.rs`, the root of the new crate:

```rust,editable
{{#include ../../crates/proc_macros/src/lib.rs:function_macro}}
```

- Then, in your main crate, use the macro like this:

```rust,editable
{{#include ../../crates/language/examples/macros/proc_macro_function.rs:example}}
```

### Avoid Common Pitfalls with Macros {#macro-pitfalls}

Here are some common pitfalls to watch out for:

- Overusing Macros: Macros can obscure logic and make code harder to read or debug, if used unnecessarily.
- Poor Macro Hygiene: If your macro introduces variables without care, it can clash with names in the surrounding scope. This can lead to confusing bugs. Rust tries to help with hygiene, but it's not foolproof - especially in procedural macros.
- Complex or Unreadable Macros: Macros that try to do too much can become cryptic. If you can't understand what your macro expands to without a mental workout, it might be time to simplify or rethink the design.
- Lack of Testing: Macros don't get tested like functions do. If you don't write tests that exercise all the macro's patterns and edge cases, you might miss subtle bugs.
- Unexpected Expansion Behavior: Macros expand before type checking, so they can produce code that compiles incorrectly or fails in surprising ways. Debugging macro expansion can be tricky.
- Lifetime and Type Issues: Especially in procedural macros, forgetting to handle lifetimes or generic parameters properly can lead to compiler errors that are hard to trace.
- Unclear Error Messages: When something goes wrong inside a macro, the compiler often points to the macro invocation site, not the actual problem. Using the `compile_error!` macro can help provide clearer diagnostics.

## References {#skip}

- [Rust Reference - Macros][book~rust-reference~macros]⮳.
- [Rust by Example - Macros][book~rust-by-example~macros]⮳.
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
[final review](https://github.com/john-cd/rust_howto/issues/1382)
</div>
