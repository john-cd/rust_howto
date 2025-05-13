# Strings

{{#include strings.incl.md}}

## `String` and `&str` {#string}

[![std][c-std-badge]][c-std]{{hi:std}}{{hi:Strings}}

`str` (String Slice) is a primitive type representing a view into a sequence of UTF-8 encoded bytes.

- It is immutable.
- Since `str`'s size is unknown at compile time, one can only handle it behind a pointer. It is most often seen in its borrowed form as `&str`.
- `&str` is a fat pointer containing a pointer to the string data (which can reside on the heap, stack, or in static memory) and its length.
- `&str` is used when you need a view of existing string data without taking ownership. This is common for function arguments, where you don't need to own the string.

String literals (e.g., "hello") are of type `&'static str`, meaning they exist for the entire lifetime of the program.

`String` is a growable, mutable, owned string type allocated on the heap.

- It is similar to a `Vec<u8>` that is guaranteed to hold valid UTF-8.
- String owns its data, meaning when a String goes out of scope, the memory it occupies on the heap is automatically deallocated.
- Use String when you need to modify a string, own string data (e.g., to pass it to another thread or store it in a struct that owns it), or create a new string at runtime.

The relationship between `String` and `&str` is similar to that between `Vec<T>` and `&[T]` (a vector and a slice of a vector). `String` owns the underlying buffer, while `&str` is a reference to a portion of that buffer or some other string data.

```rust,editable
{{#include ../../crates/standard_library/tests/strings/strings.rs:example}}
```

## String Formatting; Placeholders {#placeholders}

[![std][c-std-badge]][c-std]

```rust,editable
{{#include ../../crates/standard_library/tests/strings/strings2.rs:example}}
```

Use `{:?}` to use the [`std::fmt::Debug`][c-std::fmt::Debug]{{hi:std::fmt::Debug}}⮳ output format{{hi:Output format}} (annotate type with `#[derive(Debug)]` ) or `{:#?}` for pretty print{{hi:Pretty print}}.

Also use `dbg!(&rect1);` for debug output{{hi:Debug output}} (returns ownership of the expression's value).

## Concatenate Strings {#string-concatenation}

[![std][c-std-badge]][c-std]

Here are several common methods to concatenate{{hi:Concatenation}} [`String`][c-std::string::String]s⮳{{hi:String}}:

```rust,editable
{{#include ../../crates/standard_library/tests/strings3.rs:example}}
```

Examples from [concatenation_benchmarks-rs][concatenation_benchmarks-github]⮳.

## String Manipulation {#skip}

- [[regex | Regex (Regular Expressions)]].
- [[parsing | Parsing]].
- [[text-processing | Text Processing]].
- [[string_parsing | String Parsing]].
- [[string_concat | String Concatenation]].

## Related Data Structures {#skip}

- [[slices | Slices]].
- [[vectors | Vectors]].
- [[cow | COW]].

## Related Topics {#skip}

- [[algorithms | Algorithms]].
- [[encoding | Encoding]].
- [[internationalization | Internationalization]].
- [[localization | Localization]].
- [[search | Search]].
- [[rust_search_engines | Rust Search Engines]].
- [[template-engine | Template Engine]].
- [[text_layout | Text Layout]].
- [[unicode | Unicode]] handling.
- [[value-formatting | Value Formatting]].
- Working with [[other_strings | Other Strings]] (CString, OsString).

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[strings: add NOW](https://github.com/john-cd/rust_howto/issues/630)
</div>
