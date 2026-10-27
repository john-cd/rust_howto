// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Assembles WebAssembly text (`wat`) into a binary module, lists its exports
//! with `wasmparser` and prints it back as text with `wasmprinter`.

use anyhow::Result;
use wasmparser::Parser;
use wasmparser::Payload;

/// Returns the names of the functions a module exports.
fn exported_functions(binary: &[u8]) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for payload in Parser::new(0).parse_all(binary) {
        if let Payload::ExportSection(reader) = payload? {
            for export in reader {
                let export = export?;
                if matches!(export.kind, wasmparser::ExternalKind::Func) {
                    names.push(export.name.to_owned());
                }
            }
        }
    }
    Ok(names)
}

fn main() -> Result<()> {
    // Text format to binary format.
    let binary = wat::parse_str(
        r#"(module
            (func $add (export "add") (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add)
            (func (export "answer") (result i32)
                i32.const 42))"#,
    )?;
    // Every module starts with the magic bytes `\0asm`.
    assert_eq!(&binary[..4], b"\0asm");

    // Binary validation and inspection.
    assert!(wasmparser::validate(&binary).is_ok());
    let exports = exported_functions(&binary)?;
    println!("exports: {exports:?}");
    assert_eq!(exports, ["add", "answer"]);

    // Binary format back to text.
    let text = wasmprinter::print_bytes(&binary)?;
    println!("{text}");
    assert!(text.contains("i32.add"));

    // Malformed input is reported as an error, not a panic.
    assert!(wat::parse_str("(module (oops))").is_err());
    Ok(())
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the wasm roundtrip example through its entry point.
    #[test]
    fn test_wasm_roundtrip_example_entry_point() -> Result<()> {
        main()
    }

    #[test]
    fn invalid_module_fails_validation() -> Result<()> {
        // A function that promises an `i32` but leaves the stack empty
        // parses fine but is rejected by validation.
        let binary = wat::parse_str("(module (func (result i32)))")?;
        assert!(wasmparser::validate(&binary).is_err());
        Ok(())
    }
}
