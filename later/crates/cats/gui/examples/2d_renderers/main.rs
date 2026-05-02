use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "femtovg")]
mod femtovg;
mod minifb;
#[cfg(feature = "skia")]
mod skia_safe;
#[cfg(feature = "vello")]
mod vello;
mod vger;
#[cfg(feature = "webrender")]
mod webrender;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "femtovg")]
    #[command(name = "femtovg")]
    Femtovg,
    #[command(name = "minifb")]
    Minifb,
    #[cfg(feature = "skia")]
    #[command(name = "skia_safe")]
    SkiaSafe,
    #[cfg(feature = "vello")]
    #[command(name = "vello")]
    Vello,
    #[command(name = "vger")]
    Vger,
    #[cfg(feature = "webrender")]
    #[command(name = "webrender")]
    Webrender,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "femtovg")]
            Commands::Femtovg => {
                femtovg::run();
            }
            Commands::Minifb => {
                minifb::run();
            }
            #[cfg(feature = "skia")]
            Commands::SkiaSafe => {
                skia_safe::run();
            }
            #[cfg(feature = "vello")]
            Commands::Vello => {
                vello::run();
            }
            Commands::Vger => {
                vger::run();
            }
            #[cfg(feature = "webrender")]
            Commands::Webrender => {
                webrender::run();
            }
        }
    }
}
