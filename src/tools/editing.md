# Editing

## IDEs

[VS Code][vs-code]⮳

[Rust plugin for VS Code][rust-analyzer]⮳

[rust-analyzer (home page)][rust-analyzer-website]⮳

## IntelliJ Rust

If you don’t have a JetBrains license, IntelliJ IDEA is available for free and supports IntelliJ Rust.

If you have a JetBrains license, CLion is your go-to editor for Rust in JetBrains’ IDE suite.

## Formatting and Linting

### Rustfmt

[rustfmt][rustfmt-github]⮳

Install with `rustup component add rustfmt`

```sh
rustfmt <filename e.g. lib.rs> <main.rs> ...

# or for the whole project
cargo fmt
```

Using `--check` instructs `rustfmt` to exit with an error code if the input is not formatted correctly (useful for CI).

```sh
cargo fmt --all -- --check
```

#### Configuration

[Configuring Rustfmt][rustfmt-config]⮳

Create a `rustfmt.toml` in the project root folder.
For example,

```toml
edition = "2021"
version = "Two"
unstable_features = true

newline_style = "Unix"
#max_width = 100 # default: 100
use_small_heuristics = "Max"
format_code_in_doc_comments = true
indent_style = "Visual"

# Imports
imports_granularity = "Item"  # or "Crate" or "Module"
imports_layout = "Vertical"
group_imports = "StdExternalCrate"

# Comments
comment_width = 100
wrap_comments = true
normalize_comments = true
normalize_doc_attributes = true

# Functions
fn_params_layout = "Compressed"

# Impl
reorder_impl_items = true

# Structs
use_field_init_shorthand = true

# Macros
use_try_shorthand = true
```

List config options with

```sh
rustfmt --help=config
```

#### Formatting attributes

For things you do not want rustfmt to mangle, use `#[rustfmt::skip]` , `#[rustfmt::skip::macros(macro_name)]` , or `#![rustfmt::skip::attributes(custom_attribute)]`

{{#include ../refs/link-refs.md}}
