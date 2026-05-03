use clap::Parser;
use clap::Subcommand;

mod option_combinators;
mod option_match;
mod option_question_mark;
mod option_ref;
mod option_unwrap;
mod options1;
mod options2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "option_combinators")]
    OptionCombinators,
    #[command(name = "option_match")]
    OptionMatch,
    #[command(name = "option_question_mark")]
    OptionQuestionMark,
    #[command(name = "option_ref")]
    OptionRef,
    #[command(name = "option_unwrap")]
    OptionUnwrap,
    #[command(name = "options1")]
    Options1,
    #[command(name = "options2")]
    Options2,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::OptionCombinators => {
                option_combinators::run();
            }
            Commands::OptionMatch => {
                option_match::run();
            }
            Commands::OptionQuestionMark => {
                option_question_mark::run();
            }
            Commands::OptionRef => {
                option_ref::run();
            }
            Commands::OptionUnwrap => {
                option_unwrap::run();
            }
            Commands::Options1 => {
                options1::run();
            }
            Commands::Options2 => {
                options2::run();
            }
        }
    }
}
