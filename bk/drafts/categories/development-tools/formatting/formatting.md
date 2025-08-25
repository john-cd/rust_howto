# Formatting

{{#include formatting.incl.md}}

## Key Tools {#skip}

| Topic | Rust Crates |
|---|---|
| Code Formatter | [`rustfmt`][c~rustfmt~repo]↗{{hi:rustfmt}} is the standard Rust code formatter. |
| Formatting Configuration | [Configuration][p~configuration] is done through `rustfmt.toml` files. |
| Integration | Most [IDEs][p~ides] have built-in [`rustfmt`][c~rustfmt~repo]↗{{hi:rustfmt}} integration. |

## Format your Rust code with `rustfmt` {#rustfmt}

[![rustfmt~repo][c~rustfmt~repo~badge]][c~rustfmt~repo]{{hi:rustfmt}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

Install [`rustfmt`][c~rustfmt~repo]↗{{hi:rustfmt}} with `rustup component add rustfmt`

```sh
rustfmt <filename e.g. lib.rs> <main.rs> ...

# or for the whole project
cargo fmt
```

Using `--check` instructs [`rustfmt`][c~rustfmt~repo]↗{{hi:rustfmt}}{{hi:rustfmt}} to exit with an error code if the input is not formatted correctly (useful for CI).

```sh
cargo fmt --all -- --check
```

### Configure `rustfmt` {#rustfmt-config}

Create a `rustfmt.toml`{{hi:rustfmt.toml}} in the project root folder.
For example,

```toml
edition = "2021"
style_edition = "2021"
unstable_features = true

newline_style = "Unix"
#max_width = 100 # default: 100
use_small_heuristics = "Max"
format_code_in_doc_comments = true
indent_style = "Visual"

# Imports
imports_granularity = "Item" # or "Crate" or "Module"
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

### Use Attributes to Skip Code Formatting in Your Code {#formatting-attributes}

[![rustfmt~repo][c~rustfmt~repo~badge]][c~rustfmt~repo]{{hi:rustfmt}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

For things [`rustfmt`][c~rustfmt~repo]↗{{hi:rustfmt}} should not mangle, use `#[rustfmt::skip]`{{hi:rustfmt::skip}} , `#[rustfmt::skip::macros(macro_name)]`{{hi:rustfmt::skip::macros(macro_name)}} , or `#![rustfmt::skip::attributes(custom_attribute)]`{{hi:rustfmt::skip::attributes(custom_attribute)}}

## References {#references}

- [The Rust Style Guide][style-guide~website]↗.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[formatting: add link for formatting attributes](https://github.com/john-cd/rust_howto/issues/300)
</div>
