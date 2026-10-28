//! Runs one of the `interfacing_with_javascript` examples, chosen on the
//! command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod js_import;
mod js_sys;
mod promises;
mod serde_wasm_bindgen;
mod web_sys;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the JavaScript import example.
    #[command(name = "js_import")]
    JsImport,
    /// Runs the `js_sys` example.
    #[command(name = "js_sys")]
    JsSys,
    /// Runs the Promise example.
    #[command(name = "promises")]
    Promises,
    /// Runs the Serde conversion example.
    #[command(name = "serde_wasm_bindgen")]
    SerdeWasmBindgen,
    /// Runs the `web_sys` example.
    #[command(name = "web_sys")]
    WebSys,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::JsImport => js_import::run(),
            Commands::JsSys => js_sys::run(),
            Commands::Promises => promises::run(),
            Commands::SerdeWasmBindgen => serde_wasm_bindgen::run(),
            Commands::WebSys => web_sys::run(),
        }
    }
}
