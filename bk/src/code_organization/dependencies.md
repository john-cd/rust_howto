# Dependencies

{{#include dependencies.incl.md}}

## Use Items from External Crates (Dependencies) in your Code {#dependencies}

To use a (public) item (structs, enums, traits, functions, etc.) of an external crate, first make sure to include the external crate as a dependency{{hi:Dependency}} in your crate's [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file, for example:

```toml
[dependencies]
anyhow = "1.0.95"
```

The name of the external crate is followed by `=` and a semantic version specifier. See the [[cargo | Cargo]] chapters for details.

You can then refer to its items using a path that starts with the external crate's name:

```rust,noplayground
// Returns a `Result` from the `anyhow` crate:
fn a_func() -> anyhow::Result<()> {
    Ok(())
}
```

The path may include one or more modules as needed e.g., `<crate_name>::<module_name>::<item_name>`.

You may start the path with `::` followed by the name of the crate, if a crate name is the same than a module in your crate.

Once adding an external crate as a dependency, you will very often bring items from external crates into scope with the `use` keyword. See the [[use_keyword | `use` Keyword]] chapter for details.

```rust,noplayground
// Bring the following items into scope:
use anyhow::Result; // A type alias.
use anyhow::anyhow; // The `anyhow!` macro.
use anyhow::Error;  // A struct.

// Refer to `Result` without having to write its full path every time:
fn main() -> Result<()> {
  // ...
  Ok(())
}
```

## References {#references}

- [Using a Crate to Get More Functionality][book~rust~ch02-00-guessing-game-tutorial.html#using-a-crate-to-get-more-functionality]↗.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
