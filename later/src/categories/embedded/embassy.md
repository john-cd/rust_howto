<!-- TODO Reviewed ??-??-2026 -->

# Embassy and Friends

{{#include embassy.incl.md}}

## `embassy` {#embassy}

[![embassy][c~embassy~docs~badge]][c~embassy~docs]{{hi:embassy}}
[![embassy~crates.io][c~embassy~crates.io~badge]][c~embassy~crates.io]
[![embassy~repo][c~embassy~repo~badge]][c~embassy~repo]
[![embassy~lib.rs][c~embassy~lib.rs~badge]][c~embassy~lib.rs]

[`embassy`][c~embassy~docs]↗{{hi:embassy}} is the next-generation framework for embedded applications. Write safe, correct and energy-efficient embedded code faster, using the Rust programming language, its async facilities, and the Embassy libraries.

The code below is a board-specific sketch, not a host-buildable Cargo example. A real firmware project must select compatible Embassy executor and time-driver crates, configure a chip-specific HAL, and provide the pin and interrupt setup. The sketch remains unregistered until that target setup is complete ([#751](https://github.com/john-cd/rust_howto/issues/751)).

```rust,editable
{{#include ../../../crates/cats/embedded/examples/embassy/embassy.rs:example}}
```

## Related Topics {#related-topics .skip}

- Async embedded programming.
- Embedded hardware abstraction.
- Real-time systems and low power design.
- HAL and PAC-based development.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [embassy: add embassy, others](https://github.com/john-cd/rust_howto/issues/345)
</div>
