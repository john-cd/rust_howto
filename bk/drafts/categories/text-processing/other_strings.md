<!-- TODO Reviewed ??-??-2026 -->

# OS and Non-UTF-8 Strings

{{#include other_strings.incl.md}}

## Work with platform-specific strings with `OsString` and `OsStr` {#osstring}

Use [`std::ffi::OsString`][c~std::ffi::OsString~docs]↗{{hi:std::ffi::OsString}} and [`std::ffi::OsStr`][c~std::ffi::OsStr~docs]↗{{hi:std::ffi::OsStr}} for values supplied by the operating system, such as paths, environment variables, and command-line arguments. Their representation is platform-specific and may not be valid Unicode. Rust `String` and `str` values, by contrast, are always valid UTF-8.

`OsString` owns a value; `OsStr` borrows one. Use `OsStr::to_str` when the value must be valid Unicode and handle the `None` case. `to_string_lossy` replaces invalid sequences, so use it for display or diagnostics only when losing the original representation is acceptable. It is not a round-trip conversion.

An `OsString` is not a NUL-terminated C string. Pass paths and process arguments through Rust's platform-aware APIs. If an FFI function needs a string, follow that function's platform and encoding contract and convert explicitly.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/osstring.rs:example}}
```

## Work with C-style strings with `CString` and `CStr` {#cstring}

A C interface defines the representation and encoding its function expects. [`CString`][c~std::ffi::CString~docs]↗{{hi:std::ffi::CString}} and [`CStr`][c~std::ffi::CStr~docs]↗{{hi:std::ffi::CStr}} represent NUL-terminated byte strings; they do not choose or convert an encoding for the C API. A wide-character API needs its corresponding platform type and conversion instead.

`CString` owns bytes, adds a trailing NUL terminator, and rejects interior NUL bytes. `CStr` is a borrowed view of a NUL-terminated byte string. `CStr::to_str` validates that its bytes are UTF-8; use `to_string_lossy` only when replacement characters are acceptable.

For FFI declarations, use the pointer type required by the API, such as `*const c_char`, rather than putting `CStr` directly in the signature. Keep the owning `CString` alive for as long as foreign code may read its pointer, and follow the API's ownership and mutability rules. Constructing a `CStr` from a raw pointer is unsafe because the pointer must reference a readable, terminated string for the chosen lifetime.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/cstring.rs:example}}
```

## Work with arbitrary bytes with `bstr` {#bstr}

{{#badges bstr encoding=Encoding text-processing="Text processing"}}{{hi:Text}}{{hi:String}}{{hi:Byte}}{{hi:Bytes}}{{hi:Str}}

[`bstr`][c~bstr~docs]↗{{hi:bstr}} provides string-like operations on byte sequences that may be invalid UTF-8. Use it when the input is byte-oriented and must be preserved, such as data read from a file or external process. Operations on bytes do not automatically correspond to Unicode characters or grapheme clusters.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/bstr.rs:example}}
```

## Intern strings with `ustr` {#ustr}

{{#badges ustr caching=Caching data-structures="Data structures"}}{{hi:Ffi}}{{hi:Interning}}{{hi:String}}

The [`ustr`][c~ustr~docs]↗{{hi:ustr}} crate interns strings so equal values can share a stored representation. This can reduce repeated string storage and make equality checks inexpensive, at the cost of interning-table lookups and retained entries. It is useful when strings are reused often and are not frequently assembled or changed; measure with the application's workload.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/other_strings/ustr.rs:example}}
```

You may also use the [string_cache][c~string_cache~docs]↗ library for interning values that implement `AsRef<str>`.

## Store short strings with `compact_str` {#compact_str}

{{#badges compact_str encoding=Encoding memory-management="Memory management" parsing="Parsing tools" text-processing="Text processing"}}{{hi:Compact}}{{hi:Memory}}{{hi:Mutable}}{{hi:Small}}{{hi:String}}

[`compact_str`][c~compact_str~docs]↗{{hi:compact_str}} provides `CompactString`, which stores short strings inline in the value and longer strings in allocated storage. Inline storage is part of the value; it is not necessarily on the stack. Consider it when an application has many short strings, and compare memory use and performance with `String` for that workload.

## Related topics {#related-topics .skip}

- [[development-tools_ffi | Development Tools: FFI]]
- [[strings | Strings]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  review
</div>
