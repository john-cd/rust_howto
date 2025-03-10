# AsRef and &T {#asref}

{{#include asref.incl.md}}

The [`AsRef`][c-std::convert::AsRef]⮳{{hi:AsRef}} trait in Rust is used for cheap reference-to-reference conversions.
It provides a way to convert an object into a reference to another type.
This trait is often used to allow [functions][p-functions] to accept arguments in multiple forms.

```rust,editable
{{#include ../../crates/standard_library/tests/other/asref.rs:example}}
```

NOTES:

- [`AsRef`][c-std::convert::AsRef]⮳{{hi:AsRef}} is similar to [`AsMut`][c-std::convert::AsMut]⮳{{hi:AsMut}}, which is used for converting between mutable references.
- If you need to do a costly conversion, it is better to implement `From` with type `&T` or write a custom function.

[When and why to use AsRef<T>instead of &T][stackoverflow-asref]⮳{{hi:asref}}

## Related Topics {#skip}

- [[smart_pointers | Smart Pointers]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[asref: write](https://github.com/john-cd/rust_howto/issues/619)
</div>
