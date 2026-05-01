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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "common_traits")]
    CommonTraits,
    #[command(name = "generic_structs")]
    GenericStructs,
    #[command(name = "struct_constant")]
    StructConstant,
    #[command(name = "struct_fields")]
    StructFields,
    #[command(name = "struct_impl")]
    StructImpl,
    #[command(name = "struct_init")]
    StructInit,
    #[command(name = "struct_trait")]
    StructTrait,
    #[command(name = "struct_update")]
    StructUpdate,
    #[command(name = "structs")]
    Structs,
    #[command(name = "tuple_structs")]
    TupleStructs,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CommonTraits => {
                let _ = common_traits::run();
            }
            Commands::GenericStructs => {
                let _ = generic_structs::run();
            }
            Commands::StructConstant => {
                let _ = struct_constant::run();
            }
            Commands::StructFields => {
                let _ = struct_fields::run();
            }
            Commands::StructImpl => {
                let _ = struct_impl::run();
            }
            Commands::StructInit => {
                let _ = struct_init::run();
            }
            Commands::StructTrait => {
                let _ = struct_trait::run();
            }
            Commands::StructUpdate => {
                let _ = struct_update::run();
            }
            Commands::Structs => {
                let _ = structs::run();
            }
            Commands::TupleStructs => {
                let _ = tuple_structs::run();
            }
        }
    }
}
