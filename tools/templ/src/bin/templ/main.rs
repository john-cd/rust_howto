#![allow(unused)]
use std::error::Error;

use templ::*;

mod cli;

pub fn main() -> Result<(), Box<dyn Error>> {
    let config = cli::get_args();
    if (config.verbose) {
        tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        //.with_target(false)
        .init();
    }
    if let Some(b) = config.badge {
        for name in b.names {
            create_badge(&name)?;
        }
    }
    Ok(())
}
