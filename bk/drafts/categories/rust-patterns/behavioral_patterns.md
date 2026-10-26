<!-- TODO Reviewed ??-??-2026 -->

# Behavioral Patterns

{{#include behavioral_patterns.incl.md}}

Behavioral patterns describe how components communicate and how operations are represented. In Rust, enums, traits, closures, and ownership often express these patterns directly, without a class hierarchy. The recipes below show a typestate API, data-driven business rules, and a command-based undo/redo history.

## Implement the Typestate Pattern in Rust {#typestate-pattern}

The typestate pattern is a design pattern that encodes information about an object's run-time state in its compile-time type.

- [Typestate Pattern][blog~typestate-pattern]↗.

## Externalize Business Rules in a Decision Table {#business-rules}

[![zen-engine][c~zen-engine~docs~badge]][c~zen-engine~docs] [![zen-engine~crates.io][c~zen-engine~crates.io~badge]][c~zen-engine~crates.io]{{hi:zen-engine}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rules engine}}

When rules change more often than releases, or are owned by non-developers, keep them as data. [`zen-engine`][c~zen-engine~docs]↗{{hi:zen-engine}} evaluates JSON Decision Model (JDM) documents (decision tables, expressions and graphs of them) against a JSON context. Compared with a `match`, rules as data can be edited in a visual editor, reloaded without a deploy and tested independently. Validate required fields and turn an unmatched result into a clear application error. Rule evaluation adds parsing and runtime work, so benchmark with realistic rule counts and input sizes before using it on a latency-sensitive path.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/business_rules/decision_table.rs:example}}
```

## Add Undo and Redo to a Text Editor with Commands {#undo-redo}

A command records the data needed to apply and reverse one edit. Keeping the undo and redo stacks in the editor makes the state transitions explicit and deterministic.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/behavioral_patterns/undo_redo.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Builder Pattern](builder_pattern.md) covers object construction.
- [Rust-specific Patterns](rust_specific_patterns.md) describes type-level patterns.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
