# Pattern Matching

{{#include match.incl.md}}

Pattern matching is a powerful control flow construct for inspecting data, changing your program's flow based on whether a value conforms to a certain pattern, destructuring the value into its constituent parts, and binding those parts to variables. For example, `match` expressions branch code based on different patterns:

```rust,editable
// For example, let's match a sample `Result` against two patterns:
let res = Ok::<i32, String>(42);

match res {
    // The pattern `Ok(value)` below matches the provided `res`, therefore its expression after `=>` is evaluated:
    Ok(value) => { println!("Result: {value}"); } // It destructures `Ok` and binds its inner value to the new variable `value`, which is then printed.
    Err(error) => { eprintln!("Error: {error}"); }
    // All possible values must be covered.
}
```

You will see complex pattern matching most commonly in `match` expressions, but pattern matching can be used in several other places:

- `if let` and `while let` expressions are convenient for handling a single pattern or looping as long as a pattern matches, respectively.
- `for` loops can destructure elements from an iterator. For example, `for (index, value) in my_vec.iter().enumerate() { ... }` destructures a tuple into two variables `index` and `value`.
- `let` statements can destructure tuples, structs, enums, and bind their parts to variables. For example, `let (x, y) = (1, 2);` assigns `1` to `x` and `2` to `y`.
- Patterns can also be used in function and closure parameter lists to destructure arguments directly. For example, `fn print_coordinates(&(x, y): &(i32, i32))` assigns `i32` values to `x` and `y`.

## Use `match` to Branch on a Pattern {#use-match-to-branch-on-a-pattern}

[![Rust by example - match][book~rust-by-example~match~badge]][book~rust-by-example~match]{{hi:match}}

The following code demonstrates pattern matching against an enumeration (`enum`):

```rust,editable
{{#include ../../crates/language/examples/match/match1.rs:example}}
```

The following example shows pattern matching using a `struct`:

```rust,editable
{{#include ../../crates/language/examples/match/match2.rs:example}}
```

## Handle a Single Pattern with `if let` {#if-let}

[![Rust by example - if let][book~rust-by-example~if_let~badge]][book~rust-by-example~if_let]

`if let` is a concise way to match a single pattern:

```rust,editable
{{#include ../../crates/language/examples/match/if_let.rs:example}}
```

## Loop as Long as a Pattern Matches with `while let` {#while-let}

`while let` is similar to `if let`, but it allows you to loop as long as the pattern continues to match:

```rust,editable
{{#include ../../crates/language/examples/match/while_let.rs:example}}
```

## Destructure Elements from an Iterator within a `for` Loop {#destructure-for}

`for` loops can destructure elements from an iterator:

```rust,editable
{{#include ../../crates/language/examples/match/for1.rs:example}}
```

## Destructure during Assignment with `let` and `let else` {#let-else}

`let`, beyond simple assignments of a value to a variable, can also destructure - provided that binding to the pattern can't fail:

```rust,editable
{{#include ../../crates/language/examples/match/let1.rs:example}}
```

With `let ... else ...`, a refutable pattern (a pattern that can fail) can match and bind variables in the surrounding scope like a normal `let`, or else diverge (e.g. `break`, `return`, `panic!`) when the pattern doesn't match:

```rust,editable
{{#include ../../crates/language/examples/match/let_else.rs:example}}
```

## Destructure Arguments of a Function or Closure {#destructure-function-or-closure}

Destructuring in function or closure parameters lets you unpack complex data types right in their signature, making your code cleaner:

```rust,editable
{{#include ../../crates/language/examples/match/fn_closure_arguments.rs:example}}
```

## Pattern Syntax {#pattern-syntax}

The following table summarizes possible patterns, which, of course, can be combined with one another:

| Pattern | Description |
|---|---|
| Literals | Literals match exact values like `1`, "hello"... |
| Ranges | Ranges work for `char` and `integer` types. They match a value within an interval: `1..6` (1 to 5, 6 is excluded) or `1..=5` (1 to 5 inclusive) or `'a'..='z'`. |
| Variable Bindings | When a pattern matches, it can bind (parts of) the value to new variables. For example, `Some(data)` binds an `Option`'s inner value to the `data` variable. |
| Destructurings | Destructuring break down structs, enums, and tuples into their components. For example, for a `Point` `struct`, use `Point { x: a, y: b }` to destructure its fields into the `a` and `b` variables. For enums, use e.g. `MyEnum::Variant(a, b)` or `MyEnum::Variant2 { x: a, y: b }`; for tuples, use e.g. `(first, second,)` to retrieve the first and second components of a 2-tuple. |
| Ignoring Values | `_` ignores a single value without binding. For example, `(first, _)` binds the first component of a tuple only. `_name`, a variable name starting with an underscore, binds a value but the compiler won't warn if it's unused, signaling an intentional non-use. `..` ignores multiple remaining items in a tuple, struct, or slice. For example, `Point3D { x: 0, .. }` matches any `Point3D` with x = 0, regardless of the values of `y` and `z`. |
| Or Patterns | The character `\|` between subpatterns match if any one of the subpatterns matches. `1 \| 2 \| 3` matches 1, 2, or 3. All patterns in an "or" must bind variables of the same name and type, if they do. |
| Match Guards (`if` conditions) | Match guards add an additional conditional check to a pattern arm. The pattern must match, and the guard's `if` condition must be true for the arm to be chosen. |
| `@` Bindings to Subpatterns | `@` allow you to bind a variable name to a value while also testing that value against a further pattern. The syntax is `variable_name @ pattern`, e.g., `id_val @ 3..=7`. |
| `ref` and `ref mut` Bindings | `ref` or `ref mut` create references (`&T` or `&mut T`) to (parts of) the matched value, rather than moving or copying them. This is useful when you want to borrow parts of a value. Note that Rust's "match ergonomics" often make explicit `ref` and `ref mut` unnecessary when matching on references. |

### Match against Literals {#match-literals}

You can match against literal values like `1`, "Rust", `true`, `(0, 0)`, `[1, 2]`, or `Some(42)` directly:

```rust,editable
{{#include ../../crates/language/examples/match/literals.rs:example}}
```

### Match against a Range {#match-ranges}

Ranges match a value between two numbers either inclusively (`start..=end`) or exclusively at the upper bound (`start_included..end_not_included`). Ranges work for both `char` and integer types:

```rust,editable
{{#include ../../crates/language/examples/match/ranges.rs:example}}
```

### Bind to Variables {#variable-bindings}

Patterns can bind parts of a matched value to new variables:

```rust,editable
{{#include ../../crates/language/examples/match/variable_binding.rs:example}}
```

### Destructure {#skip}

Patterns can destructure structs, enums, tuples, and references.

#### Destructure Structs {#destructure-structs}

The pattern to destructure a struct is similar to the `struct` assignment syntax:

```rust,editable
{{#include ../../crates/language/examples/match/destructure_struct.rs:example}}
```

#### Destructure Enums {#destructure-enums}

You can destructure enums variant by variant and bind their fields, if any, to variables:

```rust,editable
{{#include ../../crates/language/examples/match/destructure_enum.rs:example}}
```

#### Destructure Tuples {#destructure-tuples}

Tuples can be destructured into their components:

```rust,editable
{{#include ../../crates/language/examples/match/destructure_tuple.rs:example}}
```

#### Destructure References {#destructure-references}

References can be destructured as well:

```rust,editable
{{#include ../../crates/language/examples/match/destructure_reference.rs:example}}
```

### Ignore Values {#ignore-values}

- `_` ignores a single value.
- A variable starting with `_` e.g., `_name`, binds the value but silences unused variable warnings.
- `..` ignores remaining parts of a struct, tuple, or slice.

```rust,editable
{{#include ../../crates/language/examples/match/ignore_values.rs:example}}
```

### Use an Or (`|`) to Match Multiple Patterns {#or-patterns}

A sequence of patterns separated by `|` match if any of the sub-patterns match. All sub-patterns must bind variables of the same name and type:

```rust,editable
{{#include ../../crates/language/examples/match/or_patterns.rs:example}}
```

### Bind a Variable with `@` Bindings (Bind to Subpatterns) {#at-bindings}

`variable @ pattern` binds a variable name to a value while also testing that value against a further pattern:

```rust,editable
{{#include ../../crates/language/examples/match/at_bindings.rs:example}}
```

### Match Guards (`if condition`) {#match-guards}

Match guards add an additional conditional check to a pattern:

```rust,editable
{{#include ../../crates/language/examples/match/match_guards.rs:example}}
```

### Create References While Binding with `ref` and `ref mut` {#ref-and-ref-mut-bindings}

Variable binding moves or copies a matched value into the new variable. `ref` (or `ref mut`) create (mutable) references instead:

```rust,editable
{{#include ../../crates/language/examples/match/ref_bindings.rs:example}}
```

Note that Rust's "match ergonomics" often make explicit `ref` and `ref mut` unnecessary when matching on references.

## Related Topics {#related-topics}

- [[control_flow | Control Flow]].
- [[error_handling | Error Handling]].
- [[option | Option]].
- [[result | Result]].
- [[rust-patterns | Rust Patterns]].

## References {#references}

- [Pattern matching][book~rust~pattern-matching]{{hi:Pattern matching}}⮳.
- [Patterns Are Not Expressions](https://h2co3.github.io/pattern)⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
