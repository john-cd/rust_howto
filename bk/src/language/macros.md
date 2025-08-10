# Macros

{{#include macros.incl.md}}

Rust macros enable metaprogramming, allowing you to write code that generates other code. This capability allows for code reuse by reducing boilerplate:

- Macros are commonly used as variadic interfaces: They can handle a variable number of arguments and therefore are more flexible than functions. `println!`, for example, can take any number of arguments.
- Macros can implement common traits automatically or create domain-specific languages.
- Macros can achieve performance optimizations through compile-time computation and code transformation.

Unlike C or C++ macros, which rely on simple text substitution via the preprocessor and can introduce subtle bugs, Rust macros are part of the language's syntax system and are expanded during _compilation_, so any errors in the generated code are caught early.

## Use Macros {#use-macros}

Macros comes in three forms:

- Function-like macros, e.g. `println!("Hello, world!");`,
- Attribute-like macros e.g. `#[foo = "bar"]`,
- Custom [derive macros](https://doc.rust-lang.org/reference/attributes/derive.html)↗ e.g., `#[derive(MyTrait)]`.

### Use Function-like Macros {#function-like-macros}

"Function-like" macros are called most frequently, well, like a [[function | function]] would be, with a list of parameters and within a parent function's body:

```rust,editable
fn main() {
    let name = "Alice";
    let age = 30;
    println!("My name is {name} and my age is {}", age);
}
```

- Note the `!` suffix after the macro's name.
- Square or curly brackets (`[]` or `{}`)  may be used as outer delimiters instead of parentheses e.g., `println!{ "Curly" };`. This is most often used with `vec!` to give it an array-like syntax: `let _ = vec![1, 2, 3];`.
- When a function-like macro is used as an item or a statement (see below) and is not using curly braces, a semicolon is required at the end.

This said, function-like macros are more general and powerful than function calls:

- Macros may accept a variable number of arguments.
- The macro arguments are not restricted to be a comma-separated list of expressions. In fact, it _does not need to be valid Rust_, which can be exploited to create a _domain specific language_ within Rust code (see section below).
- What a macro generates is not limited to an expression or statement.
- Some macros may be called in places where a function could not, for example outside of a function's body.

The following example demonstrates where function-like macros can be used in Rust. They may appear in expressions or statements; in a pattern, type alias, or declaration; or as an associated item. It is also possible to call macros within a macro definition. Macros, however, may not appear in identifiers, match arms, or struct fields:

```rust,editable
{{#include ../../crates/language/examples/macros/macros.rs:example}}
```

### Apply Attribute-like Macros {#attribute-like-macros}

[[attributes | Attribute]]-like macros modify the behavior of items (e.g., functions or modules) they are applied to:

```rust,noplayground
// If `log_calls` is a custom attribute macro, apply it as follows:
#[log_calls]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Read the [procedural macros](#procedural-macros) sections below for more details.

### Use Custom Derive Macros {#custom-derive-macros}

Custom [[derive]] macros automatically generate [[trait]] implementations for structs and enums.

```rust,noplayground
// Automatically implement `MyCustomTrait` for the `User` struct:
#[derive(MyCustomTrait)]
struct User {
    id: u64,
    name: String,
}
```

Read the [procedural macros](#procedural-macros) sections below for more details.

## Common Macros from the Standard Library {#common-macros}

### Print Messages to the Console with `println!` and friends {#print-macros}

[`println!`](https://doc.rust-lang.org/std/macro.println.html)↗, [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html)↗, [`print!`](https://doc.rust-lang.org/std/macro.print.html)↗, and [`eprint!`](https://doc.rust-lang.org/std/macro.eprint.html)↗ handle formatted output to standard output (`println!`, `print!`) or standard error (`eprintln!`, `eprint!`). They work similarly to `C`'s `printf` and support various formatting specifiers. `println!` and `eprintln!` automatically add a newline at the end.

[`format!`](https://doc.rust-lang.org/std/macro.format.html)↗ is similar to `print!`, but instead of printing to the console, it constructs and returns a `String` with the formatted content.

[`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html)↗ is a debugging macro. It prints the value of an expression to standard error along with its file, line number, and the expression itself, and then returns the expression's value. This is useful for quickly inspecting values during development.

### Concatenate String Literals with `concat!` {#concat-macro}

[`concat!`](https://doc.rust-lang.org/std/macro.concat.html)↗ concatenates literals into a static string slice.

See [[string_concat | String Concatenation]] for more details.

### Create a Vector with `vec!` {#vec-macro}

[`vec!`](https://doc.rust-lang.org/std/macro.vec.html)↗ allows for convenient creation and initialization of vectors (`Vec<T>`). For example, `vec![1, 2, 3]` creates a vector with integers `1`, `2`, and `3`.

See [[vector | Vector]] for more details.

### Immediately Terminate the Current Thread with `panic!` {#panic-macro}

[`panic!`](https://doc.rust-lang.org/std/macro.panic.html)↗ immediately terminates the current thread. It is used for unrecoverable errors. You can provide a custom message that will be displayed when the panic occurs.

### Assert Conditions in your Tests with `assert!`, `assert_eq!`, and `assert_ne!` {#assert-macros}

[`assert!`](https://doc.rust-lang.org/std/macro.assert.html)↗, [`assert_eq!`](https://doc.rust-lang.org/std/macro.assert_eq.html)↗, [`assert_ne!`](https://doc.rust-lang.org/std/macro.assert_ne.html)↗ are used for assertions, primarily in testing.

- `assert!(condition)` panics if the condition is false.
- `assert_eq!(left, right)` panics if `left` is not equal to `right`, providing a helpful message showing both values.
- `assert_ne!(left, right)` panics if `left` is equal to `right`.

There are also [`debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html)↗, [`debug_assert_eq`](https://doc.rust-lang.org/std/macro.debug_assert_eq.html)↗, and [`debug_assert_ne`](https://doc.rust-lang.org/std/macro.debug_assert_ne.html)↗ macros that are only enabled in non-optimized builds (i.e. disabled in release builds) by default.

### Mark Sections of your Code as Unreachable or Unimplemented {#unreachable-macros}

[`unreachable!`](https://doc.rust-lang.org/std/macro.unreachable.html)↗ is used to mark code paths that should logically never be reached. If the code reaches an `unreachable!` macro, it will cause a panic, indicating a logical error in the program.

[`todo!`](https://doc.rust-lang.org/std/macro.todo.html)↗ is a placeholder for not-yet-implemented code. If executed, it will cause a panic with a message indicating that the functionality is "not yet implemented." It's useful during development to mark areas that still need attention. You may also use the similar [`unimplemented!`](https://doc.rust-lang.org/std/macro.unimplemented.html)↗.

### Conditionally Compile Code with `cfg!` {#cfg-macro}

[`cfg!`](https://doc.rust-lang.org/std/macro.cfg.html)↗ evaluates boolean combinations of configuration flags at compile-time. It is often used with or instead of the [`#[cfg]`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)↗ attribute. Both use the same [syntax](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)↗.

## Write Macros {#write-macros}

There are two main types of macros in Rust:

- _Declarative Macros_ (a.k.a. "macros-by-example") are the most common. Macros-by-example are defined as a set of pattern-matching rules and are always called similarly to a function (they are "function-like"): `a_declarative_macro!(...)`. Examples in the Standard Library include `println!(...)`, `vec![...]`, and `format!(...)`.

Macros-by-example are expanded at compile-time, meaning they incur no runtime performance cost, and their output is checked syntactically and type-checked.

- _Procedural Macros_ are more advanced and allow you to operate on one or more [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)↗ (representing the source code). They are compiled before the main program and can perform more complex code generation. They come in three flavors:
  - Custom [derive macros](https://doc.rust-lang.org/reference/attributes/derive.html)↗ (e.g., `#[derive(MyTrait)]`) automatically generate trait implementations for structs and enums.
  - Attribute-like macros (outer attribute `#[route(GET, "/")]`, where `route` is a custom name; or inner attribute `#![foo="bar"]`) modify the behavior of items (e.g., functions or modules) they are applied to.
  - Function-like procedural macros (`my_proc_macro!(...)`) have the same call syntax as macros-by-example.

Read the sections below; the [macro](https://doc.rust-lang.org/book/ch20-05-macros.html)↗ chapter of the Rust book; and the ["Little Book of Rust Macros"](https://lukaswirth.dev/tlborm)↗ for more details.

### Write Macros-by-Example {#macros-by-example}

[![Rust by example - macros][book~rust-by-example~macros~badge]][book~rust-by-example~macros]{{hi:Macros}}{{hi:Metaprogramming}}

Macros-by-example allow you to define reusable code snippets. They are excellent for simple syntactic transformations and reducing boilerplate based on structural patterns.

Macros-by-example are defined using the [`macro_rules!`](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.intro)↗ syntax and work by pattern matching, where you provide patterns and corresponding code templates. They are expanded into source code that gets compiled with the rest of the program.

The `macro_rules!` syntax, which resembles a `match` expression, consists of a name for the macro (e.g., `say_hello`), followed by a set of rules enclosed in curly braces and separated by semicolons: `{ <rule1>; <rule2> }`. Each rule has a _matcher_, a pattern within `()` describing the syntax that it matches, followed by `=>` and a _transcriber_, code within `{}` that will be substituted when the macro is invoked: `( matcher ) => { transcriber };`.

The transcriber can be an expression, a pattern, a type, zero or more items, or zero or more statements.

For example, the following defines a basic `say_hello` macro (with one rule without arguments), then calls it:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_basic.rs:example}}
```

Note: To be more precise, `macro_rules!` interchangeably accepts `{}`, `()` and `[]` as outer delimiters for rule groups, matchers, and transcribers, but what is described above is conventional.

#### Capture the Arguments of a Macro-by-Example with Metavariables {#metavariables}

You can use _metavariables_ to capture parts of a macro's input. They are prefixed with a dollar sign (`$`). Inside matchers, `$name:fragment-specifier` matches a Rust syntax fragment of the kind specified and binds it to the metavariable `$name`, which can then be used in the transcriber. Valid [syntax fragment specifiers](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.meta.specifier)↗, also called designators, are:

- `block`: a block expression e.g. `{ let x = 1; x + 2 }`,
- `expr`: an expression that produces a value, e.g. `a + 2` or `some_function()` or even `"a literal"`,
- `ident`: a variable or function name, e.g. `foo` or `_identifier` or even a raw identifier like `r#true`; or a keyword.
- `item`: an item definition, i.e. a function, module, `use` declaration, `type` alias, `struct`, `enum`, `const`, `trait`, `impl` block...
- `lifetime`: a lifetime token e.g. `'a`,
- `literal`: a literal e.g. `'c'`, `42`, `1.2`, `b'bin'`, `"hello"`, `true`,
- `meta`: the contents of an attribute (use the `#[$meta:meta]` or `#![$meta:meta]` patterns to match an attribute),
- `pat`: a pattern (in `match`, `if let`, `for`... expressions) e.g., `Message::Move{ x, y: 0 }`, `Some((a, _))`, `0..3` or `0 | 42`,
- `pat_param`: (part of) a pattern without `|`,
- `path`: a type path, e.g. `std::future::Future` or `::std::ops::FnOnce(isize) -> isize`,
- `stmt`: a statement without the trailing semicolon (except for item statements that require semicolons) , e.g. `let x = 5;`,
- `tt`: a [token tree](https://lukaswirth.dev/tlborm/syntax-extensions/source-analysis.html#token-trees)↗, i.e. a single token (identifier, Rust keyword, operator, or literal) or multiple tokens within matching delimiters `()`, `[]`, or `{}`,
- `ty`: a type, e.g. `bool`, `[(i32, i32); 3]`, `impl Trait`, `dyn Trait + Send + Sync`...
- `vis`: a possibly empty visibility qualifier, e.g. `pub`, `pub(crate)`...

In the transcriber (code within `{}` after `=>`), metavariables are referred to simply by `$name`. Metavariables are replaced with the syntax element that matched them.

The special metavariable `$crate` can be used to refer to the crate defining the macro.

In the following example, `$e:expr`, `$e1:expr`, `$e2:expr` each capture a Rust expression, which must be complete and valid:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_metavariables.rs:example}}
```

More complex [metavariable expressions](https://lukaswirth.dev/tlborm/decl-macros/macros-methodical.html#metavariable-expressions)↗ support is available in nightly Rust.

#### Use Repetitions in Macros-by-Example {#repetitions}

In a macro _matcher_, a pattern within `()` describing the syntax that it matches, repetitions are indicated by placing the tokens to be repeated inside `$( )`, followed by a repetition operator ( `*`, `+`, or `?`, similar to regular expressions), optionally with a separator token in-between.

In the example below, `$($x:expr),*` is read as follows:

- `$()` groups a pattern.
- `$x:expr` captures an expression into metavariable `$x`, as described above.
- `,` is the literal comma that separates the expressions.
- `*` is a repetition operator, meaning "zero or more" occurrences of the preceding pattern. Other operators include `+` (one or more) and `?` (zero or one).

Repetition in the transcriber is also possible: the `$(...)*` syntax repeats the `temp += $x;` statement for each captured expression `$x`:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_repetitions.rs:example}}
```

#### Understand the Hygiene of Macros-by-Example {#hygiene}

Macros-by-example have "mixed-site hygiene". This means that loop labels, block labels, and local variables are looked up at the _macro definition site_, while other symbols are looked up at the _macro invocation site_. Refer to the [Rust Reference - Macros][book~rust-reference~macros]↗ for full details.

The following example demonstrates how to use a local variable; and how to define an item in a macro and use it at the invocation site:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_hygiene.rs:example}}
```

####  Macros-by-Example {#macros-by-example-limitations}

Macros operate on syntax, not types. As a result, you can't specify the type of an argument in the matcher (the pattern before `=>`) of a rule of a macro-by-example.

While there is no type checking during macro expansion, there is _after_. Your code won't compile if there is a type error:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_type_checking.rs:example}}
```

You may therefore use compile-time type checking to ensure that a macro argument is of a specific type:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_type_checking2.rs:example}}
```

### Create a Domain-Specific Language (DSL) with Macros {#dsl-macros}

Macros can be used to create domain-specific languages (DSLs) within Rust code. A DSL is a specialized language tailored to a specific problem domain, making the code easier to write - it may be JSON-like; `printf`-like; SQL-like; or whatever fits your needs.

Indeed, what is passed to a macro does not need to be valid Rust. Macros-by-example, for example, can accept arbitrary sequences of tokens by matching multiple [token trees](https://lukaswirth.dev/tlborm/syntax-extensions/source-analysis.html#token-trees)↗:

```rust,editable
{{#include ../../crates/language/examples/macros/token_tree.rs:example}}
```

The following example demonstrates how to create a simple configuration DSL:

```rust,editable
{{#include ../../crates/language/examples/macros/macro_by_example_dsl.rs:example}}
```

The caveat of DSLs, of course, is that an unfamiliar syntax embedded within Rust code may be confusing to the reader.

To parse Rust-like syntax, refer to the [relevant section](https://lukaswirth.dev/tlborm/decl-macros/building-blocks/parsing.html)↗ of the little book of macros or use the [`syn`](https://docs.rs/syn/latest/syn)↗ crate.

### Write Procedural Macros {#procedural-macros}

Procedural macros are more flexible than declarative macros (`macro_rules!`). While `macro_rules!` macros work by pattern matching and substitution on tokens, procedural macros are essentially Rust functions that run _at compile time_ and receive the Rust code they are applied to as "token streams". They return a new token stream, which the compiler then uses to replace the original code.

Procedural macros are one of the more complex but powerful parts of Rust, allowing for code generation and manipulation of the abstract syntax tree (AST):

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
- You will always use types from the [`proc_macro`](https://doc.rust-lang.org/proc_macro/index.html)↗ crate (provided by the compiler) to interact with token streams [(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.proc_macro)↗.
  - All procedural macros take one or two [`proc_macro::TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)↗ as input and return a `proc_macro::TokenStream`.
  - A token stream is roughly equivalent to `Vec<TokenTree>` where a [`TokenTree`](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html)↗ can roughly be thought of as a lexical token or a group of tokens within `()`, `[]`, or `{}` delimiters. For example `foo` is an `Ident` token, `.` is a `Punct` token, and `1.2` is a `Literal` token [(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.token)↗.

While you could manually parse and generate token streams, it is highly recommended to use helper crates (found on [`crates.io`](https://crates.io)↗):

- [`syn`](https://crates.io/crates/syn)↗ is a parser for Rust syntax. It allows you to parse a `proc_macro::TokenStream` into an Abstract Syntax Tree (AST) that's easy to work with (e.g., [`syn::ItemStruct`](https://docs.rs/syn/2.0.104/syn/struct.ItemStruct.html)↗, [`syn::FnArg`](https://docs.rs/syn/latest/syn/enum.FnArg.html)↗).
- `quote` is a quasi-quoting library that makes it easy to generate Rust code from the parsed AST. It allows you to write Rust code directly and "splice in" variables.

Procedural macros are "unhygienic." They behave as if the output token stream was simply written inline to the code it's next to. This means that they are affected by external items and also affects external imports. Use full absolute paths to items in libraries (for example, `::std::option::Option` instead of `Option`) and make sure that generated functions have names that are unlikely to clash with other functions (like `__internal_foo` instead of `foo`) [(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.hygiene)↗.

Procedural macros can increase compilation times, especially for large projects or complex macros.

#### Derive Custom Traits with Procedural Macros {#derive-macros}

[Derive macros](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.derive)↗ define new inputs for the `derive` attribute. These macros can create new items, when applied to a struct, enum, or union.

- Create a separate crate and add the following to its `Cargo.toml`:

```toml
[lib]
proc-macro = true

# Typical dependencies for procedural macros:
[dependencies]
syn = { version = "2.0", features = ["full"] } # The "full" feature is often needed for parsing various Rust constructs.
quote = "1.0"
proc-macro2 = "1.0" # TokenStream manipulation.
```

- Add the following to `lib.rs`, the root of the new crate:

```rust,editable
{{#include ../../crates/proc_macros/src/lib.rs:derive_macro}}
```

- To use the `derive` macro, add it to your main crate's `Cargo.toml`:

```toml
[dependencies]
proc-macros = { path = "../proc_macros" } # Adjust the path as necessary.
```

- Then, in your main crate, use the macro like this:

```rust,editable
{{#include ../../crates/language/examples/macros/proc_macro_derive.rs:example}}
```

Note that `derive` macros can add additional attributes ("derive macro helper attributes") into the scope of the item they are on [(Rust reference)](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.derive.attributes)↗. This is useful for adding metadata or additional functionality to the item being derived:

```rust,editable
# [derive(MyCustomDeriveMacro)]
struct MyStruct {
    // `helper` attributes can be added here.
    // They are inert.
    #[helper] field: ()
}
```

#### Create Custom Attributes with Procedural Macros {#attribute-macros}

[Attribute procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.attribute)↗ define new outer attributes.

- Create a separate crate and add it to your main crate's `Cargo.toml`, as above.
- Add the following to `lib.rs`, the root of the new crate:

```rust,editable
{{#include ../../crates/proc_macros/src/lib.rs:attribute_macro}}
```

- Then, in your main crate, use the macro like this:

```rust,editable
{{#include ../../crates/language/examples/macros/proc_macro_attribute.rs:example}}
```

#### Create Function-like Procedural Macros {#function-like-procedural-macros}

[Function-like procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.attribute)↗ are invoked using the macro invocation operator (!).

- Create a separate crate and add it to your main crate's `Cargo.toml`, as above.
- Add the following to `lib.rs`, the root of the new crate:

```rust,editable
{{#include ../../crates/proc_macros/src/lib.rs:function_macro}}
```

Then, in your main crate, use the macro like this:

```rust,editable
{{#include ../../crates/language/examples/macros/proc_macro_function.rs:example}}
```

### Avoid Common Pitfalls with Macros {#macro-pitfalls}

Watch out for the following common macro pitfalls:

- Overusing Macros: Macros can obscure logic and make code harder to read or debug, if used unnecessarily.
- Poor Macro Hygiene: If your (procedural) macro introduces variables without care, it can clash with names in the surrounding scope. This can lead to confusing bugs.
- Lack of Testing: If you don't write tests that exercise all of the macro's patterns and edge cases, you might miss subtle bugs.
- Unexpected Expansion Behavior: Macros expand before type checking, so they can produce code that compiles incorrectly or fails in surprising ways.
- Lifetime and Type Issues: Especially in procedural macros, forgetting to handle lifetimes or generic parameters properly can lead to compiler errors that are hard to trace.
- Unclear Error Messages.

### Report Errors From and Debug Macros {#debug-macros}

Macros have two ways of reporting errors. The first is to `panic!`. The second is to invoke the `compile_error!` macro.

When something goes wrong inside a macro, the compiler often points to the macro invocation site, not the actual problem. Using the [`compile_error!`](https://doc.rust-lang.org/core/macro.compile_error.html)↗ macro provides clearer diagnostics:

```rust,editable
macro_rules! must_be_an_identifier {
    ($x:ident) => {
        // Your code goes here.
    };
    ($_:tt) => {
        // Causes compilation to fail with the given error message.
        // It is the compiler-level form of `panic!`,
        // but emits an error during compilation rather than at runtime.
        compile_error!("This macro only accepts an identifier.");
    };
}

fn main() {

    must_be_an_identifier!(foo);
    // must_be_an_identifier!(42);
}
```

Debugging macros can be challenging. To see what a macro expands to and debug it, use the [`cargo-expand`](https://crates.io/crates/cargo-expand)↗ cargo plugin. See also the [debugging](https://lukaswirth.dev/tlborm/decl-macros/minutiae/debugging.html)↗ section of the little book of macros for more suggestions.

## References {#references}

- [Rust Reference - Macros][book~rust-reference~macros]↗.
- [Rust by Example - Macros][book~rust-by-example~macros]↗.
- The [Little Book of Rust Macros][book~rust-macros]↗.
- [Macros in Rust: A tutorial with examples](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples)↗.

## Related Topics {#related-topics}

- [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]].
  - [[compile_macros | Compile Macros]].
  - [[macro_tools | Macro Tools]].
  - [[write_proc_macros | Write Proc Macros]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
