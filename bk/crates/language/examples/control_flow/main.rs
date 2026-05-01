use clap::Parser;
use clap::Subcommand;

mod for1;
mod if_else;
mod labeled_block_expression;
mod loop1;
mod while1;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "for1")]
    For1,
    #[command(name = "if_else")]
    IfElse,
    #[command(name = "labeled_block_expression")]
    LabeledBlockExpression,
    #[command(name = "loop1")]
    Loop1,
    #[command(name = "while1")]
    While1,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::For1 => {
                for1::run();
            }
            Commands::IfElse => {
                if_else::run();
            }
            Commands::LabeledBlockExpression => {
                labeled_block_expression::run();
            }
            Commands::Loop1 => {
                loop1::run();
            }
            Commands::While1 => {
                while1::run();
            }
        }
    }
}
