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
