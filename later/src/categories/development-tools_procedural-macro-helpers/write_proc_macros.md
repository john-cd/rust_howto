<!-- TODO Reviewed ??-??-2026 -->

# Write Procedural Macros

{{#include write_proc_macros.incl.md}}

A procedural macro's input and output are token streams. A useful implementation flow is `proc_macro` for the compiler entry point, `proc-macro2` for testable token handling, `syn` for parsing, `quote` for generating output, and optionally `darling` for derive-attribute parsing. Handle errors, tests, and documentation as part of the same authoring workflow.

## `proc_macro` {#proc-macro}

A procedural macro crate uses the compiler-provided [`proc_macro`][c~proc_macro~docs] API for its public entry points. Function-like macros, derives, and attribute macros receive a `TokenStream` and return generated tokens. Keep the entry point small and delegate parsing and generation to ordinary functions.

## `proc-macro2` {#proc-macro2}

[![proc-macro2][c~proc-macro2~docs~badge]][c~proc-macro2~docs] [![proc-macro2~crates.io][c~proc-macro2~crates.io~badge]][c~proc-macro2~crates.io] [![proc-macro2~repo][c~proc-macro2~repo~badge]][c~proc-macro2~repo] [![proc-macro2~lib.rs][c~proc-macro2~lib.rs~badge]][c~proc-macro2~lib.rs]

Use [`proc-macro2`][c~proc-macro2~docs] types in reusable parsing and generation code. They work in ordinary libraries and tests as well as proc-macro crates, so most behavior can be tested without invoking the compiler's macro interface.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs:example}}
```

## Parse Input with `syn` {#syn}

{{#badges syn development-tools::procedural-macro-helpers="Procedural macro helpers" parser-implementations="Parser implementations"}}{{hi:Macros}}

[`syn`][c~syn~docs] parses Rust tokens into syntax-tree types such as `DeriveInput`, `Item`, and `Expr`. Parse only the syntax your macro needs, then validate its shape before generating code.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs:example}}
```

See the [`syn` examples][syn-examples~repo] for additional parsing and visiting patterns.

A `#[proc_macro_derive]` entry point must live in a crate with `proc-macro = true`. Keep the reusable helper on `proc_macro2::TokenStream`; the compiler-facing wrapper can be thin:

```rust,ignore
#[proc_macro_derive(Describe)]
pub fn describe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_describe(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
```

## Generate Output with `quote` {#quote}

[![quote][c~quote~docs~badge]][c~quote~docs] [![quote~crates.io][c~quote~crates.io~badge]][c~quote~crates.io] [![quote~repo][c~quote~repo~badge]][c~quote~repo] [![quote~lib.rs][c~quote~lib.rs~badge]][c~quote~lib.rs]

[`quote`][c~quote~docs] builds `proc_macro2::TokenStream` values from Rust-like syntax. Use interpolation for parsed identifiers and types instead of assembling generated source as strings.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs:example}}
```

## Parse Derive Attributes with `darling` {#darling}

[![darling][c~darling~docs~badge]][c~darling~docs] [![darling~crates.io][c~darling~crates.io~badge]][c~darling~crates.io] [![darling~repo][c~darling~repo~badge]][c~darling~repo] [![darling~lib.rs][c~darling~lib.rs~badge]][c~darling~lib.rs]

[`darling`][c~darling~docs] derives parsers for common derive-macro inputs and attributes. Use it when its input model fits the macro; direct `syn` parsing remains available for custom syntax.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs:example}}
```

## Report Errors from a Procedural Macro {#report-errors-from-within-a-procedural-macro}

Return compiler diagnostics for invalid input instead of panicking. For example, convert a spanned `syn::Error` into the macro's output:

```rust
return syn::Error::new_spanned(input, "unsupported attribute")
    .to_compile_error()
    .into();
```

When validation finds more than one problem, combine the errors where possible so users can fix several inputs in one compile.

## Test and Debug Macros {#testing-and-debugging}

Unit-test parsing and generation helpers that use `proc-macro2`. Add compile-pass and compile-fail cases for the public macro interface, and keep expected diagnostics under review. See the [macro development tools][p~macro-tools] chapter for expansion inspection, compile-fail testing, snapshots, and Rust Analyzer.

The [procedural macro workshop][proc-macro-workshop~repo] is a hands-on learning resource.

## Document the Macro Contract {#documentation}

Document each macro's accepted syntax, generated behavior, attributes, limitations, and diagnostics. Include compile-tested examples for common and invalid inputs; users need to know what code the macro expects and what it produces.

## Related Topics {#related-topics .skip}

- [[macros | Macros]].
- [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]].
  - [[compile_macros | Compile Macros]].
  - [[macro_tools | Macro Tools]].
- [[rust-patterns | Rust Patterns]].

### `paste` {#paste}

[`paste`][c~paste~docs] helps concatenate identifiers in `macro_rules!` definitions; it is an adjacent tool rather than a required step in procedural-macro authoring.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/paste.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
