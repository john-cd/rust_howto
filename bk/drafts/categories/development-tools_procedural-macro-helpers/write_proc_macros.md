# Write procedural macros

{{#include write_proc_macros.incl.md}}

Procedural macro [crates][p-crates] almost always will link to the compiler-provided [`proc_macro`][c-proc_macro]⮳{{hi:proc_macro}} crate. The proc_macro crate provides types required for writing procedural macros and facilities to make it easier.

This crate primarily contains a TokenStream type. Procedural [macros][p-macros] operate over token [streams][p-streams] instead of AST nodes, which is a far more stable interface over time for both the compiler and for procedural [macros][p-macros] to target. A token stream is roughly equivalent to Vec<TokenTree> where a TokenTree can roughly be thought of as lexical token. For example foo is an Ident token, . is a Punct token, and 1.2 is a Literal token. The TokenStream type, unlike Vec<TokenTree>, is cheap to clone [(reference)]( https://doc.rust-lang.org/reference/procedural-macros.html#r-macro.proc.proc_macro.token-stream ).

## Parse Rust source code into an abstract syntax tree {#syn}

[![syn][c-syn-badge]][c-syn] [![syn-crates.io][c-syn-crates.io-badge]][c-syn-crates.io] [![syn-github][c-syn-github-badge]][c-syn-github] [![syn-lib.rs][c-syn-lib.rs-badge]][c-syn-lib.rs]{{hi:syn}}{{hi:Macros}}{{hi:syn}}[![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}[![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}

The [`syn`][c-syn]{{hi:syn}}⮳ crate in Rust is a fundamental library for parsing a stream of Rust tokens into an equivalent syntax tree. It's primarily used when working with procedural macros, but it can also be helpful for other code analysis or manipulation tasks, such as:

- Static analysis: Building tools to check code for specific patterns or enforce coding standards.
- Code generation: Creating tools that automatically generate Rust code based on some input or configuration.
- Refactoring: Developing tools to automate code refactoring tasks.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/tests/syn.rs:example}}
```

## `paste` {#paste}

[![paste][c-paste-badge]][c-paste] [![paste-crates.io][c-paste-crates.io-badge]][c-paste-crates.io] [![paste-github][c-paste-github-badge]][c-paste-github] [![paste-lib.rs][c-paste-lib.rs-badge]][c-paste-lib.rs]{{hi:paste}}{{hi:Macros}}[![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`paste`][c-paste]⮳ provides a flexible way to paste together identifiers in a macro, including using pasted identifiers to define new items.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/tests/paste.rs:example}}
```

## `quote` {#quote}

[![quote][c-quote-badge]][c-quote] [![quote-crates.io][c-quote-crates.io-badge]][c-quote-crates.io] [![quote-github][c-quote-github-badge]][c-quote-github] [![quote-lib.rs][c-quote-lib.rs-badge]][c-quote-lib.rs]{{hi:quote}}{{hi:Macros}}{{hi:Syn}}[![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

[`quote`][c-quote]⮳ provides the `quote!` macro for turning Rust syntax tree{{hi:Syntax tree}} data structures into tokens{{hi:Tokens}} of source code.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/tests/quote.rs:example}}
```

## `proc-macro2` {#proc-macro2}

[![proc-macro2][c-proc_macro2-badge]][c-proc_macro2] [![proc-macro2-crates.io][c-proc_macro2-crates.io-badge]][c-proc_macro2-crates.io] [![proc-macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github] [![proc-macro2-lib.rs][c-proc_macro2-lib.rs-badge]][c-proc_macro2-lib.rs]{{hi:proc-macro2}}{{hi:Macros}}{{hi:Syn}}[![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

[`proc_macro2`][c-proc_macro2]{{hi:proc-macro2}}⮳ [![proc_macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github] bring proc-macro-like functionality to other contexts like build.rs and main.rs and makes procedural macros unit testable.

A substitute implementation of the compiler's [`proc_macro`][c-proc_macro]⮳{{hi:proc_macro}} API to decouple token-based libraries from the procedural macro use case.

A wrapper around the procedural macro API of the compiler's proc_macro crate. This library serves two purposes:

Bring proc-macro-like functionality to other contexts like build.rs and main.rs. Types from proc_macro are entirely specific to procedural [macros][p-macros] and cannot ever exist in code outside of a procedural macro. Meanwhile `proc_macro2` types may exist anywhere including non-macro code. By developing foundational libraries like `syn` and `quote` against `proc_macro2` rather than proc_macro, the procedural macro ecosystem becomes easily applicable to many other use cases and we avoid re-implementing non-macro equivalents of those libraries.

Make procedural [macros][p-macros] unit testable. As a consequence of being specific to procedural [macros][p-macros], nothing that uses proc_macro can be executed from a unit test. In order for helper libraries or components of a macro to be testable in isolation, they must be implemented using proc_macro2.

[![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

[![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/tests/proc_macro2.rs:example}}
```

### `darling` {#darling}

[![darling][c-darling-badge]][c-darling] [![darling-crates.io][c-darling-crates.io-badge]][c-darling-crates.io] [![darling-github][c-darling-github-badge]][c-darling-github] [![darling-lib.rs][c-darling-lib.rs-badge]][c-darling-lib.rs]{{hi:darling}}

[`darling`][c-darling]⮳{{hi:darling}} provides derive macros to easily parse derive macro inputs.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/tests/darling.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write_proc_macros: write (P2)](https://github.com/john-cd/rust_howto/issues/331)

## Report errors from within a procedural macro

Procedural [macros][p-macros] have two ways of reporting errors. The first is to [`panic`][c-panic]⮳{{hi:panic}}. The second is to emit a `compile_error` macro invocation.

</div>
