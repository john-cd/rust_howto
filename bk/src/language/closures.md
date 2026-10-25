<!-- TODO Reviewed ??-??-2026 -->

# Closures

{{#include closures.incl.md}}

## Closure Syntax {#closure}

[![Rust by example - Closures][book~rust-by-example~closures~badge]][book~rust-by-example~closures]

Closures are anonymous functions you can define inline, often right where you need to use them.
Here is an example of a short, inline closure with a simple expression as its body:

```rust,editable
{{#include ../../crates/language/examples/closures/closures.rs:example}}
```

{{i:Closures}} can capture variables{{hi:Capture of variables}} from their surrounding scope:

- by reference: `&T`, where `T` is a type,
- by mutable reference: `&mut T`, or
- by value: `T`.

They preferentially capture variables by reference and only go lower when required.

```rust,editable
{{#include ../../crates/language/examples/closures/closures_capture.rs:example}}
```

## Force Closure Capture by Value {#move-closures}

[![std][c~std~docs~badge]][c~std~docs]

A closure can be forced to capture the values it uses by copying or moving them with the `move` keyword. This is often useful when spawning a thread, because `thread::spawn` requires a `'static` closure. `move` does not by itself make a closure `'static`: any captured references must still point to data that lives for the whole program.

Since Rust 2021, closures can capture only the fields they use from a struct or tuple rather than capturing the whole value.

```rust,editable
{{#include ../../crates/language/examples/closures/closures_move.rs:example}}
```

## Annotate Closure Types {#closure-with-type-annotations}

[![std][c~std~docs~badge]][c~std~docs]

Closures can often infer parameter and return-value types, but you can add them manually:

```rust,editable
{{#include ../../crates/language/examples/closures/closures_with_type_annotations.rs:example}}
```

## Use Closures as Function Arguments {#closures-as-input-parameters}

[![std][c~std~docs~badge]][c~std~docs]

Closures are frequently used as arguments to functions, especially higher-order functions like those found on iterators (`map`, `filter`, `fold`, etc.).

A closure implements the call traits its body supports. The traits form a hierarchy: `Fn` implies `FnMut`, and `FnMut` implies `FnOnce`.

- [`std::ops::Fn`][c~std::ops::Fn~docs]↗{{hi:std::ops::Fn}} if calling it only needs shared access to captures,
- [`std::ops::FnMut`][c~std::ops::FnMut~docs]↗{{hi:std::ops::FnMut}} if calling it may mutate captures, and
- [`std::ops::FnOnce`][c~std::ops::FnOnce~docs]↗{{hi:std::ops::FnOnce}} if calling it may move a captured value out.

Every closure implements `FnOnce`. Whether it also implements `FnMut` or `Fn` depends on what its body does with captured values, not on whether it uses `move`. A `move` closure that only reads its captures can still implement `Fn`.

Additionally, for any type `F` that implements `Fn` / `FnMut` / `FnOnce`, `&F` implements them, too.

When defining a function that accepts a closure, you use the `Fn`, `FnMut`, or `FnOnce` traits as bounds:

```rust,editable
{{#include ../../crates/language/examples/closures/closures_as_input_parameters.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[functions | Functions]].
- [[rust-patterns | Rust Patterns]].
- [[functional_programming | Functional Programming]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

Methods expecting a closure can also accept an enum variant (tuple-like)

https://www.reddit.com/r/rust/comments/1l8okiy/im_blown_that_this_is_a_thing/

Enum variants and tuple-like structs can be used as function items in general https://doc.rust-lang.org/reference/types/function-item.html

[accuracy review: introduction and language basics](https://github.com/john-cd/rust_howto/issues/1887)

</div>
