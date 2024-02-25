# Copy-on-Write

{{#include cow.incl.md }}

## Convert `Cow` to `&str`

[![std][std-badge]][std]

Use `Borrow`:

```rust,ignore
use std::borrow::Borrow;
my_string.push_str(example.borrow());
```

Use `AsRef`:

```rust,ignore
my_string.push_str(example.as_ref());
```

Use `Deref` explicitly:

```rust,ignore
use std::ops::Deref;
my_string.push_str(example.deref());
```

Use `Deref` implicitly through a coercion:

```rust,ignore
my_string.push_str(&example);
```

## Convert `Cow` to `String`

[![std][std-badge]][std]

Use `ToString`:

```rust,ignore
example.to_string();
```

Use `IntoOwned`:

```rust,ignore
Use Cow::into_owned:
example.into_owned();
```

Use any method to get a reference and then call `to_owned`:

```rust,ignore
example.as_ref().to_owned();
```

Adapted from this [StackOverflow discussion](https://stackoverflow.com/questions/47147844/how-do-i-get-a-str-or-string-from-stdborrowcowstr)

{{#include ../refs/link-refs.md}}
