// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod common_traits;
mod generic_structs;
mod struct_constant;
mod struct_fields;
mod struct_impl;
mod struct_init;
mod struct_trait;
mod struct_update;
mod structs;
mod tuple_structs;

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
    /// Runs the `common_traits` example.
    #[command(name = "common_traits")]
    CommonTraits,
    /// Runs the `generic_structs` example.
    #[command(name = "generic_structs")]
    GenericStructs,
    /// Runs the `struct_constant` example.
    #[command(name = "struct_constant")]
    StructConstant,
    /// Runs the `struct_fields` example.
    #[command(name = "struct_fields")]
    StructFields,
    /// Runs the `struct_impl` example.
    #[command(name = "struct_impl")]
    StructImpl,
    /// Runs the `struct_init` example.
    #[command(name = "struct_init")]
    StructInit,
    /// Runs the `struct_trait` example.
    #[command(name = "struct_trait")]
    StructTrait,
    /// Runs the `struct_update` example.
    #[command(name = "struct_update")]
    StructUpdate,
    /// Runs the `structs` example.
    #[command(name = "structs")]
    Structs,
    /// Runs the `tuple_structs` example.
    #[command(name = "tuple_structs")]
    TupleStructs,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CommonTraits => {
                common_traits::run();
            }
            Commands::GenericStructs => {
                generic_structs::run();
            }
            Commands::StructConstant => {
                struct_constant::run();
            }
            Commands::StructFields => {
                struct_fields::run();
            }
            Commands::StructImpl => {
                struct_impl::run();
            }
            Commands::StructInit => {
                struct_init::run();
            }
            Commands::StructTrait => {
                struct_trait::run();
            }
            Commands::StructUpdate => {
                struct_update::run();
            }
            Commands::Structs => {
                structs::run();
            }
            Commands::TupleStructs => {
                tuple_structs::run();
            }
        }
    }
}
