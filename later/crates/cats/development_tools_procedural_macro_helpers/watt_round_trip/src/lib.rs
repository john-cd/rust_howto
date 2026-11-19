//! Exposes the compiled Watt guest as a Rust procedural macro.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(include_bytes!(env!("WATT_DEMO_WASM")));

/// Expands the input using the WebAssembly guest.
#[proc_macro]
pub fn parenthesize(input: TokenStream) -> TokenStream {
    MACRO.proc_macro("parenthesize", input)
}
// ANCHOR_END: example
