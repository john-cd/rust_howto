#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to avoid discarding errors during error
//! conversions using the `anyhow` crate.
//!
//! The `?` operator automatically converts different error types (e.g.,
//! [`std::io::Error`], [`std::num::ParseIntError`]) into [`anyhow::Error`],
//! retaining the full error information along the call chain.

/// Parses a string as a `u32`, propagating any [`std::num::ParseIntError`]
/// via `?`.
fn parse_value(s: &str) -> anyhow::Result<u32> {
    let n = s.trim().parse::<u32>()?;
    Ok(n)
}

/// Reads a number from a file.
/// Both [`std::io::Error`] (from file reading) and
/// [`std::num::ParseIntError`] (from parsing) are automatically converted
/// to [`anyhow::Error`] by `?`, so no error information is lost.
fn read_number_from_file(path: &str) -> anyhow::Result<u32> {
    let content = std::fs::read_to_string(path)?;
    let n = parse_value(&content)?;
    Ok(n)
}

// The `main` function may return a `Result` itself.
// We return `anyhow::Result<()>` but could also use
// `Result<(), Box<dyn Error>>` as the return type.
fn main() -> anyhow::Result<()> {
    let n = read_number_from_file("temp/number.txt")?;
    println!("The number is: {n}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use std::io::Write;

        std::fs::create_dir_all("temp").unwrap();
        let mut f = std::fs::File::create("temp/number.txt").unwrap();
        writeln!(f, "42").unwrap();

        main().unwrap();
    }
}
