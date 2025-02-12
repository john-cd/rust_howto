use std::fs::OpenOptions;
use std::io;
use std::process;
use std::sync::Mutex;

use clap::Arg;
use clap::ArgMatches;
use clap::Command;
use clap::crate_name;
use mdbook::errors::Error;
use mdbook::preprocess::CmdPreprocessor;
use mdbook::preprocess::Preprocessor;
use semver::Version;
use semver::VersionReq;

// Adapted from https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs

pub fn make_app() -> Command {
    Command::new(crate_name!())
        .about("A mdbook preprocessor which removes hidden sections and stops the inclusion of hidden chapters")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("mdbook-scrub.log")
        .unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .json()
        .with_writer(Mutex::new(log_file))
        .init();

    let preprocessor = mdbook_scrub::Preproc::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

// If the preprocessor supports the renderer, then mdbook runs it a second time,
// passing JSON data into stdin. The JSON consists of an array of [context,
// book] where context is the serialized object PreprocessorContext and book is
// a Book object containing the content of the book.
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    // The preprocessor should return the JSON format of the Book object to
    // stdout, with any modifications it wishes to perform.
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

// The first time it runs the preprocessor to determine if it supports the given
// renderer. mdBook passes two arguments to the process: the first argument is
// the string supports and the second argument is the renderer name.
// The preprocessor should exit with a status code 0 if it supports the given
// renderer, or return a non-zero exit code if it does not.
fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
