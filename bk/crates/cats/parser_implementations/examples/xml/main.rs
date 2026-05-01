use clap::Parser;
use clap::Subcommand;

mod quick_xml;
mod roxmltree;
mod xml;
mod xml5ever;
mod xmlparser;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "quick_xml")]
    QuickXml,
    #[command(name = "roxmltree")]
    Roxmltree,
    #[command(name = "xml")]
    Xml,
    #[command(name = "xml5ever")]
    Xml5ever,
    #[command(name = "xmlparser")]
    Xmlparser,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::QuickXml => {
                let _ = quick_xml::run();
            }
            Commands::Roxmltree => {
                let _ = roxmltree::run();
            }
            Commands::Xml => {
                let _ = xml::run();
            }
            Commands::Xml5ever => {
                let _ = xml5ever::run();
            }
            Commands::Xmlparser => {
                let _ = xmlparser::run();
            }
        }
    }
}
