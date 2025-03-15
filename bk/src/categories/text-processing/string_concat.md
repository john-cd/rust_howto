# String Concatenation

{{#include string_concat.incl.md}}

## Concatenate Strings {#string-concat}

String concatenation is the operation of joining two or more strings together end-to-end to create a new, single string.

[![std][c-std-badge]][c-std] [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

[![joinery][c-joinery-badge]][c-joinery] [![joinery-crates.io][c-joinery-crates.io-badge]][c-joinery-crates.io] [![joinery-github][c-joinery-github-badge]][c-joinery-github] [![joinery-lib.rs][c-joinery-lib.rs-badge]][c-joinery-lib.rs]{{hi:joinery}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}} [![cat-value-formatting][cat-value-formatting-badge]][cat-value-formatting]{{hi:Value formatting}}

`joinery` is a small crate for generically joining iterators with a separator.

```rust,editable
{{#include ../../../crates/cats/text_processing/tests/string_concat.rs:example}}
```

## References

[String concatenation benchmark][concat-benchmark-github]⮳.

## Related Topics {#skip}

- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/964)
</div>
