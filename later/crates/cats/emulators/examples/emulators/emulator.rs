#![allow(dead_code)]
// ANCHOR: example
//! A simple PolkaVM host example.
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
use std::env;
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

type ExampleResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Default)]
struct HostState {
    counter: u32,
}

fn main() -> ExampleResult<()> {
    let guest_path = match guest_path_from_args() {
        Some(path) => path,
        None => {
            print_usage();
            return Ok(());
        }
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
        .map(|export| String::from_utf8_lossy(export.symbol()).into_owned())
        .collect();

    println!("Loaded guest blob: {}", guest_path.display());
    println!(
        "Module word size: {}-bit; exports discovered: {}",
        if module.is_64_bit() { 64 } else { 32 },
        export_names.len()
    );
    println!("Exports: {:?}", export_names);

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

fn guest_path_from_args() -> Option<PathBuf> {
    env::args_os().nth(1).map(PathBuf::from)
}

fn print_usage() {
    eprintln!(
        "Usage: cargo run --example emulators -- <path-to-guest.polkavm>"
    );
    eprintln!(
        "The guest should import `host_increment` and export `add_one_with_host`."
    );
}

#[allow(clippy::unnecessary_wraps)]
fn _guest_contract(_path: &Path) -> ExampleResult<()> {
    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main();
}

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
