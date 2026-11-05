//! Lists the dependencies declared by the book's example crates.
//!
//! Run with `cargo run -p tool_lib --example dependencies`.
//!
//! TODO Reviewed ??-??-2026.

use std::path::Path;

use tool_lib::get_dependencies;

/// Prints the number of distinct dependencies and the first few names.
fn main() -> anyhow::Result<()> {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../xmpl");
    let dependencies = get_dependencies(crates)?;
    println!("{} distinct dependencies", dependencies.len());
    for name in dependencies.iter().take(5) {
        println!("  {name}");
    }
    Ok(())
}
