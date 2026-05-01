use clap::Parser;
use clap::Subcommand;

mod lifetime_parameter_function;
mod lifetime_parameter_struct;
mod lifetime_parameters;
mod self_referential_struct;
mod self_referential_struct2;
mod static_lifetime;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "lifetime_parameter_function")]
    LifetimeParameterFunction,
    #[command(name = "lifetime_parameter_struct")]
    LifetimeParameterStruct,
    #[command(name = "lifetime_parameters")]
    LifetimeParameters,
    #[command(name = "self_referential_struct")]
    SelfReferentialStruct,
    #[command(name = "self_referential_struct2")]
    SelfReferentialStruct2,
    #[command(name = "static_lifetime")]
    StaticLifetime,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::LifetimeParameterFunction => {
                let _ = lifetime_parameter_function::run();
            }
            Commands::LifetimeParameterStruct => {
                let _ = lifetime_parameter_struct::run();
            }
            Commands::LifetimeParameters => {
                let _ = lifetime_parameters::run();
            }
            Commands::SelfReferentialStruct => {
                let _ = self_referential_struct::run();
            }
            Commands::SelfReferentialStruct2 => {
                let _ = self_referential_struct2::run();
            }
            Commands::StaticLifetime => {
                let _ = static_lifetime::run();
            }
        }
    }
}
