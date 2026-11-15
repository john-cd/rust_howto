// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Runs an in-memory WebAssembly module with Wasmer.

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
    let add: TypedFunction<(i32, i32), i32> = instance.exports.get_typed_function(&store, "add")?;
    assert_eq!(add.call(&mut store, 20, 22)?, 42);
    Ok(())
}
// ANCHOR_END: example

/// Runs the Wasmer recipe without files, network, or WASI imports.
/// Runs an in-memory WebAssembly module with Wasmer.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Verifies compilation and the guest's exported function result.
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

    /// Adds two `i32` values with WebAssembly wrapping semantics when the sum overflows.
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

    /// Rejects malformed text module.
    #[test]
    fn malformed_text_module_is_rejected() {
        let store = Store::default();
        assert!(Module::new(&store, "(module (func").is_err());
    }
}
