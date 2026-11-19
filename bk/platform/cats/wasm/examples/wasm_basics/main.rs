//! Runnable entry point for the wasm basics example.
/// Provides inspect wasm module behavior for the wasm basics component.
mod inspect_wasm_module;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    inspect_wasm_module::run()
}
