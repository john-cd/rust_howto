# Copy-on-Write

{{#include cow.incl.md }}

The type [`std::borrow::Cow`][c-std::borrow::Cow]  is a smart pointer providing clone-on-write functionality.

## Convert `Cow` to `&str`

Use [`std::borrow::Borrow`][c-std::borrow::Borrow]{{hi:std::borrow::Borrow}}⮳:

```rust
{{#include ../../deps/tests/cow1.rs}}
```

Use [`std::convert::AsRef`][c-std::convert::AsRef]{{hi:std::convert::AsRef}}⮳:

```rust
{{#include ../../deps/tests/cow2.rs}}
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ explicitly:

```rust
{{#include ../../deps/tests/cow3.rs}}
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ implicitly through a coercion{{hi:Coercion}}:

```rust
{{#include ../../deps/tests/cow4.rs}}
```

## Convert `Cow` to `String`

[![std][c-std-badge]][c-std]

Use [`std::string::ToString`][c-std::string::ToString]{{hi:std::string::ToString}}⮳:

```rust
{{#include ../../deps/tests/cow5.rs}}
```

Use [`std::borrow::Cow::into_owned`][c-std::borrow::Cow::into_owned]{{hi:std::borrow::Cow::into_owned}}⮳:

```rust
{{#include ../../deps/tests/cow6.rs}}
```

Use any method to get a reference and then call [`std::borrow::ToOwned`][c-std::borrow::ToOwned]{{hi:std::borrow::ToOwned}}⮳:

```rust
{{#include ../../deps/tests/cow7.rs}}
```

These examples were adapted from a [StackOverflow discussion][stackoverflow-how-do-i-get-a-str-or-string-from-stdborrowcowstr]⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: add more
</div>
