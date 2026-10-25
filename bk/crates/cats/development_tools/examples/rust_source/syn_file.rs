// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parses Rust source text with `syn` (outside of a procedural macro) and
//! counts the items it defines with a `Visit` implementation.

use syn::visit::Visit;

/// Counts functions (free functions and methods) and structs.
#[derive(Default)]
struct ItemCounter {
    functions: usize,
    structs: usize,
}

impl<'ast> Visit<'ast> for ItemCounter {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.functions += 1;
        // Keep walking so nested items are counted too.
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.functions += 1;
        syn::visit::visit_impl_item_fn(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.structs += 1;
        syn::visit::visit_item_struct(self, node);
    }
}

/// Parses `source` and returns `(functions, structs)`.
fn count_items(source: &str) -> syn::Result<(usize, usize)> {
    let file = syn::parse_file(source)?;
    let mut counter = ItemCounter::default();
    counter.visit_file(&file);
    Ok((counter.functions, counter.structs))
}

fn main() -> syn::Result<()> {
    let source = "
        struct Point { x: f64, y: f64 }

        impl Point {
            fn norm(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
        }

        fn main() {
            fn helper() {}
            helper();
        }
    ";
    let (functions, structs) = count_items(source)?;
    println!("{functions} functions, {structs} struct");
    assert_eq!((functions, structs), (3, 1));

    // Invalid code is reported as an error, not a panic.
    assert!(count_items("fn broken(").is_err());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> syn::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> syn::Result<()> {
        main()
    }
}
