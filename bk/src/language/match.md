# Patterns, Match, If Let, and While Let

{{#include match.incl.md}}

## Patterns {#patterns}

Patterns provide a way to match against the structure of types, whether simple or complex. You can use them to destructure data into its constituent parts, bind those parts to variables, and control your program's flow based on whether a value conforms to a certain shape.

The following table summarizes possible patterns, which, of course, can be combined with one another. See examples below for more details.

| Pattern | Description |
|---|---|
| Literals | Literals match exact values like `1`, "hello", `true` or `Some(42)`. |
| Ranges | Ranges work for `char` and `integer` types. They match a value within an interval: `1..6` (1 to 5, 6 is excluded) or `1..=5` (1 to 5 inclusive) or `'a'..='z'`. |
| Variable Binding | When a pattern matches, it can bind parts of the value to new variables (e.g., `Some(data)` binds its inner value to the `data` variable). |
| Destructuring | Destructuring break down structs, enums, and tuples into their components. For example, for a `Point` `struct`, use `Point { x: a, y: b }` to destructure its fields into the `a` and `b` variables. For enums, use e.g. `MyEnum::Variant(a, b)` or `MyEnum::Variant2 { x: a }`; for tuples, use e.g. `(first, second,)` to retrieve the first and second components of a 2-tuple, or `(0, 0)` to match the origin. |
| Ignoring Values | `_` ignores a single value without binding. For example, `(first, _)` binds the first component of a tuple only. `_name` is a variable name starting with an underscore binds the value but the compiler won't warn if it's unused, signaling an intentional non-use. `..` ignores multiple remaining items in a tuple, struct, or slice. For example, `Point3D { x: 0, .. }` matches any `Point3D` with x = 0, regardless of the values of `y` and `z`. |
| Or Pattern | The character `\|` between subpatterns match if any one of the subpatterns matches. `1 \| 2 \| 3` matches 1, 2, or 3. All patterns in an "or" must bind variables of the same name and type, if they do. |
| Match Guards (`if` condition) | Match guards add an additional conditional check to a pattern arm. The pattern must match, and the guard's if condition must be true for the arm to be chosen. |
| `@` Bindings (Bind to Subpatterns) | `@` allow you to bind a variable name to a value while also testing that value against a further pattern. The syntax is `variable_name @ pattern`, e.g., `id_val @ 3..=7`. |
| `ref` and `ref mut` Bindings | Create references (`&T` or `&mut T`) to parts of the matched value, rather than moving or copying them. This is useful when you want to borrow parts of a value. Note that Rust's "match ergonomics" often make explicit `ref` and `ref mut` unnecessary when matching on references. |

Patterns are used in several places:

- `match` expressions branch code based on different patterns. They are the most common place you'll see complex pattern matching.
- `if let` and `while let` expressions are convenient for handling a single pattern or looping as long as a pattern matches, respectively.
- `for` loops can destructure elements from an iterator. For example, `for (index, value) in my_vec.iter().enumerate() { ... }` destructures into a tuple.
- `let` statements can destructure tuples, structs, enums, and bind their parts to variables. For example, `let (x, y) = (1, 2);` assigns `1` to `x` and `2` to `y`.
- Patterns can also be used in function and closure parameter lists to destructure arguments directly. For example, `fn print_coordinates(&(x, y): &(i32, i32))` assigns `i32` values to `x` and `y`.

## Use `match` to Branch on a Pattern {#use-match-to-branch-on-a-pattern}

[![Rust by example - match][book-rust-by-example-match-badge]][book-rust-by-example-match]{{hi:match}}

The following code demonstrates pattern matching against an enumeration (enum):

```rust,editable
{{#include ../../crates/language/tests/match/match1.rs:example}}
```

This example shows pattern matching using a `struct`:

```rust,editable
{{#include ../../crates/language/tests/match/match2.rs:example}}
```

## Handle a Single Pattern with `if let` {#if-let}

[![Rust by example - if let][book-rust-by-example-if_let-badge]][book-rust-by-example-if_let]

`if let` is a concise way to match a single pattern:

```rust,editable
{{#include ../../crates/language/tests/match/if_let.rs:example}}
```

## Loop as Long as a Pattern Matches with `while let` {#while-let}

`while let` is similar to `if let`, but it allows you to loop as long as the pattern continues to match:

```rust,editable
{{#include ../../crates/language/tests/match/while_let.rs:example}}
```

## Destructure elements from an Iterator within a `for` Loop {#for}

`for` loops can destructure elements from an iterator:

```rust,editable
{{#include ../../crates/language/tests/match/for1.rs:example}}
```

## Destructure during Assignment with `let` and `let else` {#let-else}

`let`, beyond simple assignments of a value to a variable, can also destructure - provided that binding to the pattern can't fail:

```rust,editable
{{#include ../../crates/language/tests/match/let1.rs:example}}
```

With `let-else`, a refutable pattern (a pattern that can fail) can match and bind variables in the surrounding scope like a normal `let`, or else diverge (e.g. `break`, `return`, `panic!`) when the pattern doesn't match.

```rust,editable
{{#include ../../crates/language/tests/match/let_else.rs:example}}
```

## Destructure Arguments of a Function or Closure {#function-or-closure}

```rust,editable
{{#include ../../crates/language/tests/match/fn_closure_arguments.rs:example}}
```

## Pattern Examples {#skip}

### Match against Literals {#literals}

You can match against literal values directly:

```rust,editable
{{#include ../../crates/language/tests/match/literals.rs:example}}
```

### Match against a Range {#ranges}

Ranges match a value between two numbers either inclusively (`start..=end`) or exclusively at the upper bound (`start_included..end_not_included`). Ranges work for both `char` and integer types:

```rust,editable
{{#include ../../crates/language/tests/match/ranges.rs:example}}
```

### Bind to Variables {#variable-bindings}

Patterns can bind parts of a matched value to new variables:

```rust,editable
{{#include ../../crates/language/tests/match/variable_binding.rs:example}}
```

### Destructure {#skip}

Patterns can destructure structs, enums, tuples, and references.

#### Destructure Structs {#structs}

The pattern to destructure a struct is similar to the struct's assignment syntax:

```rust,editable
{{#include ../../crates/language/tests/match/destructure_struct.rs:example}}
```

#### Destructure Enums {#enums}

```rust,editable
{{#include ../../crates/language/tests/match/destructure_enum.rs:example}}
```

#### Destructure Tuples {#tuples}

```rust,editable
{{#include ../../crates/language/tests/match/destructure_tuple.rs:example}}
```

### Ignore Values {#ignore-values}

- `_` ignores a single value.
- A variable starting with `_` e.g., `_name`, binds the value but silences unused variable warnings.
- `..` ignores remaining parts of a struct, tuple, or slice.

```rust,editable
{{#include ../../crates/language/tests/match/ignore_values.rs:example}}
```

### Use an Or (`|`) to Match Multiple Patterns {#or-patterns}

A sequence of patterns separated by `|` match if any of the sub-patterns match. All sub-patterns must bind variables of the same name and type:

```rust,editable
{{#include ../../crates/language/tests/match/or_patterns.rs:example}}
```

### Bind a Variable with `@` Bindings (Bind to Subpatterns) {#@-bindings}

Bind a variable name to a value while also testing that value against a further pattern (`variable @ pattern`).

```rust,editable
{{#include ../../crates/language/tests/match/at_bindings.rs:example}}
```

### Match Guards (`if condition`) {#match-guards}

Match guards add an additional conditional check to a pattern:

```rust,editable
{{#include ../../crates/language/tests/match/match_guards.rs:example}}
```

### Bind and Create References with `ref` and `ref mut` {#ref-and-ref-mut-bindings}

These create references to parts of the matched value.

Note that Rust's "match ergonomics" often make explicit `ref` and `ref mut` unnecessary when matching on references.

```rust,editable
{{#include ../../crates/language/tests/match/ref_bindings.rs:example}}
```

## See Also {#skip}

- [[control_flow | Control Flow]].
- [[error_handling | Error Handling]].
- [[option | Option]].
- [[result | Result]].
- [[rust-patterns | Rust Patterns]].

## References {#skip}

- [Pattern matching][book-rust-pattern-matching]{{hi:Pattern matching}}⮳.
- [Patterns Are Not Expressions](https://h2co3.github.io/pattern)⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
