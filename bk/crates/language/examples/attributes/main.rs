use clap::Parser;
use clap::Subcommand;

mod allow_dead_code;
mod attributes_deprecated;
mod attributes_derive;
mod attributes_early_development;
mod attributes_must_use;
mod attributes_production;
mod cfg_if;
mod conditional_compilation;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "allow_dead_code")]
    AllowDeadCode,
    #[command(name = "attributes_deprecated")]
    AttributesDeprecated,
    #[command(name = "attributes_derive")]
    AttributesDerive,
    #[command(name = "attributes_early_development")]
    AttributesEarlyDevelopment,
    #[command(name = "attributes_must_use")]
    AttributesMustUse,
    #[command(name = "attributes_production")]
    AttributesProduction,
    #[command(name = "cfg_if")]
    CfgIf,
    #[command(name = "conditional_compilation")]
    ConditionalCompilation,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AllowDeadCode => {
                allow_dead_code::run();
            }
            Commands::AttributesDeprecated => {
                attributes_deprecated::run();
            }
            Commands::AttributesDerive => {
                attributes_derive::run();
            }
            Commands::AttributesEarlyDevelopment => {
                attributes_early_development::run();
            }
            Commands::AttributesMustUse => {
                attributes_must_use::run();
            }
            Commands::AttributesProduction => {
                attributes_production::run();
            }
            Commands::CfgIf => {
                cfg_if::run();
            }
            Commands::ConditionalCompilation => {
                conditional_compilation::run();
            }
        }
    }
}
