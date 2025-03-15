# Format Values for Display to a User, Potentially Adapting the Display to Various Languages and Regions

[![cat-value-formatting][cat-value-formatting-badge]][cat-value-formatting]{{hi:Value formatting}}

{{#include number_formatting.incl.md}}

## Convert Floating Point to String Quickly with `ryu` {#ryu}

[![ryu][c-ryu-badge]][c-ryu] [![ryu-crates.io][c-ryu-crates.io-badge]][c-ryu-crates.io] [![ryu-github][c-ryu-github-badge]][c-ryu-github] [![ryu-lib.rs][c-ryu-lib.rs-badge]][c-ryu-lib.rs]{{hi:ryu}}{{hi:Float}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}} [![cat-value-formatting][cat-value-formatting-badge]][cat-value-formatting]{{hi:Value formatting}}

Fast floating point to string conversion.

```rust,editable
{{#include ../../../crates/cats/value_formatting/tests/ryu.rs:example}}
```

## Convert Integers to String Quickly with `iota` {#itoa}

[![itoa][c-itoa-badge]][c-itoa] [![itoa-crates.io][c-itoa-crates.io-badge]][c-itoa-crates.io] [![itoa-github][c-itoa-github-badge]][c-itoa-github] [![itoa-lib.rs][c-itoa-lib.rs-badge]][c-itoa-lib.rs]{{hi:itoa}}{{hi:Integer}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}} [![cat-value-formatting][cat-value-formatting-badge]][cat-value-formatting]{{hi:Value formatting}}

Fast integer primitive to string conversion.

## See Also

- [[internationalization | Internationalization]].
- [[localization | Localization]].
- [[text-processing | Text Processing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[value-formatting: choose crates, write](https://github.com/john-cd/rust_howto/issues/490)
cover [`num-format`][c-num_format]⮳{{hi:num-format}}, [`icu`][c-icu]⮳{{hi:icu}}, [`fluent`][c-fluent]⮳{{hi:fluent}}
</div>
