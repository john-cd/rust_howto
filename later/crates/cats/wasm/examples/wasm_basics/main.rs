// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Assemble a WebAssembly module and inspect what it asks of its host.
//!
//! A `.wasm` file is a portable binary: it starts with the magic bytes
//! `\0asm` and a version number. The module has no operating-system or browser
//! access of its own. It lists the functions it *imports* from the host and the
//! ones it *exports* for the host to call. `wat` turns the text format into
//! binary, and `wasmparser` reads the binary back without running it.

use anyhow::Result;
use wasmparser::ExternalKind;
use wasmparser::Parser;
use wasmparser::Payload;
use wasmparser::TypeRef;

/// A module that imports a logging function and exports `run`.
const MODULE: &str = r#"
(module
  (import "host" "log" (func $log (param i32)))
  (func (export "run") (param i32)
    local.get 0
    call $log))
"#;

/// What a module requires from, and offers to, its host.
#[derive(Debug, Default, PartialEq, Eq)]
struct Interface {
    /// Imported functions as `module.name`, which the host must provide.
    imports: Vec<String>,
    /// Exported functions, which the host can call.
    exports: Vec<String>,
}

/// Reads the imported and exported functions of a binary module.
fn inspect(wasm: &[u8]) -> Result<Interface> {
    let mut interface = Interface::default();
    for payload in Parser::new(0).parse_all(wasm) {
        match payload? {
            Payload::ImportSection(section) => {
                for import in section.into_imports() {
                    let import = import?;
                    if matches!(import.ty, TypeRef::Func(_)) {
                        interface
                            .imports
                            .push(format!("{}.{}", import.module, import.name));
                    }
                }
            }
            Payload::ExportSection(section) => {
                for export in section {
                    let export = export?;
                    if export.kind == ExternalKind::Func {
                        interface.exports.push(export.name.to_owned());
                    }
                }
            }
            _ => {}
        }
    }
    Ok(interface)
}

/// Assembles the module and prints its interface.
fn main() -> Result<()> {
    let wasm = wat::parse_str(MODULE)?;
    assert_eq!(&wasm[..4], b"\0asm");
    assert_eq!(wasm[4..8], [1, 0, 0, 0]);

    let interface = inspect(&wasm)?;
    println!("{interface:#?}");
    assert_eq!(interface.imports, ["host.log"]);
    assert_eq!(interface.exports, ["run"]);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }

    /// Garbage is not a module.
    #[test]
    fn rejects_invalid_binary() {
        assert!(inspect(b"not wasm").is_err());
    }

    /// A module without imports needs nothing from its host.
    #[test]
    fn module_without_imports_is_self_contained() -> Result<()> {
        let wasm = wat::parse_str("(module (func (export \"f\")))")?;
        let interface = inspect(&wasm)?;
        assert!(interface.imports.is_empty());
        assert_eq!(interface.exports, ["f"]);
        Ok(())
    }
}
