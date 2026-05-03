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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::QuickXml => {
                quick_xml::run()?;
            }
            Commands::Roxmltree => {
                roxmltree::run()?;
            }
            Commands::Xml => {
                xml::run()?;
            }
            Commands::Xml5ever => {
                xml5ever::run();
            }
            Commands::Xmlparser => {
                xmlparser::run();
            }
        }
    }
    Ok(())
}
