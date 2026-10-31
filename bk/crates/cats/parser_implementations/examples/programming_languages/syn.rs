// ANCHOR: example
//! Parses Rust source text into an abstract syntax tree with `syn`.
//!
//! An abstract syntax tree (AST) represents the structure of source code as a
//! tree of items, expressions and so on, without the whitespace. Tools such as
//! compilers, linters and formatters work on it. `syn` parses Rust.

use syn::Item;

/// Returns the names of top-level functions, structs, and enums in Rust code.
fn parse_item_names(source: &str) -> anyhow::Result<Vec<String>> {
    let file = syn::parse_file(source)?;
    let names = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Fn(item) => Some(item.sig.ident.to_string()),
            Item::Struct(item) => Some(item.ident.to_string()),
            Item::Enum(item) => Some(item.ident.to_string()),
            _ => None,
        })
        .collect();

    Ok(names)
}

/// Parses sample Rust declarations and prints their names.
fn main() -> anyhow::Result<()> {
    let source = "fn greet() {} struct Person; enum State { Ready }";
    for name in parse_item_names(source)? {
        println!("{name}");
    }
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies top-level Rust declaration names are collected.
    #[test]
    fn parse_item_names_returns_named_declarations() -> anyhow::Result<()> {
        assert_eq!(
            parse_item_names(
                "fn greet() {} struct Person; enum State { Ready }"
            )?,
            ["greet", "Person", "State"]
        );
        Ok(())
    }

    /// Verifies invalid Rust source returns a parse error.
    #[test]
    fn parse_item_names_rejects_invalid_source() {
        assert!(parse_item_names("fn broken(").is_err());
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
