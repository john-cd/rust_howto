# Naming Conventions

{{#include naming_conventions.incl.md}}

Rust prefers snake case for variables and functions, so a function or method would be called `read_str` instead of `readStr`. For structs, traits and enums, camel case is used, for example `HttpClient`.

## Name your Functions, Types, etc. (Naming Conventions) {#naming-conventions}

Rust follows specific naming conventions to maintain consistency and readability in code.{{hi:Naming conventions}} Here are some key guidelines:

- Crates: Use lowercase with underscores (`snake_case`), avoiding `-rs` or `-rust` suffixes. Prefer a single word.{{hi:snake_case}}
- Modules: Use `snake_case` for module names.
- Types & Traits: Use `UpperCamelCase` (e.g., `MyStruct`, `MyTrait`).{{hi:UpperCamelCase}}
- Enum Variants: Use `UpperCamelCase` (e.g., `MyEnum::VariantName`).
- Functions & Methods: Use `snake_case` (e.g., `my_function()`).
  - General constructors: Use `new` or `with_more_details`.
  - Conversion constructors: Use `from_some_other_type`.
- Local Variables: Use `snake_case` (e.g., `my_variable`).
- Macros: Use `snake_case!`.
- Constants & Statics: Use `SCREAMING_SNAKE_CASE` (e.g., `MY_CONSTANT`).{{hi:SCREAMING_SNAKE_CASE}}
- Type Parameters: Use concise `UpperCamelCase`, usually a single uppercase letter (`T`, `U`, etc.). Use more descriptive names like `ItemType` only when clarity is needed.
- Lifetimes: Use short `lowercase` names, usually a single letter (`'a`, `'b`, `'de`, `'src`).

In `UpperCamelCase`, acronyms and contractions of compound words count as one word: use `Uuid` rather than `UUID`, `Usize` rather than `USize` or `Stdin` rather than `StdIn`. In `snake_case`, acronyms are lower-cased: `is_xid_start`. In `UpperCamelCase` names multiple numbers can be separated by a `_` for clarity: `Windows10_1709` instead of `Windows101709`.

In `snake_case` or `SCREAMING_SNAKE_CASE`, a "word" should never consist of a single letter unless it is the last "word". So, we have `btree_map` rather than `b_tree_map`, but `PI_2` rather than `PI2`.

## Decide Between `as_*` vs `to_*` vs `into_*` for Conversion Methods {#conversion-conventions}

Conversions should be provided as methods, with names prefixed as follows:

| Prefix | Cost | Ownership |
|---|---|---|
| `as_` | Cheap | Borrowed -> borrowed. Use if it returns another "view" of the data. |
| `to_` | Expensive | Borrowed -> borrowed; borrowed -> owned (non-`Copy` types); owned -> owned (`Copy` types) |
| `into_` | Variable | Owned -> owned (non-Copy types). `into_*` consumes its input. |

Examples:

- `str::as_bytes()` gives a view of a `str` as a slice of UTF-8 bytes, which is free. The input is a borrowed `&str` and the output is a borrowed `&[u8]`.
- `Path::to_str` performs an expensive UTF-8 check on the bytes of an operating system path. The input and output are both borrowed. It would not be correct to call this `as_str` because this method has nontrivial cost at runtime.
- `str::to_lowercase()` produces the Unicode-correct lowercase equivalent of a `str`, which involves iterating through characters of the string and may require memory allocation. The input is a borrowed `&str` and the output is an owned `String`.

When a type wraps a single value to associate it with higher-level semantics, access to the wrapped value should be provided by an `into_inner()` method. This applies to wrappers that provide buffering like `BufReader`, encoding or decoding like `GzDecoder`, atomic access like `AtomicBool`, or any similar semantics.

## Name Getters {#getter-conventions}

With a few exceptions, the `get_` prefix is not used for getters in Rust code.{{hi:Getters}} Follow the field name by `_mut` for mutating getters.

```rust,noplayground
pub struct S {
    first: First,
}

impl S {
    // Not `get_first`.
    pub fn first(&self) -> &First {
        &self.first
    }

    // Not `get_first_mut`, `get_mut_first`, or `mut_first`.
    pub fn first_mut(&mut self) -> &mut First {
        &mut self.first
    }
}
```

For getters that do runtime validation such as bounds checking, consider adding unsafe `_unchecked` variants.

## Name Iterators {#iterator-conventions}

For a container with elements of type `U`, iterator methods should be named:{{hi:Iterator methods}}

```rust,noplayground
fn iter(&self) -> Iter             // The `Iter` struct implements `Iterator<Item = &U>`.
fn iter_mut(&mut self) -> IterMut  // `IterMut` implements `Iterator<Item = &mut U>`.
fn into_iter(self) -> IntoIter     // `IntoIter` implements `Iterator<Item = U>`.
```

## References {#references .skip}

The above is adapted from:

- Rust API Guidelines: [Naming][book~api-guidelines-naming]↗.
- [RFC430][finalizing-naming-conventions.md~repo]↗.
- [RFC199][ownership-variants~repo]↗.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
