//! Calls a PolkaVM guest that invokes a typed Rust callback.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::convert::Infallible;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

use polkavm::BackendKind;
use polkavm::Caller;
use polkavm::Config;
use polkavm::Engine;
use polkavm::GasMeteringKind;
use polkavm::Linker;
use polkavm::Module;
use polkavm::ModuleConfig;
use polkavm::ProgramBlob;
use polkavm_common::program::InstructionSetKind;
use polkavm_common::program::Reg;
use polkavm_common::program::asm;
use polkavm_common::writer::ProgramBlobBuilder;

type ExampleResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
const DEMO_GAS: polkavm::Gas = 1_000;
const EXPORT: &str = "add_one_with_host";

/// Builds a 32-bit guest that calls its first import and returns through RA.
///
/// The ABI passes the argument and the result in A0. The jump table is empty
/// because the guest only returns to the host.
fn demo_guest() -> ExampleResult<Vec<u8>> {
    let mut guest = ProgramBlobBuilder::new(InstructionSetKind::Latest32);
    guest.add_import(b"host_increment");
    guest.add_export_by_basic_block(0, EXPORT.as_bytes());
    guest.set_code(&[asm::ecalli(0), asm::jump_indirect(Reg::RA, 0)], &[]);
    guest
        .into_vec()
        .map_err(|error| io::Error::other(error).into())
}

/// Links and calls the guest with 41. Returns its result and the callback count.
///
/// Fails if the blob, import, export or execution is invalid.
fn call_guest(bytes: Vec<u8>, gas: polkavm::Gas) -> ExampleResult<(u32, u32)> {
    let blob = ProgramBlob::parse(bytes.into())?;
    let mut config = Config::default();
    config.set_backend(Some(BackendKind::Interpreter));
    let engine = Engine::new(&config)?;

    let mut module_config = ModuleConfig::default();
    module_config.set_gas_metering(Some(GasMeteringKind::Sync));
    let module = Module::from_blob(&engine, &module_config, blob)?;

    let mut linker: Linker<u32, Infallible> = Linker::new();
    linker.define_typed("host_increment", |caller: Caller<'_, u32>, value: u32| {
        *caller.user_data = (*caller.user_data).saturating_add(1);
        value.wrapping_add(1)
    })?;

    let mut callback_count = 0;
    let mut instance = linker.instantiate_pre(&module)?.instantiate()?;
    instance.set_gas(gas);
    let result = instance
        .call_typed_and_get_result::<u32, (u32,)>(&mut callback_count, EXPORT, (41,))
        .map_err(|error| io::Error::other(format!("guest call failed: {error:?}")))?;
    Ok((result, callback_count))
}

/// Runs the in-memory demonstration or reads a compatible guest from disk.
///
/// Returns an error if the optional path cannot be read or the guest fails.
pub fn run(path: Option<&Path>) -> ExampleResult<()> {
    let bytes = match path {
        Some(path) => fs::read(path)?,
        None => demo_guest()?,
    };
    let (result, callback_count) = call_guest(bytes, DEMO_GAS)?;
    println!("Guest returned {result}; host callback calls: {callback_count}.");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the generated guest through the loader, linker and callback.
    #[test]
    fn guest_calls_the_host_once() -> ExampleResult<()> {
        assert_eq!(call_guest(demo_guest()?, DEMO_GAS)?, (42, 1));
        Ok(())
    }

    /// A guest with no gas fails before the first host callback.
    #[test]
    fn exhausted_budget_is_an_error() -> ExampleResult<()> {
        let error = call_guest(demo_guest()?, 0).expect_err("zero gas must prevent execution");
        assert!(error.to_string().contains("NotEnoughGas"));
        Ok(())
    }

    /// Invalid bytes fail before guest execution.
    #[test]
    fn invalid_blob_is_rejected() {
        assert!(call_guest(b"not a guest".to_vec(), DEMO_GAS).is_err());
    }

    /// The guest must export the function the host calls.
    #[test]
    fn missing_export_is_an_error() -> ExampleResult<()> {
        let mut blob = ProgramBlobBuilder::new(InstructionSetKind::Latest32);
        blob.set_code(&[asm::jump_indirect(Reg::RA, 0)], &[]);
        let bytes = blob.into_vec().map_err(io::Error::other)?;
        assert!(call_guest(bytes, DEMO_GAS).is_err());
        Ok(())
    }
}
