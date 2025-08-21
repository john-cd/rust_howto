# Control Flow

{{#include control_flow.incl.md}}

{{i:Control flow}} constructs allow you to run code conditionally or repeatedly, directing the "flow" of your program's execution. Rust provides several familiar ways to do this.

## Execute Code based on a Condition Using `if` and `else` {#if-else}

[![Rust by example - if else][book~rust-by-example~if_else~badge]][book~rust-by-example~if_else]

`if`, `else`, and `else if` allow you to execute different blocks of code based on whether a boolean condition is true or false:

```rust,editable
{{#include ../../crates/language/examples/control_flow/if_else.rs:example}}
```

Refer to the [[match | match]] chapter for `if let` expressions.

## Create Loops Using `loop` {#loop}

[![Rust by example - loop][book~rust-by-example~loop~badge]][book~rust-by-example~loop]

`loop` creates an infinite loop that runs until explicitly stopped, usually with the `break` keyword.
Like `if`, `loop` is an expression, and `break` can return a value from the loop.

```rust,editable
{{#include ../../crates/language/examples/control_flow/loop1.rs:example}}
```

You can also use `continue` to skip the rest of the current iteration and start the next one. Loops can also have labels (`'label: loop { ... break 'label; }`) for breaking or continuing to outer loops from within nested loops.

## Execute Code Repeatedly While a Condition is True with `while` {#while}

[![Rust by example - while][book~rust-by-example~while~badge]][book~rust-by-example~while]

`while` executes a block of code repeatedly as long as a boolean condition remains true.
The condition is checked before each iteration.

```rust,editable
{{#include ../../crates/language/examples/control_flow/while1.rs:example}}
```

`while` supports `break` expressions and loop labels. Refer to the [[match | match]] chapter for `while let` loops.

## Iterate Through a Collection with `for` {#for}

[![Rust by example - for][book~rust-by-example~for~badge]][book~rust-by-example~for]

`for` is used to iterate over the items produced by an iterator.

Many collection types (like ranges, arrays, vectors, strings) implement the `IntoIterator` trait, which provides a method `into_iter()` that returns an iterator. They may also provide additional methods such as `iter()` or `iter_mut()`.

```rust,editable
{{#include ../../crates/language/examples/control_flow/for1.rs:example}}
```

`for` loops support `break` expressions and loop labels.

## Exit Early from Blocks {#labeled-block-expressions}

Labeled block expressions are like block expressions, except that they allow using `break` expressions within the block.
You may think of them as a `loop` that executes only once and allows exiting early.

```rust,editable
{{#include ../../crates/language/examples/control_flow/labeled_block_expression.rs:example}}
```

## References {#references}

- Rust by Example - [Control flow][book~rust-by-example~flow_control]↗.

## Related Topics {#related-topics}

- [[match | Match]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
