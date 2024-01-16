#![allow(unused)]

use std::collections::HashMap;
use std::path::Path;

use anyhow::Error;
use anyhow::Result;
use url::ParseError;
use url::Url;

mod args;
mod file;
mod link;
mod parser;
mod rules;

// TODO
fn main() -> Result<()> {
    let cli = args::parse_arguments();

    println!("{:?}", cli);

    match &cli.command {
        Some(args::Commands::Parse) => {
            // TODO parser::debug_parse_to_stdout(markdown_input);
        }
        None => {}
    }
    Ok(())
}
