use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "femtovg")]
mod femtovg;
mod minifb;
#[cfg(feature = "skia")]
mod skia_safe;
#[cfg(all(feature = "vello", not(windows)))]
// TODO review Vello support for Windows
mod vello;
mod vger;
#[cfg(all(feature = "webrender", not(windows)))]
// TODO Webrender support Windows
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
    #[cfg(all(feature = "vello", not(windows)))]
    #[command(name = "vello")]
    Vello,
    #[command(name = "vger")]
    Vger,
    #[cfg(all(feature = "webrender", not(windows)))]
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
            #[cfg(all(feature = "vello", not(windows)))]
            Commands::Vello => {
                vello::run();
            }
            Commands::Vger => {
                vger::run();
            }
            #[cfg(all(feature = "webrender", not(windows)))]
            Commands::Webrender => {
                webrender::run();
            }
        }
    }
}
