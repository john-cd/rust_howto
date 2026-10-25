<!-- TODO Reviewed ??-??-2026 -->

# Behavioral Patterns

{{#include behavioral_patterns.incl.md}}

## Implement the Typestate Pattern in Rust {#typestate-pattern}

The typestate pattern is a design pattern that encodes information about an object's run-time state in its compile-time type.

- [Typestate Pattern][blog~typestate-pattern]↗.

## Externalize Business Rules in a Decision Table {#business-rules}

[![zen-engine][c~zen-engine~docs~badge]][c~zen-engine~docs] [![zen-engine~crates.io][c~zen-engine~crates.io~badge]][c~zen-engine~crates.io]{{hi:zen-engine}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rules engine}}

When rules change more often than releases, or are owned by non-developers, keep them as data. [`zen-engine`][c~zen-engine~docs]↗{{hi:zen-engine}} evaluates JSON Decision Model (JDM) documents (decision tables, expressions and graphs of them) against a JSON context. Compared with a `match`, rules as data can be edited in a visual editor, reloaded without a deploy and tested independently. The cost is an extra dependency and a second place to look when debugging.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/business_rules/decision_table.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/461)
</div>
