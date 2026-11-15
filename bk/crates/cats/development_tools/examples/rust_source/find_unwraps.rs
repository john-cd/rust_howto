// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! A project check in the style of `cargo xtask`: find `unwrap()` and
//! `expect()` calls with `syn`, reporting file-relative line numbers.
//!
//! Static analysis reads code without running it. The `syn` crate parses Rust
//! source into a syntax tree that a visitor can walk, for example to find every
//! `unwrap()` call that could panic.

use syn::visit::Visit;

/// A call to `unwrap` or `expect`.
#[derive(Debug, PartialEq)]
struct Finding {
    /// Name of the method call (`unwrap` or `expect`).
    method: String,

    /// Source line number.
    line: usize,
}

#[derive(Default)]
struct UnwrapFinder {
    /// Calls to `unwrap` and `expect` found in the parsed source.
    findings: Vec<Finding>,
}

impl<'ast> Visit<'ast> for UnwrapFinder {
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        let method = call.method.to_string();
        if method == "unwrap" || method == "expect" {
            // Needs proc-macro2's `span-locations` feature.
            let line = call.method.span().start().line;
            self.findings.push(Finding { method, line });
        }
        // Visit the receiver and arguments: `a.unwrap().b.unwrap()`.
        syn::visit::visit_expr_method_call(self, call);
    }
}

/// Returns every `unwrap`/`expect` call in `source`.
fn find_unwraps(source: &str) -> syn::Result<Vec<Finding>> {
    let file = syn::parse_file(source)?;
    let mut finder = UnwrapFinder::default();
    finder.visit_file(&file);
    Ok(finder.findings)
}

/// Parses sample Rust source and reports calls to `unwrap` and `expect`.
///
/// # Errors
///
/// Returns an error if the embedded Rust source cannot be parsed.
///
/// # Panics
///
/// Panics if the sample does not produce the expected calls and line numbers.
fn main() -> syn::Result<()> {
    let source = "\
/// # Panics
/// Panics if an internal invariant is violated.
fn main() {
    let n: u32 = \"1\".parse().unwrap();
    let m = Some(n).expect(\"present\");
    let _ = m;
}
";
    let findings = find_unwraps(source)?;
    for f in &findings {
        println!("line {}: `{}`", f.line, f.method);
    }
    // A real check would exit with a non-zero status when findings exist.
    assert_eq!(
        findings,
        vec![
            Finding {
                method: "unwrap".into(),
                line: 2
            },
            Finding {
                method: "expect".into(),
                line: 3
            },
        ]
    );
    Ok(())
}
// ANCHOR_END: example

/// Finds `unwrap` and `expect` calls in sample Rust source and reports their line numbers.
///
/// # Errors
///
/// Returns an error if the embedded Rust source cannot be parsed.
///
/// # Panics
///
/// Panics if the sample does not produce the expected calls and line numbers.
pub fn run() -> syn::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the find unwraps example through its entry point.
    #[test]
    fn test_find_unwraps_example_entry_point() -> syn::Result<()> {
        main()
    }
}
