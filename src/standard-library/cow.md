# Copy-on-Write

{{#include cow.incl.md}}

[![std][c-std-badge]][c-std]

The type [`std::borrow::Cow`][c-std::borrow::Cow]{{hi:Cow}} is a smart pointer providing clone-on-write{{hi:Clone-on-write}} functionality.

## Convert `Cow` to `&str` {#convert-cow-to-str}

Use [`std::borrow::Borrow`][c-std::borrow::Borrow]{{hi:std::borrow::Borrow}}⮳:

```rust,editable
{{#include ../../deps/tests/standard-library/cow1.rs:example}}
```

Use [`std::convert::AsRef`][c-std::convert::AsRef]{{hi:std::convert::AsRef}}⮳:

```rust,editable
{{#include ../../deps/tests/standard-library/cow2.rs:example}}
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ explicitly:

```rust,editable
{{#include ../../deps/tests/standard-library/cow3.rs:example}}
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ implicitly through a coercion{{hi:Coercion}}:

```rust,editable
{{#include ../../deps/tests/standard-library/cow4.rs:example}}
```

## Convert `Cow` to `String` {#convert-cow-to-string}

[![std][c-std-badge]][c-std]{{hi:std}}

Use [`std::string::ToString`][c-std::string::ToString]{{hi:std::string::ToString}}⮳:

```rust,editable
{{#include ../../deps/tests/standard-library/cow5.rs:example}}
```

Use [`std::borrow::Cow::into_owned`][c-std::borrow::Cow::into_owned]{{hi:std::borrow::Cow::into_owned}}⮳:

```rust,editable
{{#include ../../deps/tests/standard-library/cow6.rs:example}}
```

Use any method to get a reference and then call [`std::borrow::ToOwned`][c-std::borrow::ToOwned]{{hi:std::borrow::ToOwned}}⮳:

```rust,editable
{{#include ../../deps/tests/standard-library/cow7.rs:example}}
```

These examples were adapted from a [StackOverflow discussion][stackoverflow-how-do-i-get-a-str-or-string-from-stdborrowcowstr]⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: add more
</div>
