# Default Trait

{{#include default.incl.md}}

## Define Default Values for a Struct with `Default` {#default}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

The [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)⮳ trait provides a way to create a default value for a type. This is useful when you want to initialize a variable with a default value without having to specify it explicitly.

The `Default` trait has a single method, `default()`, which returns the default value for the type. You can implement this trait for your own types to provide a default value. Alternatively, the `#[derive(Default)]` attribute automatically implement the trait for a type.

```rust,editable
{{#include ../../crates/cats/standard_library/examples/default/default.rs:example}}
```

## Related Topics {#related-topics}

- [[derive | Derive]].
- [[traits | Traits]].

## References {#references}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
