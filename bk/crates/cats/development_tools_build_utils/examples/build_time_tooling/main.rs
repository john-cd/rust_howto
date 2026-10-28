//! Runs one of the `build_time_tooling` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod c_library;
mod cc_bundled_cpp;
mod cc_bundled_cpp1;
mod cc_bundled_static;
mod cc_bundled_static1;
mod cc_defines;
mod cc_defines1;
mod cpp_library;
mod out_dir_codegen;

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
    /// Runs the `cc_bundled_cpp` example.
    #[command(name = "cc_bundled_cpp")]
    CcBundledCpp,
    /// Runs the `cc_bundled_cpp1` example.
    #[command(name = "cc_bundled_cpp1")]
    CcBundledCpp1,
    /// Runs the `cc_bundled_static` example.
    #[command(name = "cc_bundled_static")]
    CcBundledStatic,
    /// Runs the `cc_bundled_static1` example.
    #[command(name = "cc_bundled_static1")]
    CcBundledStatic1,
    /// Runs the `cc_defines` example.
    #[command(name = "cc_defines")]
    CcDefines,
    /// Runs the `cc_defines1` example.
    #[command(name = "cc_defines1")]
    CcDefines1,
    /// Runs the bundled C library example.
    #[command(name = "c_library")]
    CLibrary,
    /// Runs the bundled C++ library example.
    #[command(name = "cpp_library")]
    CppLibrary,
    /// Runs the generated Rust source example.
    #[command(name = "out_dir_codegen")]
    OutDirCodegen,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CcBundledCpp => {
                cc_bundled_cpp::run();
            }
            Commands::CcBundledCpp1 => {
                cc_bundled_cpp1::run();
            }
            Commands::CcBundledStatic => {
                cc_bundled_static::run()?;
            }
            Commands::CcBundledStatic1 => {
                cc_bundled_static1::run()?;
            }
            Commands::CcDefines => {
                cc_defines::run()?;
            }
            Commands::CcDefines1 => {
                cc_defines1::run()?;
            }
            Commands::CLibrary => {
                c_library::run();
            }
            Commands::CppLibrary => {
                cpp_library::run();
            }
            Commands::OutDirCodegen => {
                out_dir_codegen::run();
            }
        }
    }

    Ok(())
}
