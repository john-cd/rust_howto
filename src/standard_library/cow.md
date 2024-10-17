# Copy-on-Write

{{#include cow.incl.md }}

The type [`std::borrow::Cow`][c-std::borrow::Cow]  is a smart pointer providing clone-on-write functionality.

## Convert `Cow` to `&str`

Use [`std::borrow::Borrow`][c-std::borrow::Borrow]{{hi:std::borrow::Borrow}}⮳:

```rust, editable
use std::borrow::Borrow;
# let mut my_string = String::new();
let example = std::borrow::Cow::from("example");
my_string.push_str(example.borrow());
# println!("{}", my_string);
```

Use [`std::convert::AsRef`][c-std::convert::AsRef]{{hi:std::convert::AsRef}}⮳:

```rust
# let mut my_string = String::new();
let example = std::borrow::Cow::from("example");
my_string.push_str(example.as_ref());
# println!("{}", my_string);
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ explicitly:

```rust
use std::ops::Deref;
# let mut my_string = String::new();
let example = std::borrow::Cow::from("example");
my_string.push_str(example.deref());
# println!("{}", my_string);
```

Use [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ implicitly through a coercion{{hi:Coercion}}:

```rust
# let mut my_string = String::new();
let example = std::borrow::Cow::from("example");
my_string.push_str(&example);
# println!("{}", my_string);
```

## Convert `Cow` to `String`

[![std][c-std-badge]][c-std]

Use [`std::string::ToString`][c-std::string::ToString]{{hi:std::string::ToString}}⮳:

```rust
let example = std::borrow::Cow::from("example");
println!("{}", example.to_string());
```

Use [`std::borrow::Cow::into_owned`][c-std::borrow::Cow::into_owned]{{hi:std::borrow::Cow::into_owned}}⮳:

```rust
let example = std::borrow::Cow::from("example");
println!("{}", example.into_owned());
```

Use any method to get a reference and then call [`std::borrow::ToOwned`][c-std::borrow::ToOwned]{{hi:std::borrow::ToOwned}}⮳:

```rust
let example = std::borrow::Cow::from("example");
println!("{}", example.as_ref().to_owned());
```

These examples were adapted from a [StackOverflow discussion][stackoverflow-how-do-i-get-a-str-or-string-from-stdborrowcowstr]⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: add more
</div>
