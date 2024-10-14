# Copy-on-Write

{{#include cow.incl.md }}

## Convert `Cow` to `&str`

[![std][c-std-badge]][c-std]

Use [`std::borrow::Borrow`][c-std::borrow::Borrow]{{hi:std::borrow::Borrow}}⮳:

```rust,ignore
use std::borrow::Borrow;
my_string.push_str(example.borrow());
```

Use [`std::convert::AsRef`][c-std::convert::AsRef]{{hi:std::convert::AsRef}}⮳:

```rust,ignore
my_string.push_str(example.as_ref());
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ explicitly:

```rust,ignore
use std::ops::Deref;
my_string.push_str(example.deref());
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ implicitly through a coercion{{hi:Coercion}}:

```rust,ignore
my_string.push_str(&example);
```

## Convert `Cow` to `String`

[![std][c-std-badge]][c-std]

Use [`std::string::ToString`][c-std::string::ToString]{{hi:std::string::ToString}}⮳:

```rust,ignore
example.to_string();
```

Use [`std::borrow::Cow::into_owned`][c-std::borrow::Cow::into_owned]{{hi:std::borrow::Cow::into_owned}}⮳:

```rust,ignore
Use Cow::into_owned:
example.into_owned();
```

Use any method to get a reference and then call [`std::borrow::ToOwned`][c-std::borrow::ToOwned]{{hi:std::borrow::ToOwned}}⮳:

```rust,ignore
example.as_ref().to_owned();
```

Adapted from this [StackOverflow discussion][stackoverflow-how-do-i-get-a-str-or-string-from-stdborrowcowstr]⮳

[stackoverflow-how-do-i-get-a-str-or-string-from-stdborrowcowstr]: https://stackoverflow.com/questions/47147844/how-do-i-get-a-str-or-string-from-stdborrowcowstr
{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO:
</div>
