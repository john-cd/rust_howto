# Dependencies

{{#include dependencies.incl.md}}

## Use External Crates (Dependencies) in your Code {#dependencies}

To use a (public) item (`struct`s, functions, etc.) of an external crate, first make sure to include said external crate as a dependency in your crate's `Cargo.toml` file, for example:

```toml
[dependencies]
anyhow = "1.0.95"
```

You can then refer to its items using a path that starts with the external crate's name:

```rust,noplayground
// Returns a `Result` from the `anyhow` crate:
fn a_func() -> anyhow::Result<()> {
    Ok(())
}
```

The path may include one or more modules as needed e.g., `<crate_name>::<module_name>::<item_name>`.

You may start the path with `::` followed by the name of the crate, if a crate name is the same than a module in your crate.

## Bring Items from External Crates into Scope with the `use` Keyword {#use-keyword-for-dependencies}

Once you add an external crate as a dependency, you will often bring items from external crates into scope with the `use` keyword:

```rust,noplayground
// Bring the following items into scope:
use anyhow::Result; // A type alias.
use anyhow::anyhow; // The `anyhow!` macro.
use anyhow::Error;  // A struct.

// Refer to `Result` without having to write its full path:
fn main() -> Result<()> {
  // ...
  Ok(())
}
```

As with paths to modules within your crate, you may use globs, define aliases, and combine multiple `use` statements:

```rust,noplayground
// Use a glob (the `*`) to bring all public objects within a module (here
// `default`) in scope. Use sparingly.
use std::default::*;
// Use `as` to define aliases, for example in case of name conflict.
use std::io::Result as IoResult;
// You can combine multiple `use` lines together with { }.
use std::{cmp::Ordering, fmt};
// The following is equivalent to `use std::io; use std::io::Write;`.
use std::io::{self, Write};

fn main() -> IoResult { // Use the alias.

    // Use `std::default::Default` without writing down the whole path,
    // because we imported all public objects from the `default` module.
    let _i: i8 = Default::default();

    Ok::<(), io::Error>(()) // Use the `io` module imported above.
}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO review
</div>
