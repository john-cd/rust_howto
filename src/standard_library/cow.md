# Copy-on-Write

{{#include cow.incl.md }}

## Convert `Cow` to `&str`

[![std][c-std-badge]][c-std]

Use {{hi:Borrow}}[`Borrow`][c-std::borrow::Borrow]⮳:

```rust,ignore
use std::borrow::Borrow;
my_string.push_str(example.borrow());
```

Use {{hi:AsRef}}[`AsRef`][c-std::convert::AsRef]⮳:

```rust,ignore
my_string.push_str(example.as_ref());
```

Use {{hi:Deref}}[`Deref`][c-std::ops::Deref]⮳ explicitly:

```rust,ignore
use std::ops::Deref;
my_string.push_str(example.deref());
```

Use {{hi:Deref}}[`Deref`][c-std::ops::Deref]⮳ implicitly through a {{i:coercion}}:

```rust,ignore
my_string.push_str(&example);
```

## Convert `Cow` to `String`

[![std][c-std-badge]][c-std]

Use {{hi:ToString}}[`ToString`][c-std::string::ToString]⮳:

```rust,ignore
example.to_string();
```

Use {{hi:IntoOwned}}[`IntoOwned`][c-std::borrow::Cow::into_owned]⮳:

```rust,ignore
Use Cow::into_owned:
example.into_owned();
```

Use any method to get a reference and then call {{hi:to_owned}}[`to_owned`][c-std::borrow::ToOwned]⮳:

```rust,ignore
example.as_ref().to_owned();
```

Adapted from this [StackOverflow discussion](https://stackoverflow.com/questions/47147844/how-do-i-get-a-str-or-string-from-stdborrowcowstr)⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
