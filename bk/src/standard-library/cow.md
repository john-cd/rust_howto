# Copy-on-Write

{{#include cow.incl.md}}

[![std][c-std-badge]][c-std]

The type [`std::borrow::Cow`][c-std::borrow::Cow]{{hi:Cow}} is a smart pointer providing clone-on-write{{hi:Clone-on-write}} functionality.

## Convert `Cow` to `&str` {#convert-cow-to-str}

`Cow`
`&str`

Use [`std::borrow::Borrow`][c-std::borrow::Borrow]{{hi:std::borrow::Borrow}}⮳:

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow1.rs:example}}
```

Use [`std::convert::AsRef`][c-std::convert::AsRef]{{hi:std::convert::AsRef}}⮳:

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow2.rs:example}}
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ explicitly:

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow3.rs:example}}
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ implicitly through a coercion{{hi:Coercion}}:

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow4.rs:example}}
```

## Convert `Cow` to `String` {#convert-cow-to-string}

[![std][c-std-badge]][c-std]{{hi:std}}

`Cow`

Use [`std::string::ToString`][c-std::string::ToString]{{hi:std::string::ToString}}⮳:

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow5.rs:example}}
```

Use [`std::borrow::Cow::into_owned`][c-std::borrow::Cow::into_owned]{{hi:std::borrow::Cow::into_owned}}⮳:

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow6.rs:example}}
```

Use any method to get a reference and then call [`std::borrow::ToOwned`][c-std::borrow::ToOwned]{{hi:std::borrow::ToOwned}}⮳:

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow7.rs:example}}
```

These examples were adapted from a [StackOverflow discussion][stackoverflow-how-do-i-get-a-str-or-string-from-stdborrowcowstr]⮳.

## Related Data Structures

- [[strings | Strings]].

## See also

- [[lifetimes | Lifetimes]].
- [[memory-management | Memory Management]].
- [[ownership_borrowing | Ownership & Borrowing]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[cow: add more](https://github.com/john-cd/rust_howto/issues/620)
</div>
