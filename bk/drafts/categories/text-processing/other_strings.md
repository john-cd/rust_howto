# OS and non-UTF8 Strings

{{#include other_strings.incl.md}}

## `OsString` and `OsStr` {#osstring}

[`std::ffi::OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html) is a type that can represent owned, mutable platform-native strings, but is cheaply inter-convertible with Rust strings.

The need for this type arises from the fact that:

- On Unix systems, strings are often arbitrary sequences of non-zero bytes, in many cases interpreted as UTF-8.
- On Windows, strings are often arbitrary sequences of non-zero 16-bit values, interpreted as UTF-16 when it is valid to do so.
- In Rust, strings are always valid UTF-8, which may contain zeros.

`OsString` and `OsStr` bridge this gap by simultaneously representing Rust and platform-native string values, and in particular allowing a Rust string to be converted into an “OS” string with no cost if possible. A consequence of this is that `OsString` instances are not NUL terminated; in order to pass to e.g., Unix system call, you should create a `CStr`.

[`std::ffi::OsStr`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html) is a borrowed reference to an OS string. `&OsStr` is to `OsString` as `&str` is to `String`: the former in each pair are borrowed references; the latter are owned strings.

```rust,editable
{{#include ../../../crates/cats/text_processing/tests/other_strings/osstring.rs:example}}
```

## `CString` and `CStr` {#cstring}

C strings are different from Rust strings:

- Rust strings are UTF-8, but C strings may use other encodings.
- Their character sizes may be different.
- C strings are nul-terminated, i.e., they have a \0 character at the end.
- C strings cannot have nul characters in the middle.

Use `CString` and `CStr` when you need to convert Rust UTF-8 strings to and from C-style strings. Their primary use case is **FFI**, Foreign Function Interface, the mechanism by which Rust interacts with code written in other languages with a C ABI, like C and Python.

[`std::ffi::CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html) represents an owned, C-compatible, nul-terminated string with no nul bytes in the middle. A `CString` can be created from either a byte slice or a byte vector, or anything that implements `Into<Vec<u8>>` (for example, you can build a `CString` straight out of a `String` or a `&str`, since both implement that trait).

`std::ffi::CStr` represents a borrowed reference to a nul-terminated array of bytes. It can be constructed safely from a `&[u8]` slice, or unsafely from a raw `*const c_char`. It can be expressed as a literal in the form `c"Hello world"`. Note that this structure does not have a guaranteed layout (the `repr(transparent)` notwithstanding) and should not be directly placed in the signatures of FFI functions. Instead, safe wrappers of FFI functions may leverage `CStr::as_ptr` and the unsafe `CStr::from_ptr` constructor to provide a safe interface to other consumers.

`&CStr` is to `CString` as `&str` is to `String`: the former in each pair are borrowed references; the latter are owned strings.

```rust,editable
{{#include ../../../crates/cats/text_processing/tests/other_strings/cstring.rs:example}}
```

## `bstr` {#bstr}

[![bstr][c-bstr-badge]][c-bstr] [![bstr-crates.io][c-bstr-crates.io-badge]][c-bstr-crates.io] [![bstr-github][c-bstr-github-badge]][c-bstr-github] [![bstr-lib.rs][c-bstr-lib.rs-badge]][c-bstr-lib.rs]{{hi:bstr}}{{hi:Text}}{{hi:String}}{{hi:Byte}}{{hi:Bytes}}{{hi:Str}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

[`bstr`][c-bstr]⮳{{hi:bstr}} offers a string type that is not required to be valid UTF-8.

This crate provides extension traits for `&[u8]` and `Vec<u8>` that enable their use as byte strings, where byte strings are conventionally UTF-8. This differs from the standard library's `String` and `str` types in that they are *not* required to be valid UTF-8, but may be fully or partially valid UTF-8.

```rust,editable
{{#include ../../../crates/cats/text_processing/tests/other_strings/bstr.rs:example}}
```

## Related Topics {#skip}

- [[development-tools_ffi | Development Tools: FFI]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1194)

- [compact_str](https://crates.io/crates/compact_str)
- [string_cache](https://docs.rs/string_cache/latest/string_cache/) A library for interning things that are AsRef<str>.

</div>
