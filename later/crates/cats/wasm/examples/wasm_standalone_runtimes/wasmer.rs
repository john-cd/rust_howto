//! Runs an in-memory WebAssembly module with Wasmer.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use wasmer::Instance;
use wasmer::Module;
use wasmer::Store;
use wasmer::TypedFunction;
use wasmer::imports;

/// Compiles inline WAT and checks an exported addition function.
fn main() -> anyhow::Result<()> {
    let mut store = Store::default();
    let module = Module::new(
        &store,
        r#"(module
        (func (export "add") (param i32 i32) (result i32)
            local.get 0 local.get 1 i32.add))"#,
    )?;
    let instance = Instance::new(&mut store, &module, &imports! {})?;
    let add: TypedFunction<(i32, i32), i32> =
        instance.exports.get_typed_function(&store, "add")?;
    assert_eq!(add.call(&mut store, 20, 22)?, 42);
    Ok(())
}
// ANCHOR_END: example

/// Runs the Wasmer recipe without files, network, or WASI imports.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    /// Verifies compilation and the guest's exported function result.
    #[test]
    fn inline_module_adds_two_numbers() -> anyhow::Result<()> {
        super::main()
    }

    use wasmer::Instance;
    use wasmer::Module;
    use wasmer::Store;
    use wasmer::TypedFunction;
    use wasmer::imports;

    #[test]
    fn addition_wraps_on_overflow() -> anyhow::Result<()> {
        let mut store = Store::default();
        let module = Module::new(
            &store,
            r#"(module
            (func (export "add") (param i32 i32) (result i32)
                local.get 0 local.get 1 i32.add))"#,
        )?;
        let instance = Instance::new(&mut store, &module, &imports! {})?;
        let add: TypedFunction<(i32, i32), i32> =
            instance.exports.get_typed_function(&store, "add")?;
        assert_eq!(add.call(&mut store, i32::MAX, 1)?, i32::MIN);
        Ok(())
    }

    #[test]
    fn malformed_text_module_is_rejected() {
        let store = Store::default();
        assert!(Module::new(&store, "(module (func").is_err());
    }
}
