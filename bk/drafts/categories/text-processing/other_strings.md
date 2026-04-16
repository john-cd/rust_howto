# OS and non-UTF8 Strings

{{#include other_strings.incl.md}}

## Work with Platform-specific Strings with `OsString` and `OsStr` {#osstring}

[`std::ffi::OsString`][c~std::ffi::OsString~docs]↗ is a type that can represent owned, mutable platform-native strings, but is cheaply inter-convertible with Rust strings.

The need for this type arises from the fact that:

- On Unix systems, strings are often arbitrary sequences of non-zero bytes, in many cases interpreted as UTF-8.
- On Windows, strings are often arbitrary sequences of non-zero 16-bit values, interpreted as UTF-16 when it is valid to do so.
- In Rust, strings are always valid UTF-8, which may contain zeros.

[`OsString`][c~std::ffi::OsString~docs]↗{{hi:std::ffi::OsString}} and [`OsStr`][c~std::ffi::OsStr~docs]↗{{hi:std::ffi::OsStr}} bridge this gap by simultaneously representing Rust and platform-native string values, and in particular allowing a Rust string to be converted into an "OS" string with no cost if possible. A consequence of this is that `OsString` instances are not NUL terminated; in order to pass to e.g., Unix system call, create a [`CStr`][c~std::ffi::CStr~docs]↗{{hi:std::ffi::CStr}}.

[`std::ffi::OsStr`][c~std::ffi::OsStr~docs]↗ is a borrowed reference to an OS string. `&OsStr`{{hi:&OsStr}} is to `OsString` as `&str` is to `String`: the former in each pair are borrowed references; the latter are owned strings.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/osstring.rs:example}}
```

## Work with C-style, NUL-terminated Strings with `CString` and `CStr` {#cstring}

C strings are different from Rust strings:

- Rust strings are UTF-8, but C strings may use other encodings.
- Their character sizes may be different.
- C strings are NUL-terminated, i.e., they have a \0 character at the end.
- C strings cannot have NUL characters in the middle.

Use [`CString`][c~std::ffi::CString~docs]↗{{hi:std::ffi::CString}} and [`CStr`][c~std::ffi::CStr~docs]↗{{hi:std::ffi::CStr}} to convert Rust UTF-8 strings to and from C-style strings. Their primary use case is **FFI**, Foreign Function Interface, the mechanism by which Rust interacts with code written in other languages with a C ABI, like C and Python.

[`std::ffi::CString`][c~std::ffi::CString~docs]↗ represents an owned, C-compatible, nul-terminated string with no nul bytes in the middle. A `CString` can be created from either a byte slice or a byte vector, or anything that implements `Into<Vec<u8>>` (for example, we can build a `CString` straight out of a `String` or a `&str`, since both implement that trait).

`std::ffi::CStr` represents a borrowed reference to a nul-terminated array of bytes. It can be constructed safely from a `&[u8]` slice, or unsafely from a raw `*const c_char`. It can be expressed as a literal in the form `c"Hello world"`. Note that this structure does not have a guaranteed layout (the `repr(transparent)` notwithstanding) and should not be directly placed in the signatures of FFI functions. Instead, safe wrappers of FFI functions may leverage `CStr::as_ptr` and the unsafe `CStr::from_ptr` constructor to provide a safe interface to other consumers.

`&CStr` is to `CString` as `&str` is to `String`: the former in each pair are borrowed references; the latter are owned strings.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/cstring.rs:example}}
```

## Work with Non-UTF8 Strings with `bstr` {#bstr}

[![bstr][c~bstr~docs~badge]][c~bstr~docs] [![bstr~crates.io][c~bstr~crates.io~badge]][c~bstr~crates.io] [![bstr~repo][c~bstr~repo~badge]][c~bstr~repo] [![bstr~lib.rs][c~bstr~lib.rs~badge]][c~bstr~lib.rs]{{hi:bstr}}{{hi:Text}}{{hi:String}}{{hi:Byte}}{{hi:Bytes}}{{hi:Str}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`bstr`][c~bstr~docs]↗{{hi:bstr}} offers a string type that is not required to be valid UTF-8.

This crate provides extension traits for `&[u8]` and `Vec<u8>` that enable their use as byte strings, where byte strings are conventionally UTF-8. This differs from the standard library's `String` and `str` types in that they are *not* required to be valid UTF-8, but may be fully or partially valid UTF-8.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/bstr.rs:example}}
```

## Intern Strings with `ustr` {#ustr}

[![ustr][c~ustr~docs~badge]][c~ustr~docs] [![ustr~crates.io][c~ustr~crates.io~badge]][c~ustr~crates.io] [![ustr~repo][c~ustr~repo~badge]][c~ustr~repo] [![ustr~lib.rs][c~ustr~lib.rs~badge]][c~ustr~lib.rs]{{hi:ustr}}{{hi:Ffi}}{{hi:Interning}}{{hi:String}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The [`ustr`][c~ustr~docs]↗{{hi:ustr}} crate implements string interning, i.e. storing only one copy of each distinct string value (which must be immutable). String interning is useful in scenarios where:

- You use a large number of identical strings (e.g., keywords, hashmap keys, enum variants represented as strings, common configuration values),
- You frequently assign or compare strings for equality,
- The creation of strings from raw characters is relatively rare compared to copying or comparing to existing strings,
- You use strings as keys in hash maps or elements in hash sets,
- There is little character-by-character assembly of strings, string concatenation, or other string manipulation (other than equality testing).

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/ustr.rs:example}}
```

You may also use the [string_cache][c~string_cache~docs]↗ library for interning things that are `AsRef<str>`.

## Work with Small Formatted Strings with `compact_str` {#compact_str}

[![compact_str][c~compact_str~docs~badge]][c~compact_str~docs] [![compact_str~crates.io][c~compact_str~crates.io~badge]][c~compact_str~crates.io] [![compact_str~repo][c~compact_str~repo~badge]][c~compact_str~repo] [![compact_str~lib.rs][c~compact_str~lib.rs~badge]][c~compact_str~lib.rs]{{hi:compact_str}}{{hi:Compact}}{{hi:Memory}}{{hi:Mutable}}{{hi:Small}}{{hi:String}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

For small formatted string, consider a string type implementing small-string optimization. For example, [compact_str][c~compact_str~crates.io]↗ implements `CompactString`, a memory efficient string type that can store smaller strings on the stack and transparently stores longer strings on the heap.

## Related Topics {#related-topics .skip}

- [[development-tools_ffi | Development Tools: FFI]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1194)
</div>
