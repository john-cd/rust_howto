// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A simple `PolkaVM` host example.
//!
//! This example loads a compiled `.polkavm` guest blob from the command line,
//! registers a typed host callback, instantiates the guest module, and invokes
//! an exported guest function.
//!
//! Run it with:
//!
//! ```
//! cargo run --example emulators -- <path-to-guest.polkavm>
//! ```
//!
//! The guest should import `host_increment` and export `add_one_with_host`.

use std::convert::Infallible;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use polkavm::BackendKind;
use polkavm::Caller;
use polkavm::Config;
use polkavm::Engine;
use polkavm::Linker;
use polkavm::Module;
use polkavm::ProgramBlob;

/// The result of the functions of the example.
type ExampleResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// The state of the host, shared with the guest program.
#[derive(Debug, Default)]
struct HostState {
    /// The number of calls to the host function.
    counter: u32,
}

/// Runs the emulator example.
fn main() -> ExampleResult<()> {
    run_with_args(std::env::args_os().skip(1))
}

/// Runs the example with command-line arguments supplied by the caller.
fn run_with_args(
    mut args: impl Iterator<Item = std::ffi::OsString>,
) -> ExampleResult<()> {
    run_guest(args.next().map(PathBuf::from))
}

/// Loads the guest program from `guest_path`, links a host function and calls
/// the guest function. Prints the usage if no path is given.
fn run_guest(guest_path: Option<PathBuf>) -> ExampleResult<()> {
    let guest_path = if let Some(path) = guest_path {
        path
    } else {
        print_usage();
        return Ok(());
    };

    let raw_blob = fs::read(&guest_path)?;
    let blob = ProgramBlob::parse(raw_blob.into())?;

    let mut config = Config::default();
    config.set_backend(Some(BackendKind::Interpreter));

    let engine = Engine::new(&config)?;
    let module = Module::from_blob(&engine, &Default::default(), blob)?;

    let mut linker: Linker<HostState, Infallible> = Linker::new();
    linker.define_typed(
        "host_increment",
        |caller: Caller<'_, HostState>, value: u32| -> u32 {
            let state = caller.user_data;
            state.counter += value;
            state.counter
        },
    )?;

    let instance_pre = linker.instantiate_pre(&module)?;

    let export_names: Vec<_> = module
        .exports()
        .map(|export| {
            String::from_utf8_lossy(export.symbol().as_bytes()).into_owned()
        })
        .collect();

    println!("Loaded guest blob: {}", guest_path.display());
    println!(
        "Module word size: {}-bit; exports discovered: {}",
        if module.is_64_bit() { 64 } else { 32 },
        export_names.len()
    );
    println!("Exports: {export_names:?}");

    /// Name of the WebAssembly function exported by the guest module.
    const EXPECTED_EXPORT: &str = "add_one_with_host";
    if !export_names.iter().any(|name| name == EXPECTED_EXPORT) {
        println!(
            "The guest linked successfully, but it does not export `{EXPECTED_EXPORT}`."
        );
        println!(
            "Use a guest that imports `host_increment` and exports `{EXPECTED_EXPORT}`."
        );
        return Ok(());
    }

    let mut state = HostState::default();
    let mut instance = instance_pre.instantiate()?;
    let result = instance
        .call_typed_and_get_result::<u32, (u32,)>(
            &mut state,
            EXPECTED_EXPORT,
            (41,),
        )
        .map_err(|error| {
            io::Error::other(format!("guest call failed: {error:?}"))
        })?;

    println!(
        "Guest returned {result}; host counter is {}.",
        state.counter
    );
    Ok(())
}

/// Prints how to run the example.
fn print_usage() {
    eprintln!(
        "Usage: cargo run --example emulators -- <path-to-guest.polkavm>"
    );
    eprintln!(
        "The guest should import `host_increment` and export `add_one_with_host`."
    );
}

/// Documents the contract that the guest program must fulfill.
#[allow(clippy::unnecessary_wraps)]
fn _guest_contract(_path: &Path) -> ExampleResult<()> {
    Ok(())
}
// ANCHOR_END: example

/// Runs the optional PolkaVM guest supplied on the command line.
pub fn run() -> ExampleResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that no guest path prints usage.
    #[test]
    fn no_guest_path_prints_usage() {
        run_guest(None).expect("the emulator example invariant should hold");
    }

    /// Verifies that host runtime builds.
    #[test]
    fn test_host_runtime_builds() {
        let mut config = Config::default();
        config.set_backend(Some(BackendKind::Interpreter));

        let _engine = Engine::new(&config).expect("engine should initialize");

        let mut linker: Linker<HostState, Infallible> = Linker::new();
        linker
            .define_typed(
                "host_increment",
                |caller: Caller<'_, HostState>, value: u32| -> u32 {
                    let state = caller.user_data;
                    state.counter += value;
                    state.counter
                },
            )
            .expect("host callback should register");
    }

    /// Runs the entry point without a guest path, which prints the usage.
    #[test]
    fn test_main_entry_point() -> ExampleResult<()> {
        run_with_args(std::iter::empty())
    }

    /// Verifies that a missing guest path follows the CLI usage path.
    /// Verifies that a nonexistent guest blob returns an I/O error.
    #[test]
    fn missing_guest_file_is_an_error() {
        let path =
            std::env::temp_dir().join("rust_howto_no_such_guest.polkavm");
        assert!(run_guest(Some(path)).is_err());
    }

    /// Verifies that an invalid guest blob returns a parse error.
    #[test]
    fn invalid_guest_blob_is_an_error() {
        let path =
            std::env::temp_dir().join("rust_howto_invalid_guest.polkavm");
        fs::write(&path, b"not a polkavm blob").expect("temp dir is writable");
        let result = run_guest(Some(path.clone()));
        fs::remove_file(&path).expect("the file was just created");
        assert!(result.is_err());
    }
}
